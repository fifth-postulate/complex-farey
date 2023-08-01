use crate::gcd::Gcd;
use num::Zero;
use std::convert::From;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Gaussian<T> {
    real: T,
    complex: T,
}

impl<T> Gaussian<T> {
    pub fn new(real: T, complex: T) -> Self {
        Self { real, complex }
    }
}

impl<T> Gaussian<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    pub fn norm(&self) -> T {
        self.real * self.real + self.complex * self.complex
    }
}

impl<T> Gaussian<T>
where
    T: Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Neg<Output = T>
        + Copy,
{
    fn quo_rem(self, rhs: Self) -> (Self, Self) {
        let n = rhs.norm();
        let a = self.real * rhs.real + self.complex * rhs.complex;
        let b = self.complex * rhs.real - self.real * rhs.complex;

        let q = Self::new(a / n, b / n);
        let r = Self::new(a % n, b % n);

        (q, r)
    }
}

impl<T> Gcd for Gaussian<T>
where
    T: Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Neg<Output = T>
        + Zero
        + Copy,
{
    fn gcd(self, rhs: Self) -> Self {
        if rhs.is_zero() {
            self
        } else {
            rhs.gcd(self % rhs)
        }
    }
}
impl<T> Zero for Gaussian<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.real.is_zero() && self.complex.is_zero()
    }
}

impl<T> Add<Self> for Gaussian<T>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.real + rhs.real, self.complex + rhs.complex)
    }
}

impl<T> Mul<Self> for Gaussian<T>
where
    T: Mul<T, Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.real * rhs.real - self.complex * rhs.complex,
            self.real * rhs.complex + self.complex * rhs.real,
        )
    }
}

impl<T> Neg for Gaussian<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.real, -self.complex)
    }
}

impl<T> Sub for Gaussian<T>
where
    T: Neg<Output = T> + Add<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.neg())
    }
}

impl<T> Div for Gaussian<T>
where
    T: Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Neg<Output = T>
        + Copy,
{
    type Output = Gaussian<T>;

    fn div(self, rhs: Self) -> Self::Output {
        let (q, _) = self.quo_rem(rhs);

        q
    }
}

impl<T> Rem for Gaussian<T>
where
    T: Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Neg<Output = T>
        + Copy,
{
    type Output = Gaussian<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        let (_, r) = self.quo_rem(rhs);

        r
    }
}

impl<T> Display for Gaussian<T>
where
    T: Display + Zero,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.complex.is_zero() {
            f.write_fmt(format_args!("{}", self.real))
        } else if self.real.is_zero() {
            f.write_fmt(format_args!("{}i", self.complex))
        } else {
            f.write_fmt(format_args!("{}+{}i", self.real, self.complex))
        }
    }
}

macro_rules! impl_from {
    ($a_type:ty) => {
        impl From<$a_type> for Gaussian<$a_type> {
            fn from(t: $a_type) -> Self {
                Gaussian::new(t, 0)
            }
        }
    };
}

macro_rules! impl_from_tuple {
    ($a_type:ty) => {
        impl From<($a_type, $a_type)> for Gaussian<$a_type> {
            fn from(t: ($a_type, $a_type)) -> Self {
                Gaussian::new(t.0, t.1)
            }
        }
    };
}

impl_from_tuple!(i8);
impl_from_tuple!(i16);
impl_from_tuple!(i32);
impl_from_tuple!(i64);
impl_from_tuple!(i128);
impl_from_tuple!(u8);
impl_from_tuple!(u16);
impl_from_tuple!(u32);
impl_from_tuple!(u64);
impl_from_tuple!(u128);

impl_from!(i8);
impl_from!(i16);
impl_from!(i32);
impl_from!(i64);
impl_from!(i128);
impl_from!(u8);
impl_from!(u16);
impl_from!(u32);
impl_from!(u64);
impl_from!(u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gaussian_integers_can_be_added() {
        let expected: Gaussian<i32> = (1, 1).into();
        let left: Gaussian<i32> = (1, 0).into();
        let right: Gaussian<i32> = (0, 1).into();

        let actual = left + right;

        assert_eq!(actual, expected);
    }

    #[test]
    fn gaussian_integers_can_be_multiplied() {
        let expected: Gaussian<i32> = (-5, 10).into();
        let left: Gaussian<i32> = (1, 2).into();
        let right: Gaussian<i32> = (3, 4).into();

        let actual = left * right;

        assert_eq!(actual, expected);
    }

    #[test]
    fn gaussian_integers_can_be_divided() {
        let expected: Gaussian<i64> = (2, 1).into();
        let left: Gaussian<i64> = 5i64.into();
        let right: Gaussian<i64> = (2, -1).into();

        let actual = left / right;

        assert_eq!(actual, expected);
    }

    #[test]
    fn gaussian_integers_have_gcd() {
        let expected: Gaussian<i64> = (2, 1).into();
        let left: Gaussian<i64> = 5i64.into();
        let right: Gaussian<i64> = (2, 1).into();

        let actual = left.gcd(right);

        assert_eq!(actual, expected);
    }
}
