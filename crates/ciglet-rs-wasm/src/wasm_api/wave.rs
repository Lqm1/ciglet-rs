use crate::{wave, wave_write};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum WaveEncoding {
    Pcm = 0,
    Float = 1,
}

fn encoding(code: u32) -> Result<wave::Encoding, JsValue> {
    match code {
        0 => Ok(wave::Encoding::Pcm),
        1 => Ok(wave::Encoding::Float),
        _ => Err(JsValue::from_str("invalid WAVE encoding")),
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum WavePcm8Quantization {
    Centered = 0,
    Legacy = 1,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct WaveWriteOptions {
    options: wave_write::WriteOptions,
}

#[wasm_bindgen]
impl WaveWriteOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn legacy(bits_per_sample: u16) -> Self {
        Self {
            options: wave_write::WriteOptions::legacy(bits_per_sample),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn bits_per_sample(&self) -> u16 {
        self.options.bits_per_sample
    }
    #[wasm_bindgen(setter)]
    pub fn set_bits_per_sample(&mut self, value: u16) {
        self.options.bits_per_sample = value;
    }
    #[wasm_bindgen(getter)]
    pub fn pcm8_quantization(&self) -> u32 {
        match self.options.pcm8_quantization {
            wave_write::Pcm8Quantization::Centered => 0,
            wave_write::Pcm8Quantization::Legacy => 1,
        }
    }
    #[wasm_bindgen(setter)]
    pub fn set_pcm8_quantization(&mut self, code: u32) -> Result<(), JsValue> {
        self.options.pcm8_quantization = match code {
            0 => wave_write::Pcm8Quantization::Centered,
            1 => wave_write::Pcm8Quantization::Legacy,
            _ => return Err(JsValue::from_str("invalid PCM8 quantization")),
        };
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn pad_odd_data(&self) -> bool {
        self.options.pad_odd_data
    }
    #[wasm_bindgen(setter)]
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
#[wasm_bindgen]
pub struct WaveBytes {
    bytes: Vec<u8>,
    position: u64,
}
#[wasm_bindgen]
impl WaveBytes {
    #[wasm_bindgen(getter)]
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
    #[wasm_bindgen(getter)]
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

macro_rules! wave_api {
    ($scalar:ty, $owner:ident, $read:ident, $read_at:ident, $write:ident, $write_at:ident) => {
        #[wasm_bindgen]
        pub struct $owner {
            wave: wave::Wave<$scalar>,
            position: u64,
        }
        #[wasm_bindgen]
        impl $owner {
            #[wasm_bindgen(constructor)]
            pub fn new(
                sample_rate: u32,
                bits_per_sample: u16,
                channels: u16,
                encoding_code: u32,
                samples: Vec<$scalar>,
            ) -> Result<Self, JsValue> {
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
            #[wasm_bindgen(getter)]
            pub fn sample_rate(&self) -> u32 {
                self.wave.sample_rate
            }
            #[wasm_bindgen(setter)]
            pub fn set_sample_rate(&mut self, value: u32) {
                self.wave.sample_rate = value;
            }
            #[wasm_bindgen(getter)]
            pub fn bits_per_sample(&self) -> u16 {
                self.wave.bits_per_sample
            }
            #[wasm_bindgen(setter)]
            pub fn set_bits_per_sample(&mut self, value: u16) {
                self.wave.bits_per_sample = value;
            }
            #[wasm_bindgen(getter)]
            pub fn channels(&self) -> u16 {
                self.wave.channels
            }
            #[wasm_bindgen(setter)]
            pub fn set_channels(&mut self, value: u16) {
                self.wave.channels = value;
            }
            #[wasm_bindgen(getter)]
            pub fn encoding(&self) -> u32 {
                match self.wave.encoding {
                    wave::Encoding::Pcm => 0,
                    wave::Encoding::Float => 1,
                }
            }
            #[wasm_bindgen(setter)]
            pub fn set_encoding(&mut self, code: u32) -> Result<(), JsValue> {
                self.wave.encoding = encoding(code)?;
                Ok(())
            }
            #[wasm_bindgen(getter)]
            pub fn samples(&self) -> Vec<$scalar> {
                self.wave.samples.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_samples(&mut self, values: Vec<$scalar>) {
                self.wave.samples = values;
            }
            #[wasm_bindgen(getter)]
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
        #[wasm_bindgen]
        pub fn $read(bytes: &[u8], max_frames: usize) -> Result<$owner, JsValue> {
            $read_at(bytes, 0, max_frames)
        }
        #[wasm_bindgen]
        pub fn $read_at(bytes: &[u8], position: u64, max_frames: usize) -> Result<$owner, JsValue> {
            let mut cursor = Cursor::new(bytes);
            cursor.set_position(position);
            let wave = wave::read(&mut cursor, max_frames)
                .map_err(|error| JsValue::from_str(&error.to_string()))?;
            Ok($owner {
                wave,
                position: cursor.position(),
            })
        }
        #[wasm_bindgen]
        pub fn $write(
            samples: &[$scalar],
            sample_rate: u32,
            options: &WaveWriteOptions,
        ) -> Result<Vec<u8>, JsValue> {
            let mut bytes = Vec::new();
            wave_write::write_pcm(&mut bytes, samples, sample_rate, options.options)
                .map_err(|error| JsValue::from_str(&error.to_string()))?;
            Ok(bytes)
        }
        #[wasm_bindgen]
        pub fn $write_at(
            bytes: &[u8],
            position: u64,
            samples: &[$scalar],
            sample_rate: u32,
            options: &WaveWriteOptions,
        ) -> Result<WaveBytes, JsValue> {
            // Encode before extending the stream: native validation precedes output.
            let encoded = $write(samples, sample_rate, options)?;
            let start = usize::try_from(position)
                .map_err(|_| JsValue::from_str("WAVE stream offset cannot be represented"))?;
            let end = start
                .checked_add(encoded.len())
                .ok_or_else(|| JsValue::from_str("WAVE stream output size overflow"))?;
            let mut output = Vec::new();
            output
                .try_reserve_exact(bytes.len().max(end))
                .map_err(|_| JsValue::from_str("WAVE stream allocation failed"))?;
            output.extend_from_slice(bytes);
            output.resize(output.len().max(end), 0);
            output[start..end].copy_from_slice(&encoded);
            Ok(WaveBytes {
                bytes: output,
                position: end as u64,
            })
        }
    };
}
wave_api!(
    f32,
    WaveF32,
    wave_read_f32,
    wave_read_at_f32,
    wave_write_pcm_f32,
    wave_write_pcm_at_f32
);
wave_api!(
    f64,
    WaveF64,
    wave_read_f64,
    wave_read_at_f64,
    wave_write_pcm_f64,
    wave_write_pcm_at_f64
);
