use crate::item::combine::{CombineItems, CombineItemsError, CombinedItem, FailedEnchant};
use crate::item::enchant::{Enchant, EnchantError};
use crate::item::{Item, ItemKindId};

#[derive(Debug)]
pub struct StandardItemCombiner<Ench, Book>
where
    Ench: Enchant,
    Book: Fn(&ItemKindId) -> bool,
{
    enchanter: Ench,
    is_book: Book,
}

impl<Ench, Book> StandardItemCombiner<Ench, Book>
where
    Ench: Enchant,
    Book: Fn(&ItemKindId) -> bool,
{
    pub fn new(enchanter: Ench, is_book: Book) -> Self {
        Self { enchanter, is_book }
    }

    fn are_kinds_compatible(
        &self,
        target: impl AsRef<ItemKindId>,
        sacrifice: impl AsRef<ItemKindId>,
    ) -> bool {
        let target = target.as_ref();
        let sacrifice = sacrifice.as_ref();
        target == sacrifice || (self.is_book)(sacrifice)
    }
}

impl<Ench, Book> CombineItems for StandardItemCombiner<Ench, Book>
where
    Ench: Enchant,
    Book: Fn(&ItemKindId) -> bool,
{
    fn combine(
        &self,
        mut target: Item,
        mut sacrifice: Item,
    ) -> Result<CombinedItem, CombineItemsError> {
        if self.are_kinds_compatible(&target, &sacrifice) {
            return Err(CombineItemsError::IncompatibleItemKinds);
        };

        let sacrifice_enchantment_count = sacrifice.enchantment_count();
        let mut failed_enchants = Vec::<FailedEnchant>::new();
        for sacrifice_enchantment in sacrifice.drain_enchantments() {
            match self.enchanter.enchant(target, sacrifice_enchantment) {
                Ok(item) => {
                    target = item;
                }
                Err(EnchantError { item, enchantment, kind: error_kind }) => {
                    target = item;
                    let failed_enchant = FailedEnchant { enchantment, error_kind };
                    failed_enchants.push(failed_enchant);
                }
            }
        }

        if failed_enchants.len() == sacrifice_enchantment_count {
            return Err(CombineItemsError::NoCompatibleEnchantments);
        }

        return Ok(CombinedItem { item: target, failed_enchants });
    }
}
