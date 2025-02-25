// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Timestamp;
use solana::pumpfun::repo::{SlotTrade, SlotTrades};

mod log;
mod upsert;

pub(crate) fn inserting_slot_trades() -> SlotTrades {
    SlotTrades {
        slot: 12345.into(),
        timestamp: Timestamp::now(),
        trades: vec![SlotTrade {
            mint: "mint1".into(),
            base_amount: 1_000_000.into(),
            quote_amount: 2_000_000_000.into(),
            is_buy: true,
            wallet: "7PQ3nyAJHXiFQd5c8HgRBMYLF748MQKgq3uYfTuFioHX".into(),
            virtual_base_reserves: 512561011366544i64.into(),
            virtual_quote_reserves: 62802280169i64.into(),
            signature: "signature1".into(),
        }],
    }
}

pub(crate) fn updating_slot_trades() -> SlotTrades {
    SlotTrades {
        slot: 23456.into(),
        timestamp: Timestamp::now(),
        trades: vec![SlotTrade {
            mint: "mint1".into(),
            base_amount: 1_000_000.into(),
            quote_amount: 2_000_000_000.into(),
            is_buy: false,
            wallet: "7PQ3nyAJHXiFQd5c8HgRBMYLF748MQKgq3uYfTuFioHX".into(),
            virtual_base_reserves: 0.into(),
            virtual_quote_reserves: 0.into(),
            signature: "signature2".into(),
        }],
    }
}
