#[macro_export]
macro_rules! item {
    ($kind:expr) => {
        Item::new($kind)
    };
    ($kind:expr, $( $enchantment:expr ),+ ) => {
        {
            let mut item = Item::new($kind);
            $(
                item.add_enchantment($enchantment);
            )*
            item
        }
    };
}
