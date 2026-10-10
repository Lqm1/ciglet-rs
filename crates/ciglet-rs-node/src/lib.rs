//! Node.js bindings to the native Rust implementation.
#[path = "bindings/binding_array2f32.rs"]
mod binding_array2f32;
pub use binding_array2f32::Array2F32;
#[path = "bindings/binding_array2f64.rs"]
mod binding_array2f64;
pub use binding_array2f64::Array2F64;
#[path = "bindings/binding_array2u16.rs"]
mod binding_array2u16;
pub use binding_array2u16::Array2U16;
#[path = "bindings/binding_array2u32.rs"]
mod binding_array2u32;
pub use binding_array2u32::Array2U32;
#[path = "bindings/binding_array2u64.rs"]
mod binding_array2u64;
pub use binding_array2u64::Array2U64;
#[path = "bindings/binding_array2u8.rs"]
mod binding_array2u8;
pub use binding_array2u8::Array2U8;
#[path = "bindings/binding_bandcepstraloptions.rs"]
mod binding_bandcepstraloptions;
pub use binding_bandcepstraloptions::BandCepstralOptions;
#[path = "bindings/binding_complexf32.rs"]
mod binding_complexf32;
pub use binding_complexf32::ComplexF32;
#[path = "bindings/binding_complexf64.rs"]
mod binding_complexf64;
pub use binding_complexf64::ComplexF64;
#[path = "bindings/binding_complexvaluesf32.rs"]
mod binding_complexvaluesf32;
pub use binding_complexvaluesf32::ComplexValuesF32;
#[path = "bindings/binding_complexvaluesf64.rs"]
mod binding_complexvaluesf64;
pub use binding_complexvaluesf64::ComplexValuesF64;
#[path = "bindings/binding_correlogramframe.rs"]
mod binding_correlogramframe;
pub use binding_correlogramframe::CorrelogramFrame;
#[path = "bindings/binding_cztplanf32.rs"]
mod binding_cztplanf32;
pub use binding_cztplanf32::CztPlanF32;
#[path = "bindings/binding_cztplanf64.rs"]
mod binding_cztplanf64;
pub use binding_cztplanf64::CztPlanF64;
#[path = "bindings/binding_fftplanf32.rs"]
mod binding_fftplanf32;
pub use binding_fftplanf32::FftPlanF32;
#[path = "bindings/binding_fftplanf64.rs"]
mod binding_fftplanf64;
pub use binding_fftplanf64::FftPlanF64;
#[path = "bindings/binding_figure.rs"]
mod binding_figure;
pub use binding_figure::Figure;
#[path = "bindings/binding_filterbankf32.rs"]
mod binding_filterbankf32;
pub use binding_filterbankf32::FilterBankF32;
#[path = "bindings/binding_filterbankf64.rs"]
mod binding_filterbankf64;
pub use binding_filterbankf64::FilterBankF64;
#[path = "bindings/binding_glottalmodelf32.rs"]
mod binding_glottalmodelf32;
pub use binding_glottalmodelf32::GlottalModelF32;
#[path = "bindings/binding_glottalmodelf64.rs"]
mod binding_glottalmodelf64;
pub use binding_glottalmodelf64::GlottalModelF64;
#[path = "bindings/binding_glottalpreparedf32.rs"]
mod binding_glottalpreparedf32;
pub use binding_glottalpreparedf32::GlottalPreparedF32;
#[path = "bindings/binding_glottalpreparedf64.rs"]
mod binding_glottalpreparedf64;
pub use binding_glottalpreparedf64::GlottalPreparedF64;
#[path = "bindings/binding_glottalspectrumf32.rs"]
mod binding_glottalspectrumf32;
pub use binding_glottalspectrumf32::GlottalSpectrumF32;
#[path = "bindings/binding_glottalspectrumf64.rs"]
mod binding_glottalspectrumf64;
pub use binding_glottalspectrumf64::GlottalSpectrumF64;
#[path = "bindings/binding_instantaneousfrequencydetectorf32.rs"]
mod binding_instantaneousfrequencydetectorf32;
pub use binding_instantaneousfrequencydetectorf32::InstantaneousFrequencyDetectorF32;
#[path = "bindings/binding_instantaneousfrequencydetectorf64.rs"]
mod binding_instantaneousfrequencydetectorf64;
pub use binding_instantaneousfrequencydetectorf64::InstantaneousFrequencyDetectorF64;
#[path = "bindings/binding_instantaneousfrequencykernelsf32.rs"]
mod binding_instantaneousfrequencykernelsf32;
pub use binding_instantaneousfrequencykernelsf32::InstantaneousFrequencyKernelsF32;
#[path = "bindings/binding_instantaneousfrequencykernelsf64.rs"]
mod binding_instantaneousfrequencykernelsf64;
pub use binding_instantaneousfrequencykernelsf64::InstantaneousFrequencyKernelsF64;
#[path = "bindings/binding_kalmanestimatef32.rs"]
mod binding_kalmanestimatef32;
pub use binding_kalmanestimatef32::KalmanEstimateF32;
#[path = "bindings/binding_kalmanestimatef64.rs"]
mod binding_kalmanestimatef64;
pub use binding_kalmanestimatef64::KalmanEstimateF64;
#[path = "bindings/binding_luf32.rs"]
mod binding_luf32;
pub use binding_luf32::LuF32;
#[path = "bindings/binding_luf64.rs"]
mod binding_luf64;
pub use binding_luf64::LuF64;
#[path = "bindings/binding_matrixf32.rs"]
mod binding_matrixf32;
pub use binding_matrixf32::MatrixF32;
#[path = "bindings/binding_matrixf64.rs"]
mod binding_matrixf64;
pub use binding_matrixf64::MatrixF64;
#[path = "bindings/binding_meloptionsf32.rs"]
mod binding_meloptionsf32;
pub use binding_meloptionsf32::MelOptionsF32;
#[path = "bindings/binding_meloptionsf64.rs"]
mod binding_meloptionsf64;
pub use binding_meloptionsf64::MelOptionsF64;
#[path = "bindings/binding_plotcommands.rs"]
mod binding_plotcommands;
pub use binding_plotcommands::PlotCommands;
#[path = "bindings/binding_polynomialrootsf32.rs"]
mod binding_polynomialrootsf32;
pub use binding_polynomialrootsf32::PolynomialRootsF32;
#[path = "bindings/binding_polynomialrootsf64.rs"]
mod binding_polynomialrootsf64;
pub use binding_polynomialrootsf64::PolynomialRootsF64;
#[path = "bindings/binding_predictioncorrelationparts.rs"]
mod binding_predictioncorrelationparts;
pub use binding_predictioncorrelationparts::PredictionCorrelationParts;
#[path = "bindings/binding_predictionf32.rs"]
mod binding_predictionf32;
pub use binding_predictionf32::PredictionF32;
#[path = "bindings/binding_predictionf64.rs"]
mod binding_predictionf64;
pub use binding_predictionf64::PredictionF64;
#[path = "bindings/binding_predictiongainparts.rs"]
mod binding_predictiongainparts;
pub use binding_predictiongainparts::PredictionGainParts;
#[path = "bindings/binding_predictionresonanceestimatef32.rs"]
mod binding_predictionresonanceestimatef32;
pub use binding_predictionresonanceestimatef32::PredictionResonanceEstimateF32;
#[path = "bindings/binding_predictionresonanceestimatef64.rs"]
mod binding_predictionresonanceestimatef64;
pub use binding_predictionresonanceestimatef64::PredictionResonanceEstimateF64;
#[path = "bindings/binding_predictionresonancef32.rs"]
mod binding_predictionresonancef32;
pub use binding_predictionresonancef32::PredictionResonanceF32;
#[path = "bindings/binding_predictionresonancef64.rs"]
mod binding_predictionresonancef64;
pub use binding_predictionresonancef64::PredictionResonanceF64;
#[path = "bindings/binding_predictionspectrumplanf32.rs"]
mod binding_predictionspectrumplanf32;
pub use binding_predictionspectrumplanf32::PredictionSpectrumPlanF32;
#[path = "bindings/binding_predictionspectrumplanf64.rs"]
mod binding_predictionspectrumplanf64;
pub use binding_predictionspectrumplanf64::PredictionSpectrumPlanF64;
#[path = "bindings/binding_processoutput.rs"]
mod binding_processoutput;
pub use binding_processoutput::ProcessOutput;
#[path = "bindings/binding_randomsampler.rs"]
mod binding_randomsampler;
pub use binding_randomsampler::RandomSampler;
#[path = "bindings/binding_randomwordsource.rs"]
mod binding_randomwordsource;
pub use binding_randomwordsource::RandomWordSource;
#[path = "bindings/binding_realcepstrumplanf32.rs"]
mod binding_realcepstrumplanf32;
pub use binding_realcepstrumplanf32::RealCepstrumPlanF32;
#[path = "bindings/binding_realcepstrumplanf64.rs"]
mod binding_realcepstrumplanf64;
pub use binding_realcepstrumplanf64::RealCepstrumPlanF64;
#[path = "bindings/binding_scaledpredictionf32.rs"]
mod binding_scaledpredictionf32;
pub use binding_scaledpredictionf32::ScaledPredictionF32;
#[path = "bindings/binding_scaledpredictionf64.rs"]
mod binding_scaledpredictionf64;
pub use binding_scaledpredictionf64::ScaledPredictionF64;
#[path = "bindings/binding_sinusoidf32.rs"]
mod binding_sinusoidf32;
pub use binding_sinusoidf32::SinusoidF32;
#[path = "bindings/binding_sinusoidf64.rs"]
mod binding_sinusoidf64;
pub use binding_sinusoidf64::SinusoidF64;
#[path = "bindings/binding_sortedvaluesf32.rs"]
mod binding_sortedvaluesf32;
pub use binding_sortedvaluesf32::SortedValuesF32;
#[path = "bindings/binding_sortedvaluesf64.rs"]
mod binding_sortedvaluesf64;
pub use binding_sortedvaluesf64::SortedValuesF64;
#[path = "bindings/binding_spectralenvelopef32.rs"]
mod binding_spectralenvelopef32;
pub use binding_spectralenvelopef32::SpectralEnvelopeF32;
#[path = "bindings/binding_spectralenvelopef64.rs"]
mod binding_spectralenvelopef64;
pub use binding_spectralenvelopef64::SpectralEnvelopeF64;
#[path = "bindings/binding_spectralenvelopeplanf32.rs"]
mod binding_spectralenvelopeplanf32;
pub use binding_spectralenvelopeplanf32::SpectralEnvelopePlanF32;
#[path = "bindings/binding_spectralenvelopeplanf64.rs"]
mod binding_spectralenvelopeplanf64;
pub use binding_spectralenvelopeplanf64::SpectralEnvelopePlanF64;
#[path = "bindings/binding_spectralpeakf32.rs"]
mod binding_spectralpeakf32;
pub use binding_spectralpeakf32::SpectralPeakF32;
#[path = "bindings/binding_spectralpeakf64.rs"]
mod binding_spectralpeakf64;
pub use binding_spectralpeakf64::SpectralPeakF64;
#[path = "bindings/binding_stftanalysisf32.rs"]
mod binding_stftanalysisf32;
pub use binding_stftanalysisf32::StftAnalysisF32;
#[path = "bindings/binding_stftanalysisf64.rs"]
mod binding_stftanalysisf64;
pub use binding_stftanalysisf64::StftAnalysisF64;
#[path = "bindings/binding_stftanalysisoptions.rs"]
mod binding_stftanalysisoptions;
pub use binding_stftanalysisoptions::StftAnalysisOptions;
#[path = "bindings/binding_stftframegrid.rs"]
mod binding_stftframegrid;
pub use binding_stftframegrid::StftFrameGrid;
#[path = "bindings/binding_stftplanf32.rs"]
mod binding_stftplanf32;
pub use binding_stftplanf32::StftPlanF32;
#[path = "bindings/binding_stftplanf64.rs"]
mod binding_stftplanf64;
pub use binding_stftplanf64::StftPlanF64;
#[path = "bindings/binding_stftsynthesisoptionsf32.rs"]
mod binding_stftsynthesisoptionsf32;
pub use binding_stftsynthesisoptionsf32::StftSynthesisOptionsF32;
#[path = "bindings/binding_stftsynthesisoptionsf64.rs"]
mod binding_stftsynthesisoptionsf64;
pub use binding_stftsynthesisoptionsf64::StftSynthesisOptionsF64;
#[path = "bindings/binding_wavebytes.rs"]
mod binding_wavebytes;
pub use binding_wavebytes::WaveBytes;
#[path = "bindings/binding_wavef32.rs"]
mod binding_wavef32;
pub use binding_wavef32::WaveF32;
#[path = "bindings/binding_wavef64.rs"]
mod binding_wavef64;
pub use binding_wavef64::WaveF64;
#[path = "bindings/binding_wavewriteoptions.rs"]
mod binding_wavewriteoptions;
pub use binding_wavewriteoptions::WaveWriteOptions;
#[path = "bindings/binding_functions_aliased.rs"]
mod binding_functions_aliased;
pub use binding_functions_aliased::*;
#[path = "bindings/binding_functions_approximation.rs"]
mod binding_functions_approximation;
pub use binding_functions_approximation::*;
#[path = "bindings/binding_functions_argument.rs"]
mod binding_functions_argument;
pub use binding_functions_argument::*;
#[path = "bindings/binding_functions_band.rs"]
mod binding_functions_band;
pub use binding_functions_band::*;
#[path = "bindings/binding_functions_bandpass.rs"]
mod binding_functions_bandpass;
pub use binding_functions_bandpass::*;
#[path = "bindings/binding_functions_bark.rs"]
mod binding_functions_bark;
pub use binding_functions_bark::*;
#[path = "bindings/binding_functions_complete.rs"]
mod binding_functions_complete;
pub use binding_functions_complete::*;
#[path = "bindings/binding_functions_convolve.rs"]
mod binding_functions_convolve;
pub use binding_functions_convolve::*;
#[path = "bindings/binding_functions_correlation.rs"]
mod binding_functions_correlation;
pub use binding_functions_correlation::*;
#[path = "bindings/binding_functions_correlogram.rs"]
mod binding_functions_correlogram;
pub use binding_functions_correlogram::*;
#[path = "bindings/binding_functions_covariance.rs"]
mod binding_functions_covariance;
pub use binding_functions_covariance::*;
#[path = "bindings/binding_functions_cross.rs"]
mod binding_functions_cross;
pub use binding_functions_cross::*;
#[path = "bindings/binding_functions_cumulative.rs"]
mod binding_functions_cumulative;
pub use binding_functions_cumulative::*;
#[path = "bindings/binding_functions_czt.rs"]
mod binding_functions_czt;
pub use binding_functions_czt::*;
#[path = "bindings/binding_functions_dct.rs"]
mod binding_functions_dct;
pub use binding_functions_dct::*;
#[path = "bindings/binding_functions_difference.rs"]
mod binding_functions_difference;
pub use binding_functions_difference::*;
#[path = "bindings/binding_functions_dot.rs"]
mod binding_functions_dot;
pub use binding_functions_dot::*;
#[path = "bindings/binding_functions_equal.rs"]
mod binding_functions_equal;
pub use binding_functions_equal::*;
#[path = "bindings/binding_functions_extremum.rs"]
mod binding_functions_extremum;
pub use binding_functions_extremum::*;
#[path = "bindings/binding_functions_fetch.rs"]
mod binding_functions_fetch;
pub use binding_functions_fetch::*;
#[path = "bindings/binding_functions_fft.rs"]
mod binding_functions_fft;
pub use binding_functions_fft::*;
#[path = "bindings/binding_functions_filter.rs"]
mod binding_functions_filter;
pub use binding_functions_filter::*;
#[path = "bindings/binding_functions_fir.rs"]
mod binding_functions_fir;
pub use binding_functions_fir::*;
#[path = "bindings/binding_functions_forward.rs"]
mod binding_functions_forward;
pub use binding_functions_forward::*;
#[path = "bindings/binding_functions_frequency.rs"]
mod binding_functions_frequency;
pub use binding_functions_frequency::*;
#[path = "bindings/binding_functions_hyperbolic.rs"]
mod binding_functions_hyperbolic;
pub use binding_functions_hyperbolic::*;
#[path = "bindings/binding_functions_integer.rs"]
mod binding_functions_integer;
pub use binding_functions_integer::*;
#[path = "bindings/binding_functions_interpolate.rs"]
mod binding_functions_interpolate;
pub use binding_functions_interpolate::*;
#[path = "bindings/binding_functions_inverse.rs"]
mod binding_functions_inverse;
pub use binding_functions_inverse::*;
#[path = "bindings/binding_functions_itakura.rs"]
mod binding_functions_itakura;
pub use binding_functions_itakura::*;
#[path = "bindings/binding_functions_kalman.rs"]
mod binding_functions_kalman;
pub use binding_functions_kalman::*;
#[path = "bindings/binding_functions_lanes.rs"]
mod binding_functions_lanes;
pub use binding_functions_lanes::*;
#[path = "bindings/binding_functions_legacy.rs"]
mod binding_functions_legacy;
pub use binding_functions_legacy::*;
#[path = "bindings/binding_functions_linear.rs"]
mod binding_functions_linear;
pub use binding_functions_linear::*;
#[path = "bindings/binding_functions_log.rs"]
mod binding_functions_log;
pub use binding_functions_log::*;
#[path = "bindings/binding_functions_magnitude.rs"]
mod binding_functions_magnitude;
pub use binding_functions_magnitude::*;
#[path = "bindings/binding_functions_matrix.rs"]
mod binding_functions_matrix;
pub use binding_functions_matrix::*;
#[path = "bindings/binding_functions_maximum.rs"]
mod binding_functions_maximum;
pub use binding_functions_maximum::*;
#[path = "bindings/binding_functions_mean.rs"]
mod binding_functions_mean;
pub use binding_functions_mean::*;
#[path = "bindings/binding_functions_median.rs"]
mod binding_functions_median;
pub use binding_functions_median::*;
#[path = "bindings/binding_functions_mel.rs"]
mod binding_functions_mel;
pub use binding_functions_mel::*;
#[path = "bindings/binding_functions_minimum.rs"]
mod binding_functions_minimum;
pub use binding_functions_minimum::*;
#[path = "bindings/binding_functions_moving.rs"]
mod binding_functions_moving;
pub use binding_functions_moving::*;
#[path = "bindings/binding_functions_peak.rs"]
mod binding_functions_peak;
pub use binding_functions_peak::*;
#[path = "bindings/binding_functions_phase.rs"]
mod binding_functions_phase;
pub use binding_functions_phase::*;
#[path = "bindings/binding_functions_plot.rs"]
mod binding_functions_plot;
pub use binding_functions_plot::*;
#[path = "bindings/binding_functions_polar.rs"]
mod binding_functions_polar;
pub use binding_functions_polar::*;
#[path = "bindings/binding_functions_polynomial.rs"]
mod binding_functions_polynomial;
pub use binding_functions_polynomial::*;
#[path = "bindings/binding_functions_prediction.rs"]
mod binding_functions_prediction;
pub use binding_functions_prediction::*;
#[path = "bindings/binding_functions_random.rs"]
mod binding_functions_random;
pub use binding_functions_random::*;
#[path = "bindings/binding_functions_real.rs"]
mod binding_functions_real;
pub use binding_functions_real::*;
#[path = "bindings/binding_functions_resample.rs"]
mod binding_functions_resample;
pub use binding_functions_resample::*;
#[path = "bindings/binding_functions_reverse.rs"]
mod binding_functions_reverse;
pub use binding_functions_reverse::*;
#[path = "bindings/binding_functions_root.rs"]
mod binding_functions_root;
pub use binding_functions_root::*;
#[path = "bindings/binding_functions_scalar.rs"]
mod binding_functions_scalar;
pub use binding_functions_scalar::*;
#[path = "bindings/binding_functions_select.rs"]
mod binding_functions_select;
pub use binding_functions_select::*;
#[path = "bindings/binding_functions_sinusoid.rs"]
mod binding_functions_sinusoid;
pub use binding_functions_sinusoid::*;
#[path = "bindings/binding_functions_sinusoids.rs"]
mod binding_functions_sinusoids;
pub use binding_functions_sinusoids::*;
#[path = "bindings/binding_functions_sort.rs"]
mod binding_functions_sort;
pub use binding_functions_sort::*;
#[path = "bindings/binding_functions_special.rs"]
mod binding_functions_special;
pub use binding_functions_special::*;
#[path = "bindings/binding_functions_spectral.rs"]
mod binding_functions_spectral;
pub use binding_functions_spectral::*;
#[path = "bindings/binding_functions_stft.rs"]
mod binding_functions_stft;
pub use binding_functions_stft::*;
#[path = "bindings/binding_functions_sum.rs"]
mod binding_functions_sum;
pub use binding_functions_sum::*;
#[path = "bindings/binding_functions_trigonometric.rs"]
mod binding_functions_trigonometric;
pub use binding_functions_trigonometric::*;
#[path = "bindings/binding_functions_variance.rs"]
mod binding_functions_variance;
pub use binding_functions_variance::*;
#[path = "bindings/binding_functions_wave.rs"]
mod binding_functions_wave;
pub use binding_functions_wave::*;
#[path = "bindings/binding_functions_window.rs"]
mod binding_functions_window;
pub use binding_functions_window::*;
#[path = "bindings/binding_functions_windowed.rs"]
mod binding_functions_windowed;
pub use binding_functions_windowed::*;
pub use ciglet_rs::*;
use napi_derive::napi;
mod api;
mod callbacks;
fn borrowed(_: impl std::fmt::Display) -> napi::Error {
    napi::Error::from_reason("owner is already borrowed by an active call")
}
fn consumed() -> napi::Error {
    napi::Error::from_reason("owner has been consumed")
}
fn checked_usize(value: f64) -> napi::Result<usize> {
    if !value.is_finite()
        || value < 0.0
        || value.fract() != 0.0
        || value > 9007199254740991.0
        || value >= ((usize::MAX as f64) + 1.0)
    {
        Err(napi::Error::from_reason(
            "integer is outside the safe usize range",
        ))
    } else {
        Ok(value as usize)
    }
}
fn checked_i64(value: &napi::bindgen_prelude::BigInt) -> napi::Result<i64> {
    let (value, lossless) = value.get_i64();
    if lossless {
        Ok(value)
    } else {
        Err(napi::Error::from_reason("integer is outside i64 range"))
    }
}
fn checked_u64(value: &napi::bindgen_prelude::BigInt) -> napi::Result<u64> {
    let (negative, value, lossless) = value.get_u64();
    if negative || !lossless {
        Err(napi::Error::from_reason("integer is outside u64 range"))
    } else {
        Ok(value)
    }
}
#[napi]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
pub use crate::api::ApproximationPrecision;
pub use crate::api::BoundaryPolicy;
pub use crate::api::CepstralLifter;
pub use crate::api::CorrelogramMethod;
pub use crate::api::CorrelogramPolicy;
pub use crate::api::DistancePolicy;
pub use crate::api::ExtremumDirection;
pub use crate::api::FftDirection;
pub use crate::api::FilterBankCompression;
pub use crate::api::FilterPassband;
pub use crate::api::FilterStartup;
pub use crate::api::InstantaneousFrequencyPolicy;
pub use crate::api::KernelPolicy;
pub use crate::api::PlotColor;
pub use crate::api::PredictionArithmetic;
pub use crate::api::RandomPolicy;
pub use crate::api::SincPolicy;
pub use crate::api::SmoothingPolicy;
pub use crate::api::SpectrumNyquist;
pub use crate::api::SpectrumSymmetry;
pub use crate::api::StatisticsPolicy;
pub use crate::api::WaveEncoding;
pub use crate::api::WavePcm8Quantization;
pub use crate::api::WindowKind;
pub use crate::api::WindowPrecision;
