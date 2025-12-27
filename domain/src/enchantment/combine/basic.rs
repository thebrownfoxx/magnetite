use crate::enchantment::{EnchantmentKindId, EnchantmentLevel, combine::CombineEnchantments};

#[derive(Debug)]
pub struct BasicEnchantmentCombiner;

impl CombineEnchantments for BasicEnchantmentCombiner {
    fn combine(
        &self,
        _: impl AsRef<EnchantmentKindId>,
        target_level: EnchantmentLevel,
        sacrifice_level: EnchantmentLevel,
    ) -> Option<EnchantmentLevel> {
        Some(target_level.combine(sacrifice_level))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine() {
        let kind = EnchantmentKindId::new("im_an_enchantment");
        let target = EnchantmentLevel::new(1);
        let sacrifice = EnchantmentLevel::new(1);

        let result = BasicEnchantmentCombiner.combine(&kind, target, sacrifice);
        let expected = Some(target.combine(sacrifice));
        assert_eq!(result, expected);
    }
}
