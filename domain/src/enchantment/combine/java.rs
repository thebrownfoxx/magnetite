use std::cmp::min;

use crate::enchantment::combine::CombineEnchantments;
use crate::enchantment::{EnchantmentKindId, EnchantmentLevel};

#[derive(Debug)]
pub struct JavaEnchantmentCombiner<Max>
where
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    max_level: Max,
}

impl<Max> JavaEnchantmentCombiner<Max>
where
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    pub fn new(max_level: Max) -> Self {
        Self { max_level }
    }
}

impl<Max> CombineEnchantments for JavaEnchantmentCombiner<Max>
where
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    fn combine(
        &self,
        kind: &EnchantmentKindId,
        target_level: EnchantmentLevel,
        sacrifice_level: EnchantmentLevel,
    ) -> Option<EnchantmentLevel> {
        let level = target_level.combine(sacrifice_level);
        let max_level = (self.max_level)(&kind);
        Some(min(level, max_level))
    }
}
