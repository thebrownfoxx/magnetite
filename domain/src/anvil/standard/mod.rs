mod combiner;
mod enchanter;

pub use combiner::GenerateCombiner;
pub use enchanter::GenerateEnchanter;

use crate::anvil::{Anvil, AnvilCombination, AnvilError, AnvilItem};
use crate::item::ItemKindId;
use crate::item::combine::CombineItems;
use crate::item::enchant::EnchantReport;
use crate::item::enchant::ReportingEnchanter;
use std::cmp::max;

pub struct StandardAnvil<Enchanter, Combiner, Mult, Cost>
where
    Enchanter: GenerateEnchanter,
    Combiner: GenerateCombiner,
    Mult: Fn(&ItemKindId) -> Option<u8>,
    Cost: Fn(&EnchantReport) -> u8,
{
    enchanter: Enchanter,
    combiner: Combiner,
    cost_multiplier: Mult,
    base_enchant_cost: Cost,
}

impl<Enchanter, Combiner, Mult, Cost> StandardAnvil<Enchanter, Combiner, Mult, Cost>
where
    Enchanter: GenerateEnchanter,
    Combiner: GenerateCombiner,
    Mult: Fn(&ItemKindId) -> Option<u8>,
    Cost: Fn(&EnchantReport) -> u8,
{
    pub fn new(
        enchanter: Enchanter,
        combiner: Combiner,
        cost_multiplier: Mult,
        base_enchant_cost: Cost,
    ) -> Self {
        Self { enchanter, combiner, cost_multiplier, base_enchant_cost }
    }
}

impl<Enchanter, Combiner, Mult, Cost> Anvil for StandardAnvil<Enchanter, Combiner, Mult, Cost>
where
    Enchanter: GenerateEnchanter,
    Combiner: GenerateCombiner,
    Mult: Fn(&ItemKindId) -> Option<u8>,
    Cost: Fn(&EnchantReport) -> u8,
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

        let enchant_errors = {
            let reporting_enchanter =
                ReportingEnchanter::new(self.enchanter.generate(), |report| {
                    cost += (self.base_enchant_cost)(&report) * cost_multiplier
                });

            let combiner = self.combiner.generate(reporting_enchanter);
            combiner.combine(&mut target.item, sacrifice.item)?
        };

        target.anvil_passes = max(target.anvil_passes, sacrifice.anvil_passes) + 1;

        Ok(AnvilCombination { cost, enchant_errors })
    }
}

fn anvil_passes_cost(anvil_passes: u8) -> u8 {
    2u8.pow(anvil_passes as u32) - 1
}
