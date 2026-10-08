#![cfg(feature = "c-api")]
use ciglet_rs::c_api::*;
use std::{ffi::c_void, ptr};

macro_rules! precision_tests {
    ($name:ident, $t:ty, $add:ident, $square:ident, $min:ident, $max:ident, $fold:ident, $nan:expr) => {
        #[test]
        fn $name() {
            unsafe extern "C" fn ordered(acc: $t, value: $t, context: *mut c_void) -> $t {
                // SAFETY: Tests provide a live exclusive usize context.
                unsafe {
                    *context.cast::<usize>() += 1;
                }
                acc * 10.0 + value
            }
            let mut output: $t = 71.0;
            let mut calls = 0usize;
            let context = ptr::from_mut(&mut calls).cast();
            let mut input: [$t; 3] = [1.0, 2.0, 3.0];
            // SAFETY: Local live storage meets every pointer and callback contract.
            unsafe {
                assert_eq!(
                    $fold(input.as_ptr(), 3, 4.0, Some(ordered), context, &mut output),
                    0
                );
                assert_eq!(output, 4123.0);
                assert_eq!(calls, 3);
                assert_eq!(
                    $fold(
                        input.as_ptr(),
                        3,
                        4.0,
                        Some(ordered),
                        context,
                        input.as_mut_ptr()
                    ),
                    0
                );
                assert_eq!(input, [4123.0, 2.0, 3.0]);
                assert_eq!(calls, 6);
                assert_eq!(
                    $fold(ptr::null(), 0, -0.0, None, ptr::null_mut(), &mut output),
                    0
                );
                assert_eq!(output.to_bits(), (-0.0 as $t).to_bits());
                output = 71.0;
                assert_eq!($fold(input.as_ptr(), 3, 4.0, None, context, &mut output), 1);
                assert_eq!(
                    $fold(ptr::null(), 1, 4.0, Some(ordered), context, &mut output),
                    1
                );
                assert_eq!(
                    $fold(
                        input.as_ptr(),
                        usize::MAX,
                        4.0,
                        Some(ordered),
                        context,
                        &mut output
                    ),
                    2
                );
                assert_eq!(
                    $fold(
                        input.as_ptr(),
                        3,
                        4.0,
                        Some(ordered),
                        context,
                        ptr::null_mut()
                    ),
                    1
                );
                assert_eq!(output, 71.0);
                assert_eq!(calls, 6);
                let nan: $t = $nan;
                for (first, second) in
                    [(0.0, -0.0), (-0.0, 0.0), (nan, 3.0), (3.0, nan), (nan, nan)]
                {
                    assert_eq!($min(first, second, &mut output), 0);
                    assert_eq!(output.to_bits(), second.to_bits());
                    assert_eq!($max(first, second, &mut output), 0);
                    assert_eq!(output.to_bits(), second.to_bits());
                }
                assert_eq!($add(<$t>::INFINITY, <$t>::NEG_INFINITY, &mut output), 0);
                assert!(output.is_nan());
                assert_eq!($square(5.0, -3.0, &mut output), 0);
                assert_eq!(output, 14.0);
                assert_eq!($square(0.0, <$t>::MAX, &mut output), 0);
                assert_eq!(output, <$t>::INFINITY);
                assert_eq!($min(-5.0, 7.0, &mut output), 0);
                assert_eq!(output, -5.0);
                assert_eq!($max(-5.0, 7.0, &mut output), 0);
                assert_eq!(output, 7.0);
                for operation in [$add, $square, $min, $max] {
                    assert_eq!(operation(1.0, 2.0, ptr::null_mut()), 1);
                    assert_eq!(operation(1.0, 2.0, ptr::without_provenance_mut::<$t>(1)), 1);
                }
            }
        }
    };
}
precision_tests!(
    binary32,
    f32,
    ciglet_rs_scalar_add_f32,
    ciglet_rs_scalar_accumulate_square_f32,
    ciglet_rs_scalar_minimum_f32,
    ciglet_rs_scalar_maximum_f32,
    ciglet_rs_scalar_fold_f32,
    f32::from_bits(0x7fc00042)
);
precision_tests!(
    binary64,
    f64,
    ciglet_rs_scalar_add_f64,
    ciglet_rs_scalar_accumulate_square_f64,
    ciglet_rs_scalar_minimum_f64,
    ciglet_rs_scalar_maximum_f64,
    ciglet_rs_scalar_fold_f64,
    f64::from_bits(0x7ff8000000000042)
);

#[test]
fn original_constant_bits_and_layout() {
    let mut output = CigletRsMathConstants {
        epsilon: 0.0,
        infinity_sentinel: 0.0,
        pi: 0.0,
        half_pi: 0.0,
    };
    // SAFETY: Local descriptor is aligned and exclusively writable.
    unsafe {
        assert_eq!(ciglet_rs_math_constants(&mut output), 0);
        assert_eq!(ciglet_rs_math_constants(ptr::null_mut()), 1);
        assert_eq!(ciglet_rs_math_constants(ptr::without_provenance_mut(1)), 1);
    }
    assert_eq!(output.epsilon.to_bits(), 0x3cd203af9ee75616);
    assert_eq!(output.infinity_sentinel.to_bits(), 0x430c6bf526340000);
    assert_eq!(output.pi.to_bits(), 0x400921fb54442d18);
    assert_eq!(output.half_pi.to_bits(), 0x3ff921fb54442d18);
    assert_eq!(size_of::<CigletRsMathConstants>(), 4 * size_of::<f64>());
    assert_eq!(align_of::<CigletRsMathConstants>(), align_of::<f64>());
}

#[test]
fn original_comparator_float_corpus() {
    let bytes = include_bytes!("fixtures/scalar-math-float.bin");
    assert_eq!(&bytes[..4], b"SCM1");
    let offset = bytes.len() - 16 * 4 * 4 - 4;
    assert_eq!(
        u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()),
        16
    );
    for record in bytes[offset + 4..].chunks_exact(4 * 4) {
        let values: Vec<f32> = record
            .chunks_exact(4)
            .map(|v| f32::from_le_bytes(v.try_into().unwrap()))
            .collect();
        let mut output = 0.0;
        // SAFETY: Scalar inputs and local exclusively writable output.
        unsafe {
            assert_eq!(
                ciglet_rs_scalar_minimum_f32(values[0], values[1], &mut output),
                0
            );
            assert_eq!(output.to_bits(), values[2].to_bits());
            assert_eq!(
                ciglet_rs_scalar_maximum_f32(values[0], values[1], &mut output),
                0
            );
            assert_eq!(output.to_bits(), values[3].to_bits());
        }
    }
}

#[test]
fn original_comparator_double_corpus() {
    let bytes = include_bytes!("fixtures/scalar-math-double.bin");
    assert_eq!(&bytes[..4], b"SCM1");
    let offset = bytes.len() - 16 * 4 * 8 - 4;
    assert_eq!(
        u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()),
        16
    );
    for record in bytes[offset + 4..].chunks_exact(4 * 8) {
        let values: Vec<f64> = record
            .chunks_exact(8)
            .map(|v| f64::from_le_bytes(v.try_into().unwrap()))
            .collect();
        let mut output = 0.0;
        // SAFETY: Scalar inputs and local exclusively writable output.
        unsafe {
            assert_eq!(
                ciglet_rs_scalar_minimum_f64(values[0], values[1], &mut output),
                0
            );
            assert_eq!(output.to_bits(), values[2].to_bits());
            assert_eq!(
                ciglet_rs_scalar_maximum_f64(values[0], values[1], &mut output),
                0
            );
            assert_eq!(output.to_bits(), values[3].to_bits());
        }
    }
}
