mod bedrock;
mod java;

pub use bedrock::BedrockEnchantmentCombiner;
pub use java::JavaEnchantmentCombiner;

use crate::enchantment::{EnchantmentKindId, EnchantmentLevel};

pub trait CombineEnchantments {
    fn combine(
        &self,
        kind: &EnchantmentKindId,
        target_level: EnchantmentLevel,
        sacrifice_level: EnchantmentLevel,
    ) -> Option<EnchantmentLevel>;
}
