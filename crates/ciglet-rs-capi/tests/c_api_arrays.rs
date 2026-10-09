use ciglet_rs_capi::*;
#[test]
fn owned_array_lifecycle_supports_shape_rows_and_independent_copies() {
    let source = [1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0];
    let mut handle = std::ptr::null_mut();
    let mut clone = std::ptr::null_mut();
    let mut transposed = std::ptr::null_mut();
    let mut dimensions = [0_usize; 3];
    let mut values = [0.0_f64; 6];
    let mut value = 0.0;
    // SAFETY: All handles originate here, remain live for each operation, and
    // are exclusively held when modified/released. Caller buffers are independent.
    unsafe {
        assert_eq!(
            ciglet_rs_array_from_slice_f64(2, 3, source.as_ptr(), 6, &mut handle),
            0
        );
        assert_eq!(
            ciglet_rs_array_shape_f64(handle, dimensions.as_mut_ptr(), 3),
            0
        );
        assert_eq!(dimensions, [2, 3, 6]);
        assert_eq!(ciglet_rs_array_set_f64(handle, 0, 1, 9.0), 0);
        assert_eq!(ciglet_rs_array_get_f64(handle, 0, 1, &mut value), 0);
        assert_eq!(value, 9.0);
        assert_eq!(ciglet_rs_array_clone_f64(handle, &mut clone), 0);
        assert_ne!(clone, handle);
        assert_eq!(ciglet_rs_array_set_f64(clone, 0, 0, -1.0), 0);
        assert_eq!(ciglet_rs_array_get_f64(handle, 0, 0, &mut value), 0);
        assert_eq!(value, 1.0);
        assert_eq!(ciglet_rs_array_transposed_f64(handle, &mut transposed), 0);
        assert_eq!(
            ciglet_rs_array_copy_flat_f64(transposed, values.as_mut_ptr(), 6),
            0
        );
        assert_eq!(values, [1.0, 4.0, 9.0, 5.0, 3.0, 6.0]);
        assert_eq!(ciglet_rs_array_reshape_f64(handle, 3, 2), 0);
        assert_eq!(ciglet_rs_array_transpose_f64(handle), 0);
        assert_eq!(
            ciglet_rs_array_copy_flat_f64(handle, values.as_mut_ptr(), 6),
            0
        );
        assert_eq!(values, [1.0, 3.0, 5.0, 9.0, 4.0, 6.0]);
        let row = [-2.0, -3.0, -4.0];
        assert_eq!(ciglet_rs_array_write_row_f64(handle, 0, row.as_ptr(), 3), 0);
        let mut copied_row = [0.0; 3];
        assert_eq!(
            ciglet_rs_array_copy_row_f64(handle, 0, copied_row.as_mut_ptr(), 3),
            0
        );
        assert_eq!(copied_row, row);
        assert_eq!(
            ciglet_rs_array_write_flat_f64(handle, source.as_ptr(), 6),
            0
        );
        assert_eq!(
            ciglet_rs_array_copy_flat_f64(handle, values.as_mut_ptr(), 6),
            0
        );
        assert_eq!(values, source);
        assert_eq!(ciglet_rs_array_release_f64(&mut handle), 0);
        assert_eq!(ciglet_rs_array_release_f64(&mut handle), 0);
        assert_eq!(ciglet_rs_array_release_f64(&mut clone), 0);
        assert_eq!(ciglet_rs_array_release_f64(&mut transposed), 0);
    }
    assert!(handle.is_null() && clone.is_null() && transposed.is_null());
}

#[test]
fn row_tables_copy_special_values_and_reject_ragged_rows() {
    let first = [f32::from_bits(0xffc1_2345), -0.0];
    let second = [0.0_f32, f32::from_bits(0x7fc0_1234)];
    let rows = [first.as_ptr(), second.as_ptr()];
    let mut handle = std::ptr::null_mut();
    let mut output = [0.0_f32; 4];
    // SAFETY: Initialized row tables and arrays remain stable; handles are local.
    unsafe {
        assert_eq!(
            ciglet_rs_array_from_rows_f32(rows.as_ptr(), [2, 1].as_ptr(), 2, &mut handle),
            3
        );
        assert!(handle.is_null());
        assert_eq!(
            ciglet_rs_array_from_rows_f32(rows.as_ptr(), [2, 2].as_ptr(), 2, &mut handle),
            0
        );
        assert_eq!(
            ciglet_rs_array_copy_flat_f32(handle, output.as_mut_ptr(), 4),
            0
        );
        assert_eq!(
            output.map(f32::to_bits),
            [
                first[0].to_bits(),
                (-0.0_f32).to_bits(),
                0,
                second[1].to_bits()
            ]
        );
        assert_eq!(ciglet_rs_array_transpose_f32(handle), 0);
        assert_eq!(
            ciglet_rs_array_copy_flat_f32(handle, output.as_mut_ptr(), 4),
            0
        );
        assert_eq!(
            output.map(f32::to_bits),
            [
                first[0].to_bits(),
                0,
                (-0.0_f32).to_bits(),
                second[1].to_bits()
            ]
        );
        assert_eq!(ciglet_rs_array_release_f32(&mut handle), 0);
    }
}

#[test]
fn array_failures_preserve_handles_metadata_values_and_outputs() {
    let mut handle = std::ptr::null_mut();
    let mut output = [91.0_f32; 4];
    let mut value = 71.0_f32;
    let mut shape = [91_usize; 3];
    // SAFETY: Valid live handle and independent buffers; bad lengths/coordinates
    // and null arguments are explicitly supplied for checked rejection.
    unsafe {
        assert_eq!(ciglet_rs_array_filled_f32(2, 2, -0.0, &mut handle), 0);
        let original = handle;
        assert_eq!(
            ciglet_rs_array_filled_f32(usize::MAX, 2, 0.0, &mut handle),
            2
        );
        assert_eq!(handle, original);
        assert_eq!(ciglet_rs_array_reshape_f32(handle, 3, 2), 3);
        assert_eq!(ciglet_rs_array_set_f32(handle, 2, 0, 5.0), 3);
        assert_eq!(ciglet_rs_array_get_f32(handle, 2, 0, &mut value), 3);
        assert_eq!(value, 71.0);
        assert_eq!(ciglet_rs_array_shape_f32(handle, shape.as_mut_ptr(), 2), 2);
        assert_eq!(shape, [91; 3]);
        assert_eq!(
            ciglet_rs_array_copy_flat_f32(handle, output.as_mut_ptr(), 3),
            2
        );
        assert_eq!(output, [91.0; 4]);
        assert_eq!(
            ciglet_rs_array_write_flat_f32(handle, [1.0, 2.0, 3.0].as_ptr(), 3),
            2
        );
        assert_eq!(
            ciglet_rs_array_copy_row_f32(handle, 2, output.as_mut_ptr(), 2),
            3
        );
        assert_eq!(output, [91.0; 4]);
        assert_eq!(
            ciglet_rs_array_get_f32(std::ptr::null(), 0, 0, &mut value),
            1
        );
        assert_eq!(
            ciglet_rs_array_copy_flat_f32(handle, output.as_mut_ptr(), 4),
            0
        );
        assert_eq!(output.map(f32::to_bits), [(-0.0_f32).to_bits(); 4]);
        assert_eq!(ciglet_rs_array_shape_f32(handle, shape.as_mut_ptr(), 3), 0);
        assert_eq!(shape, [2, 2, 4]);
        assert_eq!(ciglet_rs_array_release_f32(&mut handle), 0);
    }
}

#[test]
fn empty_shapes_and_row_tables_need_no_element_storage() {
    let mut handle = std::ptr::null_mut();
    let mut dimensions = [0_usize; 3];
    // SAFETY: Empty inputs permit null pointers; output slots and metadata are
    // valid. Correctly typed handles are released exactly once.
    unsafe {
        assert_eq!(
            ciglet_rs_array_from_slice_f64(0, usize::MAX, std::ptr::null(), 0, &mut handle),
            0
        );
        assert_eq!(ciglet_rs_array_transpose_f64(handle), 0);
        assert_eq!(
            ciglet_rs_array_shape_f64(handle, dimensions.as_mut_ptr(), 3),
            0
        );
        assert_eq!(dimensions, [usize::MAX, 0, 0]);
        assert_eq!(
            ciglet_rs_array_copy_row_f64(handle, usize::MAX - 1, std::ptr::null_mut(), 0),
            0
        );
        assert_eq!(
            ciglet_rs_array_write_row_f64(handle, 0, std::ptr::null(), 0),
            0
        );
        assert_eq!(
            ciglet_rs_array_copy_flat_f64(handle, std::ptr::null_mut(), 0),
            0
        );
        assert_eq!(ciglet_rs_array_release_f64(&mut handle), 0);
        assert_eq!(
            ciglet_rs_array_from_rows_f64(std::ptr::null(), std::ptr::null(), 0, &mut handle),
            0
        );
        assert_eq!(
            ciglet_rs_array_shape_f64(handle, dimensions.as_mut_ptr(), 3),
            0
        );
        assert_eq!(dimensions, [0, 0, 0]);
        assert_eq!(ciglet_rs_array_release_f64(&mut handle), 0);
    }
}
