// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::format::{format_big_decimal, FormatPretty};
use crate::model::Percent;
use bigdecimal::{BigDecimal, FromPrimitive};

impl FormatPretty for Percent {
    fn pretty(self) -> String {
        format!(
            "{}%",
            format_big_decimal(BigDecimal::from_f32(self.0).unwrap())
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::format::FormatPretty;
    use crate::model::Percent;

    #[test]
    fn test_limit_length() {
        assert_eq!(Percent::from(23.24).pretty(), "23.24%");
        assert_eq!(Percent::from(23.245).pretty(), "23.25%");
        assert_eq!(Percent::from(10.2345).pretty(), "10.23%");
    }

    #[test]
    fn test_zero() {
        assert_eq!(Percent::from(0).pretty(), "0%");
    }

    #[test]
    fn test_one() {
        assert_eq!(Percent::from(1).pretty(), "1%");
    }

    #[test]
    fn test_small() {
        assert_eq!(Percent::from(0.011).pretty(), "0.01%");
    }

    #[test]
    fn test_tiny() {
        assert_eq!(Percent::from(0.0011).pretty(), "0%");
    }

    #[test]
    fn test_999() {
        assert_eq!(Percent::from(999).pretty(), "999%");
    }

    #[test]
    fn test_1050() {
        assert_eq!(Percent::from(1050).pretty(), "1.05k%");
    }

    #[test]
    fn test_10_500() {
        assert_eq!(Percent::from(10_500).pretty(), "10.5k%");
    }

    #[test]
    fn test_999_999() {
        assert_eq!(Percent::from(999_999).pretty(), "999k%");
    }

    #[test]
    fn test_1_000_000() {
        assert_eq!(Percent::from(1_000_000).pretty(), "1M%");
    }

    #[test]
    fn test_1_234_567() {
        assert_eq!(Percent::from(1_234_567).pretty(), "1.23M%");
    }

    #[test]
    fn test_10_000_000() {
        assert_eq!(Percent::from(10_000_000).pretty(), "10M%");
    }

    #[test]
    fn test_999_999_999() {
        // you might expect 999M%, but 999_999_999f32 = 1_000_000_000
        // should not matter much for percentage, because there is a reason why
        // it is f32 in the first place
        assert_eq!(Percent::from(999_999_999).pretty(), "1B%");
    }

    #[test]
    fn test_1_000_000_000() {
        assert_eq!(Percent::from(1_000_000_000).pretty(), "1B%");
    }

    #[test]
    fn test_2_345_678_901() {
        assert_eq!(Percent::from(2_345_678_901).pretty(), "2.34B%");
    }
}
