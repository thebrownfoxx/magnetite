use std::rc::Rc;

use crate::enchantment::{CostMultiplier, EnchantmentLevel};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct EnchantmentKindId(Rc<str>);

impl EnchantmentKindId {
    pub fn new(value: impl Into<Rc<str>>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<T: Into<Rc<str>>> From<T> for EnchantmentKindId {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl AsRef<str> for EnchantmentKindId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<EnchantmentKindId> for EnchantmentKindId {
    fn as_ref(&self) -> &EnchantmentKindId {
        self
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct EnchantmentKind {
    id: EnchantmentKindId,
    name: Rc<str>,
    max_level: EnchantmentLevel,
    cost_multiplier: CostMultiplier,
}

impl EnchantmentKind {
    pub fn new(
        id: impl Into<EnchantmentKindId>,
        name: impl Into<Rc<str>>,
        max_level: impl Into<EnchantmentLevel>,
        cost_multiplier: impl Into<CostMultiplier>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            max_level: max_level.into(),
            cost_multiplier: cost_multiplier.into(),
        }
    }

    pub fn id(&self) -> &EnchantmentKindId {
        &self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn max_level(&self) -> EnchantmentLevel {
        self.max_level
    }

    pub fn cost_multiplier(&self) -> CostMultiplier {
        self.cost_multiplier
    }
}

impl AsRef<EnchantmentKindId> for EnchantmentKind {
    fn as_ref(&self) -> &EnchantmentKindId {
        self.id()
    }
}
