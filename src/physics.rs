use num_traits::{Float, Zero};
use std::fmt::Debug;

#[allow(unused_imports)]
use rayon::prelude::*;

// This allows us to consolidate a lot of repetitve contraints needed below
pub trait Field: Debug + Clone + Copy + Sync + Send + Float {}

// Replace float with scalar (could be complex or whatever else counts as a scalar
// Probably should be a trait?)
// The requirement to be a scalar is to be part of a field
// Must implement Add, Sub, Mul, and Div in a closed fashion
// Multiplication and addition must have identities and inverses
// Mul and Add must be commutative and associative
// Mul distributive over Add
#[derive(Debug, Clone)]
pub struct Scalar<T: Field> {
    value: T,
}

// A vector is an element of a vector space over a field, which means
// it has vector addition and scalar multiplication
// Vector addition is associative, commutative, and has inverse and identity elements
#[derive(Debug, Clone)]
pub struct Vector<const SIZE: usize, T: Field> {
    raw_vector: [T; SIZE], // What about functions as vectors?
}

impl<const SIZE: usize, T: Field> Vector<SIZE, T> {
    pub fn data(&self) -> [T; SIZE] {
        self.raw_vector.clone()
    }
}

// Needed to call into_par_iter on Vector<T, SIZE> directly, and raw_vector can be left private
impl<const SIZE: usize, T: Field> IntoParallelIterator for Vector<SIZE, T> {
    type Iter = rayon::array::IntoIter<T, SIZE>;
    type Item = T;

    fn into_par_iter(self) -> Self::Iter {
        self.raw_vector.into_par_iter()
    }
}

// Scalar multiplication
impl<const SIZE: usize, T: Field> std::ops::Mul<Vector<SIZE, T>> for Scalar<T> {
    type Output = Vector<SIZE, T>;
    fn mul(self, vector: Vector<SIZE, T>) -> Self::Output {
        let result = vector
            .into_par_iter()
            .map(|v| v * self.value)
            .collect::<Vec<T>>();
        Vector {
            raw_vector: result
                .try_into()
                .unwrap_or_else(|v: Vec<T>| panic!("Could not convert {:?} to array", v)),
        }
    }
}

// Scalar multiplication again, but reverse argument order
impl<const SIZE: usize, T: Field> std::ops::Mul<Scalar<T>> for Vector<SIZE, T> {
    type Output = Vector<SIZE, T>;
    fn mul(self, scalar: Scalar<T>) -> Self {
        let result = self
            .into_par_iter()
            .map(|v| v * scalar.value)
            .collect::<Vec<T>>();
        Vector {
            raw_vector: result
                .try_into()
                .unwrap_or_else(|v: Vec<T>| panic!("Could not convert {:?} to array", v)),
        }
    }
}

// Vector addition
impl<const SIZE: usize, T: Field> std::ops::Add for Vector<SIZE, T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let result = self
            .into_par_iter()
            .zip(other.into_par_iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<T>>();
        Vector {
            raw_vector: result
                .try_into()
                .unwrap_or_else(|v: Vec<T>| panic!("Could not convert {:?} to array", v)),
        }
    }
}

impl<const SIZE: usize, T: Field + std::ops::AddAssign> InnerProductSpace for Vector<SIZE, T> {
    type TField = T;

    fn dot(&self, other: &Self) -> T {
        let mut result: T = T::zero();
        for value in self
            .clone()
            .into_par_iter()
            .zip(other.clone().into_par_iter())
            .map(|(a, b)| a * b)
            .collect::<Vec<T>>()
        {
            result += value;
        }
        result
    }
}

// Basis vectors, orthogonalization (Gram-Schmidt)

// Vector fields

// Functions as vectors (composition of functions)
// fn quadratic<T: Field>(x: Scalar<T>) -> T {
//     x.value * x.value
// }
//
// fn cubic<T: Field>(x: Scalar<T>) -> T {
//     x.value * x.value * x.value
// }

/************************************************
           Traits for Vector Spaces
*************************************************/

pub trait InnerProductSpace {
    type TField: Field;

    fn dot(&self, other: &Self) -> Self::TField;

    fn norm2(&self) -> Self::TField {
        Self::dot(self, self)
    }

    fn linearly_independent(&self, other: &Self) -> bool {
        Self::dot(self, other) == Self::TField::zero()
    }
}

pub trait OuterProductVector<Rhs> {
    fn cross(&self, rhs: &Rhs) -> Self;
}

// Tests
#[cfg(test)]
mod tests {
    use num_traits::Float;

    #[test]
    fn test_scalar_multiplication() {
        let s = crate::physics::Scalar {
            value: (10.0 as dyn Float),
        };
        let v = crate::physics::Vector {
            raw_vector: [(1.0 as dyn Float); 10],
        };
        assert_eq!(v.data(), [10.0; 10]);
    }

    // #[test]
    // fn test_vector_addition() ({}
    //
    // #[test]
    // fn test_inner_product_space() {}
}
