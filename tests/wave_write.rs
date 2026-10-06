use ciglet_rs::{
    wave,
    wave_write::{self, Pcm8Quantization, WriteOptions},
};
use rustfft::FftNum;
use std::io::{self, Cursor, Write};

struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn bytes(&mut self, count: usize) -> &[u8] {
        let (value, remaining) = self.0.split_at(count);
        self.0 = remaining;
        value
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.bytes(4).try_into().unwrap()) as usize
    }
    fn scalar(&mut self, precision: usize) -> f64 {
        match precision {
            4 => f32::from_le_bytes(self.bytes(4).try_into().unwrap()) as f64,
            8 => f64::from_le_bytes(self.bytes(8).try_into().unwrap()),
            _ => panic!("invalid precision"),
        }
    }
}

fn compare_c<T: FftNum + Into<f64>>(fixture: &[u8], precision: usize) {
    let mut reader = Reader(fixture);
    assert_eq!(reader.bytes(4), b"WVW1");
    assert_eq!(reader.integer(), precision);
    assert_eq!(reader.integer(), 12);
    for _ in 0..12 {
        let bits = reader.integer() as u16;
        let rate = reader.integer() as u32;
        let count = reader.integer();
        let samples: Vec<T> = (0..count)
            .map(|_| T::from_f64(reader.scalar(precision)).unwrap())
            .collect();
        let size = reader.integer();
        let expected = reader.bytes(size);
        let mut bytes = Vec::new();
        wave_write::write_pcm(&mut bytes, &samples, rate, WriteOptions::legacy(bits)).unwrap();
        assert_eq!(bytes, expected, "C WAVE output {bits}/{count}");
        let decoded = wave::read::<T, _>(&mut Cursor::new(&bytes), count).unwrap();
        assert_eq!(decoded.samples.len(), count);
        assert_eq!(decoded.sample_rate, rate);
    }
    assert!(reader.0.is_empty());
}

#[test]
fn float_legacy_pcm_bytes_match_c() {
    compare_c::<f32>(include_bytes!("fixtures/c-wave-write-f32.bin"), 4);
}
#[test]
fn double_legacy_pcm_bytes_match_c() {
    compare_c::<f64>(include_bytes!("fixtures/c-wave-write-f64.bin"), 8);
}

#[test]
fn centered_pcm8_padding_and_safe_clipping() {
    let mut bytes = Vec::new();
    wave_write::write_pcm(
        &mut bytes,
        &[-2.0f32, -1.0, 0.0, 1.0, 2.0],
        16000,
        WriteOptions {
            bits_per_sample: 8,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(&bytes[44..], [0, 0, 128, 255, 255, 0]);
    assert_eq!(u32::from_le_bytes(bytes[4..8].try_into().unwrap()), 42);
    assert_eq!(u32::from_le_bytes(bytes[40..44].try_into().unwrap()), 5);
    assert_eq!(
        wave::read::<f32, _>(&mut Cursor::new(&bytes), 5)
            .unwrap()
            .samples,
        [-1.0, -1.0, 0.0, 127.0 / 128.0, 127.0 / 128.0]
    );
    for bits in [16, 24, 32] {
        let mut bytes = Vec::new();
        wave_write::write_pcm(
            &mut bytes,
            &[-f32::MAX, -1.0, 0.0, 1.0, f32::MAX],
            16000,
            WriteOptions {
                bits_per_sample: bits,
                ..Default::default()
            },
        )
        .unwrap();
        let values = wave::read::<f64, _>(&mut Cursor::new(&bytes), 5)
            .unwrap()
            .samples;
        let scale = 2.0f64.powi(bits as i32 - 1);
        assert_eq!(values[0], -1.0);
        assert_eq!(values[2], 0.0);
        assert_eq!(values[3], (scale - 1.0) / scale);
        assert_eq!(values[4], values[3]);
    }
}

#[test]
fn invalid_inputs_leave_stream_and_existing_file_unchanged() {
    let original = b"unchanged".to_vec();
    for options in [
        WriteOptions {
            bits_per_sample: 0,
            ..Default::default()
        },
        WriteOptions {
            bits_per_sample: 12,
            ..Default::default()
        },
        WriteOptions {
            bits_per_sample: 64,
            ..Default::default()
        },
    ] {
        let mut bytes = original.clone();
        assert!(wave_write::write_pcm(&mut bytes, &[0.0f32], 16000, options).is_err());
        assert_eq!(bytes, original);
    }
    for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let mut bytes = original.clone();
        assert!(
            wave_write::write_pcm(&mut bytes, &[0.0, invalid], 16000, Default::default()).is_err()
        );
        assert_eq!(bytes, original);
    }
    let mut bytes = original.clone();
    assert!(wave_write::write_pcm(&mut bytes, &[0.0f32], 0, Default::default()).is_err());
    assert_eq!(bytes, original);
    let suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "ciglet-wave-write-{}-{suffix}.wav",
        std::process::id()
    ));
    std::fs::write(&path, &original).unwrap();
    let error =
        wave_write::write_pcm_file(&path, &[f32::NAN], 16000, Default::default()).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    assert_eq!(std::fs::read(&path).unwrap(), original);
    wave_write::write_pcm_file(&path, &[0.0f32, 0.5, -0.5], 16000, Default::default()).unwrap();
    let decoded = wave::read_file::<f32>(&path, 3).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(decoded.samples.len(), 3);
    assert_eq!(decoded.samples[0], 0.0);
}

struct Failing {
    remaining: usize,
}
impl Write for Failing {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.remaining == 0 {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "injected output failure",
            ));
        }
        let count = bytes.len().min(self.remaining);
        self.remaining -= count;
        Ok(count)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
#[test]
fn output_failures_and_compatibility_options() {
    for remaining in [0, 8, 44, 45] {
        assert_eq!(
            wave_write::write_pcm(
                &mut Failing { remaining },
                &[0.5f32; 3],
                16000,
                Default::default()
            )
            .unwrap_err()
            .kind(),
            io::ErrorKind::BrokenPipe
        );
    }
    let options = WriteOptions::legacy(8);
    assert_eq!(options.pcm8_quantization, Pcm8Quantization::Legacy);
    assert!(!options.pad_odd_data);
    let mut bytes = Vec::new();
    wave_write::write_pcm(&mut bytes, &[0.0f32], 16000, options).unwrap();
    assert_eq!(bytes.len(), 45);
    assert_eq!(bytes[44], 127);
}
