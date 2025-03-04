// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub(crate) fn space_default() -> &'static str {
    space::<1>()
}

pub(crate) fn space<const N: usize>() -> &'static str {
    match N {
        1 => "\u{3000}",
        2 => "\u{3000}\u{3000}",
        3 => "\u{3000}\u{3000}\u{3000}",
        4 => "\u{3000}\u{3000}\u{3000}\u{3000}",
        5 => "\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}",
        6 => "\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}",
        7 => "\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}",
        _ => unimplemented!(),
    }
}

pub fn number_f64<T: Into<f64>>(num: T) -> String {
    let num = num.into().abs();
    let mut suffix = "";
    let formatted = if num >= 1_000_000_000.0 {
        suffix = " B";
        format!("{:.1}", num / 1_000_000_000.0)
    } else if num >= 1_000_000.0 {
        suffix = " M";
        format!("{:.1}", num / 1_000_000.0)
    } else if num >= 10_000.0 {
        suffix = " K";
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
    
    if result.ends_with("."){
        result.pop().unwrap();
    }

    if result.ends_with(".0"){
        result.pop().unwrap();
        result.pop().unwrap();
    }

    // while result.len() < 4 {
    //     result.insert_str(0, space::<1>());
    // }
    
    result.push_str(suffix);
    result
}

fn main() {
    println!("'{}'", number_f64(12));        // '  12'
    println!("'{}'", number_f64(999));       // ' 999'
    println!("'{}'", number_f64(1000));      // '1.0k'
    println!("'{}'", number_f64(1500));      // '1.5k'
    println!("'{}'", number_f64(999_999));   // '999k'
    println!("'{}'", number_f64(1_000_000)); // '1.0M'
    println!("'{}'", number_f64(2_500_000)); // '2.5M'
    println!("'{}'", number_f64(1_000_000_000)); // '1.0B'
    // println!("'{}'", number(3_500_000_000_i64)); // '3.5B'
}