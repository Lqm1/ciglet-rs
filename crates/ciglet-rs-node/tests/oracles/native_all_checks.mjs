import { verifyApproximationBindings } from './native_approximation_checks.mjs';
import { verifyArrayBindings } from './native_array_checks.mjs';
import { verifyBandCepstrumBindings } from './native_cepstrum_checks.mjs';
import { verifyComplexBindings } from './native_complex_checks.mjs';
import { verifyCorrelogramBindings } from './native_correlogram_checks.mjs';
import { verifyCztBindings } from './native_czt_checks.mjs';
import { verifyEnvelopeBindings } from './native_envelope_checks.mjs';
import { verifyFilterBankBindings } from './native_filterbank_checks.mjs';
import { verifyFilteringBindings } from './native_filtering_checks.mjs';
import { verifyFourierBindings } from './native_fourier_checks.mjs';
import { verifyFrameExtremaBindings } from './native_frame_extrema_checks.mjs';
import { verifyFrequencyDistanceBindings } from './native_frequency_distance_checks.mjs';
import { verifyGlottalBindings } from './native_glottal_checks.mjs';
import { verifyHyperbolicBindings } from './native_hyperbolic_checks.mjs';
import { verifyInstantaneousFrequencyBindings } from './native_instantaneous_frequency_checks.mjs';
import { verifyInterpolationBindings } from './native_interpolation_checks.mjs';
import { verifyKalmanBindings } from './native_kalman_checks.mjs';
import { verifyLaneBindings } from './native_lanes_checks.mjs';
import { verifyMatrixBindings } from './native_matrix_checks.mjs';
import { verifyOrderingBindings } from './native_ordering_checks.mjs';
import { verifyPhaseBindings } from './native_phase_checks.mjs';
import { verifyPlottingBindings } from './native_plotting_checks.mjs';
import { verifyPolynomialBindings } from './native_polynomial_checks.mjs';
import { verifyPredictionBindings } from './native_prediction_checks.mjs';
import { verifyRandomBindings } from './native_random_checks.mjs';
import { verifyRealCepstrumBindings } from './native_real_cepstrum_checks.mjs';
import { verifyResamplingBindings } from './native_resampling_checks.mjs';
import { verifyRootBindings } from './native_root_checks.mjs';
import { verifyScalarBindings } from './native_scalar_checks.mjs';
import { verifySincBindings } from './native_sinc_checks.mjs';
import { verifySmoothingBindings } from './native_smoothing_checks.mjs';
import { verifySpecialBindings } from './native_special_checks.mjs';
import { verifySpectralPeakBindings } from './native_spectral_peak_checks.mjs';
import { verifyStatisticsBindings } from './native_statistics_checks.mjs';
import { verifyStftBindings } from './native_stft_checks.mjs';
import { verifySynthesisBindings } from './native_synthesis_checks.mjs';
import { verifyTrigonometricBindings } from './native_trigonometric_checks.mjs';
import { verifyVectorBindings } from './native_vector_checks.mjs';
import { verifyWaveBindings } from './native_wave_checks.mjs';
import { verifyWindowBindings } from './native_window_checks.mjs';

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
