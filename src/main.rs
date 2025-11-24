use domain::enchantment::{Enchantment, EnchantmentKind};

fn main() {
    let enchantment_kind = EnchantmentKind::new("aqua_affinity", "Aqua Affinity", 1, 1);
    dbg!(enchantment_kind.clone());

    let enchantment = Enchantment::new(enchantment_kind.id, 1);
    dbg!(enchantment);
}
