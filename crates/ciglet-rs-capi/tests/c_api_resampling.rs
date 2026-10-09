use ciglet_rs::{
    resampling::{self, BoundaryPolicy, KernelPolicy},
    statistics::Scalar,
};
use ciglet_rs_capi::*;
use std::ptr;
type Length<T> = unsafe extern "C" fn(usize, T, *mut usize) -> u32;
type Default<T> = unsafe extern "C" fn(*const T, usize, T, *mut T, usize) -> u32;
type Boundary<T> = unsafe extern "C" fn(*const T, usize, T, u32, *mut T, usize) -> u32;
type Full<T> = unsafe extern "C" fn(*const T, usize, T, u32, u32, *mut T, usize) -> u32;
struct Api<T> {
    length: Length<T>,
    default: Default<T>,
    boundary: Boundary<T>,
    full: Full<T>,
}
struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn take(&mut self, n: usize) -> &[u8] {
        let (a, b) = self.0.split_at(n);
        self.0 = b;
        a
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.take(4).try_into().unwrap()) as usize
    }
    fn scalar<T: Scalar>(&mut self, width: usize) -> T {
        T::from_f64(if width == 4 {
            f32::from_le_bytes(self.take(4).try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.take(8).try_into().unwrap())
        })
        .unwrap()
    }
    fn values<T: Scalar>(&mut self, width: usize, n: usize) -> Vec<T> {
        (0..n).map(|_| self.scalar(width)).collect()
    }
}
fn reference<T: Scalar>(bytes: &[u8], width: usize, api: &Api<T>) {
    let mut r = Reader(bytes);
    assert_eq!(r.take(4), b"FRS1");
    assert_eq!(r.integer(), width);
    assert_eq!([r.integer(), r.integer(), r.integer()], [300, 60, 105]);
    // FIR/convolution portions already have independent boundary tests; consume
    // records here without extending this resampling coverage claim.
    for _ in 0..300 {
        let n = r.integer();
        r.integer();
        r.integer();
        r.take((n + 2) * width);
    }
    for _ in 0..60 {
        let nx = r.integer();
        let nh = r.integer();
        r.take((nx + nh + nx + nh - 1) * width);
    }
    for case in 0..105 {
        let nx = r.integer();
        let ratio = r.scalar::<T>(width);
        let signal = r.values::<T>(width, nx);
        let ny = r.integer();
        let expected = r.values::<T>(width, ny);
        let mut count = 71;
        let mut actual = vec![T::zero(); ny];
        let mut convenience = vec![T::zero(); ny];
        // SAFETY: Initialized complete signal, independent writable count/scalars;
        // function pointer precision matches arrays and owned output staging.
        unsafe {
            assert_eq!((api.length)(nx, ratio, &mut count), 0);
            assert_eq!(count, ny);
            assert_eq!(
                (api.full)(signal.as_ptr(), nx, ratio, 1, 1, actual.as_mut_ptr(), ny),
                0
            );
            assert_eq!(
                (api.boundary)(signal.as_ptr(), nx, ratio, 1, convenience.as_mut_ptr(), ny),
                0
            );
            assert_eq!(actual, convenience);
        }
        let bound = if width == 4 { 2e-7 } else { 2e-15 };
        for (&a, &e) in actual.iter().zip(&expected) {
            let error = (a.into() - e.into()).abs() / e.into().abs().max(1.);
            assert!(error < bound, "resample {case}: {error:e}");
        }
    }
    assert!(r.0.is_empty());
}
fn policies<T: Scalar>(api: &Api<T>) {
    for n in [2, 5, 32, 65] {
        let signal: Vec<T> = (0..n)
            .map(|i| T::from_f64((i % 7) as f64 * 0.125 - 0.375).unwrap())
            .collect();
        for ratio in [0.9, 1.1, 1.5, 2.0] {
            let ratio = T::from_f64(ratio).unwrap();
            let mut count = 71;
            // SAFETY: Independent writable count scalar, valid precision function pointer.
            unsafe {
                assert_eq!((api.length)(n, ratio, &mut count), 0);
            }
            let mut actual = vec![T::zero(); count];
            let mut convenience = actual.clone();
            for (bc, boundary) in [
                (0, BoundaryPolicy::IncludeFirst),
                (1, BoundaryPolicy::LegacySkipFirst),
            ] {
                for (kc, kernel) in [(0, KernelPolicy::Stable), (1, KernelPolicy::Legacy)] {
                    // SAFETY: Full initialized readable signal and independent complete output.
                    unsafe {
                        assert_eq!(
                            (api.full)(
                                signal.as_ptr(),
                                n,
                                ratio,
                                bc,
                                kc,
                                actual.as_mut_ptr(),
                                count
                            ),
                            0
                        );
                    }
                    assert_eq!(
                        actual,
                        resampling::resample_with_kernel(&signal, ratio, boundary, kernel).unwrap()
                    );
                    if bc == 0 && kc == 0 {
                        // SAFETY: Same complete input/output contracts for default convenience.
                        unsafe {
                            assert_eq!(
                                (api.default)(
                                    signal.as_ptr(),
                                    n,
                                    ratio,
                                    convenience.as_mut_ptr(),
                                    count
                                ),
                                0
                            );
                        }
                        assert_eq!(actual, convenience);
                    }
                }
            }
        }
    }
}
fn edges<T: Scalar>(api: &Api<T>, maximum: T) {
    let one = T::one();
    let zero = T::zero();
    let sentinel = T::from_f64(71.).unwrap();
    let mut count = 71;
    let mut output = [sentinel; 2];
    // SAFETY: Independent writable results; initialized full signal for nonempty
    // ranges, null only for empty ranges or deliberately rejected pointer cases.
    unsafe {
        assert_eq!((api.length)(i32::MAX as usize, one, &mut count), 0);
        assert_eq!(count, i32::MAX as usize);
        assert_eq!((api.length)(usize::MAX, one, &mut count), 2);
        assert_eq!(count, i32::MAX as usize);
        assert_eq!((api.length)(0, one, ptr::null_mut()), 1);
        for ratio in [
            zero,
            -one,
            T::from_f64(f64::NAN).unwrap(),
            T::from_f64(f64::INFINITY).unwrap(),
            maximum,
        ] {
            count = 71;
            assert_eq!((api.length)(2, ratio, &mut count), 3);
            assert_eq!(count, 71);
        }
        let signal = [T::from_f64(0.75).unwrap()];
        let two = T::from_f64(2.).unwrap();
        for kernel in [0, 1] {
            assert_eq!(
                (api.full)(signal.as_ptr(), 1, two, 1, kernel, output.as_mut_ptr(), 2),
                0
            );
            assert_eq!(output, [zero; 2]);
            assert_eq!(
                (api.full)(signal.as_ptr(), 1, two, 0, kernel, output.as_mut_ptr(), 2),
                0
            );
            assert_eq!(output[0], signal[0]);
        }
        output.fill(sentinel);
        for (boundary, kernel) in [(2, 0), (0, 2)] {
            assert_eq!(
                (api.full)(
                    signal.as_ptr(),
                    1,
                    two,
                    boundary,
                    kernel,
                    output.as_mut_ptr(),
                    2
                ),
                3
            );
            assert_eq!(output, [sentinel; 2]);
        }
        assert_eq!(
            (api.default)(signal.as_ptr(), 1, two, output.as_mut_ptr(), 1),
            2
        );
        assert_eq!(output, [sentinel; 2]);
        assert_eq!(
            (api.default)(ptr::null(), 1, two, output.as_mut_ptr(), 2),
            1
        );
        assert_eq!(output, [sentinel; 2]);
        assert_eq!((api.default)(ptr::null(), 0, two, ptr::null_mut(), 0), 0);
        let quarter = T::from_f64(0.25).unwrap();
        assert_eq!(
            (api.default)(signal.as_ptr(), 1, quarter, ptr::null_mut(), 0),
            0
        );
        let nan = T::from_f64(f64::NAN).unwrap();
        assert_eq!((api.default)(&nan, 1, quarter, ptr::null_mut(), 0), 3);
        let mut overlap = [one, zero, sentinel, sentinel];
        assert_eq!(
            (api.full)(overlap.as_ptr(), 2, two, 0, 0, overlap.as_mut_ptr(), 4),
            0
        );
        assert_eq!(overlap[0], one);
    }
}
fn float_api() -> Api<f32> {
    Api {
        length: ciglet_rs_resample_output_length_f32,
        default: ciglet_rs_resample_f32,
        boundary: ciglet_rs_resample_with_f32,
        full: ciglet_rs_resample_with_kernel_f32,
    }
}
#[test]
fn float_original_reference() {
    reference(
        include_bytes!("../../../tests/fixtures/c-filter-resample-f32.bin"),
        4,
        &float_api(),
    );
}
#[test]
fn float_policies_and_reuse() {
    policies(&float_api());
}
#[test]
fn float_atomic_shape_errors_and_overlap() {
    edges(&float_api(), f32::MAX);
}
fn double_api() -> Api<f64> {
    Api {
        length: ciglet_rs_resample_output_length_f64,
        default: ciglet_rs_resample_f64,
        boundary: ciglet_rs_resample_with_f64,
        full: ciglet_rs_resample_with_kernel_f64,
    }
}
#[test]
fn double_original_reference() {
    reference(
        include_bytes!("../../../tests/fixtures/c-filter-resample-f64.bin"),
        8,
        &double_api(),
    );
}
#[test]
fn double_policies_and_reuse() {
    policies(&double_api());
}
#[test]
fn double_atomic_shape_errors_and_overlap() {
    edges(&double_api(), f64::MAX);
}

#[test]
fn ratio_one_preserves_bits_and_large_float_counts_bypass_rounding() {
    let mut float = [-0.0f32, f32::from_bits(1), f32::MAX];
    let mut double = [-0.0f64, f64::from_bits(1), f64::MAX];
    let float_bits = float.map(f32::to_bits);
    let double_bits = double.map(f64::to_bits);
    let mut count = 71;
    // SAFETY: Initialized in-place scalars and independent count; full staging.
    unsafe {
        assert_eq!(
            ciglet_rs_resample_f32(float.as_ptr(), 3, 1., float.as_mut_ptr(), 3),
            0
        );
        assert_eq!(
            ciglet_rs_resample_f64(double.as_ptr(), 3, 1., double.as_mut_ptr(), 3),
            0
        );
        assert_eq!(
            ciglet_rs_resample_output_length_f32(16_777_217, 1., &mut count),
            0
        );
        assert_eq!(count, 16_777_217);
        assert_eq!(
            ciglet_rs_resample_output_length_f32(16_777_217, 0.5, &mut count),
            0
        );
        assert_eq!(count, 8_388_608);
        assert_eq!(
            ciglet_rs_resample_output_length_f64(16_777_217, 0.5, &mut count),
            0
        );
        assert_eq!(count, 8_388_609);
    }
    assert_eq!(float.map(f32::to_bits), float_bits);
    assert_eq!(double.map(f64::to_bits), double_bits);
}
#[test]
fn near_integer_stable_limit_has_independent_analytic_reference() {
    let input = [0.0, 0.75, 0.0];
    let ratio = 1. + 2.0f64.powi(-50);
    let mut stable = [71.; 3];
    let mut legacy = [71.; 3];
    // SAFETY: Independent initialized input and complete exclusively writable outputs.
    unsafe {
        assert_eq!(
            ciglet_rs_resample_with_kernel_f64(
                input.as_ptr(),
                3,
                ratio,
                0,
                0,
                stable.as_mut_ptr(),
                3
            ),
            0
        );
        assert_eq!(
            ciglet_rs_resample_with_kernel_f64(
                input.as_ptr(),
                3,
                ratio,
                0,
                1,
                legacy.as_mut_ptr(),
                3
            ),
            0
        );
    }
    let angle = std::f64::consts::PI * (1. / ratio - 1.);
    let expected = 0.75 * (angle.sin() / angle) * ((angle / 4.).sin() / (angle / 4.));
    assert!((stable[1] - expected).abs() <= 3e-16);
    assert!(legacy[1] > 1e16);
}
