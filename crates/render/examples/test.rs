use image::{Rgb, RgbImage};
use render::render::{Text, TextRender};
use render::{Font, FontType};

struct Span {
    words: Vec<Text>,
    padding: (u32, u32), // (x, y) padding
}

impl Span {
    fn render(&self, img: &mut RgbImage, font: &Font, start_x: u32, start_y: u32) {
        let mut x_offset = (start_x + self.padding.0) as u32;
        let y_offset = (start_y + self.padding.1) as u32;
        for word in &self.words {
            let size = TextRender::render(img, font, x_offset, y_offset, word);
            x_offset += size.width.0;
        }
    }
}

fn main() {
    let img_width = 800;
    let img_height = 200;
    let mut img = RgbImage::new(img_width, img_height);

    // Load font
    // let font_data = include_bytes!("../asset/DejaVuSans.ttf");
    // let font = Font::try_from_bytes(font_data).expect("Failed to load font");
    let font = Font::new(FontType::DejaVuSans);

    // Define a span with styled words
    let span = Span {
        words: vec![
            Text {
                content: "Hello".into(),
                size: 50.into(),
                color: Rgb([255, 0, 0]),
            },
            Text {
                content: "$Nyan".into(),
                size: 50.into(),
                color: Rgb([0, 0, 255]),
            },
            Text {
                content: "Bot".into(),
                size: 50.into(),
                color: Rgb([255, 0, 255]),
            },
        ],
        padding: (0, 0),
    };

    span.render(&mut img, &font, 0, 50);

    img.save("/tmp/output.png").unwrap();
}
