// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::format::format::{number_f64, space, space_default};
use crate::{Font, Style};
use solana::model::SummaryTrades;

pub struct BlockFormatter {
    font: Font,
    bold_font: Font,
}

impl Default for BlockFormatter {
    fn default() -> Self {
        Self {
            font: Font::default(),
            bold_font: Font::default(),
        }
    }
}

impl BlockFormatter {
    pub fn trades(&self, summary: SummaryTrades) -> String {
        let font = Font::default();
        let bold_font = Font::new(Style::Bold);


        let mut builder = String::new();
        builder += r#"
<pre language="text">
𝙰𝚕𝚕:  𝟷𝟸𝟺𝟶
𝙱𝚞𝚢:     𝟺
</pre>        
"#;
        // builder += format!("{}", bold_font.format("Trades")).as_str();
        // builder += "\n";
        // builder += format!("{}", font.format("All:")).as_str();
        // // builder += space::<1>();
        // builder += font.format(number_f64(1240)).as_str();
        // // builder += bold_font.format(number_f64(12400)).as_str();
        // // builder += space::<2>();
        // // builder += format!(
        // //     "{}",
        // //     font.format("+\u{0020}(\u{0020}15\u{0020}|\u{0020}23.4%\u{0020})")
        // // )
        // // .as_str();
        // builder += "\n";
        // builder += format!("{}", font.format("Buy:")).as_str();
        // // builder += space::<1>();
        // builder += space::<2>();
        // builder += font.format(number_f64(4)).as_str();
        // // builder += format!(
        // //     // "\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}{}",
        // //     "\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}{}",
        // //     bold_font.format("4")
        // // )
        // // .as_str();
        // // builder += space::<2>();
        // // builder += format!(
        // //     "{}",
        // //     font.format(
        // //         "\u{0020}\u{0020}\u{0020}-\u{0020}(\u{0020}15\u{0020}|\u{0020}23.4%\u{0020})"
        // //     )
        // // )
        // // .as_str();
        // builder += "\n";
        // // builder += markdown!("{}", font.format("Sell:")).as_str();
        // // builder += "\u{2003}\u{2003}";
        // // builder += markdown!(";`\u{200B} {}{};`", bold_font.format("3.4"), font.format("M")).as_str();
        // // builder += "\u{2003}\u{2003}";
        // // builder += markdown!("{}", font.format(";` + ;`( 15 | 23.4% )")).as_str();
        // builder += "\n";

        builder
    }
}
