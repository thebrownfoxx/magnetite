mod standard;

pub use standard::StandardItemCombiner;

use crate::item::Item;
use crate::item::enchant::EnchantError;

pub trait CombineItems {
    fn combine(
        &self,
        target: &mut Item,
        sacrifice: Item,
    ) -> Result<Vec<EnchantError>, CombineItemsError>;
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub enum CombineItemsError {
    IncompatibleItemKinds,
    NoCompatibleEnchantments,
}
