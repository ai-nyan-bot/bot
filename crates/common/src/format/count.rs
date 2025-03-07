// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Count;

pub fn format_count<T: Into<Count>>(num: T) -> String {
    let num = num.into().0 as f64;
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

    let cleaned = if formatted.ends_with(".0") {
        formatted[..formatted.len() - 2].to_string()
    } else {
        formatted
    };

    let mut result = cleaned.chars().take(5).collect::<String>();
    result.push_str(suffix);
    result
}
