import { verifyApproximationBindings } from './wasm_approximation_checks.mjs';
import { verifyArrayBindings } from './wasm_array_checks.mjs';
import { verifyBandCepstrumBindings } from './wasm_cepstrum_checks.mjs';
import { verifyComplexBindings } from './wasm_complex_checks.mjs';
import { verifyCorrelogramBindings } from './wasm_correlogram_checks.mjs';
import { verifyCztBindings } from './wasm_czt_checks.mjs';
import { verifyEnvelopeBindings } from './wasm_envelope_checks.mjs';
import { verifyFilterBankBindings } from './wasm_filterbank_checks.mjs';
import { verifyFilteringBindings } from './wasm_filtering_checks.mjs';
import { verifyFourierBindings } from './wasm_fourier_checks.mjs';
import { verifyFrameExtremaBindings } from './wasm_frame_extrema_checks.mjs';
import { verifyFrequencyDistanceBindings } from './wasm_frequency_distance_checks.mjs';
import { verifyGlottalBindings } from './wasm_glottal_checks.mjs';
import { verifyHyperbolicBindings } from './wasm_hyperbolic_checks.mjs';
import { verifyInstantaneousFrequencyBindings } from './wasm_instantaneous_frequency_checks.mjs';
import { verifyInterpolationBindings } from './wasm_interpolation_checks.mjs';
import { verifyKalmanBindings } from './wasm_kalman_checks.mjs';
import { verifyLaneBindings } from './wasm_lanes_checks.mjs';
import { verifyMatrixBindings } from './wasm_matrix_checks.mjs';
import { verifyOrderingBindings } from './wasm_ordering_checks.mjs';
import { verifyPhaseBindings } from './wasm_phase_checks.mjs';
import { verifyPlottingBindings } from './wasm_plotting_checks.mjs';
import { verifyPolynomialBindings } from './wasm_polynomial_checks.mjs';
import { verifyPredictionBindings } from './wasm_prediction_checks.mjs';
import { verifyRandomBindings } from './wasm_random_checks.mjs';
import { verifyRealCepstrumBindings } from './wasm_real_cepstrum_checks.mjs';
import { verifyResamplingBindings } from './wasm_resampling_checks.mjs';
import { verifyRootBindings } from './wasm_root_checks.mjs';
import { verifyScalarBindings } from './wasm_scalar_checks.mjs';
import { verifySincBindings } from './wasm_sinc_checks.mjs';
import { verifySmoothingBindings } from './wasm_smoothing_checks.mjs';
import { verifySpecialBindings } from './wasm_special_checks.mjs';
import { verifySpectralPeakBindings } from './wasm_spectral_peak_checks.mjs';
import { verifyStatisticsBindings } from './wasm_statistics_checks.mjs';
import { verifyStftBindings } from './wasm_stft_checks.mjs';
import { verifySynthesisBindings } from './wasm_synthesis_checks.mjs';
import { verifyTrigonometricBindings } from './wasm_trigonometric_checks.mjs';
import { verifyVectorBindings } from './wasm_vector_checks.mjs';
import { verifyWaveBindings } from './wasm_wave_checks.mjs';
import { verifyWindowBindings } from './wasm_window_checks.mjs';

export const bindingFamilies = [
    ['approximation', verifyApproximationBindings],
    ['array', verifyArrayBindings],
    ['cepstrum', verifyBandCepstrumBindings],
    ['complex', verifyComplexBindings],
    ['correlogram', verifyCorrelogramBindings],
    ['czt', verifyCztBindings],
    ['envelope', verifyEnvelopeBindings],
    ['filterbank', verifyFilterBankBindings],
    ['filtering', verifyFilteringBindings],
    ['fourier', verifyFourierBindings],
    ['frame_extrema', verifyFrameExtremaBindings],
    ['frequency_distance', verifyFrequencyDistanceBindings],
    ['glottal', verifyGlottalBindings],
    ['hyperbolic', verifyHyperbolicBindings],
    ['instantaneous_frequency', verifyInstantaneousFrequencyBindings],
    ['interpolation', verifyInterpolationBindings],
    ['kalman', verifyKalmanBindings],
    ['lanes', verifyLaneBindings],
    ['matrix', verifyMatrixBindings],
    ['ordering', verifyOrderingBindings],
    ['phase', verifyPhaseBindings],
    ['plotting', verifyPlottingBindings],
    ['polynomial', verifyPolynomialBindings],
    ['prediction', verifyPredictionBindings],
    ['random', verifyRandomBindings],
    ['real_cepstrum', verifyRealCepstrumBindings],
    ['resampling', verifyResamplingBindings],
    ['root', verifyRootBindings],
    ['scalar', verifyScalarBindings],
    ['sinc', verifySincBindings],
    ['smoothing', verifySmoothingBindings],
    ['special', verifySpecialBindings],
    ['spectral_peak', verifySpectralPeakBindings],
    ['statistics', verifyStatisticsBindings],
    ['stft', verifyStftBindings],
    ['synthesis', verifySynthesisBindings],
    ['trigonometric', verifyTrigonometricBindings],
    ['vector', verifyVectorBindings],
    ['wave', verifyWaveBindings],
    ['window', verifyWindowBindings],
];
export async function verifyAllBindings(api, fixture, progress = () => {}) {
    const results = {};
    for (const [family, verify] of bindingFamilies) {
        results[family] = await verify(api, fixture);
        progress(family, Object.keys(results).length, bindingFamilies.length);
    }
    return results;
}
