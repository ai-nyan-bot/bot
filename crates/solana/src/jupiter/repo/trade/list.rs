// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::jupiter::model::Trade;
use crate::jupiter::repo::ReadTradeRepo;
use crate::model::Slot;
use base::model::{AddressId, DecimalAmount, PriceQuote, TokenPairId};
use common::model::Timestamp;
use common::repo::{RepoResult, Tx};
use sqlx::Row;

impl ReadTradeRepo {
    pub async fn list<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<Vec<Trade>> {
        Ok(sqlx::query("select * from jupiter.trade;")
            .fetch_all(&mut **tx)
            .await?
            .iter()
            .map(|r| Trade {
                slot: r.get::<Slot, _>("slot"),
                address: r.get::<AddressId, _>("address_id"),
                token_pair: r.get::<TokenPairId, _>("token_pair_id"),
                base_amount: r.get::<DecimalAmount, _>("base_amount"),
                quote_amount: r.get::<DecimalAmount, _>("quote_amount"),
                price: r.get::<PriceQuote, _>("price"),
                is_buy: r.get::<bool, _>("is_buy"),
                timestamp: r.get::<Timestamp, _>("timestamp"),
            })
            .collect::<Vec<_>>())
    }
}
