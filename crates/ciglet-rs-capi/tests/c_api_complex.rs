use ciglet_rs::{
    approximation::{self, Precision},
    statistics::Scalar,
};
use ciglet_rs_capi::*;
use std::ptr;

struct Api<T, C> {
    new: unsafe extern "C" fn(T, T, *mut C) -> u32,
    conjugate: unsafe extern "C" fn(C, *mut C) -> u32,
    add: unsafe extern "C" fn(C, C, *mut C) -> u32,
    subtract: unsafe extern "C" fn(C, C, *mut C) -> u32,
    multiply: unsafe extern "C" fn(C, C, *mut C) -> u32,
    divide: unsafe extern "C" fn(C, C, *mut C) -> u32,
    exponential: unsafe extern "C" fn(C, u32, *mut C) -> u32,
    argument: unsafe extern "C" fn(C, u32, *mut T) -> u32,
    make: fn(T, T) -> C,
    fields: fn(C) -> [T; 2],
}
struct Reader<'a> {
    remaining: &'a [u8],
    width: usize,
}
impl Reader<'_> {
    fn bytes(&mut self, length: usize) -> &[u8] {
        let (head, tail) = self.remaining.split_at(length);
        self.remaining = tail;
        head
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.bytes(4).try_into().unwrap()) as usize
    }
    fn scalar<T: Scalar>(&mut self) -> T {
        let value = if self.width == 4 {
            f64::from(f32::from_le_bytes(self.bytes(4).try_into().unwrap()))
        } else {
            f64::from_le_bytes(self.bytes(8).try_into().unwrap())
        };
        T::from_f64(value).unwrap()
    }
    fn point<T: Scalar, C>(&mut self, api: &Api<T, C>) -> C {
        (api.make)(self.scalar(), self.scalar())
    }
}
fn exact<T: Scalar>(actual: T, expected: T) {
    assert_eq!(actual.into().to_bits(), expected.into().to_bits());
}
fn host_math<T: Scalar>(actual: T, expected: T) {
    if actual == T::zero() || expected == T::zero() {
        exact(actual, expected);
        return;
    }
    let error = (actual.into() - expected.into()).abs() / (1.0 + expected.into().abs());
    let (ulp, epsilon) = if std::mem::size_of::<T>() == 8 {
        (
            actual.into().to_bits().abs_diff(expected.into().to_bits()),
            f64::EPSILON,
        )
    } else {
        (
            u64::from(
                (actual.into() as f32)
                    .to_bits()
                    .abs_diff((expected.into() as f32).to_bits()),
            ),
            f64::from(f32::EPSILON),
        )
    };
    assert!(
        ulp <= 1 && error <= epsilon,
        "host libm: {ulp} ULP, normalized error {error}"
    );
}
fn point_exact<T: Scalar, C: Copy>(api: &Api<T, C>, actual: C, expected: C) {
    for (actual, expected) in (api.fields)(actual).into_iter().zip((api.fields)(expected)) {
        exact(actual, expected);
    }
}
fn reference<T: Scalar, C: Copy>(bytes: &[u8], width: usize, api: &Api<T, C>) {
    let mut reader = Reader {
        remaining: bytes,
        width,
    };
    assert_eq!(reader.bytes(4), b"CPX1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 64);
    for _ in 0..64 {
        let first = reader.point(api);
        let second = reader.point(api);
        let [real, imaginary] = (api.fields)(first);
        let mut actual = (api.make)(T::zero(), T::zero());
        // SAFETY: C descriptors are copied by value; the output is exclusive and aligned.
        assert_eq!(unsafe { (api.new)(real, imaginary, &mut actual) }, 0);
        point_exact(api, actual, first);
        // SAFETY: All calls use copied initialized descriptors and one live writable output.
        unsafe {
            assert_eq!((api.conjugate)(first, &mut actual), 0);
            point_exact(api, actual, reader.point(api));
            for operation in [api.add, api.subtract, api.multiply, api.divide] {
                assert_eq!(operation(first, second, &mut actual), 0);
                point_exact(api, actual, reader.point(api));
            }
            for mode in [0, 1, 2] {
                assert_eq!((api.exponential)(first, mode, &mut actual), 0);
                let expected = reader.point(api);
                if mode == 2 {
                    for (a, b) in (api.fields)(actual).into_iter().zip((api.fields)(expected)) {
                        host_math(a, b);
                    }
                } else {
                    point_exact(api, actual, expected);
                }
            }
            let mut angle = T::zero();
            assert_eq!((api.argument)(first, 0, &mut angle), 0);
            exact(angle, reader.scalar());
            let expected = reader.scalar();
            for mode in [1, 2] {
                assert_eq!((api.argument)(first, mode, &mut angle), 0);
                host_math(angle, expected);
            }
        }
    }
    // CPX1 also contains scalar approximation records. Consume and verify their
    // existing native contract; scalar C bindings are tracked independently.
    assert_eq!(reader.integer(), 15);
    for _ in 0..15 {
        let value = reader.scalar::<T>();
        let exponent = reader.scalar::<T>();
        for mode in [Precision::Fast, Precision::Approximate, Precision::Exact] {
            let actual = approximation::logarithm_with(value, mode).unwrap();
            let expected = reader.scalar();
            if mode == Precision::Exact {
                host_math(actual, expected);
            } else {
                exact(actual, expected);
            }
        }
        for mode in [Precision::Fast, Precision::Approximate, Precision::Exact] {
            let actual = approximation::exponential_with(exponent, mode).unwrap();
            let expected = reader.scalar::<T>();
            if mode == Precision::Exact {
                host_math(actual, expected);
            } else {
                exact(actual, expected);
            }
        }
        exact(
            T::from_f32(approximation::logarithm2_fast(value.into() as f32).unwrap()).unwrap(),
            reader.scalar(),
        );
        exact(
            T::from_f32(approximation::exponential2_fast(exponent.into() as f32).unwrap()).unwrap(),
            reader.scalar(),
        );
    }
    assert!(reader.remaining.is_empty());
}

fn arithmetic<T: Scalar, C: Copy>(api: &Api<T, C>, largest: T, tiniest: T) {
    let v = |x| T::from_f64(x).unwrap();
    let a = (api.make)(v(2.), v(3.));
    let b = (api.make)(v(4.), v(-5.));
    let mut result = (api.make)(v(71.), v(71.));
    // SAFETY: By-value input descriptors and initialized independent scalar/descriptor outputs.
    unsafe {
        assert_eq!((api.conjugate)(a, &mut result), 0);
        point_exact(api, result, (api.make)(v(2.), v(-3.)));
        for (operation, re, im) in [
            (api.add, 6., -2.),
            (api.subtract, -2., 8.),
            (api.multiply, 23., 2.),
        ] {
            assert_eq!(operation(a, b, &mut result), 0);
            point_exact(api, result, (api.make)(v(re), v(im)));
        }
        for scale in [largest, tiniest] {
            let input = (api.make)(scale, scale);
            assert_eq!((api.divide)(input, input, &mut result), 0);
            point_exact(api, result, (api.make)(v(1.), v(0.)));
        }
        let zero = (api.make)(v(-0.), v(0.));
        assert_eq!((api.conjugate)(zero, &mut result), 0);
        point_exact(api, result, (api.make)(v(-0.), v(-0.)));
        assert_eq!(
            (api.exponential)((api.make)(v(0.), v(-0.)), 2, &mut result),
            0
        );
        point_exact(api, result, (api.make)(v(1.), v(-0.)));
        let mut angle = v(71.);
        assert_eq!((api.argument)((api.make)(v(-1.), v(-0.)), 2, &mut angle), 0);
        exact(angle, v(-std::f64::consts::PI));
        assert_eq!((api.argument)((api.make)(v(-1.), v(-0.)), 0, &mut angle), 0);
        exact(angle, v(std::f64::consts::PI));
        assert_eq!((api.new)(v(f64::NAN), v(-0.), &mut result), 0);
        assert!((api.fields)(result)[0].into().is_nan());
        exact((api.fields)(result)[1], v(-0.));
        assert_eq!(
            (api.multiply)(
                (api.make)(largest, v(0.)),
                (api.make)(largest, v(0.)),
                &mut result
            ),
            0
        );
        assert!((api.fields)(result)[0].into().is_infinite());
        // By-value copy permits reusing the descriptor's original storage.
        result = a;
        assert_eq!((api.add)(result, b, &mut result), 0);
        point_exact(api, result, (api.make)(v(6.), v(-2.)));
        let field = (&mut result as *mut C).cast::<T>();
        assert_eq!((api.argument)(result, 2, field), 0);
        exact((api.fields)(result)[0], v((-2_f64).atan2(6.)));
    }
}

fn errors<T: Scalar, C: Copy>(api: &Api<T, C>) {
    let v = |x| T::from_f64(x).unwrap();
    let valid = (api.make)(v(1.), v(1.));
    let mut result = (api.make)(v(71.), v(71.));
    let mut angle = v(71.);
    // SAFETY: Invalid null/misaligned slots are rejected before access; other
    // outputs are independent, live, aligned and exclusively writable.
    unsafe {
        assert_eq!((api.new)(v(0.), v(0.), ptr::null_mut()), 1);
        assert_eq!((api.conjugate)(valid, ptr::null_mut()), 1);
        for operation in [api.add, api.subtract, api.multiply, api.divide] {
            assert_eq!(operation(valid, valid, ptr::null_mut()), 1);
        }
        assert_eq!((api.exponential)(valid, 0, ptr::null_mut()), 1);
        assert_eq!((api.argument)(valid, 0, ptr::null_mut()), 1);
        let bad = (&mut result as *mut C).cast::<u8>().add(1).cast::<C>();
        assert_eq!((api.conjugate)(valid, bad), 1);
        let bad_angle = (&mut angle as *mut T).cast::<u8>().add(1).cast::<T>();
        assert_eq!((api.argument)(valid, 0, bad_angle), 1);
        for mode in [3, u32::MAX] {
            assert_eq!((api.exponential)(valid, mode, &mut result), 3);
            assert_eq!((api.argument)(valid, mode, &mut angle), 3);
        }
        for invalid in [
            (api.make)(v(f64::NAN), v(0.)),
            (api.make)(v(0.), v(f64::INFINITY)),
        ] {
            for mode in [0, 1, 2] {
                assert_eq!((api.exponential)(invalid, mode, &mut result), 3);
                assert_eq!((api.argument)(invalid, mode, &mut angle), 3);
            }
            assert_eq!((api.divide)(invalid, valid, &mut result), 3);
            assert_eq!((api.divide)(valid, invalid, &mut result), 3);
        }
        assert_eq!(
            (api.divide)(valid, (api.make)(v(-0.), v(0.)), &mut result),
            3
        );
        assert_eq!(
            (api.exponential)((api.make)(v(1000.), v(0.)), 2, &mut result),
            3
        );
        point_exact(api, result, (api.make)(v(71.), v(71.)));
        exact(angle, v(71.));
    }
}

const FLOAT: Api<f32, CigletRsComplexF32> = Api {
    new: ciglet_rs_complex_new_f32,
    conjugate: ciglet_rs_complex_conjugate_f32,
    add: ciglet_rs_complex_add_f32,
    subtract: ciglet_rs_complex_subtract_f32,
    multiply: ciglet_rs_complex_multiply_f32,
    divide: ciglet_rs_complex_divide_f32,
    exponential: ciglet_rs_complex_exponential_f32,
    argument: ciglet_rs_complex_argument_f32,
    make: |real, imaginary| CigletRsComplexF32 { real, imaginary },
    fields: |value| [value.real, value.imaginary],
};
const DOUBLE: Api<f64, CigletRsComplexF64> = Api {
    new: ciglet_rs_complex_new_f64,
    conjugate: ciglet_rs_complex_conjugate_f64,
    add: ciglet_rs_complex_add_f64,
    subtract: ciglet_rs_complex_subtract_f64,
    multiply: ciglet_rs_complex_multiply_f64,
    divide: ciglet_rs_complex_divide_f64,
    exponential: ciglet_rs_complex_exponential_f64,
    argument: ciglet_rs_complex_argument_f64,
    make: |real, imaginary| CigletRsComplexF64 { real, imaginary },
    fields: |value| [value.real, value.imaginary],
};
#[test]
fn float_original_reference() {
    reference(
        include_bytes!("../../../tests/fixtures/complex-float.bin"),
        4,
        &FLOAT,
    );
}
#[test]
fn double_original_reference() {
    reference(
        include_bytes!("../../../tests/fixtures/complex-double.bin"),
        8,
        &DOUBLE,
    );
}
#[test]
fn float_arithmetic_and_extremes() {
    arithmetic(&FLOAT, f32::MAX, f32::from_bits(1));
}
#[test]
fn double_arithmetic_and_extremes() {
    arithmetic(&DOUBLE, f64::MAX, f64::from_bits(1));
}
#[test]
fn float_atomic_errors() {
    errors(&FLOAT);
}
#[test]
fn double_atomic_errors() {
    errors(&DOUBLE);
}
