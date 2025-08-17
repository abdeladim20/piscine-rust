use std::ops::{Add, Sub};

mod scalar;
pub use scalar::*;

mod matrix;
pub use matrix::*;

impl<T: Scalar + Copy + Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result_data = Vec::new();

        for i in 0..self.0.len() {
            if self.0[i].len() != other.0[i].len() {
                return None;
            }

            let mut new_row = Vec::new();
            for j in 0..self.0[i].len() {
                new_row.push(self.0[i][j] + other.0[i][j]);
            }
            result_data.push(new_row);
        }

        Some(Matrix(result_data))
    }
}

impl<T: Scalar + Copy + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result_data = Vec::new();

        for i in 0..self.0.len() {
            if self.0[i].len() != other.0[i].len() {
                return None;
            }

            let mut new_row = Vec::new();
            for j in 0..self.0[i].len() {
                new_row.push(self.0[i][j] - other.0[i][j]);
            }
            result_data.push(new_row);
        }

        Some(Matrix(result_data))
    }
}