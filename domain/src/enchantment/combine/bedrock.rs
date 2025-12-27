use crate::enchantment::combine::{
    BasicEnchantmentCombiner, CombineEnchantments, RejectLevelOverflowEnchantmentCombiner,
    RejectWeakerSacrificeEnchantmentCombiner,
};
use crate::enchantment::{EnchantmentKindId, EnchantmentLevel};

pub fn bedrock_enchantment_combiner(
    max_level: impl Fn(&EnchantmentKindId) -> EnchantmentLevel,
) -> impl CombineEnchantments {
    let implementation = BasicEnchantmentCombiner;
    let implementation = RejectLevelOverflowEnchantmentCombiner::new(implementation, max_level);
    RejectWeakerSacrificeEnchantmentCombiner::new(implementation)
}
