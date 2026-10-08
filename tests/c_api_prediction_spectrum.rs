#![cfg(feature = "c-api")]
use ciglet_rs::{c_api::*, statistics::Scalar};
use std::ptr;
type New<H> = unsafe extern "C" fn(usize, *mut *mut H) -> u32;
type Release<H> = unsafe extern "C" fn(*mut *mut H) -> u32;
type Shape<H> = unsafe extern "C" fn(*const H, *mut usize, usize) -> u32;
type Estimate<T, H> =
    unsafe extern "C" fn(*const H, *const T, usize, usize, u32, u32, *mut T, *mut T, usize) -> u32;
type ScaledEstimate<T, H, P> =
    unsafe extern "C" fn(*const H, *const T, usize, usize, u32, *mut *mut P) -> u32;
type Spectrum<T, H> = unsafe extern "C" fn(*const H, *const T, usize, T, *mut T, usize) -> u32;
type Gain<T> = unsafe extern "C" fn(*const T, usize, *const T, usize, u32, *mut T) -> u32;
type ScaledGain<T, P> = unsafe extern "C" fn(*const P, *mut T) -> u32;
struct Api<T, H, P> {
    new: New<H>,
    release: Release<H>,
    shape: Shape<H>,
    estimate: Estimate<T, H>,
    scaled: ScaledEstimate<T, H, P>,
    spectrum: Spectrum<T, H>,
    gain: Gain<T>,
    scaled_gain: ScaledGain<T, P>,
    scaled_release: Release<P>,
}
fn float_api() -> Api<f32, CigletRsPredictionSpectrumF32, CigletRsScaledPredictionF32> {
    Api {
        new: ciglet_rs_prediction_spectrum_new_f32,
        release: ciglet_rs_prediction_spectrum_release_f32,
        shape: ciglet_rs_prediction_spectrum_shape_f32,
        estimate: ciglet_rs_prediction_spectrum_estimate_f32,
        scaled: ciglet_rs_prediction_spectrum_estimate_scaled_f32,
        spectrum: ciglet_rs_prediction_spectrum_spectrum_f32,
        gain: ciglet_rs_prediction_gain_f32,
        scaled_gain: ciglet_rs_scaled_prediction_gain_f32,
        scaled_release: ciglet_rs_scaled_prediction_release_f32,
    }
}
fn double_api() -> Api<f64, CigletRsPredictionSpectrumF64, CigletRsScaledPredictionF64> {
    Api {
        new: ciglet_rs_prediction_spectrum_new_f64,
        release: ciglet_rs_prediction_spectrum_release_f64,
        shape: ciglet_rs_prediction_spectrum_shape_f64,
        estimate: ciglet_rs_prediction_spectrum_estimate_f64,
        scaled: ciglet_rs_prediction_spectrum_estimate_scaled_f64,
        spectrum: ciglet_rs_prediction_spectrum_spectrum_f64,
        gain: ciglet_rs_prediction_gain_f64,
        scaled_gain: ciglet_rs_scaled_prediction_gain_f64,
        scaled_release: ciglet_rs_scaled_prediction_release_f64,
    }
}
struct Reader<'a>(&'a [u8], usize);
impl Reader<'_> {
    fn take(&mut self, count: usize) -> &[u8] {
        let (a, b) = self.0.split_at(count);
        self.0 = b;
        a
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.take(4).try_into().unwrap()) as usize
    }
    fn scalar<T: Scalar>(&mut self) -> T {
        T::from_f64(if self.1 == 4 {
            f32::from_le_bytes(self.take(4).try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.take(8).try_into().unwrap())
        })
        .unwrap()
    }
    fn vector<T: Scalar>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.scalar()).collect()
    }
}
fn measure<T: Scalar>(actual: T, expected: T, maximum: &mut f64) {
    let error = (actual.into() - expected.into()).abs() / (1. + expected.into().abs());
    assert!(error.is_finite());
    *maximum = maximum.max(error);
}
fn reference<T: Scalar, H, P>(bytes: &[u8], width: usize, api: &Api<T, H, P>) {
    let mut reader = Reader(bytes, width);
    assert_eq!(reader.take(4), b"LPS1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 48);
    let mut maxima = [0.; 5];
    for _ in 0..48 {
        let bins = reader.integer();
        let length = reader.integer();
        let order = reader.integer();
        let input = reader.vector::<T>(bins);
        let expected_correlation = reader.vector::<T>(order + 1);
        let expected_coefficients = reader.vector::<T>(order + 1);
        let expected_gain = reader.scalar::<T>();
        let expected_spectrum = reader.vector::<T>(length);
        let mut coefficients = vec![T::zero(); order + 1];
        let mut correlation = vec![T::zero(); order + 1];
        let mut spectrum = vec![T::zero(); length];
        let mut shape = [0; 2];
        let mut gain = T::zero();
        let mut handle = ptr::null_mut();
        // SAFETY: Initialized independent input/output storage and typed owner
        // retained through every call, released exactly once after all reads.
        unsafe {
            assert_eq!((api.new)(length, &mut handle), 0);
            assert_eq!((api.shape)(handle, shape.as_mut_ptr(), 2), 0);
            assert_eq!(shape, [length, length / 2 + 1]);
            for _ in 0..2 {
                assert_eq!(
                    (api.estimate)(
                        handle,
                        input.as_ptr(),
                        bins,
                        order,
                        1,
                        1,
                        coefficients.as_mut_ptr(),
                        correlation.as_mut_ptr(),
                        order + 1
                    ),
                    0
                );
            }
            assert_eq!(
                (api.gain)(
                    coefficients.as_ptr(),
                    order + 1,
                    correlation.as_ptr(),
                    order + 1,
                    1,
                    &mut gain
                ),
                0
            );
            for (&actual, &expected) in correlation.iter().zip(&expected_correlation) {
                measure(actual, expected, &mut maxima[0]);
            }
            for (&actual, &expected) in coefficients.iter().zip(&expected_coefficients) {
                measure(actual, expected, &mut maxima[1]);
            }
            measure(gain, expected_gain, &mut maxima[2]);
            assert_eq!(
                (api.spectrum)(
                    handle,
                    expected_coefficients.as_ptr(),
                    order + 1,
                    expected_gain,
                    spectrum.as_mut_ptr(),
                    length
                ),
                0
            );
            for (&actual, &expected) in spectrum.iter().zip(&expected_spectrum) {
                measure(actual, expected, &mut maxima[3]);
            }
            assert_eq!(
                (api.spectrum)(
                    handle,
                    coefficients.as_ptr(),
                    order + 1,
                    gain,
                    spectrum.as_mut_ptr(),
                    length
                ),
                0
            );
            for (&actual, &expected) in spectrum.iter().zip(&expected_spectrum) {
                measure(actual, expected, &mut maxima[4]);
            }
            assert_eq!((api.release)(&mut handle), 0);
            assert!(handle.is_null());
        }
    }
    let cases = reader.integer();
    for _ in 0..cases {
        let n = reader.integer();
        let _ = reader.vector::<T>(n);
        let n = reader.integer();
        let _ = reader.vector::<T>(n * 3);
    }
    assert!(reader.0.is_empty());
    let bounds = if width == 4 {
        [2e-8, 3e-8, 0., 7e-8, 8e-8]
    } else {
        [5e-17, 2e-15, 4e-16, 2e-15, 3e-15]
    };
    for (actual, bound) in maxima.into_iter().zip(bounds) {
        assert!(actual <= bound, "{actual:e} > {bound:e}");
    }
}
#[test]
fn float_all_original_estimates_and_spectra() {
    reference(
        include_bytes!("fixtures/prediction-spectrum-float.bin"),
        4,
        &float_api(),
    );
}
#[test]
fn double_all_original_estimates_and_spectra() {
    reference(
        include_bytes!("fixtures/prediction-spectrum-double.bin"),
        8,
        &double_api(),
    );
}
fn edges<T: Scalar, H, P>(api: &Api<T, H, P>, maximum: T, tiny: T) {
    let one = T::one();
    let zero = T::zero();
    let sentinel = T::from_f64(71.).unwrap();
    let mut handle = ptr::null_mut();
    let mut scaled = ptr::null_mut();
    let mut shape = [71; 2];
    let mut coefficients = [sentinel; 4];
    let mut correlation = [sentinel; 4];
    let mut spectrum = [sentinel; 4];
    let input = [one, one, T::from_f64(f64::NAN).unwrap()];
    // SAFETY: Valid live owners, initialized inputs, correctly sized independent
    // writable outputs; raw rejected overlap creates no overlapping Rust references.
    unsafe {
        assert_eq!((api.new)(4, &mut handle), 0);
        let retained = handle;
        assert_eq!((api.new)(3, &mut handle), 3);
        assert_eq!(handle, retained);
        assert_eq!((api.shape)(handle, shape.as_mut_ptr(), 1), 2);
        assert_eq!(shape, [71; 2]);
        for arithmetic in [0, 1] {
            assert_eq!(
                (api.estimate)(
                    handle,
                    input.as_ptr(),
                    3,
                    1,
                    1,
                    arithmetic,
                    coefficients.as_mut_ptr(),
                    correlation.as_mut_ptr(),
                    2
                ),
                0
            );
            assert_eq!(&coefficients[..2], &[one, zero]);
            assert_eq!(&correlation[..2], &[one, zero]);
            coefficients.fill(sentinel);
            correlation.fill(sentinel);
            assert_eq!(
                (api.estimate)(
                    handle,
                    input.as_ptr(),
                    3,
                    1,
                    0,
                    arithmetic,
                    coefficients.as_mut_ptr(),
                    correlation.as_mut_ptr(),
                    2
                ),
                3
            );
            assert_eq!(coefficients, [sentinel; 4]);
            assert_eq!(correlation, [sentinel; 4]);
        }
        assert_eq!(
            (api.estimate)(
                handle,
                input.as_ptr(),
                3,
                1,
                2,
                0,
                coefficients.as_mut_ptr(),
                correlation.as_mut_ptr(),
                2
            ),
            3
        );
        assert_eq!(
            (api.estimate)(
                handle,
                input.as_ptr(),
                3,
                1,
                1,
                2,
                coefficients.as_mut_ptr(),
                correlation.as_mut_ptr(),
                2
            ),
            3
        );
        assert_eq!(
            (api.estimate)(
                handle,
                input.as_ptr(),
                3,
                1,
                1,
                0,
                coefficients.as_mut_ptr(),
                coefficients.as_mut_ptr(),
                2
            ),
            2
        );
        assert_eq!(coefficients, [sentinel; 4]);
        assert_eq!(correlation, [sentinel; 4]);
        let mut long = ptr::null_mut();
        assert_eq!((api.new)(16, &mut long), 0);
        assert_eq!(
            (api.estimate)(
                long,
                input.as_ptr(),
                3,
                1,
                1,
                0,
                coefficients.as_mut_ptr(),
                correlation.as_mut_ptr(),
                2
            ),
            3
        );
        assert_eq!((api.release)(&mut long), 0);
        let pole = [one, T::from_f64(-1.).unwrap()];
        assert_eq!(
            (api.spectrum)(handle, pole.as_ptr(), 2, one, spectrum.as_mut_ptr(), 4),
            3
        );
        assert_eq!(spectrum, [sentinel; 4]);
        let negative_zero = T::from_f64(-0.).unwrap();
        assert_eq!(
            (api.spectrum)(
                handle,
                pole.as_ptr(),
                2,
                negative_zero,
                spectrum.as_mut_ptr(),
                4
            ),
            0
        );
        assert!(
            spectrum
                .iter()
                .all(|&x| x.into().to_bits() == (-0.0_f64).to_bits())
        );
        for amplitude in [maximum, tiny] {
            assert_eq!(
                (api.spectrum)(handle, &amplitude, 1, amplitude, spectrum.as_mut_ptr(), 4),
                0
            );
            assert_eq!(spectrum, [one; 4]);
            let magnitudes = [amplitude; 3];
            assert_eq!(
                (api.scaled)(handle, magnitudes.as_ptr(), 3, 3, 0, &mut scaled),
                0
            );
            let mut gain = sentinel;
            assert_eq!((api.scaled_gain)(scaled, &mut gain), 0);
            assert_eq!(gain, amplitude);
            let retained = scaled;
            assert_eq!(
                (api.scaled)(handle, magnitudes.as_ptr(), 3, 3, 2, &mut scaled),
                3
            );
            assert_eq!(scaled, retained);
            assert_eq!((api.scaled_release)(&mut scaled), 0);
        }
        let mut overlap = [one, zero, zero, zero];
        assert_eq!(
            (api.spectrum)(handle, overlap.as_ptr(), 1, one, overlap.as_mut_ptr(), 4),
            0
        );
        assert_eq!(overlap, [one; 4]);
        assert_eq!(
            (api.spectrum)(handle, &one, 1, one, spectrum.as_mut_ptr(), 3),
            2
        );
        assert_eq!((api.release)(&mut handle), 0);
        assert!(handle.is_null());
        assert_eq!((api.release)(&mut handle), 0);
        assert_eq!((api.new)(1, &mut handle), 0);
        assert_eq!((api.shape)(handle, shape.as_mut_ptr(), 2), 0);
        assert_eq!(shape, [1, 1]);
        assert_eq!(
            (api.spectrum)(handle, &one, 1, one, spectrum.as_mut_ptr(), 1),
            0
        );
        assert_eq!(spectrum[0], one);
        assert_eq!(
            (api.estimate)(
                handle,
                input.as_ptr(),
                2,
                0,
                0,
                0,
                coefficients.as_mut_ptr(),
                correlation.as_mut_ptr(),
                1
            ),
            3
        );
        assert_eq!((api.release)(&mut handle), 0);
    }
}
#[test]
fn float_policies_shapes_extremes_and_recovery() {
    edges(&float_api(), f32::MAX, f32::from_bits(1));
}
#[test]
fn double_policies_shapes_extremes_and_recovery() {
    edges(&double_api(), f64::MAX, f64::from_bits(1));
}
#[test]
fn concurrent_callers_serialize_native_scratch() {
    let mut handle = ptr::null_mut();
    // SAFETY: Sole ownership retained until every scoped worker has joined.
    assert_eq!(
        unsafe { ciglet_rs_prediction_spectrum_new_f64(4, &mut handle) },
        0
    );
    let address = handle as usize;
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(move || {
                let handle = address as *const CigletRsPredictionSpectrumF64;
                for _ in 0..16 {
                    let input = [1.; 3];
                    let mut coefficients = [71.; 2];
                    let mut correlation = [71.; 2];
                    let mut output = [71.; 4];
                    // SAFETY: Live retained typed owner and each worker's independent
                    // initialized local arrays; scratch mutation serialized by the API.
                    unsafe {
                        assert_eq!(
                            ciglet_rs_prediction_spectrum_estimate_f64(
                                handle,
                                input.as_ptr(),
                                3,
                                1,
                                0,
                                0,
                                coefficients.as_mut_ptr(),
                                correlation.as_mut_ptr(),
                                2
                            ),
                            0
                        );
                        assert_eq!(coefficients, [1., 0.]);
                        assert_eq!(correlation, [1., 0.]);
                        assert_eq!(
                            ciglet_rs_prediction_spectrum_spectrum_f64(
                                handle,
                                coefficients.as_ptr(),
                                2,
                                2.,
                                output.as_mut_ptr(),
                                4
                            ),
                            0
                        );
                        assert_eq!(output, [2.; 4]);
                    }
                }
            });
        }
    });
    // SAFETY: All scoped users have joined; exclusive initialized owner slot.
    assert_eq!(
        unsafe { ciglet_rs_prediction_spectrum_release_f64(&mut handle) },
        0
    );
}
