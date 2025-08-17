use crate::scalar::*;

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Self(Vec::new())
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut result = Matrix::new();
        for _i in 0..row {
            let mut v = Vec::new();
            for _j in 0..col {
                v.push(T::zero());
            }
            result.0.push(v);
        }
        result
        
    }

	pub fn identity(n: usize) -> Matrix<T> {
        let mut result = Matrix::new();
        for i in 0..n {
            let mut v = Vec::new();
            for j in 0..n {
                if i == j {
                    v.push(T::one());
                }else {
                    v.push(T::zero());
                }
            }
            result.0.push(v);
        }
        result
	}
}