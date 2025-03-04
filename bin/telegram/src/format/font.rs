// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Style {
    Regular,
    Bold,
}

pub struct Font {
    unicode: HashMap<char, char>,
}


impl Font {
    pub fn new(style: Style) -> Self {
        let mut unicode_map: HashMap<char, char> = HashMap::new();

        let (offset_upper, offset_lower, offset_digit) = match style {
            Style::Regular => (
                '𝙰' as u32 - 'A' as u32,
                '𝚊' as u32 - 'a' as u32,
                '𝟶' as u32 - '0' as u32,
            ),
            Style::Bold => (
                '𝗔' as u32 - 'A' as u32,
                '𝗮' as u32 - 'a' as u32,
                '𝟬' as u32 - '0' as u32,
            ),
        };

        for c in 'A'..='Z' {
            unicode_map.insert(c, char::from_u32((c as u32) + offset_upper).unwrap());
        }
        for c in 'a'..='z' {
            unicode_map.insert(c, char::from_u32((c as u32) + offset_lower).unwrap());
        }
        for c in '0'..='9' {
            unicode_map.insert(c, char::from_u32((c as u32) + offset_digit).unwrap());
        }

        Self {
            unicode: unicode_map,
        }
    }

    pub fn format(&self, text: impl AsRef<str>) -> String {
        text.as_ref()
            .chars()
            .map(|c| self.unicode.get(&c).cloned().unwrap_or(c))
            .collect()
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::new(Style::Regular)
    }
}
