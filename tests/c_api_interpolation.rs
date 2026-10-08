#![cfg(feature = "c-api")]
use ciglet_rs::{c_api::*, statistics::Scalar};
use std::ptr;
type Between<T> = unsafe extern "C" fn(T, T, T, *mut T) -> u32;
type Linear<T> =
    unsafe extern "C" fn(*const T, *const T, usize, *const T, usize, *mut T, usize) -> u32;
type Uniform<T> =
    unsafe extern "C" fn(T, T, *const T, usize, *const T, usize, *mut T, usize) -> u32;
type SincWith<T> =
    unsafe extern "C" fn(T, T, *const T, usize, *const T, usize, u32, *mut T, usize) -> u32;
type SincFull<T> =
    unsafe extern "C" fn(T, T, *const T, usize, *const T, usize, u32, u32, *mut T, usize) -> u32;
type Fill<T> = unsafe extern "C" fn(*const T, usize, T, *mut T, usize) -> u32;
struct Api<T> {
    between: Between<T>,
    linear: Linear<T>,
    uniform: Uniform<T>,
    sinc: Uniform<T>,
    sinc_with: SincWith<T>,
    sinc_full: SincFull<T>,
    fill: Fill<T>,
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
fn linear<T: Scalar>(bytes: &[u8], width: usize, api: &Api<T>) {
    let mut r = Reader(bytes, width);
    assert_eq!(r.take(4), b"INT1");
    assert_eq!(r.integer(), width);
    assert_eq!(r.integer(), 24);
    for case in 0..24 {
        let kind = r.integer();
        let count = r.integer();
        let queries = r.integer();
        let start = r.scalar::<T>();
        let end = r.scalar::<T>();
        let coordinates = r.values::<T>(count);
        let values = r.values::<T>(count);
        let positions = r.values::<T>(queries);
        let expected = r.values::<T>(queries);
        let mut actual = vec![T::zero(); queries];
        // SAFETY: Full initialized inputs and independent complete writable output.
        unsafe {
            let status = if kind == 0 {
                (api.linear)(
                    coordinates.as_ptr(),
                    values.as_ptr(),
                    count,
                    positions.as_ptr(),
                    queries,
                    actual.as_mut_ptr(),
                    queries,
                )
            } else {
                (api.uniform)(
                    start,
                    end,
                    values.as_ptr(),
                    count,
                    positions.as_ptr(),
                    queries,
                    actual.as_mut_ptr(),
                    queries,
                )
            };
            assert_eq!(status, 0);
        }
        assert_eq!(actual, expected, "linear case {case}");
    }
    assert!(r.0.is_empty());
}
fn extended<T: Scalar>(bytes: &[u8], width: usize, boundary: u32, api: &Api<T>) {
    let mut r = Reader(bytes, width);
    assert_eq!(r.take(4), b"EXT1");
    assert_eq!(r.integer(), width);
    assert_eq!(r.integer(), 28);
    for case in 0..28 {
        let kind = r.integer();
        let count = r.integer();
        let queries = r.integer();
        let start = r.scalar::<T>();
        let end = r.scalar::<T>();
        let values = r.values::<T>(count);
        let positions = if kind == 0 {
            r.values::<T>(queries)
        } else {
            Vec::new()
        };
        let expected = r.values::<T>(queries);
        let mut actual = vec![T::zero(); queries];
        let mut convenience = actual.clone();
        // SAFETY: Complete initialized input values, including permitted sentinel
        // NaNs, independent query/output buffers and matching precision functions.
        unsafe {
            let status = if kind == 0 {
                assert_eq!(
                    (api.sinc_with)(
                        start,
                        end,
                        values.as_ptr(),
                        count,
                        positions.as_ptr(),
                        queries,
                        boundary,
                        convenience.as_mut_ptr(),
                        queries
                    ),
                    0
                );
                (api.sinc_full)(
                    start,
                    end,
                    values.as_ptr(),
                    count,
                    positions.as_ptr(),
                    queries,
                    boundary,
                    1,
                    actual.as_mut_ptr(),
                    queries,
                )
            } else {
                (api.fill)(values.as_ptr(), count, start, actual.as_mut_ptr(), queries)
            };
            assert_eq!(status, 0);
            if kind == 0 {
                assert_eq!(actual, convenience);
            }
        }
        assert_eq!(actual, expected, "extended case {case}");
    }
    assert!(r.0.is_empty());
}
fn edges<T: Scalar>(api: &Api<T>, maximum: T) {
    let zero = T::zero();
    let one = T::one();
    let two = T::from_f64(2.).unwrap();
    let sentinel = T::from_f64(71.).unwrap();
    let mut output = [sentinel; 4];
    let mut scalar = sentinel;
    let coordinates: Vec<T> = [0., 2., 5.]
        .into_iter()
        .map(|v| T::from_f64(v).unwrap())
        .collect();
    let values: Vec<T> = [1., 5., -1.]
        .into_iter()
        .map(|v| T::from_f64(v).unwrap())
        .collect();
    let positions: Vec<T> = [4., 1., -9., 10.]
        .into_iter()
        .map(|v| T::from_f64(v).unwrap())
        .collect();
    // SAFETY: Initialized stable input arrays and complete exclusively writable
    // outputs. In-place queries are staged before commit; null empty ranges remain
    // unborrowed. All failed calls retain the initialized output buffers.
    unsafe {
        assert_eq!(
            (api.linear)(
                coordinates.as_ptr(),
                values.as_ptr(),
                3,
                positions.as_ptr(),
                4,
                output.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(output, [one, T::from_f64(3.).unwrap(), one, -one]);
        output.fill(sentinel);
        assert_eq!(
            (api.linear)(
                [zero, zero].as_ptr(),
                [one, two].as_ptr(),
                2,
                positions.as_ptr(),
                4,
                output.as_mut_ptr(),
                4
            ),
            3
        );
        assert_eq!(output, [sentinel; 4]);
        assert_eq!(
            (api.linear)(
                coordinates.as_ptr(),
                values.as_ptr(),
                3,
                positions.as_ptr(),
                4,
                output.as_mut_ptr(),
                3
            ),
            2
        );
        assert_eq!(output, [sentinel; 4]);
        assert_eq!(
            (api.uniform)(
                zero,
                two,
                &one,
                1,
                positions.as_ptr(),
                4,
                output.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(output, [one; 4]);
        assert_eq!(
            (api.uniform)(zero, two, &one, 1, ptr::null(), 0, ptr::null_mut(), 0),
            0
        );
        assert_eq!(
            (api.uniform)(
                zero,
                two,
                ptr::null(),
                0,
                ptr::null(),
                0,
                ptr::null_mut(),
                0
            ),
            3
        );
        for boundary in [0, 1] {
            for kernel in [0, 1] {
                assert_eq!(
                    (api.sinc_full)(
                        zero,
                        two,
                        [one, zero].as_ptr(),
                        2,
                        &zero,
                        1,
                        boundary,
                        kernel,
                        output.as_mut_ptr(),
                        1
                    ),
                    0
                );
                assert_eq!(output[0], if boundary == 0 { one } else { zero });
            }
        }
        let mut query = [
            zero,
            T::from_f64(100.).unwrap(),
            T::from_f64(-100.).unwrap(),
        ];
        assert_eq!(
            (api.sinc)(
                zero,
                two,
                [one, zero].as_ptr(),
                2,
                query.as_ptr(),
                3,
                query.as_mut_ptr(),
                3
            ),
            0
        );
        assert_eq!(query, [one, zero, zero]);
        output.fill(sentinel);
        assert_eq!(
            (api.sinc_full)(
                zero,
                two,
                [one, zero].as_ptr(),
                2,
                &zero,
                1,
                0,
                2,
                output.as_mut_ptr(),
                1
            ),
            3
        );
        assert_eq!(output, [sentinel; 4]);
        let nan = T::from_f64(f64::NAN).unwrap();
        let mut missing = [nan, two, nan, T::from_f64(6.).unwrap(), nan];
        assert_eq!(
            (api.fill)(missing.as_ptr(), 5, nan, missing.as_mut_ptr(), 5),
            0
        );
        assert_eq!(
            missing,
            [
                two,
                two,
                T::from_f64(4.).unwrap(),
                T::from_f64(6.).unwrap(),
                T::from_f64(6.).unwrap()
            ]
        );
        assert_eq!((api.fill)(ptr::null(), 0, nan, ptr::null_mut(), 0), 0);
        assert_eq!(
            (api.fill)(
                [one, T::from_f64(f64::INFINITY).unwrap()].as_ptr(),
                2,
                nan,
                output.as_mut_ptr(),
                2
            ),
            3
        );
        assert_eq!(output, [sentinel; 4]);
        assert_eq!(
            (api.between)(-maximum, maximum, T::from_f64(0.5).unwrap(), &mut scalar),
            0
        );
        assert_eq!(scalar, zero);
        scalar = sentinel;
        assert_eq!((api.between)(zero, one, nan, &mut scalar), 3);
        assert_eq!(scalar, sentinel);
        assert_eq!((api.between)(zero, one, one, ptr::null_mut()), 1);
    }
}
fn float_api() -> Api<f32> {
    Api {
        between: ciglet_rs_interpolate_between_f32,
        linear: ciglet_rs_interpolate_linear_f32,
        uniform: ciglet_rs_interpolate_uniform_f32,
        sinc: ciglet_rs_interpolate_sinc_f32,
        sinc_with: ciglet_rs_interpolate_sinc_with_f32,
        sinc_full: ciglet_rs_interpolate_sinc_with_kernel_f32,
        fill: ciglet_rs_interpolate_fill_missing_f32,
    }
}
#[test]
fn float_linear_original() {
    linear(
        include_bytes!("fixtures/interpolation-linear-float.bin"),
        4,
        &float_api(),
    );
}
#[test]
fn float_extended_1() {
    extended(
        include_bytes!("fixtures/interpolation-extended-float.bin"),
        4,
        1,
        &float_api(),
    );
}
#[test]
fn float_extended_0() {
    extended(
        include_bytes!("fixtures/interpolation-extended-float-include.bin"),
        4,
        0,
        &float_api(),
    );
}
#[test]
fn float_edges_atomic_overlap() {
    edges(&float_api(), f32::MAX);
}
fn double_api() -> Api<f64> {
    Api {
        between: ciglet_rs_interpolate_between_f64,
        linear: ciglet_rs_interpolate_linear_f64,
        uniform: ciglet_rs_interpolate_uniform_f64,
        sinc: ciglet_rs_interpolate_sinc_f64,
        sinc_with: ciglet_rs_interpolate_sinc_with_f64,
        sinc_full: ciglet_rs_interpolate_sinc_with_kernel_f64,
        fill: ciglet_rs_interpolate_fill_missing_f64,
    }
}
#[test]
fn double_linear_original() {
    linear(
        include_bytes!("fixtures/interpolation-linear-double.bin"),
        8,
        &double_api(),
    );
}
#[test]
fn double_extended_1() {
    extended(
        include_bytes!("fixtures/interpolation-extended-double.bin"),
        8,
        1,
        &double_api(),
    );
}
#[test]
fn double_extended_0() {
    extended(
        include_bytes!("fixtures/interpolation-extended-double-include.bin"),
        8,
        0,
        &double_api(),
    );
}
#[test]
fn double_edges_atomic_overlap() {
    edges(&double_api(), f64::MAX);
}

#[test]
fn near_integer_sinc_has_independent_limit_and_legacy_policy() {
    let values = [0.0, 0.75, 0.0];
    let query = [1.0 + 2.0f64.powi(-50)];
    let mut stable = [71.0];
    let mut legacy = [71.0];
    let mut default = [71.0];
    // SAFETY: Initialized complete values/query and independent writable scalars;
    // every entry point matches binary64 and returns a staged owned result.
    unsafe {
        assert_eq!(
            ciglet_rs_interpolate_sinc_with_kernel_f64(
                0.0,
                3.0,
                values.as_ptr(),
                3,
                query.as_ptr(),
                1,
                0,
                0,
                stable.as_mut_ptr(),
                1
            ),
            0
        );
        assert_eq!(
            ciglet_rs_interpolate_sinc_with_kernel_f64(
                0.0,
                3.0,
                values.as_ptr(),
                3,
                query.as_ptr(),
                1,
                0,
                1,
                legacy.as_mut_ptr(),
                1
            ),
            0
        );
        assert_eq!(
            ciglet_rs_interpolate_sinc_f64(
                0.0,
                3.0,
                values.as_ptr(),
                3,
                query.as_ptr(),
                1,
                default.as_mut_ptr(),
                1
            ),
            0
        );
    }
    let angle = std::f64::consts::PI * (query[0] - 1.0);
    let expected = 0.75 * (angle.sin() / angle) * ((angle / 4.0).sin() / (angle / 4.0));
    assert!((stable[0] - expected).abs() <= 3e-16);
    assert_eq!(stable, default);
    assert!(legacy[0] > 1e16);
}
