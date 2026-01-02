mod basic;
mod compatible_enchantments;
mod compatible_item;

pub use basic::BasicEnchanter;
pub use compatible_enchantments::CompatibleEnchantmentsEnchanter;
pub use compatible_item::CompatibleItemEnchanter;

use crate::enchantment::{Enchantment, EnchantmentKindId, EnchantmentLevel};
use crate::item::Item;

pub trait Enchant {
    fn enchant(
        &self,
        item: &mut Item,
        enchantment: Enchantment,
    ) -> Result<EnchantmentLevel, EnchantError>;
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct EnchantError {
    pub enchantment: Enchantment,
    pub kind: EnchantErrorKind,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum EnchantErrorKind {
    IncompatibleItemKind,
    IncompatibleEnchantment(EnchantmentKindId),
}
