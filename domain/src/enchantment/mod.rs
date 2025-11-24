mod cost_multiplier;
mod kind;
mod level;

pub use cost_multiplier::CostMultiplier;
pub use kind::{EnchantmentKind, EnchantmentKindId};
pub use level::EnchantmentLevel;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Enchantment {
    pub kind: EnchantmentKindId,
    pub level: EnchantmentLevel,
}

impl Enchantment {
    pub fn new(kind: impl Into<EnchantmentKindId>, level: impl Into<EnchantmentLevel>) -> Self {
        Self {
            kind: kind.into(),
            level: level.into(),
        }
    }
}

impl AsRef<EnchantmentKindId> for Enchantment {
    fn as_ref(&self) -> &EnchantmentKindId {
        &self.kind
    }
}

impl From<Enchantment> for EnchantmentKindId {
    fn from(value: Enchantment) -> Self {
        value.kind
    }
}
