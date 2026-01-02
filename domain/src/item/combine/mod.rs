mod standard;

pub use standard::StandardItemCombiner;

use crate::enchantment::{EnchantmentKindId, EnchantmentLevel};
use crate::item::Item;
use crate::item::enchant::EnchantError;

pub trait CombineItems {
    fn combine(&self, target: &mut Item, sacrifice: Item) -> Option<Vec<EnchantResult>>;
}

pub type EnchantResult = Result<EnchantSuccess, EnchantError>;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct EnchantSuccess {
    pub kind: EnchantmentKindId,
    pub old_level: Option<EnchantmentLevel>,
    pub new_level: EnchantmentLevel,
}
