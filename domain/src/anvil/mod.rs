mod bedrock;
mod item;
mod java;
mod standard;

pub use bedrock::bedrock_base_enchant_cost;
pub use item::AnvilItem;
pub use java::java_base_enchant_cost;
pub use standard::StandardAnvil;

use crate::item::combine::EnchantResult;

pub trait Anvil {
    fn combine(
        &self,
        target: &mut AnvilItem,
        sacrifice: AnvilItem,
    ) -> Result<AnvilCombination, AnvilError>;
}

pub struct AnvilCombination {
    pub cost: u8,
    pub enchant_results: Vec<EnchantResult>,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub enum AnvilError {
    IncompatibleItemKinds,
    NoCompatibleEnchantments,
    TooExpensive { cost: u8, max_cost: u8 },
}
