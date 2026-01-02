use crate::item::Item;
use crate::item::{ItemKindId, combine::CombineItems};

use super::EnchantResult;

pub struct CompatibleItemsItemCombiner<Combine, Book>
where
    Combine: CombineItems,
    Book: Fn(&ItemKindId) -> bool,
{
    combiner: Combine,
    is_book: Book,
}

impl<Combine, Book> CompatibleItemsItemCombiner<Combine, Book>
where
    Combine: CombineItems,
    Book: Fn(&ItemKindId) -> bool,
{
    pub fn new(combiner: Combine, is_book: Book) -> Self {
        Self { combiner, is_book }
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

impl<Combine, Book> CombineItems for CompatibleItemsItemCombiner<Combine, Book>
where
    Combine: CombineItems,
    Book: Fn(&ItemKindId) -> bool,
{
    fn combine(&self, target: &mut Item, sacrifice: Item) -> Option<Vec<EnchantResult>> {
        if !self.are_kinds_compatible(&target, &sacrifice) {
            return None;
        }

        self.combiner.combine(target, sacrifice)
    }
}

#[cfg(test)]
mod tests {
    use crate::enchantment::Enchantment;
    use crate::enchantment::combine::BasicEnchantmentCombiner;
    use crate::item;
    use crate::item::combine::BasicItemCombiner;
    use crate::item::enchant::BasicEnchanter;

    use super::*;

    #[test]
    fn test_incompatible_items() {
        let mut target = item!("item_kind_1");
        let expected_target = target.clone();

        let sacrifice = item!("item_kind_2", Enchantment::new("im_an_enchantment", 1));

        let combiner = combiner(false);
        let result = combiner.combine(&mut target, sacrifice);
        let expected = None;

        assert_eq!(result, expected);
        assert_eq!(target, expected_target);
    }

    #[test]
    fn test_same_items() {
        let item_kind = ItemKindId::new("item_kind");

        let mut target = item!(item_kind.clone());
        let mut expected_target = target.clone();

        let sacrifice = item!(item_kind, Enchantment::new("im_an_enchantment", 1));

        let combiner = combiner(false);
        let result = combiner.combine(&mut target, sacrifice.clone());

        let implementation = implementation();
        let expected = implementation.combine(&mut expected_target, sacrifice);

        assert_eq!(result, expected);
        assert_eq!(target, expected_target);
    }

    fn combiner(is_book: bool) -> impl CombineItems {
        CompatibleItemsItemCombiner::new(implementation(), move |_| is_book)
    }

    fn implementation() -> impl CombineItems {
        let enchanter = BasicEnchanter::new(BasicEnchantmentCombiner);
        BasicItemCombiner::new(enchanter)
    }
}
