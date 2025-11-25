mod bedrock;
mod java;

pub use bedrock::BedrockEnchantmentCombiner;
pub use java::JavaEnchantmentCombiner;

use crate::enchantment::{Enchantment, EnchantmentKindId, EnchantmentLevel};

pub trait CombineEnchantments {
    fn combine(
        &self,
        kind: EnchantmentKindId,
        target_level: EnchantmentLevel,
        sacrifice_level: EnchantmentLevel,
    ) -> Result<Enchantment, CombineEnchantmentsError>;
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct CombineEnchantmentsError {
    pub kind: EnchantmentKindId,
    pub target_level: EnchantmentLevel,
    pub sacrifice_level: EnchantmentLevel,
}
