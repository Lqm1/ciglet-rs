use super::{Matrix, apply_swaps, finite};
use crate::Error;
use rustfft::FftNum;

/// Packed LU factors with an implicit unit diagonal in L and ordered row swaps.
#[derive(Debug, Clone)]
pub struct Lu<T> {
    packed: Matrix<T>,
    swaps: Vec<usize>,
}

impl<T: FftNum + Into<f64>> Lu<T> {
    fn square(matrix: &Matrix<T>) -> Result<usize, Error> {
        if matrix.rows != matrix.columns {
            return Err(Error("LU factorization requires a square matrix"));
        }
        Ok(matrix.rows)
    }
    fn finish(packed: Matrix<T>, swaps: Vec<usize>) -> Result<Self, Error> {
        finite(&packed.values)?;
        if (0..packed.rows).any(|i| packed.values[i + i * packed.rows] == T::zero()) {
            return Err(Error("LU factorization has a zero pivot"));
        }
        Ok(Self { packed, swaps })
    }
    /// Factor a square matrix with partial pivoting during elimination.
    /// Unlike source preliminary pivoting, this handles a later zero pivot by
    /// exchanging rows of the updated matrix, including previously computed L.
    pub fn factor(mut matrix: Matrix<T>) -> Result<Self, Error> {
        let n = Self::square(&matrix)?;
        let mut swaps = Vec::new();
        swaps
            .try_reserve_exact(n)
            .map_err(|_| Error("LU pivot allocation failed"))?;
        for column in 0..n {
            let mut best = column;
            for row in column + 1..n {
                if matrix.values[row + column * n].into().abs()
                    > matrix.values[best + column * n].into().abs()
                {
                    best = row;
                }
            }
            swaps.push(best);
            matrix.swap_rows(column, best);
            let diagonal = matrix.values[column + column * n];
            if diagonal == T::zero() {
                return Err(Error("LU factorization has a zero pivot"));
            }
            for row in column + 1..n {
                matrix.values[row + column * n] = matrix.values[row + column * n] / diagonal;
            }
            for next in column + 1..n {
                for row in column + 1..n {
                    let index = row + next * n;
                    matrix.values[index] = matrix.values[index]
                        - matrix.values[row + column * n] * matrix.values[column + next * n];
                }
            }
            finite(&matrix.values)?;
        }
        Self::finish(matrix, swaps)
    }
    /// Factor already-pivoted input with C's exact row/column/dot-product order.
    /// No automatic row exchange occurs. Singular arithmetic returns an error.
    pub fn factor_unpivoted(mut matrix: Matrix<T>) -> Result<Self, Error> {
        let n = Self::square(&matrix)?;
        for row in 0..n {
            for column in 0..n {
                let mut sum = T::zero();
                for k in 0..row.min(column) {
                    sum = sum + matrix.values[k + column * n] * matrix.values[row + k * n];
                }
                let index = row + column * n;
                matrix.values[index] = matrix.values[index] - sum;
                if column < row {
                    let diagonal = matrix.values[column + column * n];
                    if diagonal == T::zero() {
                        return Err(Error("LU factorization has a zero pivot"));
                    }
                    matrix.values[index] = matrix.values[index] / diagonal;
                }
            }
        }
        Self::finish(matrix, (0..n).collect())
    }
    pub fn packed(&self) -> &Matrix<T> {
        &self.packed
    }
    pub fn row_swaps(&self) -> &[usize] {
        &self.swaps
    }

    /// Solve A*x=b, applying the factorization's ordered swaps to b before
    /// source-compatible forward and backward substitution. Inputs are retained.
    pub fn solve(&self, right: &[T]) -> Result<Vec<T>, Error> {
        let n = self.packed.rows;
        if right.len() != n {
            return Err(Error("LU right-hand side has the wrong length"));
        }
        finite(right)?;
        let mut solution = right.to_vec();
        apply_swaps(&mut solution, &self.swaps)?;
        let packed = &self.packed.values;
        for column in 0..n {
            for row in column + 1..n {
                solution[row] = solution[row] - packed[row + column * n] * solution[column];
            }
        }
        for column in (0..n).rev() {
            solution[column] = solution[column] / packed[column + column * n];
            for row in 0..column {
                solution[row] = solution[row] - packed[row + column * n] * solution[column];
            }
        }
        finite(&solution)?;
        Ok(solution)
    }
}
