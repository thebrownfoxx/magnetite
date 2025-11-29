use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::Item;
use crate::item::enchant::{Enchant, EnchantErrorKind};

use super::EnchantError;

#[derive(Debug)]
pub struct CompatibleEnchantmentsEnchanter<Ench, Compat>
where
    Ench: Enchant,
    Compat: Fn(&EnchantmentKindId, &EnchantmentKindId) -> bool,
{
    enchanter: Ench,
    are_compatible: Compat,
}

impl<Ench, Compat> CompatibleEnchantmentsEnchanter<Ench, Compat>
where
    Ench: Enchant,
    Compat: Fn(&EnchantmentKindId, &EnchantmentKindId) -> bool,
{
    pub fn new(enchanter: Ench, compatibility: Compat) -> Self {
        Self { enchanter, are_compatible: compatibility }
    }
}

impl<Ench, Compat> Enchant for CompatibleEnchantmentsEnchanter<Ench, Compat>
where
    Ench: Enchant,
    Compat: Fn(&EnchantmentKindId, &EnchantmentKindId) -> bool,
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<(), EnchantError> {
        let incompatible_enchantment = item.enchantment_kinds().find(|existing_enchantment| {
            (self.are_compatible)(&existing_enchantment, &enchantment.kind)
        });

        if let Some(incompatible_enchantment) = incompatible_enchantment {
            let incompatible_enchantment = incompatible_enchantment.clone();
            let error_kind = EnchantErrorKind::IncompatibleEnchantment(incompatible_enchantment);
            return Err(EnchantError { enchantment, kind: error_kind });
        };

        self.enchanter.enchant(item, enchantment)
    }
}
