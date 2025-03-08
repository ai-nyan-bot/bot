// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::PriceUsd;
use bigdecimal::{BigDecimal, ToPrimitive};

pub fn format_price_usd<T: Into<PriceUsd>>(num: T) -> String {
    let num = num.into().0;
    let mut suffix = "";

    let billion = BigDecimal::from(1_000_000_000u64);
    let million = BigDecimal::from(1_000_000u64);
    let thousand = BigDecimal::from(1_000u64);

    let formatted = if num >= billion {
        suffix = "B";
        (num / &billion).to_f64().map(|x| format!("{:.2}", x))
    } else if num >= million {
        suffix = "M";
        (num / &million).to_f64().map(|x| format!("{:.2}", x))
    } else if num >= thousand.clone() {
        suffix = "K";
        (num / &thousand).to_f64().map(|x| format!("{:.2}", x))
    } else {
        num.to_f64().map(|x| format!("{:.1}", x))
    };

    let formatted = formatted.unwrap_or_else(|| "0".to_string());

    let cleaned = if formatted.ends_with(".0") {
        formatted[..formatted.len() - 2].to_string()
    } else {
        formatted
    };

    let mut result = cleaned.chars().take(5).collect::<String>();
    result.push_str(suffix);

    format!("${result}")
}

#[cfg(test)]
mod tests {

    mod price_usd {
        use crate::format::format_price_usd;

        #[test]
        fn test_9946_50() {
            let result = format_price_usd("9946.50");
            assert_eq!(result, "$9.95K");
        }
    }
}
