use crate::enchantment::{Enchantment, EnchantmentKindId, EnchantmentLevel};
use crate::item::enchant::{Enchant, EnchantErrorKind};
use crate::item::{Item, ItemKindId};

use super::EnchantError;

#[derive(Debug)]
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
    fn enchant(
        &self,
        item: &mut Item,
        enchantment: Enchantment,
    ) -> Result<EnchantmentLevel, EnchantError> {
        if !(self.are_compatible)(&item.kind, &enchantment.kind) {
            let error_kind = EnchantErrorKind::IncompatibleItemKind;
            return Err(EnchantError { enchantment, kind: error_kind });
        }

        self.enchanter.enchant(item, enchantment)
    }
}

#[cfg(test)]
mod tests {
    use crate::enchantment::combine::BasicEnchantmentCombiner;
    use crate::item;
    use crate::item::enchant::BasicEnchanter;

    use super::*;

    macro_rules! new_item {
        () => {
            item!("im_an_enchantment")
        };
        ($($enchantment:expr),+ $(,)?) => {
            item!("im_an_enchantment", $($enchantment)+)
        };
    }

    #[test]
    fn test_incompatible_item() {
        let mut item = new_item!();
        let expected_item = item.clone();

        let enchantment = Enchantment::new("enchantment", 1);

        let enchanter = enchanter(false);
        let result = enchanter.enchant(&mut item, enchantment.clone());

        let expected = Err(EnchantError {
            enchantment: enchantment.clone(),
            kind: EnchantErrorKind::IncompatibleItemKind,
        });

        assert_eq!(result, expected);
        assert_eq!(item, expected_item);
    }

    #[test]
    fn test_compatible_enchantment() {
        let mut item = new_item!();
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
        CompatibleItemEnchanter::new(enchanter, move |_, _| compatible)
    }

    fn implementation() -> impl Enchant {
        BasicEnchanter::new(BasicEnchantmentCombiner)
    }
}
