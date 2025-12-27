#![cfg(test)]

use crate::enchantment::combine::CombineEnchantments;
use crate::enchantment::{EnchantmentKindId, EnchantmentLevel};

#[derive(Debug)]
pub struct AlwaysFailEnchantmentCombiner;

impl CombineEnchantments for AlwaysFailEnchantmentCombiner {
    fn combine(
        &self,
        _: impl AsRef<EnchantmentKindId>,
        _: impl Into<EnchantmentLevel>,
        _: impl Into<EnchantmentLevel>,
    ) -> Option<EnchantmentLevel> {
        None
    }
}

mod tests {
    use crate::enchantment::EnchantmentKindId;
    use crate::enchantment::combine::CombineEnchantments;

    use super::*;

    #[test]
    fn test_combine() {
        let combiner = AlwaysFailEnchantmentCombiner;
        let kind = EnchantmentKindId::new("im_an_enchantment");

        let result = combiner.combine(kind, 1, 1);
        let expected = None;
        assert_eq!(result, expected);
    }
}
