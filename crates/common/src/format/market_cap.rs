// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::format::{format_big_decimal, FormatPretty};
use crate::model::MarketCapUsd;
use bigdecimal::BigDecimal;

impl FormatPretty for MarketCapUsd {
    fn pretty(self) -> String {
        if self.0 < BigDecimal::from(1) {
            return "$0".to_string();
        }
        format!("${}", format_big_decimal(self.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::format::FormatPretty;
    use crate::model::MarketCapUsd;
    use bigdecimal::{BigDecimal, FromPrimitive};

    #[test]
    fn test_zero() {
        assert_eq!(MarketCapUsd::from(0).pretty(), "$0");
    }

    #[test]
    fn test_tiny() {
        assert_eq!(
            MarketCapUsd(BigDecimal::from_f32(0.001).unwrap()).pretty(),
            "$0"
        );
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
