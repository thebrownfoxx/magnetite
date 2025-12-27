use crate::enchantment::combine::{
    BasicEnchantmentCombiner, CapMaxLevelEnchantmentCombiner, CombineEnchantments,
};
use crate::enchantment::{EnchantmentKindId, EnchantmentLevel};

pub fn java_enchantment_combiner(
    max_level: impl Fn(&EnchantmentKindId) -> EnchantmentLevel,
) -> impl CombineEnchantments {
    let implementation = BasicEnchantmentCombiner;
    CapMaxLevelEnchantmentCombiner::new(implementation, max_level)
}
