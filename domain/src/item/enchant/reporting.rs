use std::cell::RefCell;

use crate::enchantment::{Enchantment, EnchantmentKindId, EnchantmentLevel};
use crate::item::Item;
use crate::item::enchant::{Enchant, EnchantError};

#[derive(Debug)]
pub struct ReportingEnchanter<Ench, Report>
where
    Ench: Enchant,
    Report: FnMut(EnchantReport),
{
    enchanter: Ench,
    report: RefCell<Report>, // TODO: Look for a way to avoid interior mutability
}

pub type EnchantReport = Result<EnchantSuccess, EnchantError>;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct EnchantSuccess {
    pub kind: EnchantmentKindId,
    pub old_level: Option<EnchantmentLevel>,
    pub new_level: EnchantmentLevel,
}

impl<Ench, Report> ReportingEnchanter<Ench, Report>
where
    Ench: Enchant,
    Report: FnMut(EnchantReport),
{
    pub fn new(enchanter: Ench, report: Report) -> Self {
        Self { enchanter, report: RefCell::new(report) }
    }
}

impl<Ench, Report> Enchant for ReportingEnchanter<Ench, Report>
where
    Ench: Enchant,
    Report: FnMut(EnchantReport),
{
    fn enchant(
        &self,
        item: &mut Item,
        enchantment: Enchantment,
    ) -> Result<EnchantmentLevel, EnchantError> {
        let kind = enchantment.kind.clone();
        let old_level = item.enchantment_level(&enchantment);

        let result = self.enchanter.enchant(item, enchantment);

        let map = |new_level| EnchantSuccess { kind, old_level, new_level };
        (self.report.borrow_mut())(result.clone().map(map));

        result
    }
}
