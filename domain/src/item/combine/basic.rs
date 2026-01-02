use crate::item::Item;
use crate::item::enchant::Enchant;

use super::{CombineItems, EnchantResult, EnchantSuccess};

#[derive(Debug)]
pub struct BasicItemCombiner<Ench: Enchant> {
    enchanter: Ench,
}

impl<Ench: Enchant> BasicItemCombiner<Ench> {
    pub fn new(enchanter: Ench) -> Self {
        Self { enchanter }
    }
}

impl<Ench: Enchant> CombineItems for BasicItemCombiner<Ench> {
    fn combine(&self, target: &mut Item, sacrifice: Item) -> Option<Vec<EnchantResult>> {
        let mut sacrifice = sacrifice;
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

        return Some(enchant_results);
    }
}

#[cfg(test)]
mod tests {
    use crate::enchantment::combine::BasicEnchantmentCombiner;
    use crate::enchantment::{Enchantment, EnchantmentKindId};
    use crate::item;
    use crate::item::enchant::BasicEnchanter;

    use super::*;

    macro_rules! new_item {
        () => {
            item!("im_an_enchantment")
        };
        ($($enchantment:expr),+ $(,)?) => {
            item!("im_an_enchantment", $($enchantment,)+)
        };
    }

    #[test]
    fn test_combine() {
        let enchantment_kind_1 = EnchantmentKindId::new("enchantment_1");
        let enchantment_kind_2 = EnchantmentKindId::new("enchantment_2");
        let enchantment_kind_3 = EnchantmentKindId::new("enchantment_3");

        let mut target = new_item!(
            Enchantment::new(enchantment_kind_1.clone(), 1),
            Enchantment::new(enchantment_kind_2.clone(), 2),
        );

        let sacrifice = new_item!(
            Enchantment::new(enchantment_kind_2.clone(), 2),
            Enchantment::new(enchantment_kind_3.clone(), 3),
        );

        let combiner = combiner();
        let mut result = combiner.combine(&mut target, sacrifice.clone());

        let mut expected = Some(vec![
            Ok(EnchantSuccess::upgraded(enchantment_kind_2.clone(), 2, 3)),
            Ok(EnchantSuccess::new(enchantment_kind_3.clone(), 3)),
        ]);

        if let Some(result_content) = result.as_mut()
            && let Some(expected_content) = expected.as_mut()
        {
            result_content.sort();
            expected_content.sort();
        }

        let expected_target = new_item!(
            Enchantment::new(enchantment_kind_1, 1),
            Enchantment::new(enchantment_kind_2, 3),
            Enchantment::new(enchantment_kind_3, 3),
        );

        assert_eq!(result, expected);
        assert_eq!(target, expected_target);
    }

    fn combiner() -> impl CombineItems {
        let enchanter = BasicEnchanter::new(BasicEnchantmentCombiner);
        BasicItemCombiner::new(enchanter)
    }
}
