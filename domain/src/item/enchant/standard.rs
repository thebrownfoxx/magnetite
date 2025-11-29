use crate::enchantment::{Enchantment, combine::CombineEnchantments};
use crate::item::enchant::EnchantErrorKind;
use crate::item::{Item, enchant::Enchant};

use super::EnchantError;

#[derive(Debug)]
pub struct StandardEnchanter<Combine: CombineEnchantments> {
    combiner: Combine,
}

impl<Combine: CombineEnchantments> StandardEnchanter<Combine> {
    pub fn new(combiner: Combine) -> Self {
        Self { combiner }
    }
}

impl<Combine: CombineEnchantments> Enchant for StandardEnchanter<Combine> {
    fn enchant(&self, mut item: Item, enchantment: Enchantment) -> Result<Item, EnchantError> {
        let Some(matching_enchantment) = item.remove_enchantment(&enchantment.kind) else {
            item.add_enchantment(enchantment);
            return Ok(item);
        };

        let target_level = matching_enchantment.level;
        let sacrifice_level = enchantment.level;
        let combined_enchantment =
            self.combiner
                .combine(enchantment.kind, target_level, sacrifice_level);

        match combined_enchantment {
            Ok(combined_enchantment) => {
                item.add_enchantment(combined_enchantment);
                Ok(item)
            }
            Err(enchantment_kind) => {
                item.add_enchantment(matching_enchantment);

                let enchantment = Enchantment::new(enchantment_kind.clone(), sacrifice_level);
                let error_kind = EnchantErrorKind::IncompatibleEnchantment(enchantment_kind);
                Err(EnchantError { item, enchantment, kind: error_kind })
            }
        }
    }
}
