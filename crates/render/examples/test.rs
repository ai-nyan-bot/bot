use common::model::{Percent, PriceUsd};
use image::RgbaImage;
use render::page::pumpfun::{pumpfun_summary, PumpfunSummary};
use solana::model::{MarketCapWithChange, PriceWithChange, ProgressWithChange, SummaryCurveProgress, SummaryMarketCap, SummaryPrice, SummarySwap, SummaryVolume, SwapWithChange, TimeframeSummary, VolumeWithChange};

fn main() {
    let img_width = 1200;
    let img_height = 1200;
    let mut img = RgbaImage::new(img_width, img_height);

    pumpfun_summary(
        &mut img,
        PumpfunSummary {
            m1: Some(TimeframeSummary {
                curve: SummaryCurveProgress {
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
                cap: SummaryMarketCap {
                    open: MarketCapWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    high: MarketCapWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    low: MarketCapWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    close: MarketCapWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    avg: MarketCapWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                },
                price: SummaryPrice {
                    open: PriceWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    high: PriceWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    low: PriceWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    close: PriceWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    avg: PriceWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                },
                swap: SummarySwap {
                    all: SwapWithChange {
                        count: Some(3.into()),
                        change: Some(1.into()),
                        percent: Some(33.3.into()),
                    },
                    buy: SwapWithChange {
                        count: Some(2.into()),
                        change: Some(2.into()),
                        percent: Some(100.0.into()),
                    },
                    sell: SwapWithChange {
                        count: Some(10_200.into()),
                        change: Some(1.into()),
                        percent: Some(Percent::from(-51.0)),
                    },
                },
                volume: SummaryVolume {
                    all: VolumeWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    buy: VolumeWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    sell: VolumeWithChange {
                        quote: None,
                        usd: Some(10_200.into()),
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                },
            }),
            m5: None,
            m15: None,
            h1: Some(TimeframeSummary {
                curve: SummaryCurveProgress {
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
                cap: SummaryMarketCap {
                    open: MarketCapWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    high: MarketCapWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    low: MarketCapWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    close: MarketCapWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    avg: MarketCapWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                },
                price: SummaryPrice {
                    open: PriceWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    high: PriceWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    low: PriceWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    close: PriceWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    avg: PriceWithChange {
                        quote: None,
                        usd: Some(PriceUsd::from("0.000000102944")),
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                },
                swap: SummarySwap {
                    all: SwapWithChange {
                        count: Some(3.into()),
                        change: Some(1.into()),
                        percent: Some(33.3.into()),
                    },
                    buy: SwapWithChange {
                        count: Some(2.into()),
                        change: Some(2.into()),
                        percent: Some(100.0.into()),
                    },
                    sell: SwapWithChange {
                        count: Some(1.into()),
                        change: Some(1.into()),
                        percent: Some(Percent::from(-50.0)),
                    },
                },
                volume: SummaryVolume {
                    all: VolumeWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    buy: VolumeWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                    sell: VolumeWithChange {
                        quote: None,
                        usd: None,
                        quote_change: None,
                        usd_change: None,
                        percent: None,
                    },
                },
            }),
            h6: None,
            d1: None,
        },
    );

    img.save("/tmp/nyanbot_image.png").unwrap();
}
