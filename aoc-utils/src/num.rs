pub trait Integer: Sized + PartialOrd + Ord + Eq {
    fn gcd_binary(self, other: Self) -> Self;
    fn gcd(self, other: &Self) -> Self;
    fn lcm(self, other: &Self) -> Self;
}

/// Automatically generate gcd for integer
macro_rules! integer {
    ($($t:ty)*) => ($(
        impl Integer for $t {
            fn gcd_binary(mut self, mut other: Self) -> Self {
                if self == 0 {
                    return other;
                }
                if other == 0 {
                    return self;
                }

                let shift = (self | other).trailing_zeros();
                self >>= shift;
                other >>= shift;
                self >>= self.trailing_zeros();

                loop {
                    other >>= other.trailing_zeros();

                    #[allow(clippy::manual_swap)]
                    if self > other {
                        let temp = self;
                        self = other;
                        other = temp;
                    }

                    other -= self;

                    if other == 0 {
                        break;
                    }
                }

                self << shift
            }

            fn gcd(self, other: &Self) -> Self {
                self.gcd_binary(*other)
            }

            fn lcm(self, other: &Self) -> Self {
                if self == 0 && *other == 0 {
                    return 0
                }
                self * (*other / self.gcd(other))
            }
        }
    )*)
}

integer!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64);

#[cfg(test)]
mod test {
    use crate::num::Integer;

    #[test]
    fn gcd_test() {
        assert_eq!(0, 0u8.gcd(&0));
        assert_eq!(10, 10u8.gcd(&0));
        assert_eq!(10, 0u8.gcd(&10));
        assert_eq!(10, 10u8.gcd(&20));
        assert_eq!(44, 2024u32.gcd(&748));
    }

    #[test]
    fn lcm_test() {
        assert_eq!(7.lcm(&3), 21);
        assert_eq!(2.lcm(&4), 4);
        assert_eq!(0.lcm(&0), 0);
    }
}
