#![cfg(feature = "c-api")]
use ciglet_rs::{c_api::*, statistics::Scalar};
use std::ptr;
type Fir<T> = unsafe extern "C" fn(usize, u32, T, T, u32, *mut T, usize) -> u32;
type Simple<T> = unsafe extern "C" fn(usize, T, u32, u32, *mut T, usize) -> u32;
type Band<T> = unsafe extern "C" fn(usize, T, T, u32, *mut T, usize) -> u32;
type Convolution<T> = unsafe extern "C" fn(*const T, usize, *const T, usize, *mut T, usize) -> u32;
type Filter<T> = unsafe extern "C" fn(
    *const T,
    usize,
    *const T,
    usize,
    *const T,
    usize,
    u32,
    *mut T,
    usize,
) -> u32;
struct Api<T> {
    fir: Fir<T>,
    simple: Simple<T>,
    band: Band<T>,
    convolve: Convolution<T>,
    filter: Filter<T>,
    backward: Filter<T>,
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
fn recursive<T: Scalar>(bytes: &[u8], width: usize, api: &Api<T>) {
    let mut r = Reader(bytes);
    assert_eq!(r.take(4), b"IIR1");
    assert_eq!(r.integer(), width);
    assert_eq!(r.integer(), 210);
    for case in 0..210 {
        let nb = r.integer();
        let na = r.integer();
        let nx = r.integer();
        let b = r.values::<T>(width, nb);
        let a = r.values::<T>(width, na);
        let x = r.values::<T>(width, nx);
        for startup in [1, 0] {
            for function in [api.filter, api.backward] {
                let expected = r.values::<T>(width, nx);
                let mut actual = vec![T::zero(); nx];
                // SAFETY: Complete initialized input slices, independent writable output;
                // function pointers have the matching precision-specific ABI.
                unsafe {
                    assert_eq!(
                        function(
                            x.as_ptr(),
                            nx,
                            b.as_ptr(),
                            nb,
                            a.as_ptr(),
                            na,
                            startup,
                            actual.as_mut_ptr(),
                            nx
                        ),
                        0
                    );
                }
                assert_eq!(actual, expected, "case {case} startup {startup}");
            }
        }
    }
    assert!(r.0.is_empty());
}
fn fir_convolution<T: Scalar>(bytes: &[u8], width: usize, api: &Api<T>) {
    let mut r = Reader(bytes);
    assert_eq!(r.take(4), b"FRS1");
    assert_eq!(r.integer(), width);
    let counts = [r.integer(), r.integer(), r.integer()];
    assert_eq!(counts, [300, 60, 105]);
    let bound = if width == 4 { 2e-7 } else { 2e-15 };
    for case in 0..300 {
        let n = r.integer();
        let window = r.integer() as u32;
        let band = r.integer() as u32;
        let lo = r.scalar::<T>(width);
        let hi = r.scalar::<T>(width);
        let expected = r.values::<T>(width, n);
        let mut actual = vec![T::zero(); n];
        let mut convenience = vec![T::zero(); n];
        // SAFETY: Independent full-sized outputs and valid precision-specific functions.
        unsafe {
            assert_eq!(
                (api.fir)(n, band, lo, hi, window, actual.as_mut_ptr(), n),
                0
            );
            let two = T::from_f64(2.).unwrap();
            if band < 2 {
                assert_eq!(
                    (api.simple)(n, lo * two, band, window, convenience.as_mut_ptr(), n),
                    0
                );
                assert_eq!(actual, convenience);
            } else if band == 2 {
                assert_eq!(
                    (api.band)(n, lo * two, hi * two, window, convenience.as_mut_ptr(), n),
                    0
                );
                assert_eq!(actual, convenience);
            }
        }
        for (&a, &e) in actual.iter().zip(&expected) {
            let error = (a.into() - e.into()).abs() / e.into().abs().max(1.);
            assert!(error < bound, "FIR {case}: {error:e}");
        }
    }
    for case in 0..60 {
        let nx = r.integer();
        let nh = r.integer();
        let x = r.values::<T>(width, nx);
        let h = r.values::<T>(width, nh);
        let n = nx + nh - 1;
        let expected = r.values::<T>(width, n);
        let mut actual = vec![T::zero(); n];
        // SAFETY: Complete initialized signal/impulse and independent full output.
        unsafe {
            assert_eq!(
                (api.convolve)(x.as_ptr(), nx, h.as_ptr(), nh, actual.as_mut_ptr(), n),
                0
            );
        }
        for (&a, &e) in actual.iter().zip(&expected) {
            let error = (a.into() - e.into()).abs() / e.into().abs().max(1.);
            assert!(error < bound, "convolution {case}: {error:e}");
        }
    }
    // Consume separate resampling records without claiming their ABI coverage.
    for _ in 0..105 {
        let nx = r.integer();
        let _ratio = r.scalar::<T>(width);
        r.values::<T>(width, nx);
        let ny = r.integer();
        r.values::<T>(width, ny);
    }
    assert!(r.0.is_empty());
}
fn edges<T: Scalar>(api: &Api<T>, maximum: T) {
    let z = T::zero();
    let one = T::one();
    let two = T::from_f64(2.).unwrap();
    let sentinel = T::from_f64(71.).unwrap();
    let signal = [one, z, z];
    let b = [T::from_f64(0.5).unwrap(), z];
    let a = [one, T::from_f64(-0.5).unwrap()];
    let mut result = [sentinel; 3];
    // SAFETY: Initialized independent coefficient arrays and full writable output.
    // In-place calls use native staging; null pointers are passed only for empty
    // ranges or intentional pointer-error tests with no borrowing.
    unsafe {
        // Original FRS1 covers five windows; retain the additional public
        // Nuttall98 and MLT sine selectors using native-kernel equivalence.
        for window in [5, 6] {
            for passband in 0..4 {
                let mut taps = [sentinel; 8];
                assert_eq!(
                    (api.fir)(
                        8,
                        passband,
                        T::from_f64(0.125).unwrap(),
                        T::from_f64(0.375).unwrap(),
                        window,
                        taps.as_mut_ptr(),
                        8
                    ),
                    0
                );
                let native_window = if window == 5 {
                    ciglet_rs::window::Window::Nuttall98
                } else {
                    ciglet_rs::window::Window::MltSine
                };
                use ciglet_rs::filtering::Passband;
                let lower = T::from_f64(0.125).unwrap();
                let upper = T::from_f64(0.375).unwrap();
                let band = match passband {
                    0 => Passband::Lowpass(lower),
                    1 => Passband::Highpass(lower),
                    2 => Passband::Bandpass { lower, upper },
                    _ => Passband::Bandstop { lower, upper },
                };
                assert_eq!(
                    taps.as_slice(),
                    ciglet_rs::filtering::windowed_fir(8, band, native_window).unwrap()
                );
            }
        }
        assert_eq!(
            (api.filter)(
                signal.as_ptr(),
                3,
                b.as_ptr(),
                2,
                a.as_ptr(),
                2,
                0,
                result.as_mut_ptr(),
                3
            ),
            0
        );
        assert_eq!(
            result,
            [
                T::from_f64(0.5).unwrap(),
                T::from_f64(0.25).unwrap(),
                T::from_f64(0.125).unwrap()
            ]
        );
        assert_eq!(
            (api.filter)(
                signal.as_ptr(),
                3,
                b.as_ptr(),
                2,
                a.as_ptr(),
                2,
                1,
                result.as_mut_ptr(),
                3
            ),
            0
        );
        assert_eq!(result, [z; 3]);
        let mut inplace = [one, two, T::from_f64(3.).unwrap()];
        assert_eq!(
            (api.backward)(
                inplace.as_ptr(),
                3,
                [one, two].as_ptr(),
                2,
                ptr::null(),
                0,
                0,
                inplace.as_mut_ptr(),
                3
            ),
            0
        );
        assert_eq!(
            inplace,
            [
                T::from_f64(9.).unwrap(),
                T::from_f64(18.).unwrap(),
                T::from_f64(7.).unwrap()
            ]
        );
        result.fill(sentinel);
        assert_eq!(
            (api.filter)(
                signal.as_ptr(),
                3,
                b.as_ptr(),
                2,
                a.as_ptr(),
                2,
                2,
                result.as_mut_ptr(),
                3
            ),
            3
        );
        assert_eq!(result, [sentinel; 3]);
        assert_eq!(
            (api.filter)(
                signal.as_ptr(),
                3,
                b.as_ptr(),
                2,
                a.as_ptr(),
                2,
                0,
                result.as_mut_ptr(),
                2
            ),
            2
        );
        assert_eq!(result, [sentinel; 3]);
        assert_eq!(
            (api.filter)(
                [one, maximum, one].as_ptr(),
                3,
                &two,
                1,
                ptr::null(),
                0,
                0,
                result.as_mut_ptr(),
                3
            ),
            3
        );
        assert_eq!(result, [sentinel; 3]);
        let nan = T::from_f64(f64::NAN).unwrap();
        assert_eq!(
            (api.filter)(
                signal.as_ptr(),
                3,
                &one,
                1,
                &nan,
                1,
                0,
                result.as_mut_ptr(),
                3
            ),
            3
        );
        assert_eq!(result, [sentinel; 3]);
        assert_eq!(
            (api.convolve)(ptr::null(), 0, &nan, 1, ptr::null_mut(), 0),
            3
        );
        for function in [api.filter, api.backward] {
            assert_eq!(
                function(
                    ptr::null(),
                    0,
                    ptr::null(),
                    0,
                    ptr::null(),
                    0,
                    0,
                    ptr::null_mut(),
                    0
                ),
                0
            );
        }
        assert_eq!(
            (api.convolve)(ptr::null(), 0, &one, 1, ptr::null_mut(), 0),
            0
        );
        assert_eq!(
            (api.convolve)(ptr::null(), usize::MAX, &one, 1, ptr::null_mut(), 0),
            2
        );
        assert_eq!((api.fir)(3, 4, z, z, 0, result.as_mut_ptr(), 3), 3);
        assert_eq!(result, [sentinel; 3]);
        assert_eq!((api.fir)(3, 0, z, z, 7, result.as_mut_ptr(), 3), 3);
        assert_eq!(result, [sentinel; 3]);
        assert_eq!((api.simple)(3, z, 2, 0, result.as_mut_ptr(), 3), 3);
        assert_eq!(result, [sentinel; 3]);
        assert_eq!((api.fir)(1, 0, z, z, 0, result.as_mut_ptr(), 1), 3);
        assert_eq!(result, [sentinel; 3]);
        assert_eq!((api.band)(3, two, one, 0, result.as_mut_ptr(), 3), 3);
        assert_eq!(result, [sentinel; 3]);
        let mut convolution = [one, two, T::from_f64(3.).unwrap(), sentinel];
        assert_eq!(
            (api.convolve)(
                convolution.as_ptr(),
                3,
                [two, -one].as_ptr(),
                2,
                convolution.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(
            convolution,
            [
                two,
                T::from_f64(3.).unwrap(),
                T::from_f64(4.).unwrap(),
                T::from_f64(-3.).unwrap()
            ]
        );
    }
}
fn float_api() -> Api<f32> {
    Api {
        fir: ciglet_rs_windowed_fir_f32,
        simple: ciglet_rs_fir_f32,
        band: ciglet_rs_bandpass_fir_f32,
        convolve: ciglet_rs_convolve_f32,
        filter: ciglet_rs_filter_f32,
        backward: ciglet_rs_forward_backward_filter_f32,
    }
}
#[test]
fn float_recursive_reference() {
    recursive(
        include_bytes!("fixtures/recursive-filter-float.bin"),
        4,
        &float_api(),
    );
}
#[test]
fn float_fir_and_convolution_reference() {
    fir_convolution(
        include_bytes!("fixtures/c-filter-resample-f32.bin"),
        4,
        &float_api(),
    );
}
#[test]
fn float_atomic_errors_and_overlap() {
    edges(&float_api(), f32::MAX);
}
fn double_api() -> Api<f64> {
    Api {
        fir: ciglet_rs_windowed_fir_f64,
        simple: ciglet_rs_fir_f64,
        band: ciglet_rs_bandpass_fir_f64,
        convolve: ciglet_rs_convolve_f64,
        filter: ciglet_rs_filter_f64,
        backward: ciglet_rs_forward_backward_filter_f64,
    }
}
#[test]
fn double_recursive_reference() {
    recursive(
        include_bytes!("fixtures/recursive-filter-double.bin"),
        8,
        &double_api(),
    );
}
#[test]
fn double_fir_and_convolution_reference() {
    fir_convolution(
        include_bytes!("fixtures/c-filter-resample-f64.bin"),
        8,
        &double_api(),
    );
}
#[test]
fn double_atomic_errors_and_overlap() {
    edges(&double_api(), f64::MAX);
}
