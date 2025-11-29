mod standard;

pub use standard::StandardItemCombiner;

use crate::enchantment::Enchantment;
use crate::item::{Item, enchant::EnchantErrorKind};

pub trait CombineItems {
    fn combine(&self, target: Item, sacrifice: Item) -> Result<CombinedItem, CombineItemsError>;
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct CombinedItem {
    pub item: Item,
    pub failed_enchants: Vec<FailedEnchant>,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct FailedEnchant {
    pub enchantment: Enchantment,
    pub error_kind: EnchantErrorKind,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub enum CombineItemsError {
    IncompatibleItemKinds,
    NoCompatibleEnchantments,
}
