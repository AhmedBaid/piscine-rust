use lalgebra_scalar::*;
use std::ops::{Add, Mul};
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
impl<T: Copy> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut res = Vec::new();
        for row in &self.0 {
            res.push(row[n]);
        }
        res
    }
}

impl<T: Scalar<Item = T> + Copy + Mul<Output = T> + Add<Output = T>> Mul for Matrix<T> {
    type Output = Option<Vec<Vec<T>>>;
    fn mul(self, matrix2: Self) -> Self::Output {
        let mut res = vec![vec![T::zero(); matrix2.number_of_cols()]; self.number_of_rows()];
        for i in 0..self.number_of_rows() {
            for j in 0..matrix2.number_of_cols() {
                for k in 0..self.number_of_cols() {
                    res[i][j] = res[i][j] + (self.0[i][k] * matrix2.0[k][j]);
                }
            }
        }

        Some(res)
    }
}
