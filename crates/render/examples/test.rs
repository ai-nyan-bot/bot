use image::RgbaImage;
use base::model::TradesChangePercent;
use render::page::{pumpfun, PumpfunContext};
use solana::model::{Summary, SummaryTrades, TradesWithChange};

fn main() {
    let img_width = 1200;
    let img_height = 1200;
    let mut img = RgbaImage::new(img_width, img_height);

    pumpfun(
        &mut img,
        PumpfunContext {
            m1: None,
            h1: Some(Summary {
                token_pair: 1.into(),
                trades: SummaryTrades {
                    all: TradesWithChange {
                        trades: 3.into(),
                        change: Some(1.into()),
                        change_percent: Some(33.3.into()),
                    },
                    buy: TradesWithChange {
                        trades: 2.into(),
                        change: Some(2.into()),
                        change_percent: Some(100.0.into()),
                    },
                    sell: TradesWithChange {
                        trades: 1.into(),
                        change: Some(1.into()),
                        change_percent: Some(TradesChangePercent::from(-50.0)),
                    },
                },
            }),
            d1: None,
        },
    );

    img.save("/tmp/nyanbot_image.png").unwrap();
}
