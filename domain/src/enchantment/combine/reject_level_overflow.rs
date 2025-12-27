use crate::enchantment::combine::CombineEnchantments;
use crate::enchantment::{EnchantmentKindId, EnchantmentLevel};

#[derive(Debug)]
pub struct RejectLevelOverflowEnchantmentCombiner<Impl, Max>
where
    Impl: CombineEnchantments,
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    implementation: Impl,
    max_level: Max,
}

impl<Impl, Max> RejectLevelOverflowEnchantmentCombiner<Impl, Max>
where
    Impl: CombineEnchantments,
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    pub fn new(implementation: Impl, max_level: Max) -> Self {
        Self { implementation, max_level }
    }
}

impl<Impl, Max> CombineEnchantments for RejectLevelOverflowEnchantmentCombiner<Impl, Max>
where
    Impl: CombineEnchantments,
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    fn combine(
        &self,
        kind: &EnchantmentKindId,
        target_level: EnchantmentLevel,
        sacrifice_level: EnchantmentLevel,
    ) -> Option<EnchantmentLevel> {
        let level = self
            .implementation
            .combine(kind, target_level, sacrifice_level)?;

        let max_level = (self.max_level)(&kind);

        if level > max_level {
            return None;
        }

        Some(level)
    }
}

#[cfg(test)]
mod tests {
    use crate::enchantment::combine::BasicEnchantmentCombiner;

    use super::*;

    #[test]
    fn test_combine_overflowed_max() {
        let target = max_enchantment_level();
        let sacrifice = target;

        let result = combine(combiner(), target, sacrifice);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combine_reached_max() {
        let target = EnchantmentLevel::new(max_enchantment_level().value() - 1);
        let sacrifice = target;

        let result = combine(combiner(), target, sacrifice);
        let expected = combine(implementation(), target, sacrifice);
        assert_eq!(result, expected);
    }

    fn combine(
        combiner: impl CombineEnchantments,
        target: EnchantmentLevel,
        sacrifice: EnchantmentLevel,
    ) -> Option<EnchantmentLevel> {
        let kind = EnchantmentKindId::new("im_an_enchantment");
        combiner.combine(&kind, target, sacrifice)
    }

    fn combiner() -> impl CombineEnchantments {
        RejectLevelOverflowEnchantmentCombiner::new(implementation(), |_| max_enchantment_level())
    }

    fn implementation() -> impl CombineEnchantments {
        BasicEnchantmentCombiner
    }

    fn max_enchantment_level() -> EnchantmentLevel {
        EnchantmentLevel::new(3)
    }
}
