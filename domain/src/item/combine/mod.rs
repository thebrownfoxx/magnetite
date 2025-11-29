mod standard;

pub use standard::StandardItemCombiner;

use crate::item::Item;
use crate::item::enchant::EnchantError;

pub trait CombineItems {
    fn combine(&self, target: Item, sacrifice: Item) -> Result<CombinedItem, CombineItemsError>;
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct CombinedItem {
    pub item: Item,
    pub failed_enchants: Vec<EnchantError>,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub enum CombineItemsError {
    IncompatibleItemKinds,
    NoCompatibleEnchantments,
}
