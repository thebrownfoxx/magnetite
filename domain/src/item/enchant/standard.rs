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
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<(), EnchantError> {
        let Some(matching_enchantment) = item.remove_enchantment(&enchantment.kind) else {
            item.add_enchantment(enchantment);
            return Ok(());
        };

        let target_level = matching_enchantment.level;
        let sacrifice_level = enchantment.level;
        let combined_level =
            self.combiner
                .combine(&enchantment.kind, target_level, sacrifice_level);

        let Some(combined_level) = combined_level else {
            item.add_enchantment(matching_enchantment);

            let error_kind = EnchantErrorKind::IncompatibleEnchantment(enchantment.kind.clone());
            return Err(EnchantError { enchantment, kind: error_kind });
        };

        item.add_enchantment(Enchantment::new(enchantment.kind, combined_level));
        Ok(())
    }
}
