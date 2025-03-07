// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

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
