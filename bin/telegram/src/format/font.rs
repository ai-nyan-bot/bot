// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Style {
    Regular,
    Bold,
    Italic,
}

pub struct SansSerif {
    unicode: HashMap<char, char>,
}

impl SansSerif {
    pub fn new(style: Style) -> Self {
        let mut unicode_map: HashMap<char, char> = HashMap::new();

        let (offset_upper, offset_lower, offset_digit) = match style {
            Style::Regular => (
                '𝖠' as u32 - 'A' as u32,
                '𝖺' as u32 - 'a' as u32,
                '𝟢' as u32 - '0' as u32,
            ),
            Style::Bold => (
                '𝗔' as u32 - 'A' as u32,
                '𝗮' as u32 - 'a' as u32,
                '𝟬' as u32 - '0' as u32,
            ),
            Style::Italic => ('𝘈' as u32 - 'A' as u32, '𝘢' as u32 - 'a' as u32, 0),
        };

        // Populate uppercase letters
        for c in 'A'..='Z' {
            unicode_map.insert(c, char::from_u32((c as u32) + offset_upper).unwrap());
        }
        // Populate lowercase letters
        for c in 'a'..='z' {
            unicode_map.insert(c, char::from_u32((c as u32) + offset_lower).unwrap());
        }
        // Populate digits
        if style != Style::Italic {
            for c in '0'..='9' {
                unicode_map.insert(c, char::from_u32((c as u32) + offset_digit).unwrap());
            }
        }

        Self {
            unicode: unicode_map,
        }
    }

    pub fn format(&self, text: &str) -> String {
        text.chars()
            .map(|c| self.unicode.get(&c).cloned().unwrap_or(c))
            .collect()
    }
}

impl Default for SansSerif {
    fn default() -> Self {
        Self::new(Style::Regular)
    }
}
