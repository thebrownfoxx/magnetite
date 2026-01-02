use crate::anvil::{Anvil, AnvilCombination, AnvilError, AnvilItem};
use crate::item::ItemKindId;
use crate::item::combine::{CombineItems, EnchantResult};
use std::cmp::max;

pub struct StandardAnvil<Combine, Mult, Cost>
where
    Combine: CombineItems,
    Mult: Fn(&ItemKindId) -> Option<u8>,
    Cost: Fn(&EnchantResult) -> u8,
{
    combiner: Combine,
    cost_multiplier: Mult,
    base_enchant_cost: Cost,
}

impl<Combine, Mult, Cost> StandardAnvil<Combine, Mult, Cost>
where
    Combine: CombineItems,
    Mult: Fn(&ItemKindId) -> Option<u8>,
    Cost: Fn(&EnchantResult) -> u8,
{
    pub fn new(combiner: Combine, cost_multiplier: Mult, base_enchant_cost: Cost) -> Self {
        Self { combiner, cost_multiplier, base_enchant_cost }
    }
}

impl<Combine, Mult, Cost> Anvil for StandardAnvil<Combine, Mult, Cost>
where
    Combine: CombineItems,
    Mult: Fn(&ItemKindId) -> Option<u8>,
    Cost: Fn(&EnchantResult) -> u8,
{
    fn combine(
        &self,
        target: &mut AnvilItem,
        sacrifice: AnvilItem,
    ) -> Result<AnvilCombination, AnvilError> {
        let cost_multiplier = (self.cost_multiplier)(sacrifice.item.kind())
            .ok_or(AnvilError::IncompatibleItemKinds)?;

        let mut cost =
            anvil_passes_cost(target.anvil_passes) + anvil_passes_cost(sacrifice.anvil_passes);

        let Some(enchant_results) = self.combiner.combine(&mut target.item, sacrifice.item) else {
            return Err(AnvilError::IncompatibleItemKinds);
        };

        for enchant_result in enchant_results.iter() {
            cost += (self.base_enchant_cost)(enchant_result) * cost_multiplier;
        }

        let has_compatible_enchantments = enchant_results
            .iter()
            .any(|enchant_result| enchant_result.is_ok());

        if !has_compatible_enchantments {
            return Err(AnvilError::NoCompatibleEnchantments);
        }

        target.anvil_passes = max(target.anvil_passes, sacrifice.anvil_passes) + 1;

        Ok(AnvilCombination { cost, enchant_results })
    }
}

fn anvil_passes_cost(anvil_passes: u8) -> u8 {
    2u8.pow(anvil_passes as u32) - 1
}
