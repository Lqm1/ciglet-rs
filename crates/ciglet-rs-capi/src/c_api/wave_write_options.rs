//! Complete PCM write options, retaining native defaults and explicit legacy mode.
use super::scalar_value_result;
use crate::{
    Error,
    wave_write::{Pcm8Quantization, WriteOptions},
};
/// Centered unsigned PCM8 quantization; silence is 128.
pub const CIGLET_RS_PCM8_CENTERED: u32 = 0;
/// Original unsigned PCM8 quantization; silence is 127.
pub const CIGLET_RS_PCM8_LEGACY: u32 = 1;
/// Every native write option. Padding is checked zero/one, not a C bool layout.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CigletRsWaveWriteOptions {
    pub bits_per_sample: u16,
    pub pcm8_quantization: u32,
    pub pad_odd_data: u32,
}
impl From<WriteOptions> for CigletRsWaveWriteOptions {
    fn from(v: WriteOptions) -> Self {
        Self {
            bits_per_sample: v.bits_per_sample,
            pcm8_quantization: match v.pcm8_quantization {
                Pcm8Quantization::Centered => 0,
                Pcm8Quantization::Legacy => 1,
            },
            pad_odd_data: u32::from(v.pad_odd_data),
        }
    }
}
pub(super) fn options(v: CigletRsWaveWriteOptions) -> Result<WriteOptions, Error> {
    Ok(WriteOptions {
        bits_per_sample: v.bits_per_sample,
        pcm8_quantization: match v.pcm8_quantization {
            0 => Pcm8Quantization::Centered,
            1 => Pcm8Quantization::Legacy,
            _ => return Err(Error("unknown PCM8 quantization")),
        },
        pad_odd_data: match v.pad_odd_data {
            0 => false,
            1 => true,
            _ => return Err(Error("padding flag must be zero or one")),
        },
    })
}
/// Read native modern defaults: 16bit PCM, centered PCM8, odd-data padding.
///
/// # Safety
/// Output is aligned exclusively writable storage for one option descriptor.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_write_options_default(
    output: *mut CigletRsWaveWriteOptions,
) -> u32 {
    // SAFETY: Forward independent exclusively writable descriptor contract.
    unsafe { scalar_value_result(output, || Ok(WriteOptions::default().into())) }
}
/// Read native legacy options for the selected width. Encoding validates the width.
///
/// # Safety
/// Output is aligned exclusively writable storage for one option descriptor.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_write_options_legacy(
    bits_per_sample: u16,
    output: *mut CigletRsWaveWriteOptions,
) -> u32 {
    // SAFETY: Forward independent exclusively writable descriptor contract.
    unsafe { scalar_value_result(output, || Ok(WriteOptions::legacy(bits_per_sample).into())) }
}
