//! Checked band-cepstrum output shapes with staged caller buffers.
use super::{generated_result, range};
use crate::cepstrum::{self, CepstralOptions};
use rustfft::FftNum;
use std::panic::RefUnwindSafe;

#[derive(Clone, Copy)]
pub(super) enum Output {
    Selected,
    Legacy,
    Matrix { bands: usize },
}

pub(super) unsafe fn transform<T: FftNum + Into<f64> + RefUnwindSafe>(
    input: *const T,
    input_length: usize,
    order: usize,
    include_dc: u32,
    output: *mut T,
    output_length: usize,
    mode: Output,
) -> u32 {
    if let Err(status) = range(input, input_length) {
        return status;
    }
    let include_dc = match include_dc {
        0 => false,
        1 => true,
        _ => return 3,
    };
    let options = CepstralOptions { order, include_dc };
    let bands = match mode {
        Output::Selected | Output::Legacy => input_length,
        Output::Matrix { bands } => bands,
    };
    let count = match options.coefficient_count(bands) {
        Ok(count) => count,
        Err(_) => return 3,
    };
    let expected = match mode {
        Output::Selected => count,
        Output::Legacy => input_length,
        Output::Matrix { bands } => {
            if !input_length.is_multiple_of(bands) {
                return 3;
            }
            match (input_length / bands).checked_mul(count) {
                Some(length) => length,
                None => return 2,
            }
        }
    };
    if output_length != expected {
        return 2;
    }
    // SAFETY: Caller input is initialized and stable for computation; output is
    // exclusively writable. Checked ranges precede borrowing. Staged results
    // permit overlap because input borrowing ends before output commit.
    unsafe {
        generated_result(output, output_length, || {
            let input = if input_length == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, input_length)
            };
            match mode {
                Output::Selected => cepstrum::band_to_cepstrum(input, options),
                Output::Legacy => cepstrum::band_to_cepstrum_legacy(input, options),
                Output::Matrix { bands } => {
                    cepstrum::band_spectrogram_to_cepstra(input, bands, options)
                }
            }
        })
    }
}
