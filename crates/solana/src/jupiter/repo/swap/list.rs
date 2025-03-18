// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::jupiter::model::Swap;
use crate::jupiter::repo::ReadSwapRepo;
use crate::model::{Signature, Slot};
use base::model::{AddressId, DecimalAmount, SwapId, TokenPairId};
use common::model::{PriceQuote, Timestamp};
use common::repo::{RepoResult, Tx};
use sqlx::Row;

impl ReadSwapRepo {
    pub async fn list<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<Vec<Swap>> {
        Ok(sqlx::query("select * from jupiter.swap;")
            .fetch_all(&mut **tx)
            .await?
            .iter()
            .map(|r| Swap {
                id: r.get::<SwapId, _>("id"),
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

    pub async fn list_micro<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<Vec<Swap>> {
        Ok(sqlx::query("select * from jupiter.micro_swap;")
            .fetch_all(&mut **tx)
            .await?
            .iter()
            .map(|r| Swap {
                id: r.get::<SwapId, _>("id"),
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
    ) -> RepoResult<Vec<Swap>> {
        Ok(
            sqlx::query("select * from jupiter.swap where signature = $1 order by id;")
                .bind(signature.into())
                .fetch_all(&mut **tx)
                .await?
                .iter()
                .map(|r| Swap {
                    id: r.get::<SwapId, _>("id"),
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

    pub async fn list_micro_with_signature<'a>(
        &self,
        tx: &mut Tx<'a>,
        signature: impl Into<Signature>,
    ) -> RepoResult<Vec<Swap>> {
        Ok(
            sqlx::query("select * from jupiter.micro_swap where signature = $1 order by id;")
                .bind(signature.into())
                .fetch_all(&mut **tx)
                .await?
                .iter()
                .map(|r| Swap {
                    id: r.get::<SwapId, _>("id"),
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
