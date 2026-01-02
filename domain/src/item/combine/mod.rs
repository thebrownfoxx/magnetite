mod basic;
mod compatible_items;

pub use basic::BasicItemCombiner;
pub use compatible_items::CompatibleItemsItemCombiner;

use crate::enchantment::{EnchantmentKindId, EnchantmentLevel};
use crate::item::Item;
use crate::item::enchant::EnchantError;

pub trait CombineItems {
    fn combine(&self, target: &mut Item, sacrifice: Item) -> Option<Vec<EnchantResult>>;
}

pub type EnchantResult = Result<EnchantSuccess, EnchantError>;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct EnchantSuccess {
    pub kind: EnchantmentKindId,
    pub old_level: Option<EnchantmentLevel>,
    pub new_level: EnchantmentLevel,
}

impl EnchantSuccess {
    pub fn new(kind: impl Into<EnchantmentKindId>, new_level: impl Into<EnchantmentLevel>) -> Self {
        Self { kind: kind.into(), old_level: None, new_level: new_level.into() }
    }

    pub fn upgraded(
        kind: impl Into<EnchantmentKindId>,
        old_level: impl Into<EnchantmentLevel>,
        new_level: impl Into<EnchantmentLevel>,
    ) -> Self {
        Self { kind: kind.into(), old_level: Some(old_level.into()), new_level: new_level.into() }
    }
}
