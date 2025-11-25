use crate::enchantment::{Enchantment, EnchantmentKindId, EnchantmentLevel};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct EnchantmentReference<'a> {
    kind: &'a EnchantmentKindId,
    level: EnchantmentLevel,
}

impl<'a> EnchantmentReference<'a> {
    pub fn new(kind: &'a EnchantmentKindId, level: EnchantmentLevel) -> Self {
        Self { kind, level }
    }

    pub fn of(enchantment: &'a Enchantment) -> Self {
        Self {
            kind: enchantment.kind(),
            level: enchantment.level(),
        }
    }

    pub fn kind(&self) -> &EnchantmentKindId {
        self.kind
    }

    pub fn level(&self) -> EnchantmentLevel {
        self.level
    }

    pub fn into_enchantment(self) -> Enchantment {
        Enchantment {
            kind: self.kind().clone(),
            level: self.level(),
        }
    }
}
