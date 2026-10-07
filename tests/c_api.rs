#![cfg(feature = "c-api")]

use ciglet_rs::c_api::*;
use std::mem::MaybeUninit;

#[test]
fn statistics_keep_policy_and_transactional_outputs() {
    let values = [f32::MAX, f32::MAX];
    let mut output = -71.0_f32;
    // SAFETY: The buffers have their declared lengths and exclusive output access.
    unsafe {
        assert_eq!(
            ciglet_rs_mean_with_f32(values.as_ptr(), 2, 0, &mut output),
            0
        );
        assert_eq!(output, f32::MAX);
        assert_eq!(
            ciglet_rs_mean_with_f32(values.as_ptr(), 2, 1, &mut output),
            0
        );
        assert_eq!(output, f32::INFINITY);
        output = -71.0;
        assert_eq!(
            ciglet_rs_mean_with_f32(values.as_ptr(), 2, u32::MAX, &mut output),
            3
        );
        assert_eq!(output, -71.0);
        assert_eq!(ciglet_rs_median_f32(std::ptr::null(), 0, &mut output), 3);
        assert_eq!(output, -71.0);
        assert_eq!(ciglet_rs_sum_f32(std::ptr::null(), 0, &mut output), 0);
        assert_eq!(output.to_bits(), 0);
    }
    let mut overlapping = [1.0_f64, 3.0, 5.0];
    let pointer = overlapping.as_mut_ptr();
    // SAFETY: The output lies in the valid input allocation; overlap is supported.
    unsafe { assert_eq!(ciglet_rs_mean_f64(pointer, 3, pointer), 0) };
    assert_eq!(overlapping, [3.0, 3.0, 5.0]);
}

#[test]
fn generation_retains_endpoints_and_rejects_overflow() {
    let mut values = [9.0_f64; 3];
    let mut integers = [91_i32; 3];
    // SAFETY: Outputs are exclusive buffers of the specified types and lengths.
    unsafe {
        assert_eq!(
            ciglet_rs_linear_grid_f64(-1.0, 1.0, values.as_mut_ptr(), 3),
            0
        );
        assert_eq!(values, [-1.0, 0.0, 1.0]);
        assert_eq!(
            ciglet_rs_linear_grid_f64(f64::NAN, 1.0, values.as_mut_ptr(), 3),
            3
        );
        assert_eq!(values, [-1.0, 0.0, 1.0]);
        assert_eq!(
            ciglet_rs_integer_sequence(i32::MAX, 1, integers.as_mut_ptr(), 3),
            3
        );
        assert_eq!(integers, [91; 3]);
        assert_eq!(
            ciglet_rs_integer_sequence(-2, 3, integers.as_mut_ptr(), 3),
            0
        );
        assert_eq!(integers, [-2, 1, 4]);
        assert_eq!(
            ciglet_rs_linear_grid_f32(-0.0, 1.0, std::ptr::null_mut(), 0),
            0
        );
    }
}

#[test]
fn vector_buffers_support_overlap_and_empty_inputs() {
    let mut values = [1.0_f32, 3.0, 6.0, 10.0, 99.0];
    let pointer = values.as_mut_ptr();
    // SAFETY: Both ranges are within values, with no concurrent accesses.
    unsafe {
        assert_eq!(ciglet_rs_difference_f32(pointer, 4, pointer.add(1), 4), 0);
        assert_eq!(
            ciglet_rs_cumulative_sum_f32(pointer.add(1), 4, pointer, 4),
            0
        );
    }
    assert_eq!(values, [1.0, 3.0, 6.0, 10.0, 4.0]);
    let mut wide = [-0.0_f64, 2.0, 5.0];
    let pointer = wide.as_mut_ptr();
    // SAFETY: Same-allocation in-place buffers are supported by the ABI.
    unsafe {
        assert_eq!(ciglet_rs_reverse_f64(pointer, 3, pointer, 3), 0);
        assert_eq!(
            ciglet_rs_difference_f64(std::ptr::null(), 0, std::ptr::null_mut(), 0),
            0
        );
    }
    assert_eq!(wide[0], 5.0);
    assert_eq!(wide[2].to_bits(), (-0.0_f64).to_bits());
}

#[test]
fn vector_failures_leave_output_unchanged() {
    let input = [f32::MAX, f32::MAX];
    let mut output = [71.0_f32, 72.0];
    // SAFETY: Real buffers are valid; invalid null/oversized ranges are rejected.
    unsafe {
        assert_eq!(
            ciglet_rs_cumulative_sum_f32(input.as_ptr(), 2, output.as_mut_ptr(), 2),
            3
        );
        assert_eq!(
            ciglet_rs_difference_f32(input.as_ptr(), 2, output.as_mut_ptr(), 1),
            2
        );
        assert_eq!(
            ciglet_rs_reverse_f32(std::ptr::null(), 2, output.as_mut_ptr(), 2),
            1
        );
        assert_eq!(
            ciglet_rs_difference_f32(
                std::ptr::null(),
                usize::MAX,
                std::ptr::null_mut(),
                usize::MAX
            ),
            2
        );
        let nan = [f32::NAN, 1.0];
        assert_eq!(
            ciglet_rs_difference_f32(nan.as_ptr(), 2, output.as_mut_ptr(), 2),
            3
        );
    }
    assert_eq!(output, [71.0, 72.0]);
}

#[test]
fn scalar_outputs_preserve_ties_and_nan_payloads() {
    assert_eq!(ciglet_rs_abi_version(), 1);
    let mut result = MaybeUninit::<f32>::uninit();
    // SAFETY: result provides exclusive, correctly aligned writable storage.
    unsafe {
        assert_eq!(ciglet_rs_minimum_f32(0.0, -0.0, result.as_mut_ptr()), 0);
        assert_eq!(result.assume_init().to_bits(), (-0.0_f32).to_bits());
        let nan = f32::from_bits(0x7fc1_2345);
        assert_eq!(ciglet_rs_maximum_f32(1.0, nan, result.as_mut_ptr()), 0);
        assert_eq!(result.assume_init().to_bits(), nan.to_bits());
    }
    let mut result = MaybeUninit::<f64>::uninit();
    // SAFETY: result provides exclusive, correctly aligned writable storage.
    unsafe {
        let nan = f64::from_bits(0x7ff8_1234_5678_9abc);
        assert_eq!(ciglet_rs_minimum_f64(1.0, nan, result.as_mut_ptr()), 0);
        assert_eq!(result.assume_init().to_bits(), nan.to_bits());
        assert_eq!(ciglet_rs_maximum_f64(-0.0, 0.0, result.as_mut_ptr()), 0);
        assert_eq!(result.assume_init().to_bits(), 0);
    }
}

#[test]
fn invalid_outputs_are_rejected_without_writes() {
    let mut storage = [0xa5_u8; 24];
    let aligned = storage.as_mut_ptr().cast::<f64>();
    let invalid = if aligned.is_aligned() {
        aligned.wrapping_byte_add(1)
    } else {
        aligned
    };
    // SAFETY: Null and misaligned pointers are explicitly accepted for rejection.
    unsafe {
        assert_eq!(ciglet_rs_minimum_f32(1.0, 2.0, std::ptr::null_mut()), 1);
        assert_eq!(ciglet_rs_maximum_f64(1.0, 2.0, invalid), 1);
    }
    assert_eq!(storage, [0xa5; 24]);
}
