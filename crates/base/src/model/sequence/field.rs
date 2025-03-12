// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Field {
    AgeBase,
    AgeQuote,

    CurveProgress,
    CurveProgressAge,

    Price,
    PriceAvg,
    SwapAll,
    SwapBuy,
    SwapSell,
    Volume,

    //
    TelegramExists,
    TelegramGroupHandle,
    TwitterExists,
    TwitterAccountHandle,
}
