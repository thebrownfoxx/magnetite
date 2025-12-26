use crate::item::enchant::Enchant;

pub trait GenerateEnchanter {
    fn generate(&self) -> impl Enchant;
}
