pub mod combine;
pub mod enchant;

mod kind;

pub use kind::{ItemKind, ItemKindId};

use crate::enchantment::{Enchantment, EnchantmentKindId, EnchantmentLevel, EnchantmentReference};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Item {
    kind: ItemKindId,
    enchantments: HashMap<EnchantmentKindId, EnchantmentLevel>,
    anvil_use_count: u8,
}

impl Item {
    pub fn new(kind: impl Into<ItemKindId>) -> Self {
        Self { kind: kind.into(), enchantments: HashMap::new(), anvil_use_count: 0 }
    }

    pub fn enchantment_kinds(&self) -> impl Iterator<Item = &EnchantmentKindId> {
        self.enchantments.keys()
    }

    pub fn enchantments<'a>(&'a self) -> impl Iterator<Item = EnchantmentReference<'a>> {
        self.enchantments
            .iter()
            .map(|(kind, level)| EnchantmentReference::new(kind, *level))
    }

    pub fn enchantment_count(&self) -> usize {
        self.enchantments.len()
    }

    pub fn add_enchantment(&mut self, enchantment: Enchantment) -> Option<EnchantmentLevel> {
        let Enchantment { kind, level } = enchantment;
        self.enchantments.insert(kind, level)
    }

    pub fn remove_enchantment(&mut self, kind: &EnchantmentKindId) -> Option<Enchantment> {
        self.enchantments
            .remove_entry(kind)
            .map(|(kind, level)| Enchantment::new(kind, level))
    }

    pub fn drain_enchantments(&mut self) -> impl Iterator<Item = Enchantment> {
        self.enchantments
            .drain()
            .into_iter()
            .map(|(kind, level)| Enchantment::new(kind, level))
    }
}

impl AsRef<ItemKindId> for Item {
    fn as_ref(&self) -> &ItemKindId {
        &self.kind
    }
}
