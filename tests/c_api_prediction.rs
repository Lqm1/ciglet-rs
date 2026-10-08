#![cfg(feature = "c-api")]
use ciglet_rs::{c_api::*, statistics::Scalar};
type Levinson<T> = unsafe extern "C" fn(*const T, usize, u32, *mut T, usize) -> u32;
type Gain<T> = unsafe extern "C" fn(*const T, usize, *const T, usize, u32, *mut T) -> u32;
struct Reader<'a>(&'a [u8], usize);
impl Reader<'_> {
    fn take(&mut self, n: usize) -> &[u8] {
        let (a, b) = self.0.split_at(n);
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
fn reference<T: Scalar>(bytes: &[u8], width: usize, levinson: Levinson<T>, gain: Gain<T>) {
    let mut r = Reader(bytes, width);
    assert_eq!(r.take(4), b"LPC1");
    assert_eq!(r.integer(), width);
    assert_eq!(r.integer(), 64);
    for _ in 0..64 {
        let samples = r.integer();
        let order = r.integer();
        let _samples = r.vector::<T>(samples);
        let correlation = r.vector::<T>(order + 1);
        let expected = r.vector::<T>(order + 1);
        let expected_gain = r.scalar::<T>();
        let mut coefficients = vec![T::zero(); order + 1];
        let mut actual_gain = T::zero();
        // SAFETY: Independent initialized input/output arrays and scalar storage.
        unsafe {
            assert_eq!(
                levinson(
                    correlation.as_ptr(),
                    correlation.len(),
                    1,
                    coefficients.as_mut_ptr(),
                    coefficients.len()
                ),
                0
            );
            for (&a, &b) in coefficients.iter().zip(&expected) {
                assert_eq!(a.into().to_bits(), b.into().to_bits());
            }
            assert_eq!(
                gain(
                    coefficients.as_ptr(),
                    coefficients.len(),
                    correlation.as_ptr(),
                    correlation.len(),
                    1,
                    &mut actual_gain
                ),
                0
            );
            assert_eq!(actual_gain.into().to_bits(), expected_gain.into().to_bits());
            assert_eq!(
                levinson(
                    correlation.as_ptr(),
                    correlation.len(),
                    0,
                    coefficients.as_mut_ptr(),
                    coefficients.len()
                ),
                0
            );
            assert_eq!(
                gain(
                    coefficients.as_ptr(),
                    coefficients.len(),
                    correlation.as_ptr(),
                    correlation.len(),
                    0,
                    &mut actual_gain
                ),
                0
            );
            assert!(actual_gain.into().is_finite());
        }
        let energy = correlation[0].into();
        if energy > 0.0 {
            for row in 1..correlation.len() {
                let mut residual = correlation[row].into() / energy;
                let mut scale = 1.0 + residual.abs();
                for (column, &coefficient) in coefficients.iter().enumerate().skip(1) {
                    let covariance = if row == column {
                        1.000001
                    } else {
                        correlation[row.abs_diff(column)].into() / energy
                    };
                    let term = covariance * coefficient.into();
                    residual += term;
                    scale += term.abs();
                }
                assert!(residual.abs() / scale <= if width == 4 { 2e-7 } else { 5e-15 });
            }
        }
    }
    assert!(r.0.is_empty());
}
#[test]
fn float_recursion_and_gain_match_all_original_records() {
    reference(
        include_bytes!("fixtures/prediction-float.bin"),
        4,
        ciglet_rs_prediction_levinson_f32,
        ciglet_rs_prediction_gain_f32,
    );
}
#[test]
fn double_recursion_and_gain_match_all_original_records() {
    reference(
        include_bytes!("fixtures/prediction-double.bin"),
        8,
        ciglet_rs_prediction_levinson_f64,
        ciglet_rs_prediction_gain_f64,
    );
}
#[test]
fn stable_extremes_overlap_silence_and_atomic_errors() {
    let mut correlation = [f64::MAX, f64::MAX * 0.5];
    let pointer = correlation.as_mut_ptr();
    // SAFETY: Input/output overlap is supported after borrowing ends; other
    // initialized arrays and scalar outputs are independent valid storage.
    unsafe {
        assert_eq!(
            ciglet_rs_prediction_levinson_f64(pointer, 2, 0, pointer, 2),
            0
        );
        assert_eq!(correlation[0], 1.0);
        assert!((correlation[1] + 0.5 / 1.000001).abs() < 2e-15);
        let tiny = f64::from_bits(1);
        let input = [tiny * 4.0, tiny * 2.0];
        let mut output = [71.0; 2];
        assert_eq!(
            ciglet_rs_prediction_levinson_f64(input.as_ptr(), 2, 0, output.as_mut_ptr(), 2),
            0
        );
        assert!((output[1] + 0.5 / 1.000001).abs() < 2e-15);
        let coefficients = [2.0, -1.0];
        let energy = [f64::MAX; 2];
        let mut gain = 71.0;
        assert_eq!(
            ciglet_rs_prediction_gain_f64(
                coefficients.as_ptr(),
                2,
                energy.as_ptr(),
                2,
                0,
                &mut gain
            ),
            0
        );
        assert!((gain / f64::MAX.sqrt() - 1.0).abs() < 2e-15);
        output.fill(71.0);
        let invalid = [1.0, 2.0];
        assert_eq!(
            ciglet_rs_prediction_levinson_f64(invalid.as_ptr(), 2, 0, output.as_mut_ptr(), 2),
            3
        );
        assert_eq!(
            ciglet_rs_prediction_levinson_f64(input.as_ptr(), 2, 2, output.as_mut_ptr(), 2),
            3
        );
        assert_eq!(
            ciglet_rs_prediction_levinson_f64(input.as_ptr(), 2, 0, output.as_mut_ptr(), 1),
            2
        );
        assert_eq!(output, [71.0; 2]);
        gain = 71.0;
        assert_eq!(
            ciglet_rs_prediction_gain_f64(
                coefficients.as_ptr(),
                2,
                energy.as_ptr(),
                1,
                0,
                &mut gain
            ),
            2
        );
        assert_eq!(
            ciglet_rs_prediction_gain_f64(
                coefficients.as_ptr(),
                2,
                energy.as_ptr(),
                2,
                2,
                &mut gain
            ),
            3
        );
        assert_eq!(gain, 71.0);
        let silence = [0.0_f32; 3];
        let mut silent_coefficients = [71.0; 3];
        let mut silent_gain = 71.0;
        for policy in [0, 1] {
            assert_eq!(
                ciglet_rs_prediction_levinson_f32(
                    silence.as_ptr(),
                    3,
                    policy,
                    silent_coefficients.as_mut_ptr(),
                    3
                ),
                0
            );
            assert_eq!(silent_coefficients, [1.0, 0.0, 0.0]);
            assert_eq!(
                ciglet_rs_prediction_gain_f32(
                    silent_coefficients.as_ptr(),
                    3,
                    silence.as_ptr(),
                    3,
                    policy,
                    &mut silent_gain
                ),
                0
            );
            assert_eq!(silent_gain, 0.0);
        }
        assert_eq!(
            ciglet_rs_prediction_levinson_f32(std::ptr::null(), 0, 0, std::ptr::null_mut(), 0),
            3
        );
    }
}
