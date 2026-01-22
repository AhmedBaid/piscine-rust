use std::ops::{Add, Mul};

pub trait Scalar: Copy + Add<Output = Self> + Mul<Output = Self> {
    fn zero() -> Self;
}
impl Scalar for i32 {
    fn zero() -> Self {
        0
    }
}
#[derive(Debug, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let result: Vec<T> = self
            .0
            .into_iter()
            .zip(rhs.0.into_iter())
            .map(|(a, b)| a + b)
            .collect();

        Some(Vector(result))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn dot(self, rhs: Self) -> Option<T> {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let mut sum = T::zero();

        for (a, b) in self.0.into_iter().zip(rhs.0.into_iter()) {
            sum = sum + (a * b);
        }

        Some(sum)
    }
}
