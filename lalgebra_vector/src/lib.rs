#[derive(Debug, Eq, PartialEq, Clone)]

pub struct Vector<T: Scalar>(pub Vec<T>);

pub trait Scalar: Sized + Copy + Default + Add<Self, Output = Self> + Mul<Self, Output = Self>{}

impl Scalar for i64 {}

use std::ops::{Add, Mul};

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = Vec::new();

        for i in 0..self.0.len() {
            result.push(self.0[i] + other.0[i])
        }

        Some(Vector(result))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut result = T::default();

        for i in 0..self.0.len() {
            result = result + (self.0[i] * other.0[i]);
        }
        Some(result)
    }
}
