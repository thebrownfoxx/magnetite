use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::enchant::{Enchant, EnchantErrorKind};
use crate::item::{Item, ItemKindId};

use super::EnchantError;

pub struct CompatibleItemEnchanter<Ench, Compat>
where
    Ench: Enchant,
    Compat: Fn(&ItemKindId, &EnchantmentKindId) -> bool,
{
    enchanter: Ench,
    are_compatible: Compat,
}

impl<Ench, Compat> CompatibleItemEnchanter<Ench, Compat>
where
    Ench: Enchant,
    Compat: Fn(&ItemKindId, &EnchantmentKindId) -> bool,
{
    pub fn new(enchanter: Ench, are_compatible: Compat) -> Self {
        Self { enchanter, are_compatible }
    }
}

impl<Ench, Compat> Enchant for CompatibleItemEnchanter<Ench, Compat>
where
    Ench: Enchant,
    Compat: Fn(&ItemKindId, &EnchantmentKindId) -> bool,
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<(), EnchantError> {
        if !(self.are_compatible)(&item.kind, &enchantment.kind) {
            let error_kind = EnchantErrorKind::IncompatibleItemKind;
            return Err(EnchantError { enchantment, kind: error_kind });
        }

        self.enchanter.enchant(item, enchantment)
    }
}
