use crate::enchantment::{EnchantmentKindId, EnchantmentLevel, combine::CombineEnchantments};

#[derive(Debug)]
pub struct RejectWeakerSacrificeEnchantmentCombiner<Combine: CombineEnchantments>(Combine);

impl<Combine: CombineEnchantments> RejectWeakerSacrificeEnchantmentCombiner<Combine> {
    pub fn new(combiner: Combine) -> Self {
        Self(combiner)
    }
}

impl<Combine: CombineEnchantments> CombineEnchantments
    for RejectWeakerSacrificeEnchantmentCombiner<Combine>
{
    fn combine(
        &self,
        kind: impl AsRef<EnchantmentKindId>,
        target_level: impl Into<EnchantmentLevel>,
        sacrifice_level: impl Into<EnchantmentLevel>,
    ) -> Option<EnchantmentLevel> {
        let target_level = target_level.into();
        let sacrifice_level = sacrifice_level.into();

        if sacrifice_level < target_level {
            return None;
        }

        self.0.combine(kind.as_ref(), target_level, sacrifice_level)
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
        let expected = combine(combiner(), target, sacrifice);
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
        RejectWeakerSacrificeEnchantmentCombiner::new(combiner())
    }

    fn combiner() -> impl CombineEnchantments {
        BasicEnchantmentCombiner
    }
}
