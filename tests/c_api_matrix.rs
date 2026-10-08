#![cfg(feature = "c-api")]
use ciglet_rs::{c_api::*, statistics::Scalar};
use std::ptr;
type From<T, M> = unsafe extern "C" fn(usize, usize, *const T, usize, *mut *mut M) -> u32;
type Zeros<M> = unsafe extern "C" fn(usize, usize, *mut *mut M) -> u32;
type Identity<M> = unsafe extern "C" fn(usize, *mut *mut M) -> u32;
type Clone<H> = unsafe extern "C" fn(*const H, *mut *mut H) -> u32;
type Release<H> = unsafe extern "C" fn(*mut *mut H) -> u32;
type Shape<H> = unsafe extern "C" fn(*const H, *mut usize, usize) -> u32;
type Copy<T, H> = unsafe extern "C" fn(*const H, *mut T, usize) -> u32;
type Get<T, M> = unsafe extern "C" fn(*const M, usize, usize, *mut T) -> u32;
type Product<M> = unsafe extern "C" fn(*const M, *const M, *mut *mut M) -> u32;
type Vector<T, H> = unsafe extern "C" fn(*const H, *const T, usize, *mut T, usize) -> u32;
type Rows<M> = unsafe extern "C" fn(*mut M, *const usize, usize) -> u32;
type Pivot<M> = unsafe extern "C" fn(*mut M, *mut usize, usize) -> u32;
type Perm<T> = unsafe extern "C" fn(*const T, usize, *const usize, usize, *mut T, usize) -> u32;
type Factor<M, L> = unsafe extern "C" fn(*const M, u32, *mut *mut L) -> u32;
struct Api<T, M, L> {
    from: From<T, M>,
    zeros: Zeros<M>,
    identity: Identity<M>,
    clone_matrix: Clone<M>,
    release: Release<M>,
    shape: Shape<M>,
    copy: Copy<T, M>,
    get: Get<T, M>,
    multiply: Product<M>,
    multiply_vector: Vector<T, M>,
    apply_rows: Rows<M>,
    pivot: Pivot<M>,
    permute: Perm<T>,
    factor: Factor<M, L>,
    clone_lu: Clone<L>,
    release_lu: Release<L>,
    shape_lu: Shape<L>,
    packed: Copy<T, L>,
    swaps: Copy<usize, L>,
    solve: Vector<T, L>,
}
struct Reader<'a>(&'a [u8], usize);
impl Reader<'_> {
    fn take(&mut self, count: usize) -> &[u8] {
        let (a, b) = self.0.split_at(count);
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
    fn vector<T: Scalar>(&mut self, count: usize) -> Vec<T> {
        (0..count).map(|_| self.scalar()).collect()
    }
}
fn reference<T: Scalar, M, L>(bytes: &[u8], width: usize, corrected: usize, api: &Api<T, M, L>) {
    let mut reader = Reader(bytes, width);
    assert_eq!(reader.take(4), b"MAT1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), corrected);
    assert_eq!(reader.integer(), 54);
    for case in 0..54 {
        let rows = reader.integer();
        let inner = reader.integer();
        let columns = reader.integer();
        let left = reader.vector::<T>(rows * inner);
        let right = reader.vector::<T>(inner * columns);
        let expected_product = reader.vector::<T>(rows * columns);
        let expected_vector = reader.vector::<T>(rows);
        let mut a = ptr::null_mut();
        let mut b = ptr::null_mut();
        let mut product = ptr::null_mut();
        let mut values = vec![T::zero(); rows * columns];
        let mut vector = vec![T::zero(); rows];
        // SAFETY: Initialized inputs, independent writable buffers/slots and live
        // typed owners retained until their unique clearing releases.
        unsafe {
            assert_eq!(
                (api.from)(rows, inner, left.as_ptr(), left.len(), &mut a),
                0
            );
            assert_eq!(
                (api.from)(inner, columns, right.as_ptr(), right.len(), &mut b),
                0
            );
            assert_eq!((api.multiply)(a, b, &mut product), 0);
            assert_eq!((api.copy)(product, values.as_mut_ptr(), values.len()), 0);
            assert_eq!(values, expected_product, "product {case}");
            assert_eq!(
                (api.multiply_vector)(a, right.as_ptr(), inner, vector.as_mut_ptr(), rows),
                0
            );
            assert_eq!(vector, expected_vector, "vector {case}");
            for slot in [&mut a, &mut b, &mut product] {
                assert_eq!((api.release)(slot), 0);
                assert!(slot.is_null());
            }
        }
    }
    let permutations = reader.integer();
    for case in 0..permutations {
        let rows = reader.integer();
        let columns = reader.integer();
        let input = reader.vector::<T>(rows * columns);
        let mut vector = reader.vector::<T>(rows);
        let swaps: Vec<_> = (0..rows).map(|_| reader.integer()).collect();
        let expected_matrix = reader.vector::<T>(rows * columns);
        let expected_vector = reader.vector::<T>(rows);
        let mut matrix = ptr::null_mut();
        let mut output = vec![T::zero(); input.len()];
        // SAFETY: Valid typed owner, complete independent swap plan and buffers;
        // in-place vector output commits only after input borrowing ends.
        unsafe {
            assert_eq!(
                (api.from)(rows, columns, input.as_ptr(), input.len(), &mut matrix),
                0
            );
            assert_eq!((api.apply_rows)(matrix, swaps.as_ptr(), rows), 0);
            assert_eq!((api.copy)(matrix, output.as_mut_ptr(), output.len()), 0);
            assert_eq!(output, expected_matrix, "row swaps {case}");
            assert_eq!(
                (api.permute)(
                    vector.as_ptr(),
                    rows,
                    swaps.as_ptr(),
                    rows,
                    vector.as_mut_ptr(),
                    rows
                ),
                0
            );
            assert_eq!(vector, expected_vector);
            assert_eq!((api.release)(&mut matrix), 0);
        }
    }
    let solves = reader.integer();
    for case in 0..solves {
        let size = reader.integer();
        let original = reader.vector::<T>(size * size);
        let right = reader.vector::<T>(size);
        let expected_swaps: Vec<_> = (0..size).map(|_| reader.integer()).collect();
        let expected_pivot = reader.vector::<T>(size * size);
        let expected_right = reader.vector::<T>(size);
        let expected_packed = reader.vector::<T>(size * size);
        let expected_solution = reader.vector::<T>(size);
        let mut matrix = ptr::null_mut();
        let mut pivot = ptr::null_mut();
        let mut factor = ptr::null_mut();
        let mut robust = ptr::null_mut();
        let mut values = vec![T::zero(); size * size];
        let mut swaps = vec![0; size];
        let mut reordered = vec![T::zero(); size];
        let mut solution = vec![T::zero(); size];
        // SAFETY: All typed owners remain live; mutation has exclusive access,
        // arrays/slots are independent and every factor is uniquely released.
        unsafe {
            assert_eq!(
                (api.from)(size, size, original.as_ptr(), original.len(), &mut matrix),
                0
            );
            assert_eq!((api.clone_matrix)(matrix, &mut pivot), 0);
            assert_eq!((api.pivot)(pivot, swaps.as_mut_ptr(), size), 0);
            assert_eq!(swaps, expected_swaps);
            assert_eq!((api.copy)(pivot, values.as_mut_ptr(), values.len()), 0);
            assert_eq!(values, expected_pivot);
            assert_eq!(
                (api.permute)(
                    right.as_ptr(),
                    size,
                    swaps.as_ptr(),
                    size,
                    reordered.as_mut_ptr(),
                    size
                ),
                0
            );
            assert_eq!(reordered, expected_right);
            assert_eq!((api.factor)(pivot, 1, &mut factor), 0);
            assert_eq!((api.packed)(factor, values.as_mut_ptr(), values.len()), 0);
            assert_eq!(values, expected_packed, "packed {case}");
            assert_eq!((api.swaps)(factor, swaps.as_mut_ptr(), size), 0);
            assert_eq!(swaps, (0..size).collect::<Vec<_>>());
            assert_eq!(
                (api.solve)(
                    factor,
                    reordered.as_ptr(),
                    size,
                    solution.as_mut_ptr(),
                    size
                ),
                0
            );
            assert_eq!(solution, expected_solution, "solve {case}");
            assert_eq!((api.factor)(matrix, 0, &mut robust), 0);
            assert_eq!(
                (api.solve)(robust, right.as_ptr(), size, solution.as_mut_ptr(), size),
                0
            );
            assert_eq!(
                (api.multiply_vector)(
                    matrix,
                    solution.as_ptr(),
                    size,
                    reordered.as_mut_ptr(),
                    size
                ),
                0
            );
            let bound = if width == 4 { 2e-6 } else { 1e-13 };
            for (&observed, &expected) in reordered.iter().zip(&right) {
                let error =
                    (observed.into() - expected.into()).abs() / (1. + expected.into().abs());
                assert!(error <= bound, "residual {case}: {error:e}");
            }
            assert_eq!((api.copy)(matrix, values.as_mut_ptr(), values.len()), 0);
            assert_eq!(values, original);
            for slot in [&mut matrix, &mut pivot] {
                assert_eq!((api.release)(slot), 0);
            }
            for slot in [&mut factor, &mut robust] {
                assert_eq!((api.release_lu)(slot), 0);
            }
        }
    }
    assert!(reader.0.is_empty());
}
fn edges<T: Scalar, M, L>(api: &Api<T, M, L>, maximum: T) {
    let one = T::one();
    let zero = T::zero();
    let sentinel = T::from_f64(71.).unwrap();
    let original: Vec<T> = (1..=6).map(|x| T::from_f64(x as f64).unwrap()).collect();
    let mut matrix = ptr::null_mut();
    let mut copy = ptr::null_mut();
    let mut identity = ptr::null_mut();
    let mut factor = ptr::null_mut();
    let mut factor_copy = ptr::null_mut();
    let mut shape = [71; 3];
    let mut values = [sentinel; 6];
    let mut value = sentinel;
    // SAFETY: Live correctly typed owners, independent complete buffers and slots,
    // exclusive mutation. Zero-length null buffers are permitted without borrowing.
    unsafe {
        assert_eq!((api.from)(2, 3, original.as_ptr(), 6, &mut matrix), 0);
        assert_eq!((api.shape)(matrix, shape.as_mut_ptr(), 3), 0);
        assert_eq!(shape, [2, 3, 6]);
        assert_eq!((api.get)(matrix, 1, 2, &mut value), 0);
        assert_eq!(value, T::from_f64(6.).unwrap());
        value = sentinel;
        assert_eq!((api.get)(matrix, 2, 0, &mut value), 2);
        assert_eq!(value, sentinel);
        assert_eq!((api.clone_matrix)(matrix, &mut copy), 0);
        assert_eq!((api.apply_rows)(copy, [1, 0].as_ptr(), 2), 0);
        assert_eq!((api.copy)(copy, values.as_mut_ptr(), 6), 0);
        assert_eq!(values.as_slice(), original);
        assert_eq!((api.apply_rows)(copy, [1, 1].as_ptr(), 2), 0);
        let expected: Vec<T> = [2., 1., 4., 3., 6., 5.]
            .into_iter()
            .map(|x| T::from_f64(x).unwrap())
            .collect();
        assert_eq!((api.copy)(copy, values.as_mut_ptr(), 6), 0);
        assert_eq!(values.as_slice(), expected);
        assert_eq!((api.apply_rows)(copy, [1, 2].as_ptr(), 2), 3);
        assert_eq!((api.copy)(copy, values.as_mut_ptr(), 6), 0);
        assert_eq!(values.as_slice(), expected);
        let mut swaps = [71; 2];
        assert_eq!((api.pivot)(copy, swaps.as_mut_ptr(), 2), 3);
        assert_eq!(swaps, [71; 2]);
        assert_eq!((api.copy)(copy, values.as_mut_ptr(), 6), 0);
        assert_eq!(values.as_slice(), expected);
        assert_eq!((api.identity)(2, &mut identity), 0);
        assert_eq!((api.factor)(identity, 0, &mut factor), 0);
        let retained = factor;
        assert_eq!((api.factor)(matrix, 0, &mut factor), 3);
        assert_eq!(factor, retained);
        assert_eq!((api.factor)(identity, 2, &mut factor), 3);
        assert_eq!(factor, retained);
        assert_eq!((api.clone_lu)(factor, &mut factor_copy), 0);
        assert_eq!((api.release_lu)(&mut factor), 0);
        assert_eq!((api.shape_lu)(factor_copy, shape.as_mut_ptr(), 3), 0);
        assert_eq!(shape, [2, 2, 4]);
        let mut right = [one, T::from_f64(2.).unwrap()];
        assert_eq!(
            (api.solve)(factor_copy, right.as_ptr(), 2, right.as_mut_ptr(), 2),
            0
        );
        assert_eq!(right, [one, T::from_f64(2.).unwrap()]);
        assert_eq!((api.release_lu)(&mut factor_copy), 0);
        assert_eq!((api.release_lu)(&mut factor_copy), 0);
        let retained = matrix;
        assert_eq!((api.zeros)(usize::MAX, 2, &mut matrix), 2);
        assert_eq!(matrix, retained);
        let nan = [T::from_f64(f64::NAN).unwrap()];
        assert_eq!((api.from)(1, 1, nan.as_ptr(), 1, &mut matrix), 3);
        assert_eq!(matrix, retained);
        assert_eq!((api.from)(1, 1, &maximum, 1, ptr::null_mut()), 1);
        for slot in [&mut matrix, &mut copy, &mut identity] {
            assert_eq!((api.release)(slot), 0);
        }
        assert_eq!((api.zeros)(2, 0, &mut matrix), 0);
        let mut result = [sentinel; 2];
        assert_eq!(
            (api.multiply_vector)(matrix, ptr::null(), 0, result.as_mut_ptr(), 2),
            0
        );
        assert_eq!(result, [zero; 2]);
        assert_eq!((api.copy)(matrix, ptr::null_mut(), 0), 0);
        assert_eq!((api.release)(&mut matrix), 0);
        assert_eq!((api.identity)(0, &mut matrix), 0);
        assert_eq!((api.factor)(matrix, 0, &mut factor), 0);
        assert_eq!((api.solve)(factor, ptr::null(), 0, ptr::null_mut(), 0), 0);
        assert_eq!((api.packed)(factor, ptr::null_mut(), 0), 0);
        assert_eq!((api.swaps)(factor, ptr::null_mut(), 0), 0);
        assert_eq!((api.release_lu)(&mut factor), 0);
        assert_eq!((api.release)(&mut matrix), 0);
        assert_eq!((api.release)(&mut matrix), 0);
        assert_eq!(
            (api.permute)(ptr::null(), 0, ptr::null(), 0, ptr::null_mut(), 0),
            0
        );
    }
}
#[test]
fn original_float() {
    reference(
        include_bytes!("fixtures/matrix-legacy-float.bin"),
        4,
        0,
        &float_api(),
    );
}
#[test]
fn original_double() {
    reference(
        include_bytes!("fixtures/matrix-legacy-double.bin"),
        8,
        0,
        &double_api(),
    );
}
#[test]
fn corrected_float() {
    reference(
        include_bytes!("fixtures/matrix-corrected-float.bin"),
        4,
        1,
        &float_api(),
    );
}
#[test]
fn corrected_double() {
    reference(
        include_bytes!("fixtures/matrix-corrected-double.bin"),
        8,
        1,
        &double_api(),
    );
}
#[test]
fn float_shapes_ownership_and_atomic_errors() {
    edges(&float_api(), f32::MAX);
}
#[test]
fn double_shapes_ownership_and_atomic_errors() {
    edges(&double_api(), f64::MAX);
}

fn float_api() -> Api<f32, CigletRsMatrixF32, CigletRsLuF32> {
    Api {
        from: ciglet_rs_matrix_from_column_major_f32,
        zeros: ciglet_rs_matrix_zeros_f32,
        identity: ciglet_rs_matrix_identity_f32,
        clone_matrix: ciglet_rs_matrix_clone_f32,
        release: ciglet_rs_matrix_release_f32,
        shape: ciglet_rs_matrix_shape_f32,
        copy: ciglet_rs_matrix_copy_column_major_f32,
        get: ciglet_rs_matrix_get_f32,
        multiply: ciglet_rs_matrix_multiply_f32,
        multiply_vector: ciglet_rs_matrix_multiply_vector_f32,
        apply_rows: ciglet_rs_matrix_apply_row_swaps_f32,
        pivot: ciglet_rs_matrix_pivot_rows_f32,
        permute: ciglet_rs_matrix_apply_swaps_f32,
        factor: ciglet_rs_lu_factor_f32,
        clone_lu: ciglet_rs_lu_clone_f32,
        release_lu: ciglet_rs_lu_release_f32,
        shape_lu: ciglet_rs_lu_shape_f32,
        packed: ciglet_rs_lu_copy_packed_f32,
        swaps: ciglet_rs_lu_copy_row_swaps_f32,
        solve: ciglet_rs_lu_solve_f32,
    }
}

fn double_api() -> Api<f64, CigletRsMatrixF64, CigletRsLuF64> {
    Api {
        from: ciglet_rs_matrix_from_column_major_f64,
        zeros: ciglet_rs_matrix_zeros_f64,
        identity: ciglet_rs_matrix_identity_f64,
        clone_matrix: ciglet_rs_matrix_clone_f64,
        release: ciglet_rs_matrix_release_f64,
        shape: ciglet_rs_matrix_shape_f64,
        copy: ciglet_rs_matrix_copy_column_major_f64,
        get: ciglet_rs_matrix_get_f64,
        multiply: ciglet_rs_matrix_multiply_f64,
        multiply_vector: ciglet_rs_matrix_multiply_vector_f64,
        apply_rows: ciglet_rs_matrix_apply_row_swaps_f64,
        pivot: ciglet_rs_matrix_pivot_rows_f64,
        permute: ciglet_rs_matrix_apply_swaps_f64,
        factor: ciglet_rs_lu_factor_f64,
        clone_lu: ciglet_rs_lu_clone_f64,
        release_lu: ciglet_rs_lu_release_f64,
        shape_lu: ciglet_rs_lu_shape_f64,
        packed: ciglet_rs_lu_copy_packed_f64,
        swaps: ciglet_rs_lu_copy_row_swaps_f64,
        solve: ciglet_rs_lu_solve_f64,
    }
}

fn elimination<T: Scalar, M, L>(api: &Api<T, M, L>, bound: f64) {
    let original: Vec<T> = [4., 2., 1., 4., 2., 0., 1., 2., 3.]
        .into_iter()
        .map(|v| T::from_f64(v).unwrap())
        .collect();
    let mut matrix = ptr::null_mut();
    let mut preliminary = ptr::null_mut();
    let mut factor = ptr::null_mut();
    let mut plan = [71; 3];
    let mut packed = [T::zero(); 9];
    let mut permuted = [T::zero(); 9];
    let mut result = [T::zero(); 3];
    let right: Vec<T> = [15., 12., 10.]
        .into_iter()
        .map(|v| T::from_f64(v).unwrap())
        .collect();
    // SAFETY: All owners retained through independent buffers and exclusively
    // mutated preliminary matrix; each owner uniquely released at the end.
    unsafe {
        assert_eq!((api.from)(3, 3, original.as_ptr(), 9, &mut matrix), 0);
        assert_eq!((api.clone_matrix)(matrix, &mut preliminary), 0);
        assert_eq!((api.pivot)(preliminary, plan.as_mut_ptr(), 3), 0);
        assert_eq!(plan, [0, 1, 2]);
        assert_eq!((api.factor)(preliminary, 1, &mut factor), 3);
        assert!(factor.is_null());
        assert_eq!((api.factor)(matrix, 0, &mut factor), 0);
        assert_eq!(
            (api.solve)(factor, right.as_ptr(), 3, result.as_mut_ptr(), 3),
            0
        );
        for (&observed, expected) in result.iter().zip([1., 2., 3.]) {
            assert!((observed.into() - expected).abs() <= bound);
        }
        assert_eq!((api.packed)(factor, packed.as_mut_ptr(), 9), 0);
        assert_eq!((api.swaps)(factor, plan.as_mut_ptr(), 3), 0);
        assert_eq!((api.apply_rows)(preliminary, plan.as_ptr(), 3), 0);
        assert_eq!((api.copy)(preliminary, permuted.as_mut_ptr(), 9), 0);
        for row in 0..3 {
            for col in 0..3 {
                let reconstructed: f64 = (0..3)
                    .map(|k| {
                        let lower = if row == k {
                            1.
                        } else if row > k {
                            packed[row + k * 3].into()
                        } else {
                            0.
                        };
                        let upper = if k <= col {
                            packed[k + col * 3].into()
                        } else {
                            0.
                        };
                        lower * upper
                    })
                    .sum();
                assert!((reconstructed - permuted[row + col * 3].into()).abs() <= bound);
            }
        }
        let retained = factor;
        let singular: Vec<T> = [1., 2., 2., 4.]
            .into_iter()
            .map(|v| T::from_f64(v).unwrap())
            .collect();
        assert_eq!((api.release)(&mut preliminary), 0);
        assert_eq!((api.from)(2, 2, singular.as_ptr(), 4, &mut preliminary), 0);
        assert_eq!((api.factor)(preliminary, 0, &mut factor), 3);
        assert_eq!(factor, retained);
        assert_eq!((api.release)(&mut matrix), 0);
        assert_eq!((api.release)(&mut preliminary), 0);
        assert_eq!((api.release_lu)(&mut factor), 0);
    }
}
#[test]
fn float_late_pivot_reconstructs_factors_and_retains_owner_on_singular_error() {
    elimination(&float_api(), 2e-6);
}
#[test]
fn double_late_pivot_reconstructs_factors_and_retains_owner_on_singular_error() {
    elimination(&double_api(), 1e-12);
}
#[test]
fn permutations_preserve_nan_payloads_and_signed_zero_bits() {
    let mut float = [f32::from_bits(0x7fc12345), -0.0, f32::INFINITY];
    let mut double = [f64::from_bits(0x7ff8123456789abc), -0.0, f64::NEG_INFINITY];
    let plan = [2usize, 1, 2];
    // SAFETY: Complete initialized scalar arrays and independent swap plan;
    // staged copying allows in-place output and preserves every scalar bit.
    unsafe {
        assert_eq!(
            ciglet_rs_matrix_apply_swaps_f32(
                float.as_ptr(),
                3,
                plan.as_ptr(),
                3,
                float.as_mut_ptr(),
                3
            ),
            0
        );
        assert_eq!(
            ciglet_rs_matrix_apply_swaps_f64(
                double.as_ptr(),
                3,
                plan.as_ptr(),
                3,
                double.as_mut_ptr(),
                3
            ),
            0
        );
    }
    assert_eq!(
        float.map(f32::to_bits),
        [f32::INFINITY.to_bits(), (-0.0f32).to_bits(), 0x7fc12345]
    );
    assert_eq!(
        double.map(f64::to_bits),
        [
            f64::NEG_INFINITY.to_bits(),
            (-0.0f64).to_bits(),
            0x7ff8123456789abc
        ]
    );
}
