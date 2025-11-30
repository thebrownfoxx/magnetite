use std::cmp::max;
use std::ops::{Add, AddAssign, Mul};

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub struct EnchantmentLevel(u8);

impl EnchantmentLevel {
    pub fn new(value: impl Into<u8>) -> Self {
        Self(value.into())
    }

    pub fn combine(self, other: EnchantmentLevel) -> EnchantmentLevel {
        if self == other {
            return self + 1;
        };

        max(self, other)
    }
}

impl From<u8> for EnchantmentLevel {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<EnchantmentLevel> for u8 {
    fn from(value: EnchantmentLevel) -> Self {
        value.0
    }
}

impl<T: Into<EnchantmentLevel>> Add<T> for EnchantmentLevel {
    type Output = EnchantmentLevel;

    fn add(self, rhs: T) -> Self::Output {
        Self::new(self.0 + rhs.into().0)
    }
}

impl<T: Into<EnchantmentLevel>> AddAssign<T> for EnchantmentLevel {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs.into().0;
    }
}

impl<T: Into<EnchantmentLevel>> Mul<T> for EnchantmentLevel {
    type Output = EnchantmentLevel;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.0 * rhs.into().0)
    }
}
