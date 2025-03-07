use common::model::Percent;
use image::RgbaImage;
use render::page::pumpfun::{pumpfun_summary, PumpfunSummary};
use solana::model::{
    ProgressWithChange, Summary, SummaryCurveProgress, SummarySwap, SwapsWithChange,
};

fn main() {
    let img_width = 1200;
    let img_height = 1200;
    let mut img = RgbaImage::new(img_width, img_height);

    pumpfun_summary(
        &mut img,
        PumpfunSummary {
            m1: Some(Summary {
                token_pair: 1.into(),
                curve_progress: SummaryCurveProgress {
                    open: ProgressWithChange {
                        progress: None,
                        change: None,
                    },
                    high: ProgressWithChange {
                        progress: None,
                        change: None,
                    },
                    low: ProgressWithChange {
                        progress: None,
                        change: None,
                    },
                    close: ProgressWithChange {
                        progress: None,
                        change: None,
                    },
                    avg: ProgressWithChange {
                        progress: Some(23.24.into()),
                        change: Some(10.23.into()),
                    },
                },
                swap: SummarySwap {
                    all: SwapsWithChange {
                        count: 3.into(),
                        change: Some(1.into()),
                        percent: Some(33.3.into()),
                    },
                    buy: SwapsWithChange {
                        count: 2.into(),
                        change: Some(2.into()),
                        percent: Some(100.0.into()),
                    },
                    sell: SwapsWithChange {
                        count: 1.into(),
                        change: Some(1.into()),
                        percent: Some(Percent::from(-50.0)),
                    },
                },
            }),
            m5: None,
            m15: None,
            h1: Some(Summary {
                token_pair: 1.into(),
                curve_progress: SummaryCurveProgress {
                    open: ProgressWithChange {
                        progress: None,
                        change: None,
                    },
                    high: ProgressWithChange {
                        progress: None,
                        change: None,
                    },
                    low: ProgressWithChange {
                        progress: None,
                        change: None,
                    },
                    close: ProgressWithChange {
                        progress: None,
                        change: None,
                    },
                    avg: ProgressWithChange {
                        progress: Some(42.24.into()),
                        change: None,
                    },
                },
                swap: SummarySwap {
                    all: SwapsWithChange {
                        count: 3.into(),
                        change: Some(1.into()),
                        percent: Some(33.3.into()),
                    },
                    buy: SwapsWithChange {
                        count: 2.into(),
                        change: Some(2.into()),
                        percent: Some(100.0.into()),
                    },
                    sell: SwapsWithChange {
                        count: 1.into(),
                        change: Some(1.into()),
                        percent: Some(Percent::from(-50.0)),
                    },
                },
            }),
            h6: None,
            d1: None,
        },
    );

    img.save("/tmp/nyanbot_image.png").unwrap();
}
