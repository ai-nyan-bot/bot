// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::jupiter::model::Trade;
use crate::jupiter::repo::ReadTradeRepo;
use crate::model::{Signature, Slot};
use base::model::{AddressId, DecimalAmount, PriceQuote, TokenPairId, TradeId};
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
                id: r.get::<TradeId, _>("id"),
                slot: r.get::<Slot, _>("slot"),
                address: r.get::<AddressId, _>("address_id"),
                token_pair: r.get::<TokenPairId, _>("token_pair_id"),
                amount_base: r.get::<DecimalAmount, _>("amount_base"),
                amount_quote: r.get::<DecimalAmount, _>("amount_quote"),
                price: r.get::<PriceQuote, _>("price"),
                is_buy: r.get::<bool, _>("is_buy"),
                timestamp: r.get::<Timestamp, _>("timestamp"),
                signature: r.get::<Signature, _>("signature"),
            })
            .collect::<Vec<_>>())
    }

    pub async fn list_with_signature<'a>(
        &self,
        tx: &mut Tx<'a>,
        signature: impl Into<Signature>,
    ) -> RepoResult<Vec<Trade>> {
        Ok(
            sqlx::query("select * from jupiter.trade where signature = $1 order by id;")
                .bind(signature.into())
                .fetch_all(&mut **tx)
                .await?
                .iter()
                .map(|r| Trade {
                    id: r.get::<TradeId, _>("id"),
                    slot: r.get::<Slot, _>("slot"),
                    address: r.get::<AddressId, _>("address_id"),
                    token_pair: r.get::<TokenPairId, _>("token_pair_id"),
                    amount_base: r.get::<DecimalAmount, _>("amount_base"),
                    amount_quote: r.get::<DecimalAmount, _>("amount_quote"),
                    price: r.get::<PriceQuote, _>("price"),
                    is_buy: r.get::<bool, _>("is_buy"),
                    timestamp: r.get::<Timestamp, _>("timestamp"),
                    signature: r.get::<Signature, _>("signature"),
                })
                .collect::<Vec<_>>(),
        )
    }
}
