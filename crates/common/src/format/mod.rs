// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use bigdecimal::BigDecimal;
use lazy_static::lazy_static;

pub use percent::format_percent;
pub use price::format_price_usd;
pub use volume::format_volume_usd;

mod count;
mod market_cap;
mod percent;
mod price;
mod volume;

pub trait FormatPretty {
    fn pretty(self) -> String;
}

lazy_static! {
    pub(crate) static ref BILLION: BigDecimal = BigDecimal::from(1_000_000_000);
    pub(crate) static ref MILLION: BigDecimal = BigDecimal::from(1_000_000);
    pub(crate) static ref THOUSAND: BigDecimal = BigDecimal::from(1_000);
    pub(crate) static ref HOUNDRED: BigDecimal = BigDecimal::from(100);
    pub(crate) static ref TEN: BigDecimal = BigDecimal::from(10);
}
