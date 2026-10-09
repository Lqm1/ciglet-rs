//! Band-feature cepstra with upstream DCT scaling and DC placement.
use crate::{Error, cosine};
use rustfft::FftNum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CepstralOptions {
    /// Number of non-DC coefficients, clamped to band count minus one.
    pub order: usize,
    /// Append the original DC coefficient after the non-DC coefficients.
    pub include_dc: bool,
}
impl CepstralOptions {
    pub fn coefficient_count(self, bands: usize) -> Result<usize, Error> {
        if bands == 0 || bands > i32::MAX as usize {
            return Err(Error(
                "cepstral band count must be positive within the legacy range",
            ));
        }
        Ok(self.order.min(bands - 1) + usize::from(self.include_dc))
    }
}

/// Return the requested coefficients in original be2cc order. Input may be
/// signed log-band features; this function does not apply an additional log.
pub fn band_to_cepstrum<T: FftNum + Into<f64>>(
    bands: &[T],
    options: CepstralOptions,
) -> Result<Vec<T>, Error> {
    let count = options.coefficient_count(bands.len())?;
    let mut output = band_to_cepstrum_legacy(bands, options)?;
    output.truncate(count);
    Ok(output)
}

/// Preserve the complete original band-count-sized be2cc buffer, including
/// its untouched DCT suffix. Use band_to_cepstrum for the requested prefix.
pub fn band_to_cepstrum_legacy<T: FftNum + Into<f64>>(
    bands: &[T],
    options: CepstralOptions,
) -> Result<Vec<T>, Error> {
    options.coefficient_count(bands.len())?;
    if bands.iter().any(|&value| !value.into().is_finite()) {
        return Err(Error("cepstral band features must be finite"));
    }
    let mut output = cosine::dct(bands)?;
    let dc = output[0];
    let order = options.order.min(bands.len() - 1);
    output.copy_within(1..order + 1, 0);
    if options.include_dc {
        output[order] = dc;
    }
    Ok(output)
}

/// Transform a flattened frame-major band-feature matrix.
pub fn band_spectrogram_to_cepstra<T: FftNum + Into<f64>>(
    features: &[T],
    bands: usize,
    options: CepstralOptions,
) -> Result<Vec<T>, Error> {
    let count = options.coefficient_count(bands)?;
    if !features.len().is_multiple_of(bands) {
        return Err(Error("cepstral feature matrix shape differs"));
    }
    let length = (features.len() / bands)
        .checked_mul(count)
        .ok_or(Error("cepstral output size overflow"))?;
    let mut output = Vec::new();
    output
        .try_reserve_exact(length)
        .map_err(|_| Error("cepstral output allocation failed"))?;
    for frame in features.chunks_exact(bands) {
        output.extend(band_to_cepstrum(frame, options)?);
    }
    Ok(output)
}
