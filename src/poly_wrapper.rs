use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;

use num_bigint::BigUint;
use num_traits::Zero;
use num_traits::One;

use polynomen::Poly;

#[derive(Clone, Debug, PartialEq)]
pub struct PolyWrapper<T>(Poly<T>);


#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct BigUintWrapper(BigUint);
// trait Dotable = Scalar + Zero + ClosedAdd + ClosedMul;
//
impl<T: Add<Output = T> + polynomen::Zero + Clone + PartialEq> Add for PolyWrapper<T> {
    type Output = PolyWrapper<T>;

    fn add(self, PolyWrapper(rhs_inner): PolyWrapper<T>) -> PolyWrapper<T> {
        let inner: Poly<T> = self.0 + rhs_inner;
        PolyWrapper(inner)
    }
}

impl<T> AddAssign<PolyWrapper<T>> for PolyWrapper<T>
where
    T: Add<Output = T> + Clone + PartialEq + polynomen::Zero,
{
    fn add_assign(&mut self, rhs: PolyWrapper<T>) {
        let b = &rhs.0 + &self.0;
        *self = PolyWrapper(b)
    }
}

impl<T> Mul for PolyWrapper<T>
where
    T: Mul<Output = T> + Clone + PartialEq + polynomen::Zero + Add<Output = T>,
{
    type Output = PolyWrapper<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        PolyWrapper(self.0 * rhs.0)
    }
}

impl<T> MulAssign for PolyWrapper<T>
where
    T: Mul<Output = T> + Clone + PartialEq + polynomen::Zero + Add<Output = T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        let new_inner = &self.0 * &rhs.0;
        *self = PolyWrapper(new_inner)
    }
}

impl<T: polynomen::Zero + num_traits::Zero + Add<Output = T> + Clone + PartialEq> num_traits::Zero
    for PolyWrapper<T>
{
    fn zero() -> Self {
        PolyWrapper(<Poly<T> as polynomen::Zero>::zero())
    }

    fn is_zero(&self) -> bool {
        <Poly<T> as polynomen::Zero>::is_zero(&self.0)
    }
}

impl<T: polynomen::One + polynomen::Zero + num_traits::One + Add<Output = T> + Clone + PartialEq> num_traits::One
    for PolyWrapper<T>
{
    fn one() -> Self {
        PolyWrapper(<Poly<T> as polynomen::One>::one())
    }
}

// impl<T> Copy for PolyWrapper<T> {

// }

pub trait PolyToWrapped {
    type Inner;
    fn wrap(self) -> PolyWrapper<Self::Inner>;

    
}

pub trait WrappedToPoly<T> {
    fn unwrap(self) -> Poly<T>;
}

impl<T> PolyToWrapped for Poly<T> {
    type Inner = T;
    fn wrap(self) -> PolyWrapper<Self::Inner> {
        PolyWrapper(self)
    }
}

impl<T> WrappedToPoly<T> for PolyWrapper<T> {
    fn unwrap(self) -> Poly<T> {
        self.0
    }
}

pub trait BigUintToWrapped {
    fn wrap(self) -> BigUintWrapper;
}

impl BigUintToWrapped for BigUint {
    fn wrap(self) -> BigUintWrapper {
        BigUintWrapper(self)
    }
}

impl Add for BigUintWrapper {
    type Output = BigUintWrapper;

    fn add(self, rhs: Self) -> Self::Output {
        BigUintWrapper(self.0 + rhs.0)
    }
}

impl Mul for BigUintWrapper {
    type Output = BigUintWrapper;

    fn mul(self, rhs: Self) -> Self::Output {
        BigUintWrapper(self.0 * rhs.0)
    }
}

impl polynomen::Zero for BigUintWrapper {

    fn zero() -> Self {
        BigUintWrapper(BigUint::from(0 as usize))
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl num_traits::Zero for BigUintWrapper {

    fn zero() -> Self {
        BigUintWrapper(BigUint::from(0 as usize))
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl num_traits::One for BigUintWrapper {

    fn one() -> Self {
        BigUintWrapper(BigUint::from(1 as usize))
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl polynomen::One for BigUintWrapper {

    fn one() -> Self {
        BigUintWrapper(BigUint::from(1 as usize))
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

#[cfg(test)]
mod tests {

    use nalgebra::SMatrix;
    use polynomen::poly;
    use crate::poly_wrapper::{PolyToWrapped, WrappedToPoly, PolyWrapper};
    use ndarray::Array2;

    #[test]
    fn test_ndarray_addition() {
        
    }

    #[test]
    fn test_wrapped_addition() {
        let poly_1 = poly!(1, 3, 5).wrap();
        let poly_2 = poly!(1, 0, 1).wrap();
        assert_eq!(poly!(1, 3, 5) + poly!(1, 0, 1), (poly_1 + poly_2).unwrap());
    }
    
    #[test]
    fn test_wrapped_multiplication() {
        let poly_1 = poly!(1, 3, 5).wrap();
        let poly_2 = poly!(1, 0, 1).wrap();
        assert_eq!(poly!(1, 3, 5) * poly!(1, 0, 1), (poly_1 * poly_2).unwrap());
    }
    
    #[test]
    fn test_wrapped_matrix_multiplication() {
        let poly_11 = poly!(5, 3, 1).wrap();
        let poly_12 = poly!(1, 0, 1).wrap();
        let poly_21 = poly!(1, 2).wrap();
        let poly_22 = poly!(1).wrap();
        let mat = SMatrix::<PolyWrapper<i32>, 2, 2>::new(poly_11, poly_12, poly_21, poly_22);
        let mat_2 = mat.clone();
        let res = mat * &mat_2;

        let cpoly_11 = poly!(26, 32, 20, 8, 1).wrap();
        let cpoly_12 = poly!(6, 3, 7, 3, 1).wrap();
        let cpoly_21 = poly!(6, 15, 7, 2).wrap();
        let cpoly_22 = poly!(2, 2, 1, 2).wrap();
        let correct_matrix = SMatrix::<PolyWrapper<i32>, 2, 2>::new(cpoly_11, cpoly_12, cpoly_21, cpoly_22);
        assert_eq!(correct_matrix, res);
    }
}
