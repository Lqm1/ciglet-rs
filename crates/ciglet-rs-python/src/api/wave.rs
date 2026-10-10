fn new_err_owned(message: &str) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(message.to_owned())
}
use crate::{wave, wave_write};
use pyo3::PyErr as BindingError;
use std::io::Cursor;
#[derive(Clone, Copy, PartialEq)]
#[pyo3::pyclass(eq, eq_int, from_py_object, module = "ciglet_rs")]
pub enum WaveEncoding {
    Pcm = 0,
    Float = 1,
}
fn encoding(code: u32) -> Result<wave::Encoding, BindingError> {
    match code {
        0 => Ok(wave::Encoding::Pcm),
        1 => Ok(wave::Encoding::Float),
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "invalid WAVE encoding",
        )),
    }
}
#[derive(Clone, Copy, PartialEq)]
#[pyo3::pyclass(eq, eq_int, from_py_object, module = "ciglet_rs")]
pub enum WavePcm8Quantization {
    Centered = 0,
    Legacy = 1,
}
#[derive(Default)]
pub struct WaveWriteOptions {
    options: wave_write::WriteOptions,
}
impl WaveWriteOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn legacy(bits_per_sample: u16) -> Self {
        Self {
            options: wave_write::WriteOptions::legacy(bits_per_sample),
        }
    }
    pub fn bits_per_sample(&self) -> u16 {
        self.options.bits_per_sample
    }
    pub fn set_bits_per_sample(&mut self, value: u16) {
        self.options.bits_per_sample = value;
    }
    pub fn pcm8_quantization(&self) -> u32 {
        match self.options.pcm8_quantization {
            wave_write::Pcm8Quantization::Centered => 0,
            wave_write::Pcm8Quantization::Legacy => 1,
        }
    }
    pub fn set_pcm8_quantization(&mut self, code: u32) -> Result<(), BindingError> {
        self.options.pcm8_quantization = match code {
            0 => wave_write::Pcm8Quantization::Centered,
            1 => wave_write::Pcm8Quantization::Legacy,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "invalid PCM8 quantization",
                ));
            }
        };
        Ok(())
    }
    pub fn pad_odd_data(&self) -> bool {
        self.options.pad_odd_data
    }
    pub fn set_pad_odd_data(&mut self, value: bool) {
        self.options.pad_odd_data = value;
    }
    pub fn cloned(&self) -> Self {
        Self {
            options: self.options,
        }
    }
}
/// Complete copied byte-stream output, including the final seek position.
pub struct WaveBytes {
    bytes: Vec<u8>,
    position: u64,
}
impl WaveBytes {
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
    pub fn cloned(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
            position: self.position,
        }
    }
}
pub struct WaveF32 {
    wave: wave::Wave<f32>,
    position: u64,
}
impl WaveF32 {
    pub fn new(
        sample_rate: u32,
        bits_per_sample: u16,
        channels: u16,
        encoding_code: u32,
        samples: Vec<f32>,
    ) -> Result<Self, BindingError> {
        Ok(Self {
            wave: wave::Wave {
                sample_rate,
                bits_per_sample,
                channels,
                encoding: encoding(encoding_code)?,
                samples,
            },
            position: 0,
        })
    }
    pub fn sample_rate(&self) -> u32 {
        self.wave.sample_rate
    }
    pub fn set_sample_rate(&mut self, value: u32) {
        self.wave.sample_rate = value;
    }
    pub fn bits_per_sample(&self) -> u16 {
        self.wave.bits_per_sample
    }
    pub fn set_bits_per_sample(&mut self, value: u16) {
        self.wave.bits_per_sample = value;
    }
    pub fn channels(&self) -> u16 {
        self.wave.channels
    }
    pub fn set_channels(&mut self, value: u16) {
        self.wave.channels = value;
    }
    pub fn encoding(&self) -> u32 {
        match self.wave.encoding {
            wave::Encoding::Pcm => 0,
            wave::Encoding::Float => 1,
        }
    }
    pub fn set_encoding(&mut self, code: u32) -> Result<(), BindingError> {
        self.wave.encoding = encoding(code)?;
        Ok(())
    }
    pub fn samples(&self) -> Vec<f32> {
        self.wave.samples.clone()
    }
    pub fn set_samples(&mut self, values: Vec<f32>) {
        self.wave.samples = values;
    }
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn cloned(&self) -> Self {
        Self {
            wave: self.wave.clone(),
            position: self.position,
        }
    }
}
pub fn wave_read_f32(bytes: &[u8], max_frames: usize) -> Result<WaveF32, BindingError> {
    wave_read_at_f32(bytes, 0, max_frames)
}
pub fn wave_read_at_f32(
    bytes: &[u8],
    position: u64,
    max_frames: usize,
) -> Result<WaveF32, BindingError> {
    let mut cursor = Cursor::new(bytes);
    cursor.set_position(position);
    let wave =
        wave::read(&mut cursor, max_frames).map_err(|error| new_err_owned(&error.to_string()))?;
    Ok(WaveF32 {
        wave,
        position: cursor.position(),
    })
}
pub fn wave_write_pcm_f32(
    samples: &[f32],
    sample_rate: u32,
    options: &WaveWriteOptions,
) -> Result<Vec<u8>, BindingError> {
    let mut bytes = Vec::new();
    wave_write::write_pcm(&mut bytes, samples, sample_rate, options.options)
        .map_err(|error| new_err_owned(&error.to_string()))?;
    Ok(bytes)
}
pub fn wave_write_pcm_at_f32(
    bytes: &[u8],
    position: u64,
    samples: &[f32],
    sample_rate: u32,
    options: &WaveWriteOptions,
) -> Result<WaveBytes, BindingError> {
    let encoded = wave_write_pcm_f32(samples, sample_rate, options)?;
    let start = usize::try_from(position).map_err(|_| {
        pyo3::exceptions::PyValueError::new_err("WAVE stream offset cannot be represented")
    })?;
    let end = start.checked_add(encoded.len()).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err("WAVE stream output size overflow")
    })?;
    let mut output = Vec::new();
    output
        .try_reserve_exact(bytes.len().max(end))
        .map_err(|_| pyo3::exceptions::PyValueError::new_err("WAVE stream allocation failed"))?;
    output.extend_from_slice(bytes);
    output.resize(output.len().max(end), 0);
    output[start..end].copy_from_slice(&encoded);
    Ok(WaveBytes {
        bytes: output,
        position: end as u64,
    })
}
pub struct WaveF64 {
    wave: wave::Wave<f64>,
    position: u64,
}
impl WaveF64 {
    pub fn new(
        sample_rate: u32,
        bits_per_sample: u16,
        channels: u16,
        encoding_code: u32,
        samples: Vec<f64>,
    ) -> Result<Self, BindingError> {
        Ok(Self {
            wave: wave::Wave {
                sample_rate,
                bits_per_sample,
                channels,
                encoding: encoding(encoding_code)?,
                samples,
            },
            position: 0,
        })
    }
    pub fn sample_rate(&self) -> u32 {
        self.wave.sample_rate
    }
    pub fn set_sample_rate(&mut self, value: u32) {
        self.wave.sample_rate = value;
    }
    pub fn bits_per_sample(&self) -> u16 {
        self.wave.bits_per_sample
    }
    pub fn set_bits_per_sample(&mut self, value: u16) {
        self.wave.bits_per_sample = value;
    }
    pub fn channels(&self) -> u16 {
        self.wave.channels
    }
    pub fn set_channels(&mut self, value: u16) {
        self.wave.channels = value;
    }
    pub fn encoding(&self) -> u32 {
        match self.wave.encoding {
            wave::Encoding::Pcm => 0,
            wave::Encoding::Float => 1,
        }
    }
    pub fn set_encoding(&mut self, code: u32) -> Result<(), BindingError> {
        self.wave.encoding = encoding(code)?;
        Ok(())
    }
    pub fn samples(&self) -> Vec<f64> {
        self.wave.samples.clone()
    }
    pub fn set_samples(&mut self, values: Vec<f64>) {
        self.wave.samples = values;
    }
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn cloned(&self) -> Self {
        Self {
            wave: self.wave.clone(),
            position: self.position,
        }
    }
}
pub fn wave_read_f64(bytes: &[u8], max_frames: usize) -> Result<WaveF64, BindingError> {
    wave_read_at_f64(bytes, 0, max_frames)
}
pub fn wave_read_at_f64(
    bytes: &[u8],
    position: u64,
    max_frames: usize,
) -> Result<WaveF64, BindingError> {
    let mut cursor = Cursor::new(bytes);
    cursor.set_position(position);
    let wave =
        wave::read(&mut cursor, max_frames).map_err(|error| new_err_owned(&error.to_string()))?;
    Ok(WaveF64 {
        wave,
        position: cursor.position(),
    })
}
pub fn wave_write_pcm_f64(
    samples: &[f64],
    sample_rate: u32,
    options: &WaveWriteOptions,
) -> Result<Vec<u8>, BindingError> {
    let mut bytes = Vec::new();
    wave_write::write_pcm(&mut bytes, samples, sample_rate, options.options)
        .map_err(|error| new_err_owned(&error.to_string()))?;
    Ok(bytes)
}
pub fn wave_write_pcm_at_f64(
    bytes: &[u8],
    position: u64,
    samples: &[f64],
    sample_rate: u32,
    options: &WaveWriteOptions,
) -> Result<WaveBytes, BindingError> {
    let encoded = wave_write_pcm_f64(samples, sample_rate, options)?;
    let start = usize::try_from(position).map_err(|_| {
        pyo3::exceptions::PyValueError::new_err("WAVE stream offset cannot be represented")
    })?;
    let end = start.checked_add(encoded.len()).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err("WAVE stream output size overflow")
    })?;
    let mut output = Vec::new();
    output
        .try_reserve_exact(bytes.len().max(end))
        .map_err(|_| pyo3::exceptions::PyValueError::new_err("WAVE stream allocation failed"))?;
    output.extend_from_slice(bytes);
    output.resize(output.len().max(end), 0);
    output[start..end].copy_from_slice(&encoded);
    Ok(WaveBytes {
        bytes: output,
        position: end as u64,
    })
}
