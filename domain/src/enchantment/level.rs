use std::cmp::max;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub struct EnchantmentLevel(u8);

impl EnchantmentLevel {
    pub fn new(value: impl Into<u8>) -> Self {
        Self(value.into())
    }

    pub fn combine(self, other: EnchantmentLevel) -> EnchantmentLevel {
        if self == other {
            return Self::new(self.0 + 1);
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
