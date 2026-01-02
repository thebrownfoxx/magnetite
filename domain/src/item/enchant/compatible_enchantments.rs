use crate::enchantment::{Enchantment, EnchantmentKindId, EnchantmentLevel};
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
    fn enchant(
        &self,
        item: &mut Item,
        enchantment: Enchantment,
    ) -> Result<EnchantmentLevel, EnchantError> {
        let incompatible_enchantment = item.enchantment_kinds().find(|existing_enchantment| {
            !(self.are_compatible)(&existing_enchantment, &enchantment.kind)
        });

        if let Some(incompatible_enchantment) = incompatible_enchantment {
            let incompatible_enchantment = incompatible_enchantment.clone();
            let error_kind = EnchantErrorKind::IncompatibleEnchantment(incompatible_enchantment);
            return Err(EnchantError { enchantment, kind: error_kind });
        };

        self.enchanter.enchant(item, enchantment)
    }
}

#[cfg(test)]
mod tests {
    use crate::enchantment::combine::BasicEnchantmentCombiner;
    use crate::item::ItemKindId;
    use crate::item::enchant::BasicEnchanter;

    use super::*;

    #[test]
    fn test_incompatible_enchantment() {
        let mut item = new_item();
        let existing_enchantment = Enchantment::new("incompatible_enchantment", 1);
        item.add_enchantment(existing_enchantment.clone());

        let expected_item = item.clone();

        let enchantment = Enchantment::new("enchantment", 1);

        let enchanter = enchanter(false);
        let result = enchanter.enchant(&mut item, enchantment.clone());

        let expected = Err(EnchantError {
            enchantment: enchantment.clone(),
            kind: EnchantErrorKind::IncompatibleEnchantment(existing_enchantment.kind),
        });

        assert_eq!(result, expected);
        assert_eq!(item, expected_item);
    }

    #[test]
    fn test_compatible_enchantment() {
        let mut item = new_item();
        let existing_enchantment = Enchantment::new("compatible_enchantment", 1);
        item.add_enchantment(existing_enchantment.clone());

        let mut expected_item = item.clone();

        let enchantment = Enchantment::new("enchantment", 1);

        let enchanter = enchanter(true);
        let result = enchanter.enchant(&mut item, enchantment.clone());

        let implementation = implementation();
        let expected = implementation.enchant(&mut expected_item, enchantment);

        assert_eq!(result, expected);
        assert_eq!(item, expected_item);
    }

    fn enchanter(compatible: bool) -> impl Enchant {
        let enchanter = implementation();
        CompatibleEnchantmentsEnchanter::new(enchanter, move |_, _| compatible)
    }

    fn implementation() -> impl Enchant {
        BasicEnchanter::new(BasicEnchantmentCombiner)
    }

    fn new_item() -> Item {
        Item::new(ItemKindId::new("im_an_item"))
    }
}
