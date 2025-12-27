use crate::enchantment::{EnchantmentKindId, EnchantmentLevel, combine::CombineEnchantments};

#[derive(Debug)]
pub struct RejectWeakerSacrificeEnchantmentCombiner<Impl: CombineEnchantments>(Impl);

impl<Impl: CombineEnchantments> RejectWeakerSacrificeEnchantmentCombiner<Impl> {
    pub fn new(implementation: Impl) -> Self {
        Self(implementation)
    }
}

impl<Impl: CombineEnchantments> CombineEnchantments
    for RejectWeakerSacrificeEnchantmentCombiner<Impl>
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

        self.0.combine(kind, target_level, sacrifice_level)
    }
}

#[cfg(test)]
mod tests {
    use crate::enchantment::combine::BasicEnchantmentCombiner;

    use super::*;

    #[test]
    fn test_combine_weaker_sacrifice() {
        let target = EnchantmentLevel::new(2);
        let sacrifice = EnchantmentLevel::new(1);

        let result = combine(combiner(), target, sacrifice);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combine_equal() {
        let target = EnchantmentLevel::new(1);
        let sacrifice = EnchantmentLevel::new(1);

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
        RejectWeakerSacrificeEnchantmentCombiner::new(implementation())
    }

    fn implementation() -> impl CombineEnchantments {
        BasicEnchantmentCombiner
    }
}
