// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use error::*;
pub use font::*;
use image::RgbaImage;
pub use render::*;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::task::spawn_blocking;

mod error;
mod font;
pub mod page;
mod render;

pub type RenderResult = std::result::Result<PathBuf, RenderError>;

pub async fn render<F>(process_image: F) -> RenderResult
where
    F: FnOnce(&mut RgbaImage) + Send + 'static,
{
    // FIXME every request needs to create an own file
    let path = spawn_blocking(move || {
        let img_width = 1200;
        let img_height = 1200;
        let mut img = RgbaImage::new(img_width, img_height);
        process_image(&mut img);
        let output_path = PathBuf::from_str("/tmp/nyanbot_image.png").unwrap();
        img.save(&output_path).unwrap();
        output_path
    })
    .await
    .unwrap();

    Ok(path)
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Width(pub u32);

impl From<u32> for Width {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl PartialEq<u32> for Width {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Height(pub u32);

impl From<u32> for Height {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl PartialEq<u32> for Height {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: Width,
    pub height: Height,
}

impl PartialEq<(u32, u32)> for Size {
    fn eq(&self, other: &(u32, u32)) -> bool {
        self.width.0 == other.0 && self.height == other.1
    }
}

impl From<(u32, u32)> for Size {
    fn from(value: (u32, u32)) -> Self {
        Self {
            width: Width(value.0),
            height: Height(value.1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct X(pub u32);

impl From<u32> for X {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl PartialEq<u32> for X {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Y(pub u32);

impl From<u32> for Y {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl PartialEq<u32> for Y {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: X,
    pub y: Y,
}

impl Point {
    pub fn new(x: impl Into<X>, y: impl Into<Y>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl PartialEq<(u32, u32)> for Point {
    fn eq(&self, other: &(u32, u32)) -> bool {
        self.x.0 == other.0 && self.y == other.1
    }
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: X(value.0),
            y: Y(value.1),
        }
    }
}
