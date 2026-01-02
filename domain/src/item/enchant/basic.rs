use crate::enchantment::combine::CombineEnchantments;
use crate::enchantment::{Enchantment, EnchantmentLevel};
use crate::item::Item;
use crate::item::enchant::{Enchant, EnchantErrorKind};

use super::EnchantError;

#[derive(Debug)]
pub struct BasicEnchanter<Combine: CombineEnchantments> {
    combiner: Combine,
}

impl<Combine: CombineEnchantments> BasicEnchanter<Combine> {
    pub fn new(combiner: Combine) -> Self {
        Self { combiner }
    }
}

impl<Combine: CombineEnchantments> Enchant for BasicEnchanter<Combine> {
    fn enchant(
        &self,
        item: &mut Item,
        enchantment: Enchantment,
    ) -> Result<EnchantmentLevel, EnchantError> {
        let Some(matching_enchantment) = item.remove_enchantment(&enchantment) else {
            let level = enchantment.level;
            item.add_enchantment(enchantment);
            return Ok(level);
        };

        let combined_level =
            self.combiner
                .combine(&enchantment, matching_enchantment.level, enchantment.level);

        let Some(combined_level) = combined_level else {
            item.add_enchantment(matching_enchantment);

            let error_kind = EnchantErrorKind::IncompatibleEnchantment(enchantment.kind.clone());
            return Err(EnchantError { enchantment, kind: error_kind });
        };

        item.add_enchantment(Enchantment::new(enchantment.kind, combined_level));
        Ok(combined_level)
    }
}

#[cfg(test)]
mod tests {
    use crate::enchantment::combine::{AlwaysFailEnchantmentCombiner, BasicEnchantmentCombiner};

    use super::*;

    #[test]
    fn test_enchant_no_matching() {
        let mut item = new_item();
        let enchantment = Enchantment::new("enchantment", 1);

        let enchanter = BasicEnchanter::new(BasicEnchantmentCombiner);
        let result = enchanter.enchant(&mut item, enchantment.clone());

        let mut expected_item = new_item();
        expected_item.add_enchantment(enchantment.clone());

        assert_eq!(result, Ok(enchantment.level));
        assert_eq!(item, expected_item);
    }

    #[test]
    fn test_enchant_incompatible() {
        let mut item = new_item();
        let enchantment = Enchantment::new("enchantment", 1);
        item.add_enchantment(enchantment.clone());

        let expected_item = item.clone();

        let enchanter = BasicEnchanter::new(AlwaysFailEnchantmentCombiner);
        let result = enchanter.enchant(&mut item, enchantment.clone());

        let expected = Err(EnchantError {
            enchantment: enchantment.clone(),
            kind: EnchantErrorKind::IncompatibleEnchantment(enchantment.kind),
        });

        assert_eq!(result, expected);
        assert_eq!(item, expected_item);
    }

    #[test]
    fn test_enchant_combinable() {
        let mut item = new_item();
        let enchantment = Enchantment::new("enchantment", 1);
        item.add_enchantment(enchantment.clone());

        let combiner = BasicEnchantmentCombiner;
        let enchanter = BasicEnchanter::new(BasicEnchantmentCombiner);
        let result = enchanter.enchant(&mut item, enchantment.clone());

        let combined_level = combiner
            .combine(&enchantment, enchantment.level, enchantment.level)
            .unwrap();

        let mut expected_item = new_item();
        expected_item.add_enchantment(Enchantment::new(enchantment.kind.clone(), combined_level));

        let expected = Ok(combined_level);
        assert_eq!(result, expected);
        assert_eq!(item, expected_item);
    }

    fn new_item() -> Item {
        Item::new("im_an_item")
    }
}
