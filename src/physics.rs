use std::fmt::Debug;

// This allows us to consolidate a lot of repetitve contraints needed below
// Consider how things with less structure like rings (hello matrix algebra!) could be specified
pub trait Field: Debug + Clone + Copy + num_traits::One + num_traits::Zero {}

// Replace float with scalar (could be complex or whatever else counts as a scalar
// Probably should be a trait?)
// The requirement to be a scalar is to be part of a field
// Must implement Add, Sub, Mul, and Div in a closed fashion
// Multiplication and addition must have identities and inverses
// Mul and Add must be commutative and associative
// Mul distributive over Add
#[derive(Debug, Clone)]
pub struct Scalar<T: Field>
{
    value: T,
}

// A vector is an element of a vector space over a field, which means
// it has vector addition and scalar multiplication
// Vector addition is associative, commutative, and has inverse and identity elements
#[derive(Debug, Clone)]
pub struct Vector<const SIZE: usize, T: Field>
{
    raw_vector: [T; SIZE], // What about functions as vectors?
}

// Scalar multiplication
impl<const SIZE: usize, T: Field> std::ops::Mul<Vector<SIZE, T>> for Scalar<T>
where
    T: std::ops::Mul<Output = T>,
{
    type Output = Vector<SIZE, T>;
    fn mul(self, vector: Vector<SIZE, T>) -> Self::Output
    {
        let result = vector
            .raw_vector
            .to_vec()
            .iter()
            .map(|v| *v * self.value)
            .collect::<Vec<T>>();
        Vector {
            raw_vector: result
                .try_into()
                .unwrap_or_else(|v: Vec<T>| panic!("Could not convert {:?} to array", v)),
        }
    }
}

// Scalar multiplication again, but such that order doesn't matter
impl<const SIZE: usize, T: Field> std::ops::Mul<Scalar<T>> for Vector<SIZE, T>
where
    T: std::ops::Mul<Output = T>,
{
    type Output = Vector<SIZE, T>;
    fn mul(self, scalar: Scalar<T>) -> Self
    {
        let result = self
            .raw_vector
            .to_vec()
            .iter()
            .map(|v| *v * scalar.value)
            .collect::<Vec<T>>();
        Vector {
            raw_vector: result
                .try_into()
                .unwrap_or_else(|v: Vec<T>| panic!("Could not convert {:?} to array", v)),
        }
    }
}

// Basis vectors, orthogonalization (Gram-Schmidt)

// Vector fields

// Functions as vectors (composition of functions)
fn quadratic<T: Field>(x: Scalar<T>) -> T
where
    T: std::ops::Mul<Output = T>,
{
    x.value * x.value
}

fn cubic<T: Field>(x: Scalar<T>) -> T
where
    T: std::ops::Mul<Output = T>,
{
    x.value * x.value * x.value
}

pub trait InnerProductVector<T: Field, Rhs = Self>
where
    T: std::ops::Mul<Output = T>,
{
    fn dot(&self, rhs: &Rhs) -> T;
    fn norm(&self) -> T;
}

pub trait CrossableVector<Rhs = Self>
{
    fn cross(&self, rhs: &Rhs) -> Self;
}
