//! Bounded RIFF/WAVE decoding, preserving the original left-channel convention.
use rustfft::FftNum;
use std::{
    fs::File,
    io::{self, BufReader, Read, Seek, SeekFrom},
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Pcm,
    Float,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wave<T> {
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub channels: u16,
    pub encoding: Encoding,
    /// Mono samples; stereo inputs retain the left channel, without downmixing.
    pub samples: Vec<T>,
}

#[derive(Debug, Clone, Copy)]
struct Format {
    sample_rate: u32,
    bits: u16,
    channels: u16,
    encoding: Encoding,
    block: u16,
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn word(bytes: &[u8]) -> u16 {
    u16::from_le_bytes(bytes[..2].try_into().unwrap())
}
fn integer(bytes: &[u8]) -> u32 {
    u32::from_le_bytes(bytes[..4].try_into().unwrap())
}

fn format(bytes: &[u8]) -> io::Result<Format> {
    if !matches!(bytes.len(), 16 | 18 | 40) {
        return Err(invalid("unsupported WAVE format chunk size"));
    }
    let mut tag = word(bytes);
    let channels = word(&bytes[2..]);
    let sample_rate = integer(&bytes[4..]);
    let block = word(&bytes[12..]);
    let bits = word(&bytes[14..]);
    if bytes.len() > 16 {
        let extra = word(&bytes[16..]) as usize;
        if extra + 18 != bytes.len() {
            return Err(invalid("invalid WAVE format extension size"));
        }
    }
    if tag == 0xfffe {
        if bytes.len() != 40 {
            return Err(invalid("truncated extensible WAVE format"));
        }
        const GUID_TAIL: [u8; 14] = [0, 0, 0, 0, 0x10, 0, 0x80, 0, 0, 0xaa, 0, 0x38, 0x9b, 0x71];
        if bytes[26..40] != GUID_TAIL {
            return Err(invalid("unsupported WAVE subformat GUID"));
        }
        tag = word(&bytes[24..]);
        let valid_bits = word(&bytes[18..]);
        if valid_bits == 0 || valid_bits > bits {
            return Err(invalid("invalid WAVE valid-bit count"));
        }
    }
    let encoding = match tag {
        1 => Encoding::Pcm,
        3 => Encoding::Float,
        _ => return Err(invalid("unsupported WAVE encoding")),
    };
    if !matches!(channels, 1 | 2) || sample_rate == 0 || sample_rate > i32::MAX as u32 {
        return Err(invalid("unsupported WAVE channels or sample rate"));
    }
    if bits == 0
        || !bits.is_multiple_of(8)
        || match encoding {
            Encoding::Pcm => bits > 128,
            Encoding::Float => !matches!(bits, 32 | 64),
        }
    {
        return Err(invalid("unsupported WAVE sample width"));
    }
    if block != channels * (bits / 8)
        || integer(&bytes[8..])
            != sample_rate
                .checked_mul(block as u32)
                .ok_or_else(|| invalid("WAVE byte rate overflow"))?
    {
        return Err(invalid("invalid WAVE block alignment or byte rate"));
    }
    Ok(Format {
        sample_rate,
        bits,
        channels,
        encoding,
        block,
    })
}

/// Decode one RIFF/WAVE stream from its current position. The frame budget is
/// checked before allocating samples. Unknown chunks are skipped by their
/// declared sizes, not by searching their payloads for magic strings.
pub fn read<T: FftNum, R: Read + Seek>(reader: &mut R, max_frames: usize) -> io::Result<Wave<T>> {
    let base = reader.stream_position()?;
    let header_end = base
        .checked_add(12)
        .ok_or_else(|| invalid("RIFF header offset overflow"))?;
    let file_end = reader.seek(SeekFrom::End(0))?;
    reader.seek(SeekFrom::Start(base))?;
    let mut header = [0u8; 12];
    reader.read_exact(&mut header)?;
    if &header[..4] != b"RIFF" || &header[8..] != b"WAVE" {
        return Err(invalid("expected RIFF/WAVE header"));
    }
    let end = base
        .checked_add(8)
        .and_then(|n| n.checked_add(integer(&header[4..]) as u64))
        .ok_or_else(|| invalid("RIFF size overflow"))?;
    if end > file_end || end < header_end {
        return Err(invalid("truncated RIFF container"));
    }
    let mut fmt = None;
    let mut data = None;
    let mut position = header_end;
    while position < end {
        if end - position < 8 {
            return Err(invalid("truncated RIFF chunk header"));
        }
        reader.seek(SeekFrom::Start(position))?;
        let mut chunk = [0u8; 8];
        reader.read_exact(&mut chunk)?;
        let length = integer(&chunk[4..]) as u64;
        let start = position + 8;
        let stop = start
            .checked_add(length)
            .filter(|&n| n <= end)
            .ok_or_else(|| invalid("truncated RIFF chunk"))?;
        match &chunk[..4] {
            b"fmt " => {
                if fmt.is_some() || !matches!(length, 16 | 18 | 40) {
                    return Err(invalid("duplicate or unsupported WAVE format chunk"));
                }
                let mut bytes = [0u8; 40];
                reader.read_exact(&mut bytes[..length as usize])?;
                fmt = Some(format(&bytes[..length as usize])?);
            }
            b"data" => {
                if data.replace((start, length)).is_some() {
                    return Err(invalid("multiple WAVE data chunks are unsupported"));
                }
            }
            _ => {}
        }
        // Upstream writers omit padding on an odd final data chunk. Accept it
        // only at the RIFF boundary; interior chunks still require alignment.
        position = if stop == end && &chunk[..4] == b"data" {
            stop
        } else {
            stop.checked_add(length & 1)
                .filter(|&n| n <= end)
                .ok_or_else(|| invalid("missing RIFF chunk padding"))?
        };
    }
    if position != end {
        return Err(invalid("missing RIFF chunk padding"));
    }
    let fmt = fmt.ok_or_else(|| invalid("missing WAVE format chunk"))?;
    let (start, length) = data.ok_or_else(|| invalid("missing WAVE data chunk"))?;
    if length % fmt.block as u64 != 0 {
        return Err(invalid("partial WAVE sample frame"));
    }
    let frames = usize::try_from(length / fmt.block as u64)
        .map_err(|_| invalid("WAVE frame count overflow"))?;
    if frames > max_frames || frames > i32::MAX as usize {
        return Err(invalid("WAVE frame budget exceeded"));
    }
    let mut samples = Vec::new();
    samples
        .try_reserve_exact(frames)
        .map_err(|_| io::Error::other("WAVE sample allocation failed"))?;
    reader.seek(SeekFrom::Start(start))?;
    let mut frame = [0u8; 32];
    for _ in 0..frames {
        reader.read_exact(&mut frame[..fmt.block as usize])?;
        let bytes = &frame[..(fmt.bits / 8) as usize];
        let value = match fmt.encoding {
            Encoding::Float if fmt.bits == 32 => {
                f32::from_le_bytes(bytes.try_into().unwrap()) as f64
            }
            Encoding::Float => f64::from_le_bytes(bytes.try_into().unwrap()),
            Encoding::Pcm if fmt.bits == 8 => (bytes[0] as f64 - 128.0) / 128.0,
            Encoding::Pcm => {
                let mut signed = [if bytes[bytes.len() - 1] & 0x80 != 0 {
                    0xff
                } else {
                    0
                }; 16];
                signed[..bytes.len()].copy_from_slice(bytes);
                i128::from_le_bytes(signed) as f64 / 2.0f64.powi(fmt.bits as i32 - 1)
            }
        };
        samples
            .push(T::from_f64(value).ok_or_else(|| invalid("WAVE sample cannot be represented"))?);
    }
    Ok(Wave {
        sample_rate: fmt.sample_rate,
        bits_per_sample: fmt.bits,
        channels: fmt.channels,
        encoding: fmt.encoding,
        samples,
    })
}

pub fn read_file<T: FftNum>(path: impl AsRef<Path>, max_frames: usize) -> io::Result<Wave<T>> {
    read(&mut BufReader::new(File::open(path)?), max_frames)
}
