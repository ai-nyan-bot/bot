// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::format::{format_big_decimal, FormatPretty, ONE, ZERO};
use crate::model::PriceUsd;
use bigdecimal::{BigDecimal, Signed};

impl FormatPretty for PriceUsd {
    fn pretty(self) -> String {
        let abs_value = self.0.abs();

        if abs_value < *ONE && abs_value != *ZERO {
            format_less_than_one(self.0)
        } else {
            format!("${}", format_big_decimal(self.0)).replace("$-", "-$")
        }
    }
}

fn format_less_than_one(value: BigDecimal) -> String {
    let mut exponent = -1;
    let mut abs_value = value.abs();
    while abs_value < *ONE {
        abs_value *= 10;
        exponent += 1;
    }

    let abs_str = abs_value.to_string().replace(".", "");
    let trimmed_value = abs_str.trim_end_matches('0');

    if value.is_negative() {
        format!("-$0.0({}){:.4}", exponent, trimmed_value)
    } else {
        format!("$0.0({}){:.4}", exponent, trimmed_value)
    }
}

#[cfg(test)]
mod tests {
    use crate::format::FormatPretty;
    use crate::model::PriceUsd;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    #[test]
    fn test_zero() {
        assert_eq!(PriceUsd::from(0).pretty(), "$0");
    }
    
    #[test]
    fn test_long_prices(){
        assert_eq!(PriceUsd::from("0.000000102944").pretty(), "$0.0(6)1029");
        assert_eq!(PriceUsd::from("0.000000000102944").pretty(), "$0.0(9)1029");
    }

    #[test]
    fn test_0_01230() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("0.01230").unwrap()).pretty(),
            "$0.0(1)123"
        );
    }

    #[test]
    fn test_0_012300() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("0.012300").unwrap()).pretty(),
            "$0.0(1)123"
        );
    }

    #[test]
    fn test_0_0123() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("0.0123").unwrap()).pretty(),
            "$0.0(1)123"
        );
    }

    #[test]
    fn test_negative_0_0123() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("-0.0123").unwrap()).pretty(),
            "-$0.0(1)123"
        );
    }

    #[test]
    fn test_0_00123() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("0.00123").unwrap()).pretty(),
            "$0.0(2)123"
        );
    }

    #[test]
    fn test_negative_0_00123() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("-0.00123").unwrap()).pretty(),
            "-$0.0(2)123"
        );
    }

    #[test]
    fn test_0_000123() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("0.000123").unwrap()).pretty(),
            "$0.0(3)123"
        );
    }

    #[test]
    fn test_negative_0_000123() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("-0.000123").unwrap()).pretty(),
            "-$0.0(3)123"
        );
    }

    #[test]
    fn test_0_0000123() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("0.0000123").unwrap()).pretty(),
            "$0.0(4)123"
        );
    }

    #[test]
    fn test_negative_0_0000123() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("-0.0000123").unwrap()).pretty(),
            "-$0.0(4)123"
        );
    }

    #[test]
    fn test_0_00000123() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("0.00000123").unwrap()).pretty(),
            "$0.0(5)123"
        );
    }

    #[test]
    fn test_negative_0_00000123() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("-0.00000123").unwrap()).pretty(),
            "-$0.0(5)123"
        );
    }

    #[test]
    fn test_0_000000123() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("0.000000123").unwrap()).pretty(),
            "$0.0(6)123"
        );
    }

    #[test]
    fn test_negative_0_000000123() {
        assert_eq!(
            PriceUsd(BigDecimal::from_str("-0.000000123").unwrap()).pretty(),
            "-$0.0(6)123"
        );
    }

    #[test]
    fn test_one() {
        assert_eq!(PriceUsd::from(1).pretty(), "$1");
    }

    #[test]
    fn test_999() {
        assert_eq!(PriceUsd::from(999).pretty(), "$999");
    }

    #[test]
    fn test_negative_999() {
        assert_eq!(PriceUsd::from(-999).pretty(), "-$999");
    }

    #[test]
    fn test_1050() {
        assert_eq!(PriceUsd::from(1050).pretty(), "$1.05k");
    }

    #[test]
    fn test_10_500() {
        assert_eq!(PriceUsd::from(10_500).pretty(), "$10.5k");
    }

    #[test]
    fn test_999_999() {
        assert_eq!(PriceUsd::from(999_999).pretty(), "$999k");
    }

    #[test]
    fn test_1_000_000() {
        assert_eq!(PriceUsd::from(1_000_000).pretty(), "$1M");
    }

    #[test]
    fn test_1_234_567() {
        assert_eq!(PriceUsd::from(1_234_567).pretty(), "$1.23M");
    }

    #[test]
    fn test_negative_1_234_567() {
        assert_eq!(PriceUsd::from(-1_234_567).pretty(), "-$1.23M");
    }

    #[test]
    fn test_10_000_000() {
        assert_eq!(PriceUsd::from(10_000_000).pretty(), "$10M");
    }

    #[test]
    fn test_999_999_999() {
        assert_eq!(PriceUsd::from(999_999_999).pretty(), "$999M");
    }

    #[test]
    fn test_1_000_000_000() {
        assert_eq!(PriceUsd::from(1_000_000_000).pretty(), "$1B");
    }

    #[test]
    fn test_2_345_678_901() {
        assert_eq!(PriceUsd::from(2_345_678_901).pretty(), "$2.34B");
    }

    #[test]
    fn test_negative_2_345_678_901() {
        assert_eq!(PriceUsd::from(-2_345_678_901).pretty(), "-$2.34B");
    }
}
