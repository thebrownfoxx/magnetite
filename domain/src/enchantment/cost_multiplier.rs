#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct CostMultiplier {
    pub for_book: u8,
    pub for_item: u8,
}

impl From<u8> for CostMultiplier {
    fn from(value: u8) -> Self {
        CostMultiplier {
            for_book: value,
            for_item: value,
        }
    }
}
