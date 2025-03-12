// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::format::FormatPretty;
use crate::model::Count;

impl FormatPretty for Count {
    fn pretty(self) -> String {
        let abs_value = self.0.abs();
        let (value, suffix) = if abs_value >= 1_000_000_000 {
            (self.0 as f64 / 1_000_000_000.0, "B")
        } else if abs_value >= 1_000_000 {
            (self.0 as f64 / 1_000_000.0, "M")
        } else if abs_value >= 1_000 {
            (self.0 as f64 / 1_000.0, "k")
        } else {
            return self.0.to_string();
        };

        let mut formatted = value.to_string();

        if let Some(dot_index) = formatted.find('.') {
            let truncate_len = if value >= 100.0 {
                dot_index
            } else if value >= 10.0 {
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
}

#[cfg(test)]
mod tests {
    use crate::format::FormatPretty;
    use crate::model::Count;

    #[test]
    fn test_zero() {
        assert_eq!(Count::from(0).pretty(), "0");
    }

    #[test]
    fn test_one() {
        assert_eq!(Count::from(1).pretty(), "1");
    }

    #[test]
    fn test_999() {
        assert_eq!(Count::from(999).pretty(), "999");
    }

    #[test]
    fn test_negative_999() {
        assert_eq!(Count::from(-999).pretty(), "-999");
    }

    #[test]
    fn test_1050() {
        assert_eq!(Count::from(1050).pretty(), "1.05k");
    }

    #[test]
    fn test_negative_1050() {
        assert_eq!(Count::from(-1050).pretty(), "-1.05k");
    }

    #[test]
    fn test_10_500() {
        assert_eq!(Count::from(10_500).pretty(), "10.5k");
    }

    #[test]
    fn test_999_999() {
        assert_eq!(Count::from(999_999).pretty(), "999k");
    }

    #[test]
    fn test_1_000_000() {
        assert_eq!(Count::from(1_000_000).pretty(), "1M");
    }

    #[test]
    fn test_1_234_567() {
        assert_eq!(Count::from(1_234_567).pretty(), "1.23M");
    }

    #[test]
    fn test_negative_1_234_567() {
        assert_eq!(Count::from(-1_234_567).pretty(), "-1.23M");
    }

    #[test]
    fn test_10_000_000() {
        assert_eq!(Count::from(10_000_000).pretty(), "10M");
    }

    #[test]
    fn test_999_999_999() {
        assert_eq!(Count::from(999_999_999).pretty(), "999M");
    }

    #[test]
    fn test_1_000_000_000() {
        assert_eq!(Count::from(1_000_000_000).pretty(), "1B");
    }

    #[test]
    fn test_2_345_678_901() {
        assert_eq!(Count::from(2_345_678_901).pretty(), "2.34B");
    }

    #[test]
    fn test_negative_2_345_678_901() {
        assert_eq!(Count::from(-2_345_678_901).pretty(), "-2.34B");
    }
}
