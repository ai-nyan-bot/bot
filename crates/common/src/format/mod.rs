// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use bigdecimal::BigDecimal;
use lazy_static::lazy_static;

pub use percent::format_percent;
pub use price::format_price_usd;

mod count;
mod market_cap;
mod percent;
mod price;
mod volume;

pub trait FormatPretty {
    fn pretty(self) -> String;
}

lazy_static! {
    pub(crate) static ref BILLION: BigDecimal = BigDecimal::from(1_000_000_000);
    pub(crate) static ref MILLION: BigDecimal = BigDecimal::from(1_000_000);
    pub(crate) static ref THOUSAND: BigDecimal = BigDecimal::from(1_000);
    pub(crate) static ref HOUNDRED: BigDecimal = BigDecimal::from(100);
    pub(crate) static ref TEN: BigDecimal = BigDecimal::from(10);
}

pub(crate) fn format_big_decimal(value: BigDecimal) -> String {
    let abs_value = value.abs();

    let (value, suffix): (BigDecimal, &str) = if abs_value >= *BILLION {
        (value / 1_000_000_000.0, "B")
    } else if abs_value >= *MILLION {
        (value / 1_000_000.0, "M")
    } else if abs_value >= *THOUSAND {
        (value / 1_000.0, "k")
    } else {
        return format!("{value}");
    };

    let mut formatted = value.to_string();

    if let Some(dot_index) = formatted.find('.') {
        let truncate_len = if value >= *HOUNDRED {
            dot_index
        } else if value >= *TEN {
            dot_index + 2
        } else {
            dot_index + 3
        };

        formatted.truncate(truncate_len);
    }

    let trimmed = formatted
        .trim_end_matches(".00")
        .trim_end_matches(".0")
        .trim_end_matches('.');

    format!("{}{}", trimmed, suffix)
}
