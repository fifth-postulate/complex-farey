pub trait Gcd {
    fn gcd(self, rhs: Self) -> Self;
}

macro_rules! impl_for {
    ($a_type:ty) => {
        impl Gcd for $a_type {
            fn gcd(self, rhs: Self) -> Self {
                if rhs == 0 {
                    return self;
                } else {
                    return rhs.gcd(self % rhs);
                }
            }
        }
    };
}

impl_for!(u8);
impl_for!(u16);
impl_for!(u32);
impl_for!(u64);
impl_for!(u128);
impl_for!(i8);
impl_for!(i16);
impl_for!(i32);
impl_for!(i64);
impl_for!(i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_of_51_and_37_is_1_for_u8() {
        assert_eq!(37u8.gcd(51), 1);
    }

    #[test]
    fn gcd_of_51_and_37_is_1_for_u16() {
        assert_eq!(37u16.gcd(51), 1);
    }

    #[test]
    fn gcd_of_51_and_37_is_1_for_u32() {
        assert_eq!(37u32.gcd(51), 1);
    }

    #[test]
    fn gcd_of_51_and_37_is_1_for_u64() {
        assert_eq!(37u64.gcd(51), 1);
    }

    #[test]
    fn gcd_of_51_and_37_is_1_for_u128() {
        assert_eq!(37u128.gcd(51), 1);
    }

    #[test]
    fn gcd_of_51_and_37_is_1_for_i8() {
        assert_eq!(37i8.gcd(51), 1);
    }

    #[test]
    fn gcd_of_51_and_37_is_1_for_i16() {
        assert_eq!(37i16.gcd(51), 1);
    }

    #[test]
    fn gcd_of_51_and_37_is_1_for_i32() {
        assert_eq!(37i32.gcd(51), 1);
    }

    #[test]
    fn gcd_of_51_and_37_is_1_for_i64() {
        assert_eq!(37i64.gcd(51), 1);
    }

    #[test]
    fn gcd_of_51_and_37_is_1_for_i128() {
        assert_eq!(37i128.gcd(51), 1);
    }
}
