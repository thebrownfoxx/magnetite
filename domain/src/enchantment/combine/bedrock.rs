use crate::enchantment::combine::CombineEnchantments;
use crate::enchantment::{EnchantmentKindId, EnchantmentLevel};

#[derive(Debug)]
pub struct BedrockEnchantmentCombiner<Max>
where
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    max_level: Max,
}

impl<Max> BedrockEnchantmentCombiner<Max>
where
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    pub fn new(max_level: Max) -> Self {
        Self { max_level }
    }
}

impl<Max> CombineEnchantments for BedrockEnchantmentCombiner<Max>
where
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    fn combine(
        &self,
        kind: &EnchantmentKindId,
        target_level: EnchantmentLevel,
        sacrifice_level: EnchantmentLevel,
    ) -> Option<EnchantmentLevel> {
        if sacrifice_level < target_level {
            return None;
        }

        let level = target_level.combine(sacrifice_level);
        let max_level = (self.max_level)(&kind);

        if level > max_level {
            return None;
        }

        Some(level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_lower() {
        let target = EnchantmentLevel::new(2);
        let sacrifice = EnchantmentLevel::new(target.value() - 1);

        let result = combine(target, sacrifice);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combine_higher() {
        let target = EnchantmentLevel::new(1);
        let sacrifice = EnchantmentLevel::new(target.value() + 1);

        let result = combine(target, sacrifice);
        let expected = Some(target.combine(sacrifice));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combine_equal() {
        let target = EnchantmentLevel::new(1);
        let sacrifice = target;

        let result = combine(target, sacrifice);
        let expected = Some(target.combine(sacrifice));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combine_overflowed_max() {
        let target = max_enchantment_level();
        let sacrifice = target;

        let result = combine(target, sacrifice);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combine_reached_max() {
        let target = EnchantmentLevel::new(max_enchantment_level().value() - 1);
        let sacrifice = target;

        let result = combine(target, sacrifice);
        let expected = Some(target.combine(sacrifice));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combine_one_away_from_max() {
        let target = EnchantmentLevel::new(max_enchantment_level().value() - 2);
        let sacrifice = target;

        let result = combine(target, sacrifice);
        let expected = Some(target.combine(sacrifice));
        assert_eq!(result, expected);
    }

    fn combine(target: EnchantmentLevel, sacrifice: EnchantmentLevel) -> Option<EnchantmentLevel> {
        let combiner = combiner();
        let kind = enchantment_kind();
        combiner.combine(&kind, target, sacrifice)
    }

    fn enchantment_kind() -> EnchantmentKindId {
        EnchantmentKindId::new("im_an_enchantment")
    }

    fn combiner() -> impl CombineEnchantments {
        BedrockEnchantmentCombiner::new(|_| max_enchantment_level())
    }

    fn max_enchantment_level() -> EnchantmentLevel {
        EnchantmentLevel::new(3)
    }
}
