// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::format::FormatPretty;
use crate::model::Percent;

pub fn format_percent<T: Into<Percent>>(num: T) -> String {
    let num = num.into().0;
    let mut suffix = "";
    let formatted = if num >= 1_000_000_000.0 {
        suffix = "B";
        format!("{:.1}", num / 1_000_000_000.0)
    } else if num >= 1_000_000.0 {
        suffix = "M";
        format!("{:.1}", num / 1_000_000.0)
    } else if num >= 10_000.0 {
        suffix = "K";
        format!("{:.1}", num / 1_000.0)
    } else {
        format!("{:.1}", num)
    };

    // Remove trailing ".0" if it exists
    // let cleaned = if formatted.ends_with(".0") {
    //     formatted[..formatted.len() - 2].to_string()
    // } else {
    //     formatted
    // };

    // Ensure the result is at most 4 characters
    let mut result = formatted.chars().take(5).collect::<String>();

    if result.ends_with(".") {
        result.pop().unwrap();
    }

    if result.ends_with(".0") {
        result.pop().unwrap();
        result.pop().unwrap();
    }

    // while result.len() < 4 {
    //     result.insert_str(0, space::<1>());
    // }

    result.push_str(suffix);
    result.push_str("%");
    result
}

impl FormatPretty for Percent {
    fn pretty(self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::{BigDecimal, FromPrimitive};
    use crate::format::FormatPretty;
    use crate::model::{Percent, VolumeUsd};

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
        assert_eq!(VolumeUsd(BigDecimal::from_f32(0.01).unwrap()).pretty(), "0.01%");
    }

    #[test]
    fn test_tiny() {
        assert_eq!(VolumeUsd(BigDecimal::from_f32(0.001).unwrap()).pretty(), "0%");
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
        assert_eq!(Percent::from(999_999_999).pretty(), "999M%");
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
