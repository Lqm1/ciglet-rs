#![cfg(feature = "c-api")]
use ciglet_rs::{approximation, c_api::*, scalar, statistics::Scalar};
use std::ptr;

type Unary<T> = unsafe extern "C" fn(T, u32, *mut T) -> u32;
struct Api<T> {
    sine: Unary<T>,
    cosine: Unary<T>,
    logarithm: Unary<T>,
    exponential: Unary<T>,
    atan2: unsafe extern "C" fn(T, T, u32, *mut T) -> u32,
}
struct Reader<'a> {
    remaining: &'a [u8],
    width: usize,
}
impl Reader<'_> {
    fn bytes(&mut self, count: usize) -> &[u8] {
        let (head, tail) = self.remaining.split_at(count);
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
}
fn exact<T: Scalar>(actual: T, expected: T) {
    assert_eq!(actual.into().to_bits(), expected.into().to_bits());
}
fn host<T: Scalar>(actual: T, expected: T) {
    if actual == T::zero() || expected == T::zero() {
        exact(actual, expected);
    } else {
        // Original tests/scalar_math.rs host-libm bounds, unchanged.
        assert!(
            (actual.into() - expected.into()).abs() / (1. + expected.into().abs()) <= f64::EPSILON
        );
        assert!(actual.into().to_bits().abs_diff(expected.into().to_bits()) <= 1);
    }
}
fn scalar_reference<T: Scalar>(bytes: &[u8], width: usize, api: &Api<T>) {
    let mut reader = Reader {
        remaining: bytes,
        width,
    };
    assert_eq!(reader.bytes(4), b"SCM1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 64);
    for _ in 0..64 {
        let angle = reader.scalar::<T>();
        let y = reader.scalar::<T>();
        let x = reader.scalar::<T>();
        let mut actual = T::zero();
        for mode in [0, 1, 2] {
            for operation in [api.sine, api.cosine] {
                // SAFETY: Scalar inputs and aligned exclusive result slot.
                assert_eq!(unsafe { operation(angle, mode, &mut actual) }, 0);
                let expected = reader.scalar();
                if mode == 2 {
                    host(actual, expected);
                } else {
                    exact(actual, expected);
                }
            }
        }
        for mode in [0, 1, 2] {
            // SAFETY: Scalar inputs and aligned exclusive result slot.
            assert_eq!(unsafe { (api.atan2)(y, x, mode, &mut actual) }, 0);
            let expected = reader.scalar();
            if mode == 0 {
                exact(actual, expected);
            } else {
                host(actual, expected);
            }
        }
    }
    // The comparator tail is a distinct native contract, already exposed/tested
    // separately. Read it completely without conflating it with approximation.
    assert_eq!(reader.integer(), 16);
    for _ in 0..16 {
        let a = reader.scalar::<T>();
        let b = reader.scalar::<T>();
        exact(scalar::minimum(a, b), reader.scalar());
        exact(scalar::maximum(a, b), reader.scalar());
    }
    assert!(reader.remaining.is_empty());
}
fn log_exp_reference<T: Scalar>(bytes: &[u8], width: usize, api: &Api<T>) {
    let mut reader = Reader {
        remaining: bytes,
        width,
    };
    assert_eq!(reader.bytes(4), b"CPX1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 64);
    // CPX1's complete complex section is covered by c_api_complex.rs. This
    // test targets its fifteen scalar records through the new log/exp exports.
    reader.bytes(64 * 22 * width);
    assert_eq!(reader.integer(), 15);
    for _ in 0..15 {
        let value = reader.scalar::<T>();
        let exponent = reader.scalar::<T>();
        let mut actual = T::zero();
        for mode in [0, 1, 2] {
            // SAFETY: Finite scalar input and aligned exclusive result slot.
            assert_eq!(unsafe { (api.logarithm)(value, mode, &mut actual) }, 0);
            // CPX1's original logarithm comparison is exact for all modes.
            exact(actual, reader.scalar());
        }
        for mode in [0, 1, 2] {
            // SAFETY: Finite scalar input and aligned exclusive result slot.
            assert_eq!(unsafe { (api.exponential)(exponent, mode, &mut actual) }, 0);
            let expected = reader.scalar();
            if mode == 2 && width == 8 {
                host(actual, expected);
            } else {
                exact(actual, expected);
            }
        }
        // Remaining base-two fields are verified natively; separate bundled
        // fastapprox bindings are required for these source functions.
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
fn features<T: Scalar>(api: &Api<T>, largest: T, tiniest: T) {
    let v = |x| T::from_f64(x).unwrap();
    let mut result = v(71.);
    assert_eq!(
        (
            CIGLET_RS_APPROXIMATION_FAST,
            CIGLET_RS_APPROXIMATION_APPROXIMATE,
            CIGLET_RS_APPROXIMATION_EXACT
        ),
        (0, 1, 2)
    );
    // SAFETY: Every result slot is aligned/exclusive. By-value inputs allow reuse.
    unsafe {
        assert_eq!((api.sine)(v(-0.), 2, &mut result), 0);
        exact(result, v(-0.));
        assert_eq!((api.cosine)(v(0.), 2, &mut result), 0);
        exact(result, v(1.));
        assert_eq!((api.logarithm)(v(1.), 2, &mut result), 0);
        exact(result, v(0.));
        assert_eq!((api.exponential)(v(0.), 2, &mut result), 0);
        exact(result, v(1.));
        for mode in [1, 2] {
            assert_eq!((api.atan2)(v(-0.), v(-1.), mode, &mut result), 0);
            exact(result, v(-std::f64::consts::PI));
        }
        assert_eq!((api.atan2)(v(-0.), v(-1.), 0, &mut result), 0);
        exact(result, v(std::f64::consts::PI));
        for operation in [api.sine, api.cosine] {
            assert_eq!(operation(largest, 2, &mut result), 0);
            assert!(result.into().is_finite());
            for mode in [0, 1] {
                assert_eq!(operation(largest, mode, &mut result), 3);
            }
        }
        assert_eq!((api.logarithm)(tiniest, 2, &mut result), 0);
        exact(result, v(tiniest.into().ln()));
        for mode in [0, 1] {
            assert_eq!((api.exponential)(v(-200.), mode, &mut result), 0);
            let clamped = result;
            assert_eq!((api.exponential)(v(-1000.), mode, &mut result), 0);
            exact(result, clamped);
        }
        result = v(0.25);
        assert_eq!((api.sine)(result, 2, &mut result), 0);
        exact(result, v(0.25_f64.sin()));
        let mut repeat = v(71.);
        assert_eq!((api.atan2)(v(1.), v(-1.), 1, &mut result), 0);
        assert_eq!((api.atan2)(v(1.), v(-1.), 2, &mut repeat), 0);
        exact(result, repeat);
    }
}
fn errors<T: Scalar>(api: &Api<T>) {
    let v = |x| T::from_f64(x).unwrap();
    let mut result = v(71.);
    // SAFETY: Invalid slots are rejected before access; all valid slots are
    // live/aligned/exclusive. Scalar invalid values require no pointer access.
    unsafe {
        for operation in [api.sine, api.cosine, api.logarithm, api.exponential] {
            assert_eq!(operation(v(1.), 0, ptr::null_mut()), 1);
            let bad = (&mut result as *mut T).cast::<u8>().add(1).cast::<T>();
            assert_eq!(operation(v(1.), 0, bad), 1);
            for mode in [3, u32::MAX] {
                assert_eq!(operation(v(1.), mode, &mut result), 3);
            }
            for mode in [0, 1, 2] {
                for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                    assert_eq!(operation(v(invalid), mode, &mut result), 3);
                }
            }
        }
        assert_eq!((api.atan2)(v(1.), v(1.), 0, ptr::null_mut()), 1);
        let bad = (&mut result as *mut T).cast::<u8>().add(1).cast::<T>();
        assert_eq!((api.atan2)(v(1.), v(1.), 0, bad), 1);
        for mode in [3, u32::MAX] {
            assert_eq!((api.atan2)(v(1.), v(1.), mode, &mut result), 3);
        }
        for mode in [0, 1, 2] {
            for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                assert_eq!((api.atan2)(v(invalid), v(1.), mode, &mut result), 3);
                assert_eq!((api.atan2)(v(1.), v(invalid), mode, &mut result), 3);
            }
            for invalid in [0., -0., -1.] {
                assert_eq!((api.logarithm)(v(invalid), mode, &mut result), 3);
            }
            assert_eq!((api.exponential)(v(1000.), mode, &mut result), 3);
        }
        exact(result, v(71.));
    }
}
const FLOAT: Api<f32> = Api {
    sine: ciglet_rs_approximation_sine_f32,
    cosine: ciglet_rs_approximation_cosine_f32,
    logarithm: ciglet_rs_approximation_logarithm_f32,
    exponential: ciglet_rs_approximation_exponential_f32,
    atan2: ciglet_rs_approximation_atan2_f32,
};
const DOUBLE: Api<f64> = Api {
    sine: ciglet_rs_approximation_sine_f64,
    cosine: ciglet_rs_approximation_cosine_f64,
    logarithm: ciglet_rs_approximation_logarithm_f64,
    exponential: ciglet_rs_approximation_exponential_f64,
    atan2: ciglet_rs_approximation_atan2_f64,
};
#[test]
fn float_original_scalar_math() {
    scalar_reference(include_bytes!("fixtures/scalar-math-float.bin"), 4, &FLOAT);
}
#[test]
fn double_original_scalar_math() {
    scalar_reference(
        include_bytes!("fixtures/scalar-math-double.bin"),
        8,
        &DOUBLE,
    );
}
#[test]
fn float_original_log_exp() {
    log_exp_reference(include_bytes!("fixtures/complex-float.bin"), 4, &FLOAT);
}
#[test]
fn double_original_log_exp() {
    log_exp_reference(include_bytes!("fixtures/complex-double.bin"), 8, &DOUBLE);
}
#[test]
fn float_features_and_boundaries() {
    features(&FLOAT, f32::MAX, f32::from_bits(1));
}
#[test]
fn double_features_and_boundaries() {
    features(&DOUBLE, f64::MAX, f64::from_bits(1));
}
#[test]
fn float_atomic_errors() {
    errors(&FLOAT);
}
#[test]
fn double_atomic_errors() {
    errors(&DOUBLE);
}
