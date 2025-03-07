// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::{Point, Size};
use rusttype::{point, Font as UnderlyingFont, LayoutIter, Scale};

pub enum FontType {
    DejaVuSans,
}

#[derive(Debug, Copy, Clone)]
pub struct FontSize(u32);

impl From<u32> for FontSize {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl FontSize {
    pub fn scale(&self) -> Scale {
        {
            Scale {
                x: self.0 as f32,
                y: self.0 as f32,
            }
        }
    }
}

pub struct Font<'font> {
    font: UnderlyingFont<'font>,
}

impl<'font> Font<'font> {
    pub fn new(font_type: FontType) -> Self {
        Self {
            font: rusttype::Font::try_from_bytes(match font_type {
                FontType::DejaVuSans => include_bytes!("../asset/DejaVuSans.ttf"),
            })
            .expect("failed to load font data"),
        }
    }

    pub fn measure(&self, text: impl AsRef<str>, size: impl Into<FontSize>) -> Size {
        let scale = size.into().scale();
        let v_metrics = self.font.v_metrics(scale);
        let height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;

        let glyphs: Vec<_> = self
            .font
            .layout(text.as_ref(), scale, point(0.0, 0.0))
            .collect();

        let width = if let Some(last) = glyphs.last() {
            last.position().x + last.unpositioned().h_metrics().advance_width
        } else {
            0.0
        }
            .ceil() as u32;

        Size {
            width: width.into(),
            height: height.into(),
        }
    }


    pub fn layout<'s>(
        &'font self,
        s: &'s str,
        size: FontSize,
        start: Point,
    ) -> LayoutIter<'font, 'font, 's> {
        self.font.layout(
            s,
            size.scale(),
            rusttype::point(start.x.0 as f32, start.y.0 as f32),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Font, FontType};

    #[test]
    fn test_measure() {
        let test_instance = Font::new(FontType::DejaVuSans);
        assert_eq!(test_instance.measure("42", 0), (0, 0));
        assert_eq!(test_instance.measure("42", 1), (2, 1));
        assert_eq!(test_instance.measure("4242", 1), (3, 1));

        assert_eq!(test_instance.measure("42", 10), (11, 10));
        assert_eq!(test_instance.measure("4242", 10), (22, 10));

        assert_eq!(
            test_instance.measure("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 14),
            (211, 14)
        );
        assert_eq!(
            test_instance.measure("abcdefghijklmnopqrstuvwxyz", 14),
            (177, 14)
        );
        assert_eq!(test_instance.measure("0123456789", 14), (77, 14));
    }
}
