mod basic;
mod bedrock;
mod cap_max_level;
mod java;
mod reject_level_overflow;
mod reject_weaker_sacrifice;

pub use basic::BasicEnchantmentCombiner;
pub use bedrock::bedrock_enchantment_combiner;
pub use cap_max_level::CapMaxLevelEnchantmentCombiner;
pub use java::java_enchantment_combiner;
pub use reject_level_overflow::RejectLevelOverflowEnchantmentCombiner;
pub use reject_weaker_sacrifice::RejectWeakerSacrificeEnchantmentCombiner;

use crate::enchantment::{EnchantmentKindId, EnchantmentLevel};

pub trait CombineEnchantments {
    fn combine(
        &self,
        kind: impl AsRef<EnchantmentKindId>,
        target_level: impl Into<EnchantmentLevel>,
        sacrifice_level: impl Into<EnchantmentLevel>,
    ) -> Option<EnchantmentLevel>;
}
