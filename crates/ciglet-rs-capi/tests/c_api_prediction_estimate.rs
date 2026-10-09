use ciglet_rs::statistics::Scalar;
use ciglet_rs_capi::*;
use std::ptr;
type Samples<T> = unsafe extern "C" fn(*const T, usize, usize, u32, *mut T, *mut T, usize) -> u32;
type Magnitudes<T> =
    unsafe extern "C" fn(*const T, usize, usize, u32, u32, *mut T, *mut T, usize) -> u32;
struct Reader<'a>(&'a [u8], usize);
impl Reader<'_> {
    fn take(&mut self, count: usize) -> &[u8] {
        let (part, remaining) = self.0.split_at(count);
        self.0 = remaining;
        part
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
    fn vector<T: Scalar>(&mut self, count: usize) -> Vec<T> {
        (0..count).map(|_| self.scalar()).collect()
    }
}
fn samples_reference<T: Scalar>(bytes: &[u8], width: usize, call: Samples<T>) {
    let mut reader = Reader(bytes, width);
    assert_eq!(reader.take(4), b"LPC1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 64);
    for _ in 0..64 {
        let input_count = reader.integer();
        let order = reader.integer();
        let input = reader.vector::<T>(input_count);
        let expected_correlation = reader.vector::<T>(order + 1);
        let expected_coefficients = reader.vector::<T>(order + 1);
        let _gain = reader.scalar::<T>();
        let mut coefficients = vec![T::zero(); order + 1];
        let mut correlation = vec![T::zero(); order + 1];
        // SAFETY: Initialized input and distinct aligned writable output arrays.
        assert_eq!(
            unsafe {
                call(
                    input.as_ptr(),
                    input_count,
                    order,
                    1,
                    coefficients.as_mut_ptr(),
                    correlation.as_mut_ptr(),
                    order + 1,
                )
            },
            0
        );
        for (&actual, &expected) in coefficients
            .iter()
            .zip(&expected_coefficients)
            .chain(correlation.iter().zip(&expected_correlation))
        {
            assert_eq!(actual.into().to_bits(), expected.into().to_bits());
        }
    }
    assert!(reader.0.is_empty());
}
fn magnitude_reference<T: Scalar>(bytes: &[u8], width: usize, call: Magnitudes<T>) {
    let mut reader = Reader(bytes, width);
    assert_eq!(reader.take(4), b"LPS1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 48);
    for case in 0..48 {
        let bins = reader.integer();
        let length = reader.integer();
        let order = reader.integer();
        let input = reader.vector::<T>(bins);
        let expected_correlation = reader.vector::<T>(order + 1);
        let expected_coefficients = reader.vector::<T>(order + 1);
        let _gain = reader.scalar::<T>();
        let _spectrum = reader.vector::<T>(length);
        let mut coefficients = vec![T::zero(); order + 1];
        let mut correlation = vec![T::zero(); order + 1];
        // SAFETY: Initialized input and independent complete output planes.
        assert_eq!(
            unsafe {
                call(
                    input.as_ptr(),
                    bins,
                    order,
                    1,
                    1,
                    coefficients.as_mut_ptr(),
                    correlation.as_mut_ptr(),
                    order + 1,
                )
            },
            0
        );
        let bounds = if width == 4 {
            [2e-8, 3e-8]
        } else {
            [5e-17, 2e-15]
        };
        for ((actual, expected), bound) in [
            (&correlation, &expected_correlation),
            (&coefficients, &expected_coefficients),
        ]
        .into_iter()
        .zip(bounds)
        {
            for (&actual, &expected) in actual.iter().zip(expected) {
                let error = (actual.into() - expected.into()).abs() / (1. + expected.into().abs());
                assert!(
                    error <= bound,
                    "case={case} error={error:e} bound={bound:e}"
                );
            }
        }
    }
    // Consume the separate resonance corpus without claiming resonance ABI coverage.
    let cases = reader.integer();
    for _ in 0..cases {
        let count = reader.integer();
        let _coefficients = reader.vector::<T>(count);
        let roots = reader.integer();
        let _frequencies_and_poles = reader.vector::<T>(roots * 3);
    }
    assert!(reader.0.is_empty());
}
#[test]
fn waveform_float_matches_all_original_records() {
    samples_reference(
        include_bytes!("../../../tests/fixtures/prediction-float.bin"),
        4,
        ciglet_rs_prediction_from_samples_f32,
    );
}
#[test]
fn waveform_double_matches_all_original_records() {
    samples_reference(
        include_bytes!("../../../tests/fixtures/prediction-double.bin"),
        8,
        ciglet_rs_prediction_from_samples_f64,
    );
}
#[test]
fn magnitude_float_matches_all_original_records() {
    magnitude_reference(
        include_bytes!("../../../tests/fixtures/prediction-spectrum-float.bin"),
        4,
        ciglet_rs_prediction_from_magnitude_f32,
    );
}
#[test]
fn magnitude_double_matches_all_original_records() {
    magnitude_reference(
        include_bytes!("../../../tests/fixtures/prediction-spectrum-double.bin"),
        8,
        ciglet_rs_prediction_from_magnitude_f64,
    );
}
macro_rules! edge_test {
    ($name:ident, $scalar:ty, $handle:ty, $samples:ident, $magnitude:ident,
     $scaled:ident, $gain:ident, $ordinary:ident, $release:ident) => {
        #[test]
        fn $name() {
            let mut coefficients = [71. as $scalar; 4];
            let mut correlation = [71. as $scalar; 4];
            // SAFETY: Live initialized input, disjoint writable arrays and owner
            // slots. Deliberate raw overlap is rejected without Rust references.
            unsafe {
                for arithmetic in [0, 1] {
                    for nyquist in [0, 1] {
                        let input = [0. as $scalar, 0., 2.];
                        assert_eq!(
                            $magnitude(
                                input.as_ptr(),
                                3,
                                0,
                                nyquist,
                                arithmetic,
                                coefficients.as_mut_ptr(),
                                correlation.as_mut_ptr(),
                                1
                            ),
                            0
                        );
                        assert_eq!(coefficients[0], 1.);
                        assert_eq!(correlation[0], if nyquist == 0 { 1. } else { 0. });
                    }
                    let ignored = [1. as $scalar, 1., <$scalar>::NAN];
                    assert_eq!(
                        $magnitude(
                            ignored.as_ptr(),
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
                    assert_eq!(&coefficients[..2], &[1., 0.]);
                    assert_eq!(&correlation[..2], &[1., 0.]);
                    coefficients.fill(71.);
                    correlation.fill(71.);
                    assert_eq!(
                        $magnitude(
                            ignored.as_ptr(),
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
                    assert_eq!(coefficients, [71.; 4]);
                    assert_eq!(correlation, [71.; 4]);
                }
                let mut input = [1. as $scalar, 0.];
                assert_eq!(
                    $samples(
                        input.as_ptr(),
                        2,
                        1,
                        0,
                        input.as_mut_ptr(),
                        correlation.as_mut_ptr(),
                        2
                    ),
                    0
                );
                assert_eq!(input, [1., 0.]);
                assert_eq!(&correlation[..2], &[1., 0.]);
                coefficients.fill(71.);
                correlation.fill(71.);
                assert_eq!(
                    $samples(
                        input.as_ptr(),
                        2,
                        1,
                        2,
                        coefficients.as_mut_ptr(),
                        correlation.as_mut_ptr(),
                        2
                    ),
                    3
                );
                assert_eq!(
                    $samples(
                        input.as_ptr(),
                        2,
                        usize::MAX,
                        0,
                        coefficients.as_mut_ptr(),
                        correlation.as_mut_ptr(),
                        2
                    ),
                    2
                );
                assert_eq!(
                    $samples(
                        input.as_ptr(),
                        2,
                        1,
                        0,
                        coefficients.as_mut_ptr(),
                        coefficients.as_mut_ptr(),
                        2
                    ),
                    2
                );
                assert_eq!(
                    $samples(
                        ptr::null(),
                        0,
                        0,
                        0,
                        coefficients.as_mut_ptr(),
                        correlation.as_mut_ptr(),
                        1
                    ),
                    3
                );
                assert_eq!(
                    $magnitude(
                        input.as_ptr(),
                        2,
                        1,
                        2,
                        0,
                        coefficients.as_mut_ptr(),
                        correlation.as_mut_ptr(),
                        2
                    ),
                    3
                );
                assert_eq!(coefficients, [71.; 4]);
                assert_eq!(correlation, [71.; 4]);
                for amplitude in [<$scalar>::MAX, <$scalar>::from_bits(1)] {
                    let magnitudes = [amplitude; 3];
                    let mut owner: *mut $handle = ptr::null_mut();
                    let mut gain = 71. as $scalar;
                    assert_eq!($scaled(magnitudes.as_ptr(), 3, 3, 0, &mut owner), 0);
                    assert_eq!(
                        $ordinary(
                            owner,
                            coefficients.as_mut_ptr(),
                            correlation.as_mut_ptr(),
                            4
                        ),
                        3
                    );
                    assert_eq!(coefficients, [71.; 4]);
                    assert_eq!(correlation, [71.; 4]);
                    assert_eq!($gain(owner, &mut gain), 0);
                    assert_eq!(gain, amplitude);
                    let retained = owner;
                    assert_eq!($scaled(magnitudes.as_ptr(), 3, 3, 2, &mut owner), 3);
                    assert_eq!(owner, retained);
                    assert_eq!($release(&mut owner), 0);
                    assert!(owner.is_null());
                    assert_eq!(
                        $magnitude(
                            magnitudes.as_ptr(),
                            3,
                            3,
                            0,
                            0,
                            coefficients.as_mut_ptr(),
                            correlation.as_mut_ptr(),
                            4
                        ),
                        3
                    );
                    assert_eq!(coefficients, [71.; 4]);
                    assert_eq!(correlation, [71.; 4]);
                }
            }
        }
    };
}
edge_test!(
    float_policies_extremes_and_atomic_errors,
    f32,
    CigletRsScaledPredictionF32,
    ciglet_rs_prediction_from_samples_f32,
    ciglet_rs_prediction_from_magnitude_f32,
    ciglet_rs_scaled_prediction_from_magnitude_f32,
    ciglet_rs_scaled_prediction_gain_f32,
    ciglet_rs_scaled_prediction_ordinary_f32,
    ciglet_rs_scaled_prediction_release_f32
);
edge_test!(
    double_policies_extremes_and_atomic_errors,
    f64,
    CigletRsScaledPredictionF64,
    ciglet_rs_prediction_from_samples_f64,
    ciglet_rs_prediction_from_magnitude_f64,
    ciglet_rs_scaled_prediction_from_magnitude_f64,
    ciglet_rs_scaled_prediction_gain_f64,
    ciglet_rs_scaled_prediction_ordinary_f64,
    ciglet_rs_scaled_prediction_release_f64
);
