//! Mono PCM WAVE encoding with explicit legacy quantization compatibility.
use rustfft::FftNum;
use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Pcm8Quantization {
    /// Unsigned PCM silence is 128.
    #[default]
    Centered,
    /// Reproduce upstream's (sample + 1) * 127 truncation and clipping.
    Legacy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteOptions {
    pub bits_per_sample: u16,
    pub pcm8_quantization: Pcm8Quantization,
    pub pad_odd_data: bool,
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self {
            bits_per_sample: 16,
            pcm8_quantization: Pcm8Quantization::Centered,
            pad_odd_data: true,
        }
    }
}

impl WriteOptions {
    /// Reproduce original WAVE bytes for samples in C's defined conversion
    /// range. Out-of-range integer conversions are safely clipped in Rust.
    pub fn legacy(bits_per_sample: u16) -> Self {
        Self {
            bits_per_sample,
            pcm8_quantization: Pcm8Quantization::Legacy,
            pad_odd_data: false,
        }
    }
}

struct Layout {
    bytes: u16,
    data_size: u32,
    riff_size: u32,
    byte_rate: u32,
    padding: bool,
}

fn bad(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn validate<T: FftNum + Into<f64>>(
    samples: &[T],
    rate: u32,
    options: WriteOptions,
) -> io::Result<Layout> {
    if !matches!(options.bits_per_sample, 8 | 16 | 24 | 32) {
        return Err(bad("PCM output width must be 8, 16, 24, or 32 bits"));
    }
    if rate == 0 || rate > i32::MAX as u32 || samples.len() > i32::MAX as usize {
        return Err(bad("PCM output rate or length exceeds the legacy range"));
    }
    if samples.iter().any(|&sample| !sample.into().is_finite()) {
        return Err(bad("PCM output samples must be finite"));
    }
    let bytes = options.bits_per_sample / 8;
    let data_size = samples
        .len()
        .checked_mul(bytes as usize)
        .and_then(|n| u32::try_from(n).ok())
        .ok_or_else(|| bad("PCM data size overflow"))?;
    let padding = options.pad_odd_data && data_size & 1 != 0;
    let riff_size = data_size
        .checked_add(36)
        .and_then(|n| n.checked_add(u32::from(padding)))
        .ok_or_else(|| bad("RIFF output size overflow"))?;
    let byte_rate = rate
        .checked_mul(bytes as u32)
        .ok_or_else(|| bad("PCM output byte rate overflow"))?;
    Ok(Layout {
        bytes,
        data_size,
        riff_size,
        byte_rate,
        padding,
    })
}

fn encode<T: FftNum + Into<f64>, W: Write>(
    writer: &mut W,
    samples: &[T],
    rate: u32,
    options: WriteOptions,
    layout: Layout,
) -> io::Result<()> {
    let mut header = [0u8; 44];
    header[..4].copy_from_slice(b"RIFF");
    header[4..8].copy_from_slice(&layout.riff_size.to_le_bytes());
    header[8..16].copy_from_slice(b"WAVEfmt ");
    header[16..20].copy_from_slice(&16u32.to_le_bytes());
    header[20..22].copy_from_slice(&1u16.to_le_bytes());
    header[22..24].copy_from_slice(&1u16.to_le_bytes());
    header[24..28].copy_from_slice(&rate.to_le_bytes());
    header[28..32].copy_from_slice(&layout.byte_rate.to_le_bytes());
    header[32..34].copy_from_slice(&layout.bytes.to_le_bytes());
    header[34..36].copy_from_slice(&options.bits_per_sample.to_le_bytes());
    header[36..40].copy_from_slice(b"data");
    header[40..44].copy_from_slice(&layout.data_size.to_le_bytes());
    let maximum = (1u32 << (options.bits_per_sample - 1)) - 1;
    let scale = T::from_u32(maximum).ok_or_else(|| bad("PCM scale cannot be represented"))?;
    writer.write_all(&header)?;
    for &sample in samples {
        if layout.bytes == 1 {
            let value = match options.pcm8_quantization {
                Pcm8Quantization::Centered => sample.into() * 128.0 + 128.0,
                Pcm8Quantization::Legacy => (sample.into() + 1.0) * 127.0,
            };
            writer.write_all(&[value.clamp(0.0, 255.0) as u8])?;
        } else {
            // Signed upstream quantization multiplies in FP_TYPE, not double.
            // Clip before conversion: C's out-of-range float-to-int cast is UB.
            let value: f64 = (sample * scale).into();
            let value = value.clamp(-(maximum as f64) - 1.0, maximum as f64) as i32;
            writer.write_all(&value.to_le_bytes()[..layout.bytes as usize])?;
        }
    }
    if layout.padding {
        writer.write_all(&[0])?;
    }
    Ok(())
}

/// Write a mono PCM WAVE. All parameters are validated before writing bytes.
/// The caller retains ownership of the stream and is responsible for flushing.
pub fn write_pcm<T: FftNum + Into<f64>, W: Write>(
    writer: &mut W,
    samples: &[T],
    sample_rate: u32,
    options: WriteOptions,
) -> io::Result<()> {
    let layout = validate(samples, sample_rate, options)?;
    encode(writer, samples, sample_rate, options, layout)
}

/// Validate before creating/truncating the file, then explicitly flush so
/// buffered output failures are returned rather than lost during destruction.
pub fn write_pcm_file<T: FftNum + Into<f64>>(
    path: impl AsRef<Path>,
    samples: &[T],
    sample_rate: u32,
    options: WriteOptions,
) -> io::Result<()> {
    let layout = validate(samples, sample_rate, options)?;
    let mut writer = BufWriter::new(File::create(path)?);
    encode(&mut writer, samples, sample_rate, options, layout)?;
    writer.flush()
}
