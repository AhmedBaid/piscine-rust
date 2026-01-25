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
pub trait Scalar: Sized {
    type Item;

    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = u32;

    fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}

impl Scalar for u64 {
    type Item = u64;

    fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i32 {
    type Item = i32;

    fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i64 {
    type Item = i64;

    fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}

impl Scalar for f32 {
    type Item = f32;

    fn zero() -> Self::Item {
        0.0
    }

    fn one() -> Self::Item {
        1.0
    }
}

impl Scalar for f64 {
    type Item = f64;

    fn zero() -> Self::Item {
        0.0
    }

    fn one() -> Self::Item {
        1.0
    }
}
