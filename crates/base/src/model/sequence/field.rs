// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Field {
    CurveProgress,
    CurveProgressAge,

    Price,
    PriceAvg,
    Trades,
    TradesBuy,
    TradesSell,
    Volume,

    //
    TelegramExists,
    TelegramGroupHandle,
    TwitterExists,
    TwitterAccountHandle,
}
