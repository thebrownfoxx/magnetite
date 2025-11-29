mod standard;

pub use standard::StandardEnchanter;

use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::Item;

pub trait Enchant {
    fn enchant(&self, item: Item, enchantment: Enchantment) -> Result<Item, EnchantError>;
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct EnchantError {
    pub item: Item,
    pub enchantment: Enchantment,
    pub kind: EnchantErrorKind,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum EnchantErrorKind {
    IncompatibleItemKind,
    IncompatibleEnchantment(EnchantmentKindId),
}
