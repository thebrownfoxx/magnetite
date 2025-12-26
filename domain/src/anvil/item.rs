use crate::item::Item;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct AnvilItem {
    pub item: Item,
    pub anvil_passes: u8,
}
