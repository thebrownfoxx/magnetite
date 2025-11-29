mod compatible_enchantments;
mod compatible_item;
mod standard;

pub use compatible_enchantments::CompatibleEnchantmentsEnchanter;
pub use compatible_item::CompatibleItemEnchanter;
pub use standard::StandardEnchanter;

use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::Item;

pub trait Enchant {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<(), EnchantError>;
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct EnchantError {
    pub enchantment: Enchantment,
    pub kind: EnchantErrorKind,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum EnchantErrorKind {
    IncompatibleItemKind,
    IncompatibleEnchantment(EnchantmentKindId),
}
