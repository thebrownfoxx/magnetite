use crate::item::combine::{EnchantResult, EnchantSuccess};
use crate::item::enchant::{EnchantError, EnchantErrorKind};

pub fn java_base_enchant_cost(report: &EnchantResult) -> u8 {
    match report {
        Ok(EnchantSuccess { new_level, .. }) => new_level.value(),
        Err(EnchantError { kind, .. }) => java_base_enchant_error_cost(kind),
    }
}

fn java_base_enchant_error_cost(error_kind: &EnchantErrorKind) -> u8 {
    match error_kind {
        EnchantErrorKind::IncompatibleEnchantment(_) => 1,
        EnchantErrorKind::IncompatibleItemKind => 0,
    }
}
