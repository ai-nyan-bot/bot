// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Signature, Slot};
use crate::pumpfun::model::Trade;
use crate::pumpfun::repo::ReadTradeRepo;
use base::model::{AddressId, Amount, DecimalAmount, Percent, PriceQuote, TokenPairId};
use common::model::Timestamp;
use common::repo::{RepoResult, Tx};
use sqlx::Row;

impl ReadTradeRepo {
    pub async fn list_most_recent<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<Box<[Trade]>> {
        Ok(sqlx::query("select * from pumpfun.trade_most_recent;")
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
                virtual_base_reserves: r.get::<Amount, _>("virtual_base_reserves"),
                virtual_quote_reserves: r.get::<Amount, _>("virtual_quote_reserves"),
                progress: r.get::<Percent, _>("progress"),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }

    pub async fn list<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<Vec<Trade>> {
        Ok(sqlx::query("select * from pumpfun.trade;")
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
                virtual_base_reserves: r.get::<Amount, _>("virtual_base_reserves"),
                virtual_quote_reserves: r.get::<Amount, _>("virtual_quote_reserves"),
                progress: r.get::<Percent, _>("progress"),
            })
            .collect::<Vec<_>>())
    }

    pub async fn list_of_tx<'a>(&self, tx: &mut Tx<'a>, signature: impl Into<Signature>) -> RepoResult<Vec<Trade>> {
        Ok(sqlx::query("select * from pumpfun.trade where signature = $1;")
            .bind(signature.into())
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
                virtual_base_reserves: r.get::<Amount, _>("virtual_base_reserves"),
                virtual_quote_reserves: r.get::<Amount, _>("virtual_quote_reserves"),
                progress: r.get::<Percent, _>("progress"),
            })
            .collect::<Vec<_>>())
    }
}
