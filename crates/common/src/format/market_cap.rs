// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::format::{FormatPretty, BILLION, HOUNDRED, MILLION, TEN, THOUSAND};
use crate::model::MarketCapUsd;
use bigdecimal::BigDecimal;

impl FormatPretty for MarketCapUsd {
    fn pretty(self) -> String {
        let value = self.0;
        let abs_value = value.abs();

        let (value, suffix): (BigDecimal, &str) = if abs_value >= *BILLION {
            (value / 1_000_000_000.0, "B")
        } else if abs_value >= *MILLION {
            (value / 1_000_000.0, "M")
        } else if abs_value >= *THOUSAND {
            (value / 1_000.0, "k")
        } else {
            return format!("${value}");
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

        format!("${}{}", trimmed, suffix)
    }
}

#[cfg(test)]
mod tests {
    use crate::format::FormatPretty;
    use crate::model::MarketCapUsd;

    #[test]
    fn test_zero() {
        assert_eq!(MarketCapUsd::from(0).pretty(), "$0");
    }

    #[test]
    fn test_one() {
        assert_eq!(MarketCapUsd::from(1).pretty(), "$1");
    }

    #[test]
    fn test_999() {
        assert_eq!(MarketCapUsd::from(999).pretty(), "$999");
    }

    #[test]
    fn test_1050() {
        assert_eq!(MarketCapUsd::from(1050).pretty(), "$1.05k");
    }

    #[test]
    fn test_10_500() {
        assert_eq!(MarketCapUsd::from(10_500).pretty(), "$10.5k");
    }

    #[test]
    fn test_999_999() {
        assert_eq!(MarketCapUsd::from(999_999).pretty(), "$999k");
    }

    #[test]
    fn test_1_000_000() {
        assert_eq!(MarketCapUsd::from(1_000_000).pretty(), "$1M");
    }

    #[test]
    fn test_1_234_567() {
        assert_eq!(MarketCapUsd::from(1_234_567).pretty(), "$1.23M");
    }

    #[test]
    fn test_10_000_000() {
        assert_eq!(MarketCapUsd::from(10_000_000).pretty(), "$10M");
    }

    #[test]
    fn test_999_999_999() {
        assert_eq!(MarketCapUsd::from(999_999_999).pretty(), "$999M");
    }

    #[test]
    fn test_1_000_000_000() {
        assert_eq!(MarketCapUsd::from(1_000_000_000).pretty(), "$1B");
    }

    #[test]
    fn test_2_345_678_901() {
        assert_eq!(MarketCapUsd::from(2_345_678_901).pretty(), "$2.34B");
    }
}
