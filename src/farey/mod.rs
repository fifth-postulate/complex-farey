use std::convert::From;
use std::ops::Add;
use std::ops::Mul;

#[derive(Debug, PartialEq)]
pub struct Fraction<T> {
    numerator: T,
    denominator: T,
}

impl<T> Fraction<T> {
    pub fn new(numerator: T, denominator: T) -> Fraction<T> {
        Fraction {
            numerator,
            denominator,
        }
    }
}

impl<T> Fraction<T>
where
    T: Add<Output = T>,
{
    pub fn mediant(self, rhs: Self) -> Self {
        Fraction::new(
            self.numerator + rhs.numerator,
            self.denominator + rhs.denominator,
        )
    }
}

impl<T> Add for Fraction<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Fraction::new(
            (self.numerator * rhs.denominator) + (self.denominator * rhs.numerator),
            self.denominator * rhs.denominator,
        )
    }
}

impl<T> Mul for Fraction<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

macro_rules! impl_from_tuple {
    ($a_type:ty) => {
        impl From<($a_type, $a_type)> for Fraction<$a_type> {
            fn from(t: ($a_type, $a_type)) -> Self {
                Fraction::new(t.0, t.1)
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
    fn fraction_can_be_created_from_a_tuple_i32() {
        let expected = Fraction::new(1, 2);

        let actual: Fraction<i32> = (1, 2).into();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fraction_can_be_created_from_a_tuple_i64() {
        let expected: Fraction<i64> = Fraction::new(1, 2);

        let actual: Fraction<i64> = (1, 2).into();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fractions_can_be_added() {
        let expected: Fraction<u64> = (5, 6).into();
        let left: Fraction<u64> = (1, 2).into();
        let right: Fraction<u64> = (1, 3).into();

        let actual = left + right;

        assert_eq!(expected, actual);
    }

    #[test]
    fn fractions_can_be_multiplied() {
        let expected: Fraction<u8> = (10, 21).into();
        let left: Fraction<u8> = (2, 3).into();
        let right: Fraction<u8> = (5, 7).into();

        let actual = left * right;

        assert_eq!(expected, actual);
    }

    #[test]
    fn fractions_have_mediant() {
        let expected: Fraction<i8> = (2, 5).into();
        let left: Fraction<i8> = (1, 2).into();
        let right: Fraction<i8> = (1, 3).into();

        let actual = left.mediant(right);

        assert_eq!(expected, actual);
    }
}
