#![cfg(feature = "c-api")]
use ciglet_rs::{c_api::*, statistics::Scalar};
use std::{ffi::c_void, ptr};
type Callback<T> = unsafe extern "C" fn(T, *mut c_void) -> T;
type Find<T> = unsafe extern "C" fn(Option<Callback<T>>, *mut c_void, T, T, *mut T) -> u32;
struct Api<T> {
    find: Find<T>,
    callback: Callback<T>,
    independent: Callback<T>,
}
struct Context<T> {
    kind: u32,
    target: T,
    scale: T,
    calls: usize,
}
unsafe extern "C" fn callback_f32(x: f32, context: *mut c_void) -> f32 {
    // SAFETY: Test caller provides a live exclusive initialized matching context.
    let context = unsafe { &mut *context.cast::<Context<f32>>() };
    context.calls += 1;
    match context.kind {
        0 => x * x - 2.,
        1 => x * x * x - x - 2.,
        2 => ((x as f64).cos() - x as f64) as f32,
        3 => ((x as f64).exp() - 3.) as f32,
        4 => x - 0.12345,
        5 => (x - context.target) * context.scale,
        6 => 1.,
        7 => f32::NAN,
        8 => {
            if x == 0. {
                f32::NAN
            } else {
                x
            }
        }
        _ => {
            if context.calls == 2 {
                f32::INFINITY
            } else {
                x - context.target
            }
        }
    }
}
unsafe extern "C" fn independent_f32(x: f32, _context: *mut c_void) -> f32 {
    x - 0.25
}
unsafe extern "C" fn callback_f64(x: f64, context: *mut c_void) -> f64 {
    // SAFETY: Test caller provides a live exclusive initialized matching context.
    let context = unsafe { &mut *context.cast::<Context<f64>>() };
    context.calls += 1;
    match context.kind {
        0 => x * x - 2.,
        1 => x * x * x - x - 2.,
        2 => x.cos() - x,
        3 => x.exp() - 3.,
        4 => x - 0.12345,
        5 => (x - context.target) * context.scale,
        6 => 1.,
        7 => f64::NAN,
        8 => {
            if x == 0. {
                f64::NAN
            } else {
                x
            }
        }
        _ => {
            if context.calls == 2 {
                f64::INFINITY
            } else {
                x - context.target
            }
        }
    }
}
unsafe extern "C" fn independent_f64(x: f64, _context: *mut c_void) -> f64 {
    x - 0.25
}
fn context<T: Scalar>(kind: u32) -> Context<T> {
    Context {
        kind,
        target: T::from_f64(0.25).unwrap(),
        scale: T::one(),
        calls: 0,
    }
}
fn reference<T: Scalar>(bytes: &[u8], width: usize, api: &Api<T>) {
    assert_eq!(&bytes[..4], b"ROT1");
    assert_eq!(
        u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize,
        width
    );
    assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()), 25);
    assert_eq!(bytes.len(), 12 + 25 * (4 + 3 * width));
    let mut cursor = 12;
    for case in 0..25 {
        let kind = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap());
        cursor += 4;
        let mut scalar = || {
            let v = if width == 4 {
                f32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as f64
            } else {
                f64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap())
            };
            cursor += width;
            T::from_f64(v).unwrap()
        };
        let start = scalar();
        let end = scalar();
        let expected = scalar();
        let mut output = T::from_f64(71.).unwrap();
        let mut context = context(kind);
        // SAFETY: Matching typed callback, live exclusive context and distinct output.
        unsafe {
            assert_eq!(
                (api.find)(
                    Some(api.callback),
                    (&mut context as *mut Context<T>).cast(),
                    start,
                    end,
                    &mut output
                ),
                0
            );
        }
        assert_eq!(output, expected, "original C root case {case}");
        assert!(context.calls >= 2);
    }
    assert_eq!(cursor, bytes.len());
}
fn analytic<T: Scalar>(api: &Api<T>, maximum: T) {
    let v = |x| T::from_f64(x).unwrap();
    let mut output = v(71.);
    let mut data = context::<T>(5);
    // SAFETY: Matching callback and exclusively borrowed live initialized context
    // independent of output. Independent callback supports null context.
    unsafe {
        assert_eq!(
            (api.find)(
                Some(api.callback),
                (&mut data as *mut Context<T>).cast(),
                v(0.),
                v(1.),
                &mut output
            ),
            0
        );
        assert_eq!(output, v(0.25));
        assert!(data.calls >= 2);
        data.target = v(1.);
        data.calls = 0;
        assert_eq!(
            (api.find)(
                Some(api.callback),
                (&mut data as *mut Context<T>).cast(),
                v(1.),
                v(3.),
                &mut output
            ),
            0
        );
        assert_eq!(output, v(1.));
        assert_eq!(data.calls, 2);
        data.target = v(3.);
        data.calls = 0;
        assert_eq!(
            (api.find)(
                Some(api.callback),
                (&mut data as *mut Context<T>).cast(),
                v(1.),
                v(3.),
                &mut output
            ),
            0
        );
        assert_eq!(output, v(3.));
        assert_eq!(data.calls, 2);
        data.kind = 6;
        data.calls = 0;
        assert_eq!(
            (api.find)(
                Some(api.callback),
                (&mut data as *mut Context<T>).cast(),
                v(1.),
                v(3.),
                &mut output
            ),
            0
        );
        assert_eq!(output, v(2.));
        assert_eq!(data.calls, 2);
        assert_eq!(
            (api.find)(
                Some(api.callback),
                (&mut data as *mut Context<T>).cast(),
                maximum,
                maximum,
                &mut output
            ),
            0
        );
        assert_eq!(output, maximum);
        assert_eq!(
            (api.find)(
                Some(api.independent),
                ptr::null_mut(),
                v(0.),
                v(1.),
                &mut output
            ),
            0
        );
        assert_eq!(output, v(0.25));
    }
}
fn errors<T: Scalar>(api: &Api<T>) {
    let v = |x| T::from_f64(x).unwrap();
    let mut output = v(71.);
    let mut data = context::<T>(5);
    // SAFETY: Initialized matching context/output. Deliberate null/misaligned
    // output/callback inputs are rejected without callback invocation or writes.
    unsafe {
        assert_eq!(
            (api.find)(None, ptr::null_mut(), v(0.), v(1.), &mut output),
            1
        );
        assert_eq!(
            (api.find)(
                Some(api.callback),
                (&mut data as *mut Context<T>).cast(),
                v(0.),
                v(1.),
                ptr::null_mut()
            ),
            1
        );
        let mut words = [0x47474747u32; 8];
        let invalid = words.as_mut_ptr().cast::<u8>().add(1).cast::<T>();
        assert_eq!(
            (api.find)(
                Some(api.callback),
                (&mut data as *mut Context<T>).cast(),
                v(0.),
                v(1.),
                invalid
            ),
            1
        );
        assert_eq!(words, [0x47474747; 8]);
        assert_eq!(data.calls, 0);
        for (start, end) in [
            (v(3.), v(1.)),
            (v(f64::NAN), v(1.)),
            (v(0.), v(f64::INFINITY)),
        ] {
            assert_eq!(
                (api.find)(
                    Some(api.callback),
                    (&mut data as *mut Context<T>).cast(),
                    start,
                    end,
                    &mut output
                ),
                3
            );
            assert_eq!(data.calls, 0);
            assert_eq!(output, v(71.));
        }
        for (kind, start, end) in [(7, v(0.), v(1.)), (8, v(-1.), v(1.)), (9, v(0.), v(1.))] {
            data.kind = kind;
            data.calls = 0;
            assert_eq!(
                (api.find)(
                    Some(api.callback),
                    (&mut data as *mut Context<T>).cast(),
                    start,
                    end,
                    &mut output
                ),
                3
            );
            assert_eq!(output, v(71.));
            assert!(data.calls >= 1);
            if kind == 8 {
                assert!(data.calls >= 3);
            }
        }
    }
}
#[test]
fn float_original_reference() {
    reference::<f32>(
        include_bytes!("fixtures/root-float.bin"),
        4,
        &Api {
            find: ciglet_rs_root_find_f32,
            callback: callback_f32,
            independent: independent_f32,
        },
    );
}
#[test]
fn float_context_endpoints_fallback_and_null_context() {
    analytic(
        &Api {
            find: ciglet_rs_root_find_f32,
            callback: callback_f32,
            independent: independent_f32,
        },
        f32::MAX,
    );
}
#[test]
fn float_atomic_errors_and_callback_validation() {
    errors(&Api {
        find: ciglet_rs_root_find_f32,
        callback: callback_f32,
        independent: independent_f32,
    });
}
#[test]
fn double_original_reference() {
    reference::<f64>(
        include_bytes!("fixtures/root-double.bin"),
        8,
        &Api {
            find: ciglet_rs_root_find_f64,
            callback: callback_f64,
            independent: independent_f64,
        },
    );
}
#[test]
fn double_context_endpoints_fallback_and_null_context() {
    analytic(
        &Api {
            find: ciglet_rs_root_find_f64,
            callback: callback_f64,
            independent: independent_f64,
        },
        f64::MAX,
    );
}
#[test]
fn double_atomic_errors_and_callback_validation() {
    errors(&Api {
        find: ciglet_rs_root_find_f64,
        callback: callback_f64,
        independent: independent_f64,
    });
}
