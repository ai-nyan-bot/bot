use image::{Rgba, RgbaImage};
use render::render::{RenderBackground, RenderText, RenderWatermark, Text};
use render::{Font, FontType};

struct Span {
    words: Vec<Text>,
    padding: (u32, u32), // (x, y) padding
}

impl Span {
    fn render(&self, img: &mut RgbaImage, font: &Font, start_x: u32, start_y: u32) {
        let mut x_offset = (start_x + self.padding.0) as u32;
        let y_offset = (start_y + self.padding.1) as u32;
        for word in &self.words {
            let size = RenderText::render(img, font, x_offset, y_offset, word);
            x_offset += size.width.0;
        }
    }
}

fn draw_line(img: &mut RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgba<u8>) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let (mut x, mut y) = (x0, y0);

    while x != x1 || y != y1 {
        if x >= 0 && y >= 0 && x < img.width() as i32 && y < img.height() as i32 {
            img.put_pixel(x as u32, y as u32, color);
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

/// Draws a thick line from (x0, y0) to (x1, y1) with the given thickness and color
fn draw_thick_line(img: &mut RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, thickness: u32, color: Rgba<u8>) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let length = ((dx * dx + dy * dy) as f64).sqrt();

    if length == 0.0 {
        return;
    }

    // Compute perpendicular direction (normalized)
    let perp_x = -(dy as f64) / length;
    let perp_y = (dx as f64) / length;

    let half_thickness = thickness as f64 / 2.0;

    // Draw multiple lines around the main line for thickness
    for i in -half_thickness as i32..=half_thickness as i32 {
        let offset_x = (i as f64 * perp_x).round() as i32;
        let offset_y = (i as f64 * perp_y).round() as i32;
        draw_single_line(img, x0 + offset_x, y0 + offset_y, x1 + offset_x, y1 + offset_y, color);
    }
}

/// Draws a single pixel-wide line using Bresenham's algorithm
fn draw_single_line(img: &mut RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgba<u8>) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let (mut x, mut y) = (x0, y0);

    while x != x1 || y != y1 {
        if x >= 0 && y >= 0 && x < img.width() as i32 && y < img.height() as i32 {
            img.put_pixel(x as u32, y as u32, color);
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}



fn main() {
    let img_width = 1200;
    let img_height = 1200;
    let mut img = RgbaImage::new(img_width, img_height);

    // Load font
    // let font_data = include_bytes!("../asset/DejaVuSans.ttf");
    // let font = Font::try_from_bytes(font_data).expect("Failed to load font");
    let font = Font::new(FontType::DejaVuSans);


    RenderBackground::render(&mut img);
    RenderWatermark::render(&mut img);

    // let span = Span {
    //     words: vec![
    //         // Text {
    //         //     content: "Trades".into(),
    //         //     size: 40.into(),
    //         //     color: Rgba([255, 0, 0, 255]),
    //         // },
    //         Text {
    //             content: "Trades:".into(),
    //             size: 40.into(),
    //             color: Rgba([255, 0, 0, 255]),
    //         },
    //         Text {
    //             content: "All".into(),
    //             size: 30.into(),
    //             color: Rgba([128, 128, 128, 255]),
    //         },
    //         Text {
    //             content: "1200".into(),
    //             size: 40.into(),
    //             color: Rgba([255, 255, 255, 255]),
    //         },
    //         Text {
    //             content: "(25.12%)".into(),
    //             size: 40.into(),
    //             color: Rgba([0, 255, 0, 255]),
    //         },
    //         Text {
    //             content: "Buy".into(),
    //             size: 30.into(),
    //             color: Rgba([255, 0, 0, 255]),
    //         },
    //         Text {
    //             content: "1200".into(),
    //             size: 40.into(),
    //             color: Rgba([0, 0, 255, 255]),
    //         },
    //         Text {
    //             content: "(25.12%)".into(),
    //             size: 40.into(),
    //             color: Rgba([255, 0, 255, 255]),
    //         },
    //         Text {
    //             content: "Sell".into(),
    //             size: 30.into(),
    //             color: Rgba([255, 0, 0, 255]),
    //         },
    //         Text {
    //             content: "1200".into(),
    //             size: 40.into(),
    //             color: Rgba([0, 0, 255, 255]),
    //         },
    //         Text {
    //             content: "(25.12%)".into(),
    //             size: 40.into(),
    //             color: Rgba([255, 0, 255, 255]),
    //         },
    //     ],
    //     padding: (0, 0),
    // };
    // 
    // span.render(&mut img, &font, 0, 90);

    draw_thick_line(&mut img, 150,0,150,1200, 2, Rgba([30, 30, 30, 255]));
    draw_line(&mut img, 150 * 2,0,150 * 2,1200, Rgba([50, 50, 50, 255]));
    draw_line(&mut img, 150 * 3,0,150 * 3,1200, Rgba([50, 50, 50, 255]));
    draw_line(&mut img, 150 * 4,0,150 * 4,1200, Rgba([50, 50, 50, 255]));
    draw_line(&mut img, 150 * 5,0,150 * 5,1200, Rgba([50, 50, 50, 255]));
    draw_line(&mut img, 150 * 6,0,150 * 6,1200, Rgba([50, 50, 50, 255]));
    draw_line(&mut img, 150 * 7,0,150 * 7,1200, Rgba([50, 50, 50, 255]));

    draw_line(&mut img, 0,100,1200,100, Rgba([50, 50, 50, 255]));
    draw_line(&mut img, 0,220,1200,220, Rgba([50, 50, 50, 255]));
    draw_line(&mut img, 0,220 + 120,1200,220 + 120, Rgba([50, 50, 50, 255]));
    draw_line(&mut img, 0,220 + 120 + 120,1200,220 + 120 + 120, Rgba([50, 50, 50, 255]));
    draw_line(&mut img, 0,220 + 120 + 120 + 120,1200,220 + 120 + 120 + 120, Rgba([50, 50, 50, 255]));
    
    draw_line(&mut img, 0,220 + 120 + 120 + 120 * 3,1200,220 + 120 + 120 + 120 * 3, Rgba([50, 50, 50, 255]));
    draw_line(&mut img, 0,220 + 120 + 120 + 120 * 4,1200,220 + 120 + 120 + 120 * 4, Rgba([50, 50, 50, 255]));

    // draw_line(&mut img, 350,100,350,250, Rgba([50, 50, 50, 255]));

    {
    let span = Span {
        words: vec![
            // Text {
            //     content: "Trades".into(),
            //     size: 36.into(),
            //     color: Rgba([255, 0, 0, 255]),
            // },
            Text {
                content: "Progress".into(),
                size: 36.into(),
                color: Rgba([100, 100, 100, 255]),
            }
        ],
        padding: (0, 0),
    };

    span.render(&mut img, &font, 36, 36);


    let span = Span {
        words: vec![
            // Text {
            //     content: "Trades".into(),
            //     size: 36.into(),
            //     color: Rgba([255, 0, 0, 255]),
            // },
            Text {
                content: "1min".into(),
                size: 36.into(),
                color: Rgba([100, 100, 100, 255]),
            }
        ],
        padding: (0, 0),
    };

    span.render(&mut img, &font, 36 + 150, 36);

    let span = Span {
        words: vec![
            // Text {
            //     content: "Trades".into(),
            //     size: 36.into(),
            //     color: Rgba([255, 0, 0, 255]),
            // },
            Text {
                content: "10(0.02%)".into(),
                size: 30.into(),
                color: Rgba([10, 200, 10, 255]),
            }
        ],
        padding: (10, 0),
    };

    span.render(&mut img, &font, 0 + 150, 80);

    let span = Span {
        words: vec![
            Text {
                content: "5min".into(),
                size: 36.into(),
                color: Rgba([100, 100, 100, 255]),
            }
        ],
        padding: (0, 0),
    };

    span.render(&mut img, &font, 36 + 150 + 150, 36);

    let span = Span {
        words: vec![
            Text {
                content: "+0.03%".into(),
                size: 36.into(),
                color: Rgba([10, 200, 10, 255]),
            }
        ],
        padding: (10, 0),
    };

    span.render(&mut img, &font, 0 + 150 + 150, 80);


    let span = Span {
        words: vec![
            Text {
                content: "15min".into(),
                size: 36.into(),
                color: Rgba([100, 100, 100, 255]),
            }
        ],
        padding: (0, 0),
    };

    span.render(&mut img, &font, 36 + 150 + 150 + 150, 36);

    let span = Span {
        words: vec![
            Text {
                content: "+0.04%".into(),
                size: 36.into(),
                color: Rgba([10, 200, 10, 255]),
            }
        ],
        padding: (10, 0),
    };

    span.render(&mut img, &font, 0 + 150 + 150 + 150, 80);

    let span = Span {
        words: vec![
            Text {
                content: "30min".into(),
                size: 36.into(),
                color: Rgba([100, 100, 100, 255]),
            }
        ],
        padding: (0, 0),
    };

    span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150, 36);

    let span = Span {
        words: vec![
            Text {
                content: "+0.05%".into(),
                size: 36.into(),
                color: Rgba([10, 200, 10, 255]),
            }
        ],
        padding: (10, 0),
    };

    span.render(&mut img, &font, 0 + 150 + 150 + 150 +150, 80);


    let span = Span {
        words: vec![
            Text {
                content: "1h".into(),
                size: 36.into(),
                color: Rgba([100, 100, 100, 255]),
            }
        ],
        padding: (0, 0),
    };

    span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150, 36);

    let span = Span {
        words: vec![
            Text {
                content: "+0.06%".into(),
                size: 36.into(),
                color: Rgba([10, 200, 10, 255]),
            }
        ],
        padding: (10, 0),
    };

    span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150, 80);

    let span = Span {
        words: vec![
            Text {
                content: "6h".into(),
                size: 36.into(),
                color: Rgba([100, 100, 100, 255]),
            }
        ],
        padding: (0, 0),
    };

    span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150, 36);

    let span = Span {
        words: vec![
            Text {
                content: "+0.07%".into(),
                size: 36.into(),
                color: Rgba([10, 200, 10, 255]),
            }
        ],
        padding: (10, 0),
    };

    span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150, 80);

    let span = Span {
        words: vec![
            Text {
                content: "24h".into(),
                size: 36.into(),
                color: Rgba([100, 100, 100, 255]),
            }
        ],
        padding: (0, 0),
    };

    span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150 + 150, 36);

    let span = Span {
        words: vec![
            Text {
                content: "+0.08%".into(),
                size: 36.into(),
                color: Rgba([10, 200, 10, 255]),
            }
        ],
        padding: (10, 0),
    };

    span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150 + 150, 80);
}

    // row 2
    {
        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "Price".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36, 36 + 120);


        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "1min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150, 36 + 120);

        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "10(0.02%)".into(),
                    size: 30.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150, 80 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "5min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150, 36 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.03%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150, 80+ 120);


        let span = Span {
            words: vec![
                Text {
                    content: "15min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150, 36 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.04%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150, 80 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "30min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150, 36 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.05%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150, 80 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "1h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150, 36 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.06%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150, 80 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "6h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.07%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150, 80 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "24h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.08%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150 + 150, 80 + 120);
    }

    // row 3 
    {
        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "Cap".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36, 36 + 120 + 120);


        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "1min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150, 36 + 120 + 120);

        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "10(0.02%)".into(),
                    size: 30.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150, 80 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "5min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150, 36 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.03%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150, 80+ 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "15min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150, 36 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.04%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150, 80 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "30min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150, 36 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.05%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150, 80 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "1h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.06%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150, 80 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "6h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.07%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150, 80 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "24h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.08%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150 + 150, 80 + 120 + 120);
    }

    // row 4
    {
        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "Holder".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36, 36 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "1min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150, 36 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "10(0.02%)".into(),
                    size: 30.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150, 80 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "5min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150, 36 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.03%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150, 80+ 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "15min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150, 36 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.04%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150, 80 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "30min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.05%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150, 80 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "1h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.06%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150, 80 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "6h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.07%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150, 80 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "24h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.08%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150 + 150, 80 + 120 + 120 + 120);
    }

    // row 5
    {
        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "Trades".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36, 36 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "1min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150, 36 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "10(0.02%)".into(),
                    size: 30.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150, 80 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "5min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150, 36 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.03%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150, 80+ 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "15min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.04%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150, 80 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "30min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.05%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150, 80 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "1h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.06%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150, 80 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "6h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.07%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150, 80 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "24h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.08%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150 + 150, 80 + 120 + 120 + 120 + 120);
    }

    // row 6
    {
        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "Buy".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36, 36 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "1min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150, 36 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "10(0.02%)".into(),
                    size: 30.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150, 80 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "5min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.03%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150, 80+ 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "15min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.04%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "30min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.05%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150, 80 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "1h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.06%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150, 80 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "6h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.07%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "24h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.08%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120);
    }

    // row 7
    {
        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "Sell".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36, 36 + 120 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "1min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "10(0.02%)".into(),
                    size: 30.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "5min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.03%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150, 80+ 120 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "15min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.04%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "30min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.05%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150, 80 + 120 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "1h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.06%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "6h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.07%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "24h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.08%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120);
    }

    // row 8
    {
        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "Volume".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "1min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "10(0.02%)".into(),
                    size: 30.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "5min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.03%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150, 80+ 120 + 120 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "15min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.04%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "30min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.05%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "1h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.06%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "6h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.07%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "24h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.08%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120);
    }

    // row 9
    {
        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "Buy".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "1min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "10(0.02%)".into(),
                    size: 30.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "5min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.03%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150, 80+ 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "15min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.04%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "30min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.05%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "1h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.06%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "6h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.07%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "24h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.08%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);
    }

    // row 10
    {
        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "Sell".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "1min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                // Text {
                //     content: "Trades".into(),
                //     size: 36.into(),
                //     color: Rgba([255, 0, 0, 255]),
                // },
                Text {
                    content: "10(0.02%)".into(),
                    size: 30.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "5min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.03%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150, 80+ 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "15min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.04%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "30min".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.05%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);


        let span = Span {
            words: vec![
                Text {
                    content: "1h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.06%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "6h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.07%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "24h".into(),
                    size: 36.into(),
                    color: Rgba([100, 100, 100, 255]),
                }
            ],
            padding: (0, 0),
        };

        span.render(&mut img, &font, 36 + 150 + 150 + 150 + 150 + 150 + 150 + 150, 36 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);

        let span = Span {
            words: vec![
                Text {
                    content: "+0.08%".into(),
                    size: 36.into(),
                    color: Rgba([10, 200, 10, 255]),
                }
            ],
            padding: (10, 0),
        };

        span.render(&mut img, &font, 0 + 150 + 150 + 150 +150 + 150 + 150 + 150, 80 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120 + 120);
    }

    img.save("/tmp/nyanbot_image.png").unwrap();
}
