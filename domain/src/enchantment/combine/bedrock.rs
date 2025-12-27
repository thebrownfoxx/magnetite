use crate::enchantment::combine::{
    BasicEnchantmentCombiner, CombineEnchantments, RejectLevelOverflowEnchantmentCombiner,
    RejectWeakerSacrificeEnchantmentCombiner,
};
use crate::enchantment::{EnchantmentKindId, EnchantmentLevel};

pub fn bedrock_enchantment_combiner(
    max_level: impl Fn(&EnchantmentKindId) -> EnchantmentLevel,
) -> impl CombineEnchantments {
    let combiner = BasicEnchantmentCombiner;
    let combiner = RejectLevelOverflowEnchantmentCombiner::new(combiner, max_level);
    RejectWeakerSacrificeEnchantmentCombiner::new(combiner)
}
