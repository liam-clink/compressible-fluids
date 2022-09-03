// Replace float with scalar (could be complex or whatever else counts as a scalar
// Probably should be a trait?)
// The requirement to be a scalar is to be part of a field
// Must implement Add, Sub, Mul, and Div in a closed fashion
// Multiplication and addition must have identities and inverses
// Mul and Add must be commutative and associative
// Mul distributive over Add
pub struct Scalar
{
    value: Float,
}

// A vector is an element of a vector space over a field, which means
// it has vector addition and scalar multiplication
// Vector addition is associative, commutative, and has inverse and identity elements
pub struct PhysicsVector
{
    raw_vector: Vec, // What about functions as vectors?
}

impl std::ops::Mul<PhysicsVector> for Scalar
{
    type Output = PhysicsVector;
    fn mul(self, vector: &PhysicsVector) -> Self::Output
    {
        PhysicsVector {
            raw_vector: vector
                .raw_vector
                .value
                .iter()
                .map(|v| v * self.value)
                .collect(),
        }
    }
}

impl std::ops::Mul<Scalar> for PhysicsVector
{
    fn mul(&self, scalar: Scalar) -> Self
    {
        Self {
            raw_vector: self.value.iter().map(|v| v * rhs.value).collect(),
        }
    }
}

// Basis vectors, orthogonalization (Gram-Schmidt)

// Vector fields

// Functions as vectors (composition of functions)
fn quadratic(x: Scalar)
{
    x.value * x.value
}
fn cubic(x: Float)
{
    x.value * x.value * x.value
}

pub trait InnerProductVector<Rhs = Self>
{
    fn dot(&self, rhs: &Rhs) -> Float;
    fn norm(&self) -> Float;
}

pub trait CrossableVector<Rhs = Self>
{
    fn cross(&self, rhs: &Rhs) -> Self;
}
