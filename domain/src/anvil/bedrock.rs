use crate::item::combine::{EnchantResult, EnchantSuccess};

pub fn bedrock_base_enchant_cost(report: &EnchantResult) -> u8 {
    let Ok(EnchantSuccess { old_level, new_level, .. }) = report else {
        return 0;
    };

    new_level.value() - old_level.map(|level| level.value()).unwrap_or(0)
}
