use ciglet_rs_capi::*;
use std::ptr;

macro_rules! scaled_tests {
    ($module:ident, $scalar:ty, $handle:ty, $fixture:literal,
     $samples:ident, $clone:ident, $release:ident, $length:ident, $coefficients:ident,
     $parts:ident, $gain_parts:ident, $gain:ident, $ordinary:ident) => {
        mod $module {
            use super::*;
            struct Reader<'a> {
                bytes: &'a [u8],
                cursor: usize,
            }
            impl Reader<'_> {
                fn integer(&mut self) -> u32 {
                    let result = u32::from_le_bytes(
                        self.bytes[self.cursor..self.cursor + 4].try_into().unwrap(),
                    );
                    self.cursor += 4;
                    result
                }
                fn double(&mut self) -> f64 {
                    let result = f64::from_le_bytes(
                        self.bytes[self.cursor..self.cursor + 8].try_into().unwrap(),
                    );
                    self.cursor += 8;
                    result
                }
                fn scalar(&mut self) -> $scalar {
                    if size_of::<$scalar>() == 4 {
                        f32::from_bits(self.integer()) as $scalar
                    } else {
                        self.double() as $scalar
                    }
                }
            }
            #[test]
            fn independent_scaled_waveform_records() {
                let bytes = include_bytes!($fixture);
                assert_eq!(&bytes[..4], b"SWV1");
                let mut reader = Reader { bytes, cursor: 4 };
                assert_eq!(reader.integer() as usize, size_of::<$scalar>());
                assert_eq!(reader.integer(), 64);
                let mut lags = 0;
                for case in 0..64 {
                    let count = reader.integer() as usize;
                    let order = reader.integer() as usize;
                    let samples: Vec<$scalar> = (0..count).map(|_| reader.scalar()).collect();
                    let mut handle: *mut $handle = ptr::null_mut();
                    let mut fractions = vec![71.; order + 1];
                    let mut exponents = vec![71; order + 1];
                    let mut coefficients = vec![71. as $scalar; order + 1];
                    let mut length = 71;
                    // SAFETY: Initialized input and distinct writable outputs; owned handle
                    // remains live until its sole release, with no outstanding aliases.
                    unsafe {
                        assert_eq!($samples(samples.as_ptr(), count, order, &mut handle), 0);
                        assert_eq!($length(handle, &mut length), 0);
                        assert_eq!(length, order + 1);
                        assert_eq!(
                            $parts(
                                handle,
                                fractions.as_mut_ptr(),
                                exponents.as_mut_ptr(),
                                length
                            ),
                            0
                        );
                        assert_eq!($coefficients(handle, coefficients.as_mut_ptr(), length), 0);
                        assert_eq!($release(&mut handle), 0);
                    }
                    assert!(handle.is_null());
                    for (&fraction, &exponent) in fractions.iter().zip(&exponents) {
                        let expected = reader.double();
                        let expected_exponent = reader.integer() as i32;
                        let error = if expected == 0. {
                            fraction.abs()
                        } else {
                            (fraction * 2.0_f64.powi(exponent - expected_exponent) / expected - 1.)
                                .abs()
                        };
                        assert!(
                            error <= 2e-15 && (expected != 0. || fraction == 0.),
                            "case={case} correlation={error:e}"
                        );
                        lags += 1;
                    }
                    for coefficient in coefficients {
                        let expected = reader.scalar() as f64;
                        let error = (coefficient as f64 - expected).abs() / (1. + expected.abs());
                        let bound = if size_of::<$scalar>() == 4 {
                            2e-15
                        } else {
                            5e-15
                        };
                        assert!(error <= bound, "case={case} coefficient={error:e}");
                    }
                }
                assert_eq!(lags, 176);
                assert_eq!(reader.cursor, bytes.len());
            }
            #[test]
            fn extremes_preserve_estimates_and_ownership_after_conversion_errors() {
                for amplitude in [<$scalar>::MAX, <$scalar>::from_bits(1)] {
                    let samples = [amplitude, 0.];
                    let mut handle: *mut $handle = ptr::null_mut();
                    let mut copy: *mut $handle = ptr::null_mut();
                    let mut coefficients = [71. as $scalar; 2];
                    let mut correlations = [71. as $scalar; 2];
                    let mut fractions = [71.; 2];
                    let mut exponents = [71; 2];
                    let mut gain = 71. as $scalar;
                    let mut fraction = 71.;
                    let mut exponent = 71;
                    // SAFETY: Live typed owners, initialized sample range and distinct
                    // aligned writable arrays/slots; all owners released exactly once.
                    unsafe {
                        assert_eq!($samples(samples.as_ptr(), 2, 1, &mut handle), 0);
                        assert_eq!($clone(handle, &mut copy), 0);
                        assert_ne!(copy, handle);
                        assert_eq!(
                            $ordinary(
                                handle,
                                coefficients.as_mut_ptr(),
                                correlations.as_mut_ptr(),
                                2
                            ),
                            3
                        );
                        assert_eq!(coefficients, [71.; 2]);
                        assert_eq!(correlations, [71.; 2]);
                        assert_eq!($coefficients(handle, coefficients.as_mut_ptr(), 2), 0);
                        assert_eq!(coefficients, [1., 0.]);
                        assert_eq!(
                            $parts(handle, fractions.as_mut_ptr(), exponents.as_mut_ptr(), 2),
                            0
                        );
                        assert_eq!((fractions[1], exponents[1]), (0., 0));
                        assert!((1.0..2.0).contains(&fractions[0]));
                        assert_eq!($gain(handle, &mut gain), 0);
                        assert_eq!(gain, amplitude);
                        assert_eq!($gain_parts(handle, &mut fraction, &mut exponent), 0);
                        assert!((1.0..2.0).contains(&fraction));
                        // Split scaling avoids powi's reciprocal overflow at -1074
                        // on targets whose runtime first computes the positive power.
                        let reconstructed = fraction
                            * 2.0_f64.powi(exponent / 2)
                            * 2.0_f64.powi(exponent - exponent / 2);
                        assert!((reconstructed / (amplitude as f64) - 1.).abs() <= 2e-15);
                        let retained = handle;
                        assert_eq!($samples(ptr::null(), 0, 0, &mut handle), 3);
                        assert_eq!(handle, retained);
                        assert_eq!($release(&mut handle), 0);
                        assert!(handle.is_null());
                        assert_eq!($gain(copy, &mut gain), 0);
                        assert_eq!(gain, amplitude);
                        assert_eq!($release(&mut copy), 0);
                        assert_eq!($release(&mut copy), 0);
                    }
                }
            }
            #[test]
            fn silence_shapes_overlap_and_failed_gain_retain_outputs() {
                let mut handle: *mut $handle = ptr::null_mut();
                let mut coefficients = [71. as $scalar; 3];
                let mut correlations = [71. as $scalar; 3];
                let mut fractions = [71.; 3];
                let mut exponents = [71; 3];
                let mut fraction = 71.;
                let mut exponent = 71;
                let mut gain = 71. as $scalar;
                // SAFETY: Explicit raw overlap uses one aligned initialized arena
                // without creating overlapping Rust references; rejected before writes.
                unsafe {
                    let silence = [0. as $scalar; 3];
                    assert_eq!($samples(silence.as_ptr(), 3, 2, &mut handle), 0);
                    assert_eq!(
                        $ordinary(
                            handle,
                            coefficients.as_mut_ptr(),
                            correlations.as_mut_ptr(),
                            3
                        ),
                        0
                    );
                    assert_eq!(coefficients, [1., 0., 0.]);
                    assert_eq!(correlations, [0.; 3]);
                    assert_eq!($gain_parts(handle, &mut fraction, &mut exponent), 0);
                    assert_eq!((fraction, exponent), (0., 0));
                    assert_eq!($coefficients(handle, coefficients.as_mut_ptr(), 2), 2);
                    assert_eq!($gain(handle, ptr::null_mut()), 1);
                    assert_eq!($length(ptr::null(), &mut 71), 1);
                    let mut arena = [71_u64; 6];
                    assert_eq!(
                        $parts(
                            handle,
                            arena.as_mut_ptr().cast(),
                            arena.as_mut_ptr().cast(),
                            3
                        ),
                        2
                    );
                    assert_eq!(
                        $gain_parts(handle, arena.as_mut_ptr().cast(), arena.as_mut_ptr().cast()),
                        2
                    );
                    assert_eq!(arena, [71; 6]);
                    assert_eq!(
                        $ordinary(
                            handle,
                            coefficients.as_mut_ptr(),
                            coefficients.as_mut_ptr(),
                            3
                        ),
                        2
                    );
                    assert_eq!(
                        $parts(handle, fractions.as_mut_ptr(), exponents.as_mut_ptr(), 3),
                        0
                    );
                    assert_eq!(fractions, [0.; 3]);
                    assert_eq!(exponents, [0; 3]);
                    let retained = handle;
                    let nan = [<$scalar>::NAN];
                    assert_eq!($samples(nan.as_ptr(), 1, 0, &mut handle), 3);
                    assert_eq!(handle, retained);
                    assert_eq!($release(&mut handle), 0);
                    let cancelled = [<$scalar>::MAX, 1., -<$scalar>::MAX];
                    assert_eq!($samples(cancelled.as_ptr(), 3, 2, &mut handle), 0);
                    assert_eq!($gain(handle, &mut gain), 3);
                    assert_eq!(gain, 71.);
                    assert_eq!($gain_parts(handle, &mut fraction, &mut exponent), 0);
                    assert!((1.0..2.0).contains(&fraction));
                    assert_eq!($coefficients(handle, coefficients.as_mut_ptr(), 3), 0);
                    assert_eq!($release(&mut handle), 0);
                }
            }
        }
    };
}
scaled_tests!(
    float,
    f32,
    CigletRsScaledPredictionF32,
    "../../../tests/fixtures/scaled-waveform-float.bin",
    ciglet_rs_scaled_prediction_from_samples_f32,
    ciglet_rs_scaled_prediction_clone_f32,
    ciglet_rs_scaled_prediction_release_f32,
    ciglet_rs_scaled_prediction_length_f32,
    ciglet_rs_scaled_prediction_coefficients_f32,
    ciglet_rs_scaled_prediction_correlation_parts_f32,
    ciglet_rs_scaled_prediction_gain_parts_f32,
    ciglet_rs_scaled_prediction_gain_f32,
    ciglet_rs_scaled_prediction_ordinary_f32
);
scaled_tests!(
    double,
    f64,
    CigletRsScaledPredictionF64,
    "../../../tests/fixtures/scaled-waveform-double.bin",
    ciglet_rs_scaled_prediction_from_samples_f64,
    ciglet_rs_scaled_prediction_clone_f64,
    ciglet_rs_scaled_prediction_release_f64,
    ciglet_rs_scaled_prediction_length_f64,
    ciglet_rs_scaled_prediction_coefficients_f64,
    ciglet_rs_scaled_prediction_correlation_parts_f64,
    ciglet_rs_scaled_prediction_gain_parts_f64,
    ciglet_rs_scaled_prediction_gain_f64,
    ciglet_rs_scaled_prediction_ordinary_f64
);
