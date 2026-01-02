use crate::item::enchant::Enchant;
use crate::item::{Item, ItemKindId};

use super::{CombineItems, EnchantResult, EnchantSuccess};

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

    fn transfer_enchantments(&self, target: &mut Item, mut sacrifice: Item) -> Vec<EnchantResult> {
        let mut enchant_results = Vec::<EnchantResult>::new();

        for sacrifice_enchantment in sacrifice.drain_enchantments() {
            let kind = sacrifice_enchantment.kind.clone();
            let old_level = target.enchantment_level(&kind);

            let enchant_result = self
                .enchanter
                .enchant(target, sacrifice_enchantment)
                .map(|new_level| EnchantSuccess { kind, old_level, new_level });

            enchant_results.push(enchant_result);
        }

        enchant_results
    }
}

impl<Ench, Book> CombineItems for StandardItemCombiner<Ench, Book>
where
    Ench: Enchant,
    Book: Fn(&ItemKindId) -> bool,
{
    fn combine(&self, target: &mut Item, sacrifice: Item) -> Option<Vec<EnchantResult>> {
        if self.are_kinds_compatible(&target, &sacrifice) {
            return None;
        };

        if sacrifice.enchantment_count() == 0 {
            return Some(vec![]);
        }

        return Some(self.transfer_enchantments(target, sacrifice));
    }
}
