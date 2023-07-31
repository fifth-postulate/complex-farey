use num::Zero;
use std::convert::From;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Eq)]
pub struct Gaussian<T> {
    real: T,
    complex: T,
}

impl<T> Gaussian<T> {
    pub fn new(real: T, complex: T) -> Self {
        Self { real, complex }
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
}
