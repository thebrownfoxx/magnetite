use std::cmp::min;

use crate::enchantment::combine::CombineEnchantments;
use crate::enchantment::{EnchantmentKindId, EnchantmentLevel};

#[derive(Debug)]
pub struct BedrockEnchantmentCombiner<Max>
where
    Max: Fn(&EnchantmentKindId) -> Option<EnchantmentLevel>,
{
    max_level: Max,
}

impl<Max> BedrockEnchantmentCombiner<Max>
where
    Max: Fn(&EnchantmentKindId) -> Option<EnchantmentLevel>,
{
    pub fn new(max_level: Max) -> Self {
        Self { max_level }
    }
}

impl<Max> CombineEnchantments for BedrockEnchantmentCombiner<Max>
where
    Max: Fn(&EnchantmentKindId) -> Option<EnchantmentLevel>,
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

        let Some(max_level) = (self.max_level)(&kind) else {
            return None;
        };

        let level = min(level, max_level);
        Some(level)
    }
}
