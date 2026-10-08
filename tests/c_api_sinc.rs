#![cfg(feature = "c-api")]
use ciglet_rs::{c_api::*, statistics::Scalar};
use std::ptr;
type Default<T> = unsafe extern "C" fn(T, T, *mut T) -> u32;
type Selected<T> = unsafe extern "C" fn(T, T, u32, *mut T) -> u32;
struct Api<T> {
    aliased: Default<T>,
    selected: Selected<T>,
    derivative: Default<T>,
    selected_derivative: Selected<T>,
}
fn reference<T: Scalar>(bytes: &[u8], width: usize, api: &Api<T>) {
    assert_eq!(&bytes[..4], b"SNC1");
    assert_eq!(
        u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize,
        width
    );
    assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()), 192);
    assert_eq!(bytes.len(), 12 + 192 * 4 * width);
    let mut maximum = [0.0f64; 2];
    for (case, record) in bytes[12..].chunks_exact(4 * width).enumerate() {
        let fields: Vec<T> = record
            .chunks_exact(width)
            .map(|v| {
                T::from_f64(if width == 4 {
                    f32::from_le_bytes(v.try_into().unwrap()) as f64
                } else {
                    f64::from_le_bytes(v.try_into().unwrap())
                })
                .unwrap()
            })
            .collect();
        let mut output = [T::zero(); 2];
        // SAFETY: Independent exclusively writable scalars, initialized value arguments
        // and matching precision-specific C function pointers.
        unsafe {
            assert_eq!((api.selected)(fields[0], fields[1], 1, &mut output[0]), 0);
            assert_eq!(
                (api.selected_derivative)(fields[0], fields[1], 1, &mut output[1]),
                0
            );
        }
        for (field, (&a, &b)) in output.iter().zip(&fields[2..]).enumerate() {
            if width == 4 || a == T::zero() || b == T::zero() {
                assert_eq!(
                    a.into().to_bits(),
                    b.into().to_bits(),
                    "bits {case}/{field}"
                );
            }
            let error = (a.into() - b.into()).abs() / (1. + b.into().abs());
            assert!(error.is_finite());
            maximum[field] = maximum[field].max(error);
        }
    }
    let bounds = if width == 4 { [0., 0.] } else { [2e-16, 5e-16] };
    for (&a, &b) in maximum.iter().zip(&bounds) {
        assert!(a <= b, "{a:e} > {b:e}");
    }
}
fn errors<T: Scalar>(api: &Api<T>, maximum: T) {
    let zero = T::zero();
    let one = T::one();
    let sentinel = T::from_f64(71.).unwrap();
    let nan = T::from_f64(f64::NAN).unwrap();
    let infinity = T::from_f64(f64::INFINITY).unwrap();
    let mut output = sentinel;
    // SAFETY: Independent exclusive initialized scalar result; null/misaligned
    // pointers are intentionally rejected before any write. All ordinary functions
    // use matching precision and preserve output for domain/policy failures.
    unsafe {
        for function in [api.selected, api.selected_derivative] {
            for mode in [0, 1] {
                assert_eq!(function(nan, zero, mode, &mut output), 3);
                assert_eq!(output, sentinel);
                assert_eq!(function(one, infinity, mode, &mut output), 3);
                assert_eq!(output, sentinel);
            }
            assert_eq!(function(one, zero, 2, &mut output), 3);
            assert_eq!(output, sentinel);
            assert_eq!(function(one, zero, 0, ptr::null_mut()), 1);
        }
        for function in [api.aliased, api.derivative] {
            assert_eq!(function(one, zero, ptr::null_mut()), 1);
        }
        for mode in [0, 1] {
            assert_eq!((api.selected)(maximum, zero, mode, &mut output), 0);
            assert_eq!(output, maximum);
            assert_eq!(
                (api.selected_derivative)(maximum, zero, mode, &mut output),
                0
            );
            assert_eq!(output, zero);
        }
    }
}
fn float_api() -> Api<f32> {
    Api {
        aliased: ciglet_rs_sinc_aliased_f32,
        selected: ciglet_rs_sinc_aliased_with_f32,
        derivative: ciglet_rs_sinc_derivative_f32,
        selected_derivative: ciglet_rs_sinc_derivative_with_f32,
    }
}
#[test]
fn float_original_reference() {
    reference(include_bytes!("fixtures/sinc-float.bin"), 4, &float_api());
}
#[test]
fn float_atomic_errors() {
    errors(&float_api(), f32::MAX);
}
fn double_api() -> Api<f64> {
    Api {
        aliased: ciglet_rs_sinc_aliased_f64,
        selected: ciglet_rs_sinc_aliased_with_f64,
        derivative: ciglet_rs_sinc_derivative_f64,
        selected_derivative: ciglet_rs_sinc_derivative_with_f64,
    }
}
#[test]
fn double_original_reference() {
    reference(include_bytes!("fixtures/sinc-double.bin"), 8, &double_api());
}
#[test]
fn double_atomic_errors() {
    errors(&double_api(), f64::MAX);
}

#[test]
fn double_analytic_integer_lengths_slopes_and_source_plateaus() {
    let api = double_api();
    let mut value = 71.;
    let mut derivative = 71.;
    let mut selected = 71.;
    for angle in [
        -std::f64::consts::TAU,
        -1.,
        -1e-4,
        -1e-8,
        0.,
        1e-12,
        1e-8,
        1e-4,
        1.,
        std::f64::consts::TAU,
        std::f64::consts::TAU + 1e-8,
        2. * std::f64::consts::TAU,
    ] {
        // SAFETY: Independent writable scalars, complete precision-specific value arguments.
        unsafe {
            assert_eq!((api.aliased)(2., angle, &mut value), 0);
            assert_eq!((api.derivative)(2., angle, &mut derivative), 0);
            assert_eq!((api.selected)(2., angle, 0, &mut selected), 0);
            assert_eq!(selected, value);
            assert!((value - 2. * (angle * 0.5).cos()).abs() < 2e-15);
            assert!((derivative + (angle * 0.5).sin()).abs() < 2e-15);
            assert_eq!((api.aliased)(3., angle, &mut value), 0);
            assert_eq!((api.derivative)(3., angle, &mut derivative), 0);
            assert!((value - (1. + 2. * angle.cos())).abs() < 3e-15);
            assert!((derivative + 2. * angle.sin()).abs() < 3e-15);
            assert_eq!((api.selected_derivative)(3., angle, 0, &mut selected), 0);
            assert_eq!(selected, derivative);
        }
    }
    // SAFETY: Same complete independent scalar output contracts.
    unsafe {
        assert_eq!((api.aliased)(2., std::f64::consts::TAU, &mut value), 0);
        assert_eq!(value, -2.);
        assert_eq!((api.selected)(2., std::f64::consts::TAU, 1, &mut value), 0);
        assert_eq!(value, 2.);
        assert_eq!((api.derivative)(0.5, 1e-8, &mut derivative), 0);
        assert!((derivative - 3.125e-10).abs() < 1e-25);
        assert_eq!((api.selected_derivative)(3., 1e-8, 1, &mut derivative), 0);
        assert_eq!(derivative, 0.);
        assert_eq!((api.derivative)(3., 1e-8, &mut derivative), 0);
        assert!((derivative + 2e-8).abs() < 1e-23);
        value = 71.;
        assert_eq!((api.aliased)(f64::MAX, 4., &mut value), 3);
        assert_eq!(value, 71.);
        assert_eq!(
            (api.aliased)(f64::MAX, std::f64::consts::TAU, &mut value),
            0
        );
        assert_eq!(value, -f64::MAX);
    }
}
#[test]
fn float_analytic_derivative_and_misaligned_output_retention() {
    let api = float_api();
    let mut value = 71.;
    for angle in [0., 1e-8, 1e-4, 1., std::f32::consts::TAU] {
        // SAFETY: Independent precision-specific scalar output.
        unsafe {
            assert_eq!((api.derivative)(2., angle, &mut value), 0);
        }
        assert!((f64::from(value) + (f64::from(angle) * 0.5).sin()).abs() < 1e-7);
    }
    let mut storage = [0x71717171u32; 2];
    let before = storage;
    // SAFETY: Deliberately misaligned output is rejected before dereference.
    unsafe {
        assert_eq!(
            (api.aliased)(1., 0., storage.as_mut_ptr().cast::<u8>().add(1).cast()),
            1
        );
    }
    assert_eq!(storage, before);
}
