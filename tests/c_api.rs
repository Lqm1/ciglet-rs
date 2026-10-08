#![cfg(feature = "c-api")]

use ciglet_rs::c_api::*;
use std::mem::MaybeUninit;

#[test]
fn ordering_preserves_total_order_payloads_and_original_indices() {
    let negative_nan = f64::from_bits(0xfff8_1234_5678_9abc);
    let positive_nan = f64::from_bits(0x7ff8_0123_4567_89ab);
    let original = [
        3.0_f64,
        -0.0,
        0.0,
        3.0,
        positive_nan,
        f64::NEG_INFINITY,
        negative_nan,
    ];
    let mut sorted = original;
    let mut positions = [usize::MAX; 7];
    let pointer = sorted.as_mut_ptr();
    // SAFETY: In-place sorted output is supported; indices are a disjoint buffer.
    unsafe {
        assert_eq!(
            ciglet_rs_sort_f64(pointer, 7, pointer, 7, positions.as_mut_ptr(), 7),
            0
        )
    };
    assert_eq!(positions, [6, 5, 1, 2, 0, 3, 4]);
    for (value, index) in sorted.iter().zip(positions) {
        assert_eq!(value.to_bits(), original[index].to_bits());
    }
    let mut partitioned = original;
    let mut selected = 71.0_f64;
    // SAFETY: Partition and scalar result storage are initialized and disjoint.
    unsafe {
        assert_eq!(
            ciglet_rs_select_nth_f64(partitioned.as_mut_ptr(), 7, 3, &mut selected),
            0
        )
    };
    assert_eq!(selected.to_bits(), 0);
    assert!(
        partitioned[..3]
            .iter()
            .all(|value| value.total_cmp(&selected).is_le())
    );
    assert!(
        partitioned[4..]
            .iter()
            .all(|value| value.total_cmp(&selected).is_ge())
    );
    let mut before = original.map(f64::to_bits);
    let mut after = partitioned.map(f64::to_bits);
    before.sort();
    after.sort();
    assert_eq!(before, after);
}

#[test]
fn ordering_rejects_overlapping_outputs_without_partial_writes() {
    let mut values = [3.0_f32, 1.0, 2.0];
    let original = values;
    let pointer = values.as_mut_ptr();
    let mut selected = -71.0_f32;
    let mut storage = [91_usize; 4];
    // SAFETY: Null or overlapping outputs are supplied only for rejection; all
    // other ranges are valid and no concurrent accesses occur.
    unsafe {
        assert_eq!(ciglet_rs_select_nth_f32(pointer, 3, 3, &mut selected), 3);
        assert_eq!(ciglet_rs_select_nth_f32(pointer, 3, 1, pointer.add(1)), 2);
        assert_eq!(
            ciglet_rs_sort_f32(
                pointer,
                3,
                storage.as_mut_ptr().cast(),
                3,
                storage.as_mut_ptr(),
                3
            ),
            2
        );
        assert_eq!(
            ciglet_rs_sort_f32(pointer, 3, pointer, 2, storage.as_mut_ptr(), 3),
            2
        );
        assert_eq!(
            ciglet_rs_sort_f32(
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0
            ),
            0
        );
    }
    assert_eq!(values, original);
    assert_eq!(selected, -71.0);
    assert_eq!(storage, [91; 4]);
}

#[test]
fn paired_statistics_preserve_scaled_moments_and_overlap() {
    let extreme = [-1e300_f64, 0.0, 1e300];
    let mut result = -71.0_f64;
    // SAFETY: Both inputs are valid readable ranges; output is exclusive.
    unsafe {
        assert_eq!(
            ciglet_rs_correlation_f64(extreme.as_ptr(), 3, extreme.as_ptr(), 3, &mut result),
            0
        );
        assert_eq!(result, 1.0);
        assert_eq!(
            ciglet_rs_covariance_f64(extreme.as_ptr(), 3, extreme.as_ptr(), 3, &mut result),
            3
        );
        assert_eq!(result, 1.0);
        assert_eq!(
            ciglet_rs_correlation_with_f64(
                extreme.as_ptr(),
                3,
                extreme.as_ptr(),
                3,
                1,
                &mut result
            ),
            3
        );
        assert_eq!(result, 1.0);
    }
    let mut left = [1.0_f32, 2.0, 3.0, 91.0];
    let right = [2.0_f32, 4.0, 6.0];
    let pointer = left.as_mut_ptr();
    // SAFETY: Output overlaps a valid input range; the adapter stages its result.
    unsafe {
        assert_eq!(
            ciglet_rs_cross_correlation_f32(pointer, 3, right.as_ptr(), 3, pointer, 4),
            0
        );
    }
    assert_eq!(left, [28.0, 16.0, 6.0, 0.0]);
    let pointer = left.as_mut_ptr();
    // SAFETY: Identical immutable inputs may alias; scalar output may overlap.
    unsafe { assert_eq!(ciglet_rs_dot_f32(pointer, 3, pointer, 3, pointer), 0) };
    assert_eq!(left[0], 1076.0);
}

#[test]
fn paired_statistics_failures_and_empty_ranges_are_transactional() {
    let left = [1.0_f64, 1.0, 1.0];
    let right = [2.0_f64, 3.0, 4.0];
    let mut output = [-71.0_f64; 4];
    // SAFETY: Buffers are valid; null inputs are supplied only for rejection or
    // zero-length ranges, and output is never accessed concurrently.
    unsafe {
        assert_eq!(
            ciglet_rs_correlation_f64(left.as_ptr(), 3, right.as_ptr(), 3, output.as_mut_ptr()),
            3
        );
        assert_eq!(
            ciglet_rs_dot_f64(left.as_ptr(), 3, right.as_ptr(), 2, output.as_mut_ptr()),
            2
        );
        assert_eq!(
            ciglet_rs_dot_f64(left.as_ptr(), 3, std::ptr::null(), 3, output.as_mut_ptr()),
            1
        );
        assert_eq!(
            ciglet_rs_covariance_with_f64(
                left.as_ptr(),
                3,
                right.as_ptr(),
                3,
                u32::MAX,
                output.as_mut_ptr()
            ),
            3
        );
        assert_eq!(output, [-71.0; 4]);
        assert_eq!(
            ciglet_rs_cross_correlation_f64(
                std::ptr::null(),
                0,
                std::ptr::null(),
                0,
                output.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(output, [0.0; 4]);
        assert_eq!(
            ciglet_rs_dot_f64(
                std::ptr::null(),
                0,
                std::ptr::null(),
                0,
                output.as_mut_ptr()
            ),
            0
        );
        assert_eq!(
            ciglet_rs_cross_correlation_f64(
                left.as_ptr(),
                3,
                right.as_ptr(),
                3,
                std::ptr::null_mut(),
                0
            ),
            0
        );
    }
}

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
