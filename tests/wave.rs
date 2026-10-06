use ciglet_rs::wave::{self, Encoding};
use rustfft::FftNum;
use std::io::{self, Cursor, Read, Seek, SeekFrom};

fn integer(bytes: &[u8]) -> usize {
    u32::from_le_bytes(bytes[..4].try_into().unwrap()) as usize
}
fn scalar(bytes: &[u8], precision: usize) -> f64 {
    match precision {
        4 => f32::from_le_bytes(bytes[..4].try_into().unwrap()) as f64,
        8 => f64::from_le_bytes(bytes[..8].try_into().unwrap()),
        _ => panic!("invalid precision"),
    }
}

fn compare_c<T: FftNum + Into<f64>>(fixture: &[u8], precision: usize) {
    assert_eq!(&fixture[..4], b"WAV1");
    assert_eq!(integer(&fixture[4..]), precision);
    assert_eq!(integer(&fixture[8..]), 36);
    let mut offset = 12;
    let mut unchanged = 0;
    let mut corrected = 0;
    for _ in 0..36 {
        let size = integer(&fixture[offset..]);
        offset += 4;
        let bytes = &fixture[offset..offset + size];
        offset += size;
        let decoded = wave::read::<T, _>(&mut Cursor::new(bytes), 7).unwrap();
        assert_eq!(decoded.sample_rate, 16000);
        assert_eq!(decoded.samples.len(), 7);
        let extensible = u16::from_le_bytes(bytes[20..22].try_into().unwrap()) == 65534;
        let scale = 2.0f64.powi(decoded.bits_per_sample as i32 - 1);
        let integer_samples = [
            -scale,
            -scale / 2.0,
            -1.0,
            0.0,
            1.0,
            scale / 4.0,
            scale - 1.0,
        ];
        let float_samples = [-1.25, -0.5, -0.0000001, 0.0, 0.25, 1.5, 1e-30];
        for (index, &actual) in decoded.samples.iter().enumerate() {
            let legacy = scalar(&fixture[offset..], precision);
            offset += precision;
            let expected = match decoded.encoding {
                Encoding::Pcm => integer_samples[index] / scale,
                Encoding::Float if decoded.bits_per_sample == 32 => {
                    float_samples[index] as f32 as f64
                }
                Encoding::Float => float_samples[index],
            };
            let expected = T::from_f64(expected).unwrap().into();
            assert_eq!(
                actual.into(),
                expected,
                "standard decode at {}/{:?}/{index}",
                decoded.bits_per_sample,
                decoded.encoding
            );
            let bug = decoded.encoding == Encoding::Pcm
                && (extensible
                    || decoded.bits_per_sample == 8
                    || (precision == 4 && decoded.bits_per_sample == 32));
            if !bug {
                assert_eq!(actual.into(), legacy);
                unchanged += 1;
            } else {
                if decoded.bits_per_sample == 8 && !extensible {
                    assert_eq!(legacy, actual.into() + 0.5);
                }
                if extensible {
                    assert_eq!(legacy, 0.0);
                }
                corrected += 1;
            }
        }
    }
    assert_eq!(offset, fixture.len());
    eprintln!(
        "C WAV {precision}-byte precision: {unchanged} unchanged and {corrected} deliberately corrected samples"
    );
}

#[test]
fn float_wave_reads_c_fixtures_with_disclosed_corrections() {
    compare_c::<f32>(include_bytes!("fixtures/c-wave-f32.bin"), 4);
}
#[test]
fn double_wave_reads_c_fixtures_with_disclosed_corrections() {
    compare_c::<f64>(include_bytes!("fixtures/c-wave-f64.bin"), 8);
}

fn basic_wave(bits: u16, data: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend(b"RIFF");
    bytes.extend((36u32 + data.len() as u32).to_le_bytes());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16u32.to_le_bytes());
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(16000u32.to_le_bytes());
    bytes.extend((16000u32 * (bits as u32 / 8)).to_le_bytes());
    bytes.extend((bits / 8).to_le_bytes());
    bytes.extend(bits.to_le_bytes());
    bytes.extend(b"data");
    bytes.extend((data.len() as u32).to_le_bytes());
    bytes.extend(data);
    bytes
}

#[test]
fn budgets_truncation_unknown_chunks_and_stream_offsets() {
    let bytes = basic_wave(16, &[0, 0, 0xff, 0x7f]);
    assert!(wave::read::<f32, _>(&mut Cursor::new(&bytes), 1).is_err());
    for length in 0..bytes.len() {
        assert!(wave::read::<f32, _>(&mut Cursor::new(&bytes[..length]), 2).is_err());
    }
    let mut with_chunk = bytes[..12].to_vec();
    with_chunk.extend(b"JUNK");
    with_chunk.extend(5u32.to_le_bytes());
    with_chunk.extend(b"dataX\0");
    with_chunk.extend(&bytes[12..]);
    let size = (with_chunk.len() - 8) as u32;
    with_chunk[4..8].copy_from_slice(&size.to_le_bytes());
    assert_eq!(
        wave::read::<f32, _>(&mut Cursor::new(&with_chunk), 2)
            .unwrap()
            .samples,
        [0.0, 32767.0 / 32768.0]
    );
    let mut embedded = b"prefix".to_vec();
    embedded.extend(&bytes);
    let mut cursor = Cursor::new(embedded);
    cursor.set_position(6);
    assert_eq!(
        wave::read::<f64, _>(&mut cursor, 2).unwrap().samples.len(),
        2
    );
    let empty = basic_wave(8, &[]);
    assert!(
        wave::read::<f32, _>(&mut Cursor::new(empty), 0)
            .unwrap()
            .samples
            .is_empty()
    );
    let odd = basic_wave(8, &[128]);
    assert_eq!(
        wave::read::<f32, _>(&mut Cursor::new(odd), 1)
            .unwrap()
            .samples,
        [0.0]
    );
    let wide = basic_wave(128, &i128::MIN.to_le_bytes());
    assert_eq!(
        wave::read::<f64, _>(&mut Cursor::new(wide), 1)
            .unwrap()
            .samples,
        [-1.0]
    );
    let partial = basic_wave(24, &[0]);
    assert!(wave::read::<f32, _>(&mut Cursor::new(partial), 1).is_err());
}

struct Broken(Cursor<Vec<u8>>);
impl Read for Broken {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "injected read error",
        ))
    }
}
impl Seek for Broken {
    fn seek(&mut self, from: SeekFrom) -> io::Result<u64> {
        self.0.seek(from)
    }
}
#[test]
fn io_errors_propagate_without_closing_borrowed_streams() {
    let mut reader = Broken(Cursor::new(basic_wave(16, &[0, 0])));
    assert_eq!(
        wave::read::<f32, _>(&mut reader, 1).unwrap_err().kind(),
        io::ErrorKind::BrokenPipe
    );
    assert_eq!(reader.seek(SeekFrom::Start(0)).unwrap(), 0);
}

#[test]
fn invalid_metadata_and_file_entry_point() {
    let bytes = basic_wave(16, &[0, 0]);
    for (offset, value) in [(20, 2), (22, 3), (24, 0), (32, 1), (34, 0)] {
        let mut bad = bytes.clone();
        bad[offset] = value;
        if offset == 24 {
            bad[25..28].fill(0);
        }
        assert!(wave::read::<f32, _>(&mut Cursor::new(bad), 1).is_err());
    }
    let mut duplicate = bytes.clone();
    duplicate.extend(&bytes[12..36]);
    let size = (duplicate.len() - 8) as u32;
    duplicate[4..8].copy_from_slice(&size.to_le_bytes());
    assert!(wave::read::<f32, _>(&mut Cursor::new(duplicate), 1).is_err());
    let suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path =
        std::env::temp_dir().join(format!("ciglet-wave-{}-{suffix}.wav", std::process::id()));
    std::fs::write(&path, &bytes).unwrap();
    let result = wave::read_file::<f32>(&path, 1);
    std::fs::remove_file(&path).unwrap();
    assert_eq!(result.unwrap().samples, [0.0]);
}
