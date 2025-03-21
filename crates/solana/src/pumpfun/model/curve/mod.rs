// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::solana::Slot;
use base::model::{Amount, TokenPairId};
use common::model::{AgeRelativeToLatestInSeconds, Percent};
pub use progress::calculate_progress;

mod progress;

#[derive(Debug, Clone)]
pub struct Curve {
    pub id: TokenPairId,
    pub slot: Slot,
    pub virtual_base_reserves: Amount,
    pub virtual_quote_reserves: Amount,
    pub progress: Percent,
    pub complete: bool,
    pub age: AgeRelativeToLatestInSeconds,
}

pub struct CurveLog {
    pub id: TokenPairId,
    pub slot: Slot,
    pub virtual_base_reserves: Amount,
    pub virtual_quote_reserves: Amount,
    pub progress: Percent,
    pub complete: bool,
}

#[derive(Debug, Clone)]
pub struct CurveInfo {
    pub virtual_base_reserves: Amount,
    pub virtual_quote_reserves: Amount,
    pub real_base_reserves: Amount,
    pub real_quote_reserves: Amount,
    pub total_supply: Amount,
    pub complete: bool,
}
