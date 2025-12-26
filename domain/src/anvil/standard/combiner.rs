use crate::item::{combine::CombineItems, enchant::Enchant};

pub trait GenerateCombiner {
    fn generate(&self, enchanter: impl Enchant) -> impl CombineItems;
}
