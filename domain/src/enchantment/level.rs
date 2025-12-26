use std::cmp::max;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub struct EnchantmentLevel(u8);

impl EnchantmentLevel {
    pub fn new(value: impl Into<u8>) -> Self {
        Self(value.into())
    }

    pub fn value(self) -> u8 {
        self.0
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
        value.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_with_lower() {
        let lhs = EnchantmentLevel::new(2);
        let rhs = EnchantmentLevel::new(lhs.value() - 1);

        let result = lhs.combine(rhs);
        let expected = lhs;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combine_with_higher() {
        let lhs = EnchantmentLevel::new(1);
        let rhs = EnchantmentLevel::new(lhs.value() + 1);

        let result = lhs.combine(rhs);
        let expected = rhs;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combine_with_equal() {
        let lhs = EnchantmentLevel::new(1);
        let rhs = lhs;

        let result = lhs.combine(rhs);
        let expected = EnchantmentLevel(lhs.value() + 1);
        assert_eq!(result, expected);
    }
}
