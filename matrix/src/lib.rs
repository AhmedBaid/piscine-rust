use lalgebra_scalar::Scalar;
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<const W: usize, const H: usize, T>(pub [[T; W]; H]);

impl<const W: usize, const H: usize, T> Matrix<W, H, T>
where
    T: Scalar<Item = T> + Copy,
{
    pub fn zero() -> Self {
        Matrix([[T::zero(); W]; H])
    }
}

impl<const S: usize, T> Matrix<S, S, T>
where
    T: Scalar<Item = T> + Copy,
{
    pub fn identity() -> Self {
        let mut res = [[T::zero(); S]; S];

        for i in 0..res.len() {
            res[i][i] = T::one();
        }
        Matrix(res)
    }
}
