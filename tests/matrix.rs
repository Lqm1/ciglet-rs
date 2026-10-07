use ciglet_rs::matrix::{self, Lu, Matrix};
use rustfft::FftNum;
struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
    width: usize,
}
impl Reader<'_> {
    fn integer(&mut self) -> usize {
        let value =
            u32::from_le_bytes(self.bytes[self.cursor..self.cursor + 4].try_into().unwrap());
        self.cursor += 4;
        value as usize
    }
    fn scalar<T: FftNum>(&mut self) -> T {
        let value = if self.width == 4 {
            f32::from_le_bytes(self.bytes[self.cursor..self.cursor + 4].try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.bytes[self.cursor..self.cursor + 8].try_into().unwrap())
        };
        self.cursor += self.width;
        T::from_f64(value).unwrap()
    }
    fn vector<T: FftNum>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.scalar()).collect()
    }
}
fn check<T: FftNum + Into<f64>>(bytes: &[u8]) {
    assert_eq!(&bytes[..4], b"MAT1");
    let mut r = Reader {
        bytes,
        cursor: 4,
        width: 0,
    };
    r.width = r.integer();
    let _corrected = r.integer();
    let products = r.integer();
    for case in 0..products {
        let m = r.integer();
        let n = r.integer();
        let l = r.integer();
        let a = Matrix::<T>::from_column_major(m, n, r.vector(m * n)).unwrap();
        let b = Matrix::from_column_major(n, l, r.vector(n * l)).unwrap();
        let product = r.vector::<T>(m * l);
        let vector = r.vector::<T>(m);
        assert_eq!(
            a.multiply(&b).unwrap().as_slice(),
            product,
            "matrix product {case}"
        );
        assert_eq!(
            a.multiply_vector(&b.as_slice()[..n]).unwrap(),
            vector,
            "vector product {case}"
        );
    }
    let permutations = r.integer();
    for case in 0..permutations {
        let m = r.integer();
        let n = r.integer();
        let mut a = Matrix::from_column_major(m, n, r.vector::<T>(m * n)).unwrap();
        let mut vector = r.vector::<T>(m);
        let swaps: Vec<_> = (0..m).map(|_| r.integer()).collect();
        let expected = r.vector::<T>(m * n);
        let expected_vector = r.vector::<T>(m);
        a.apply_row_swaps(&swaps).unwrap();
        matrix::apply_swaps(&mut vector, &swaps).unwrap();
        assert_eq!(a.as_slice(), expected, "row swaps {case}");
        assert_eq!(vector, expected_vector);
    }
    let solves = r.integer();
    for case in 0..solves {
        let n = r.integer();
        let original = Matrix::from_column_major(n, n, r.vector::<T>(n * n)).unwrap();
        let right = r.vector::<T>(n);
        let swaps: Vec<_> = (0..n).map(|_| r.integer()).collect();
        let expected_pivot = r.vector::<T>(n * n);
        let expected_right = r.vector::<T>(n);
        let expected_lu = r.vector::<T>(n * n);
        let expected_solution = r.vector::<T>(n);
        let mut pivot = original.clone();
        assert_eq!(pivot.pivot_rows().unwrap(), swaps);
        assert_eq!(pivot.as_slice(), expected_pivot);
        let mut reordered = right.clone();
        matrix::apply_swaps(&mut reordered, &swaps).unwrap();
        assert_eq!(reordered, expected_right);
        let lu = Lu::factor_unpivoted(pivot).unwrap();
        assert_eq!(lu.packed().as_slice(), expected_lu, "LU case {case}");
        assert_eq!(
            lu.solve(&reordered).unwrap(),
            expected_solution,
            "substitution case {case}"
        );
        let robust = Lu::factor(original.clone()).unwrap().solve(&right).unwrap();
        let reconstructed = original.multiply_vector(&robust).unwrap();
        let tolerance = if r.width == 4 { 2e-6 } else { 1e-13 };
        for (&observed, &expected) in reconstructed.iter().zip(&right) {
            assert!(
                (observed.into() - expected.into()).abs() / (1.0 + expected.into().abs())
                    <= tolerance,
                "robust residual case {case}"
            );
        }
    }
    assert_eq!(r.cursor, bytes.len());
}
#[test]
fn original_float() {
    check::<f32>(include_bytes!("fixtures/matrix-legacy-float.bin"));
}
#[test]
fn original_double() {
    check::<f64>(include_bytes!("fixtures/matrix-legacy-double.bin"));
}
#[test]
fn corrected_float() {
    check::<f32>(include_bytes!("fixtures/matrix-corrected-float.bin"));
}
#[test]
fn corrected_double() {
    check::<f64>(include_bytes!("fixtures/matrix-corrected-double.bin"));
}
#[test]
fn elimination_pivoting_and_independent_solution() {
    let original =
        Matrix::from_column_major(3, 3, vec![4.0f64, 2.0, 1.0, 4.0, 2.0, 0.0, 1.0, 2.0, 3.0])
            .unwrap();
    let mut preliminary = original.clone();
    assert_eq!(preliminary.pivot_rows().unwrap(), vec![0, 1, 2]);
    assert!(Lu::factor_unpivoted(preliminary).is_err());
    let lu = Lu::factor(original.clone()).unwrap();
    let solution = lu.solve(&[15.0, 12.0, 10.0]).unwrap();
    for (x, y) in solution.iter().zip([1.0, 2.0, 3.0]) {
        assert!((x - y).abs() < 1e-12);
    }
    assert_eq!(
        original.as_slice(),
        &[4.0, 2.0, 1.0, 4.0, 2.0, 0.0, 1.0, 2.0, 3.0]
    );
    let mut permuted = original.clone();
    permuted.apply_row_swaps(lu.row_swaps()).unwrap();
    for row in 0..3 {
        for col in 0..3 {
            let value = (0..3)
                .map(|k| {
                    let lower = if row == k {
                        1.0
                    } else if row > k {
                        *lu.packed().get(row, k).unwrap()
                    } else {
                        0.0
                    };
                    let upper = if k <= col {
                        *lu.packed().get(k, col).unwrap()
                    } else {
                        0.0
                    };
                    lower * upper
                })
                .sum::<f64>();
            assert!((value - permuted.get(row, col).unwrap()).abs() < 1e-12);
        }
    }
}
#[test]
fn shapes_empty_identity_singular_and_invalid_swaps() {
    let a = Matrix::<f32>::zeros(2, 0).unwrap();
    let b = Matrix::zeros(0, 3).unwrap();
    assert_eq!(a.multiply(&b).unwrap().as_slice(), &[0.0; 6]);
    assert_eq!(a.multiply_vector(&[]).unwrap(), vec![0.0; 2]);
    let empty = Matrix::<f64>::identity(0).unwrap();
    assert!(Lu::factor(empty).unwrap().solve(&[]).unwrap().is_empty());
    let rectangular =
        Matrix::from_column_major(2, 3, vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
    assert_eq!(
        rectangular.multiply(&Matrix::identity(3).unwrap()).unwrap(),
        rectangular
    );
    assert_eq!(
        rectangular.multiply_vector(&[1.0, 2.0, 3.0]).unwrap(),
        vec![22.0, 28.0]
    );
    assert!(Lu::factor(rectangular.clone()).is_err());
    assert!(rectangular.multiply_vector(&[1.0]).is_err());
    assert!(Matrix::<f32>::zeros(usize::MAX, 2).is_err());
    assert!(Matrix::<f32>::from_column_major(2, 2, vec![1.0]).is_err());
    assert!(Matrix::from_column_major(1, 1, vec![f64::NAN]).is_err());
    assert!(rectangular.get(usize::MAX, 0).is_none());
    let singular = Matrix::from_column_major(2, 2, vec![1.0, 2.0, 2.0, 4.0]).unwrap();
    assert!(Lu::factor(singular).is_err());
    let mut vector = vec![1, 2];
    assert!(matrix::apply_swaps(&mut vector, &[1, 2]).is_err());
    assert_eq!(vector, vec![1, 2]);
    let mut copy = rectangular.clone();
    assert!(copy.apply_row_swaps(&[1, 2]).is_err());
    assert_eq!(copy, rectangular);
    assert!(
        Matrix::from_column_major(1, 1, vec![f32::MAX])
            .unwrap()
            .multiply_vector(&[2.0])
            .is_err()
    );
    assert!(
        Lu::factor(Matrix::identity(2).unwrap())
            .unwrap()
            .solve(&[f32::NAN, 1.0])
            .is_err()
    );
}
