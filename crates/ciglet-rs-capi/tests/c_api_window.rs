use ciglet_rs_capi::*;
use std::ffi::c_void;
struct Reader<'a> {
    bytes: &'a [u8],
    width: usize,
}
impl Reader<'_> {
    fn take(&mut self, length: usize) -> &[u8] {
        let (value, rest) = self.bytes.split_at(length);
        self.bytes = rest;
        value
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.take(4).try_into().unwrap()) as usize
    }
    fn scalar(&mut self) -> f64 {
        if self.width == 4 {
            f32::from_le_bytes(self.take(4).try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.take(8).try_into().unwrap())
        }
    }
}
#[derive(Default)]
struct Context {
    calls: usize,
    fail_on: usize,
    nonfinite: bool,
    angles: [f64; 16],
}

unsafe extern "C" fn trig_f32(angle: f64, context: *mut c_void, output: *mut f32) -> u32 {
    // SAFETY: Tests pass a live, exclusively accessed Context and writable scalar.
    let context = unsafe { &mut *context.cast::<Context>() };
    if context.calls < context.angles.len() {
        context.angles[context.calls] = angle;
    }
    context.calls += 1;
    if context.fail_on == context.calls {
        return 9;
    }
    let value = if context.nonfinite {
        f32::NAN
    } else {
        (0.75 - angle * 0.0625 + angle * angle * 0.00390625) as f32
    };
    // SAFETY: Result pointer is live only during this synchronous call.
    unsafe { output.write(value) };
    0
}

unsafe extern "C" fn trig_f64(angle: f64, context: *mut c_void, output: *mut f64) -> u32 {
    // SAFETY: Tests pass a live, exclusively accessed Context and writable scalar.
    let context = unsafe { &mut *context.cast::<Context>() };
    if context.calls < context.angles.len() {
        context.angles[context.calls] = angle;
    }
    context.calls += 1;
    if context.fail_on == context.calls {
        return 9;
    }
    let value = if context.nonfinite {
        f64::NAN
    } else {
        0.75 - angle * 0.0625 + angle * angle * 0.00390625
    };
    // SAFETY: Result pointer is live only during this synchronous call.
    unsafe { output.write(value) };
    0
}

#[test]
fn builtin_f32_matches_original_windows() {
    let mut reader = Reader {
        bytes: include_bytes!("../../../tests/fixtures/c-window-f32.bin"),
        width: 4,
    };
    assert_eq!(reader.take(4), b"WIN1");
    assert_eq!(reader.integer(), 4);
    assert_eq!(reader.integer(), 112);
    for _ in 0..112 {
        let kind = reader.integer() as u32;
        let mode = reader.integer() as u32;
        let n = reader.integer();
        let mut output = vec![71.0; n];
        // SAFETY: Output is a live, exclusively writable n-element allocation.
        unsafe {
            assert_eq!(ciglet_rs_window_f32(kind, mode, output.as_mut_ptr(), n), 0);
        }
        for value in output {
            assert!(((value as f64) - reader.scalar()).abs() <= 1e-7);
        }
    }
    assert!(reader.bytes.is_empty());
}
#[test]
fn custom_f32_matches_original_macro_arithmetic_and_call_counts() {
    let mut reader = Reader {
        bytes: include_bytes!("../../../tests/fixtures/custom_window-float.bin"),
        width: 4,
    };
    assert_eq!(reader.take(4), b"CUS1");
    assert_eq!(reader.integer(), 4);
    assert_eq!(reader.integer(), 60);
    for _ in 0..60 {
        let kind = [1, 2, 6, 3, 4, 5][reader.integer()];
        let callback_width = reader.integer();
        let n = reader.integer();
        let expected_calls = reader.integer();
        let mut context = Context::default();
        let mut output = vec![71.0; n];
        let pointer = (&mut context as *mut Context).cast();
        // SAFETY: Matching C callback, live exclusive context and n-element output.
        let status = unsafe {
            if callback_width == 4 {
                ciglet_rs_window_custom_f32_f32(
                    kind,
                    Some(trig_f32),
                    pointer,
                    output.as_mut_ptr(),
                    n,
                )
            } else {
                ciglet_rs_window_custom_f32_f64(
                    kind,
                    Some(trig_f64),
                    pointer,
                    output.as_mut_ptr(),
                    n,
                )
            }
        };
        assert_eq!(status, 0);
        assert_eq!(context.calls, expected_calls);
        for value in output {
            assert_eq!((value as f64).to_bits(), reader.scalar().to_bits());
        }
    }
    // The unrelated fold records follow the sixty window cases.
    assert_eq!(reader.take(4), b"RED1");
}

#[test]
fn builtin_f64_matches_original_windows() {
    let mut reader = Reader {
        bytes: include_bytes!("../../../tests/fixtures/c-window-f64.bin"),
        width: 8,
    };
    assert_eq!(reader.take(4), b"WIN1");
    assert_eq!(reader.integer(), 8);
    assert_eq!(reader.integer(), 112);
    for _ in 0..112 {
        let kind = reader.integer() as u32;
        let mode = reader.integer() as u32;
        let n = reader.integer();
        let mut output = vec![71.0; n];
        // SAFETY: Output is a live, exclusively writable n-element allocation.
        unsafe {
            assert_eq!(ciglet_rs_window_f64(kind, mode, output.as_mut_ptr(), n), 0);
        }
        for value in output {
            assert!((value - reader.scalar()).abs() <= 1e-15);
        }
    }
    assert!(reader.bytes.is_empty());
}
#[test]
fn custom_f64_matches_original_macro_arithmetic_and_call_counts() {
    let mut reader = Reader {
        bytes: include_bytes!("../../../tests/fixtures/custom_window-double.bin"),
        width: 8,
    };
    assert_eq!(reader.take(4), b"CUS1");
    assert_eq!(reader.integer(), 8);
    assert_eq!(reader.integer(), 60);
    for _ in 0..60 {
        let kind = [1, 2, 6, 3, 4, 5][reader.integer()];
        let callback_width = reader.integer();
        let n = reader.integer();
        let expected_calls = reader.integer();
        let mut context = Context::default();
        let mut output = vec![71.0; n];
        let pointer = (&mut context as *mut Context).cast();
        // SAFETY: Matching C callback, live exclusive context and n-element output.
        let status = unsafe {
            if callback_width == 4 {
                ciglet_rs_window_custom_f64_f32(
                    kind,
                    Some(trig_f32),
                    pointer,
                    output.as_mut_ptr(),
                    n,
                )
            } else {
                ciglet_rs_window_custom_f64_f64(
                    kind,
                    Some(trig_f64),
                    pointer,
                    output.as_mut_ptr(),
                    n,
                )
            }
        };
        assert_eq!(status, 0);
        assert_eq!(context.calls, expected_calls);
        for value in output {
            assert_eq!(value.to_bits(), reader.scalar().to_bits());
        }
    }
    // The unrelated fold records follow the sixty window cases.
    assert_eq!(reader.take(4), b"RED1");
}

#[test]
fn window_options_degeneracies_and_null_callbacks_are_checked() {
    let mut output = [71.0_f64; 8];
    // SAFETY: Output is initialized and exclusively writable; empty null is valid.
    unsafe {
        for kind in 0..7 {
            assert_eq!(ciglet_rs_window_f64(kind, 0, std::ptr::null_mut(), 0), 0);
            assert_eq!(ciglet_rs_window_f64(kind, 1, output.as_mut_ptr(), 1), 0);
            assert_eq!(output[0], 1.0);
            assert_eq!(
                ciglet_rs_window_custom_f64_f32(
                    kind,
                    None,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    0
                ),
                0
            );
            assert_eq!(
                ciglet_rs_window_custom_f64_f64(
                    kind,
                    None,
                    std::ptr::null_mut(),
                    output.as_mut_ptr(),
                    1
                ),
                0
            );
            assert_eq!(output[0], 1.0);
        }
        assert_eq!(
            ciglet_rs_window_custom_f64_f32(0, None, std::ptr::null_mut(), output.as_mut_ptr(), 8),
            0
        );
        assert_eq!(output, [1.0; 8]);
        assert_eq!(
            ciglet_rs_window_custom_f64_f32(1, None, std::ptr::null_mut(), output.as_mut_ptr(), 8),
            3
        );
        assert_eq!(ciglet_rs_window_f64(7, 0, output.as_mut_ptr(), 8), 3);
        assert_eq!(ciglet_rs_window_f64(0, 2, output.as_mut_ptr(), 8), 3);
        assert_eq!(
            ciglet_rs_window_f64(0, 0, output.as_mut_ptr(), usize::MAX),
            2
        );
        assert_eq!(output, [1.0; 8]);
        assert_eq!(ciglet_rs_window_f64(0, 0, std::ptr::null_mut(), 8), 1);
    }
}
#[test]
fn callback_order_failure_nonfinite_results_and_blackman_correction_are_retained() {
    let mut output = [71.0_f64; 8];
    let mut context = Context {
        fail_on: 2,
        ..Context::default()
    };
    // SAFETY: Synchronous callbacks use live exclusive context and scalar storage.
    unsafe {
        assert_eq!(
            ciglet_rs_window_custom_f64_f64(
                4,
                Some(trig_f64),
                (&mut context as *mut Context).cast(),
                output.as_mut_ptr(),
                8
            ),
            3
        );
        assert_eq!(context.calls, 2);
        assert_eq!(output, [71.0; 8]);
        context = Context {
            nonfinite: true,
            ..Context::default()
        };
        assert_eq!(
            ciglet_rs_window_custom_f64_f32(
                1,
                Some(trig_f32),
                (&mut context as *mut Context).cast(),
                output.as_mut_ptr(),
                8
            ),
            3
        );
        assert_eq!(context.calls, 1);
        assert_eq!(output, [71.0; 8]);
        context = Context::default();
        assert_eq!(
            ciglet_rs_window_custom_f64_f64(
                3,
                Some(trig_f64),
                (&mut context as *mut Context).cast(),
                output.as_mut_ptr(),
                2
            ),
            0
        );
        assert_eq!(context.calls, 4);
        assert_eq!(
            &context.angles[..4],
            &[0.0, 0.0, std::f64::consts::PI, std::f64::consts::TAU]
        );
        let custom = output[..2].to_vec();
        assert_eq!(ciglet_rs_window_f64(3, 1, output.as_mut_ptr(), 2), 0);
        assert_ne!(custom, output[..2]);
    }
}
