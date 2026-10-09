use ciglet_rs::statistics::Scalar;
use ciglet_rs_capi::*;
use std::ptr;
type Average<T> = unsafe extern "C" fn(*const T, usize, T, *mut T, usize) -> u32;
type AverageWith<T> = unsafe extern "C" fn(*const T, usize, T, u32, *mut T, usize) -> u32;
type Integer<T> = unsafe extern "C" fn(*const T, usize, usize, *mut T, usize) -> u32;
type IntegerWith<T> = unsafe extern "C" fn(*const T, usize, usize, u32, *mut T, usize) -> u32;
struct Api<T> {
    average: Average<T>,
    average_with: AverageWith<T>,
    rms: Integer<T>,
    rms_with: IntegerWith<T>,
    median: Integer<T>,
}
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
    fn values<T: Scalar>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.scalar()).collect()
    }
}
fn original<T: Scalar>(bytes: &[u8], width: usize, api: &Api<T>) {
    let mut r = Reader(bytes, width);
    assert_eq!(r.take(4), b"SMO1");
    assert_eq!(r.integer(), width);
    assert_eq!(r.integer(), 60);
    for case in 0..60 {
        let n = r.integer();
        let radius = r.scalar::<T>();
        let signal = r.values::<T>(n);
        let mean = r.values::<T>(n);
        let rms = r.values::<T>(n);
        let mut actual = vec![T::zero(); n];
        // SAFETY: Complete initialized signal and independent output, matching precision.
        unsafe {
            assert_eq!(
                (api.average_with)(signal.as_ptr(), n, radius, 1, actual.as_mut_ptr(), n),
                0
            );
            assert_eq!(actual, mean, "mean {case}");
            assert_eq!(
                (api.rms_with)(
                    signal.as_ptr(),
                    n,
                    radius.into() as usize + 1,
                    1,
                    actual.as_mut_ptr(),
                    n
                ),
                0
            );
            assert_eq!(actual, rms, "RMS {case}");
        }
    }
    assert!(r.0.is_empty());
}
fn median<T: Scalar>(bytes: &[u8], width: usize, api: &Api<T>) {
    let mut r = Reader(bytes, width);
    assert_eq!(r.take(4), b"MED1");
    assert_eq!(r.integer(), width);
    assert_eq!(r.integer(), 96);
    for case in 0..96 {
        let n = r.integer();
        let width = r.integer();
        let signal = r.values::<T>(n);
        let expected = r.values::<T>(n);
        let mut actual = vec![T::zero(); n];
        // SAFETY: Complete initialized signal and independent output, matching precision.
        unsafe {
            assert_eq!(
                (api.median)(signal.as_ptr(), n, width, actual.as_mut_ptr(), n),
                0
            );
        }
        assert_eq!(actual, expected, "median {case}");
    }
    assert!(r.0.is_empty());
}
fn stable<T: Scalar>(bytes: &[u8], width: usize, api: &Api<T>) {
    let mut r = Reader(bytes, width);
    assert_eq!(r.take(4), b"SSW1");
    assert_eq!(r.integer(), width);
    assert_eq!(r.integer(), 330);
    let mut outputs = 0;
    for case in 0..330 {
        let n = r.integer();
        let rms_width = r.integer();
        let half_width = r.scalar::<T>();
        let signal = r.values::<T>(n);
        let mean = r.values::<T>(n);
        let rms = r.values::<T>(n);
        let mut actual = vec![T::zero(); n];
        let mut selected = actual.clone();
        for (field, expected) in [mean, rms].into_iter().enumerate() {
            // SAFETY: Complete initialized signal and independent full writable outputs.
            unsafe {
                if field == 0 {
                    assert_eq!(
                        (api.average)(signal.as_ptr(), n, half_width, actual.as_mut_ptr(), n),
                        0
                    );
                    assert_eq!(
                        (api.average_with)(
                            signal.as_ptr(),
                            n,
                            half_width,
                            0,
                            selected.as_mut_ptr(),
                            n
                        ),
                        0
                    );
                } else {
                    assert_eq!(
                        (api.rms)(signal.as_ptr(), n, rms_width, actual.as_mut_ptr(), n),
                        0
                    );
                    assert_eq!(
                        (api.rms_with)(signal.as_ptr(), n, rms_width, 0, selected.as_mut_ptr(), n),
                        0
                    );
                }
            }
            assert_eq!(actual, selected);
            for (index, (&a, &e)) in actual.iter().zip(&expected).enumerate() {
                let a: f64 = a.into();
                let e: f64 = e.into();
                let error = if e == 0. { a.abs() } else { (a / e - 1.).abs() };
                let tolerance = if width == 4 { 2e-7 } else { 4e-15 };
                let quantum = if width == 4 {
                    f32::from_bits(1) as f64
                } else {
                    f64::from_bits(1)
                };
                let minimum = if width == 4 {
                    f32::MIN_POSITIVE as f64
                } else {
                    f64::MIN_POSITIVE
                };
                let subnormal = field == 1 && e.abs() < minimum && (a - e).abs() <= quantum;
                assert!(
                    (error <= tolerance && (e != 0. || a == 0.)) || subnormal,
                    "case {case} field {field} index {index}: {a:e} / {e:e}"
                );
                outputs += 1;
            }
        }
    }
    assert_eq!(outputs, 10_920);
    assert!(r.0.is_empty());
}
fn edges<T: Scalar>(api: &Api<T>, maximum: T) {
    let z = T::zero();
    let one = T::one();
    let two = T::from_f64(2.).unwrap();
    let sentinel = T::from_f64(71.).unwrap();
    let mut output = [sentinel; 4];
    let signal = [T::from_f64(4.).unwrap(), one, T::from_f64(9.).unwrap(), two];
    // SAFETY: Complete initialized inputs and full writable output; null used only
    // for empty ranges or intentional rejected pointer tests. Staging permits overlap.
    unsafe {
        assert_eq!(
            (api.median)(signal.as_ptr(), 4, 2, output.as_mut_ptr(), 4),
            0
        );
        assert_eq!(
            output,
            [
                T::from_f64(4.).unwrap(),
                T::from_f64(2.5).unwrap(),
                T::from_f64(5.).unwrap(),
                two
            ]
        );
        let mut inplace = [one, two, T::from_f64(8.).unwrap()];
        assert_eq!(
            (api.average)(inplace.as_ptr(), 3, one, inplace.as_mut_ptr(), 3),
            0
        );
        assert_eq!(
            inplace,
            [
                T::from_f64(1.5).unwrap(),
                T::from_f64(5.).unwrap(),
                T::from_f64(8.).unwrap()
            ]
        );
        assert_eq!(
            (api.rms)([-one, two].as_ptr(), 2, 0, output.as_mut_ptr(), 2),
            0
        );
        assert_eq!(&output[..2], &[one, two]);
        assert_eq!((api.rms)(&maximum, 1, 1, output.as_mut_ptr(), 1), 0);
        assert_eq!(output[0], maximum);
        output.fill(sentinel);
        assert_eq!((api.rms_with)(&maximum, 1, 1, 1, output.as_mut_ptr(), 1), 3);
        assert_eq!(output, [sentinel; 4]);
        assert_eq!(
            (api.average_with)(signal.as_ptr(), 4, one, 2, output.as_mut_ptr(), 4),
            3
        );
        assert_eq!(output, [sentinel; 4]);
        assert_eq!(
            (api.median)(signal.as_ptr(), 4, 0, output.as_mut_ptr(), 4),
            3
        );
        assert_eq!(output, [sentinel; 4]);
        assert_eq!(
            (api.median)(signal.as_ptr(), 4, 5, output.as_mut_ptr(), 4),
            3
        );
        assert_eq!(output, [sentinel; 4]);
        assert_eq!(
            (api.average)(signal.as_ptr(), 4, -one, output.as_mut_ptr(), 4),
            3
        );
        assert_eq!(output, [sentinel; 4]);
        assert_eq!(
            (api.rms)(signal.as_ptr(), 4, usize::MAX, output.as_mut_ptr(), 4),
            3
        );
        assert_eq!(output, [sentinel; 4]);
        assert_eq!((api.rms)(signal.as_ptr(), 4, 1, output.as_mut_ptr(), 3), 2);
        assert_eq!(output, [sentinel; 4]);
        let nan = T::from_f64(f64::NAN).unwrap();
        assert_eq!((api.average)(&nan, 1, one, output.as_mut_ptr(), 1), 3);
        assert_eq!(output, [sentinel; 4]);
        assert_eq!((api.median)(ptr::null(), 0, 3, ptr::null_mut(), 0), 0);
        assert_eq!((api.average)(ptr::null(), 0, one, ptr::null_mut(), 0), 0);
        assert_eq!((api.rms)(ptr::null(), 0, 1, ptr::null_mut(), 0), 0);
        assert_eq!((api.average)(ptr::null(), 0, -one, ptr::null_mut(), 0), 3);
        assert_eq!(
            (api.median)([maximum; 4].as_ptr(), 4, 2, output.as_mut_ptr(), 4),
            0
        );
        assert_eq!(output, [maximum; 4]);
        let mut signed = [-z, one];
        assert_eq!(
            (api.average)(signed.as_ptr(), 2, z, signed.as_mut_ptr(), 2),
            0
        );
        assert_eq!(signed[0].into().to_bits(), (-0.0f64).to_bits());
    }
}
fn float_api() -> Api<f32> {
    Api {
        average: ciglet_rs_moving_average_f32,
        average_with: ciglet_rs_moving_average_with_f32,
        rms: ciglet_rs_moving_rms_f32,
        rms_with: ciglet_rs_moving_rms_with_f32,
        median: ciglet_rs_median_filter_f32,
    }
}
#[test]
fn float_original_reference() {
    original(
        include_bytes!("../../../tests/fixtures/smoothing-float.bin"),
        4,
        &float_api(),
    );
}
#[test]
fn float_median_reference() {
    median(
        include_bytes!("../../../tests/fixtures/median-filter-float.bin"),
        4,
        &float_api(),
    );
}
#[test]
fn float_stable_reference() {
    stable(
        include_bytes!("../../../tests/fixtures/stable_smoothing-float.bin"),
        4,
        &float_api(),
    );
}
#[test]
fn float_edges() {
    edges(&float_api(), f32::MAX);
}
fn double_api() -> Api<f64> {
    Api {
        average: ciglet_rs_moving_average_f64,
        average_with: ciglet_rs_moving_average_with_f64,
        rms: ciglet_rs_moving_rms_f64,
        rms_with: ciglet_rs_moving_rms_with_f64,
        median: ciglet_rs_median_filter_f64,
    }
}
#[test]
fn double_original_reference() {
    original(
        include_bytes!("../../../tests/fixtures/smoothing-double.bin"),
        8,
        &double_api(),
    );
}
#[test]
fn double_median_reference() {
    median(
        include_bytes!("../../../tests/fixtures/median-filter-double.bin"),
        8,
        &double_api(),
    );
}
#[test]
fn double_stable_reference() {
    stable(
        include_bytes!("../../../tests/fixtures/stable_smoothing-double.bin"),
        8,
        &double_api(),
    );
}
#[test]
fn double_edges() {
    edges(&double_api(), f64::MAX);
}
