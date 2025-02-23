// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{Signature, Slot};
use crate::pumpfun::repo::TradeRepo;
use base::model::{AddressId, Amount, DecimalAmount, PriceAvgQuote, PublicKey, TokenMint, TokenPairId};
use base::LoadTokenInfo;
use common::model::{Count, Timestamp};
use common::repo::{RepoResult, Tx};
use log::trace;
use sqlx::Row;
use std::collections::HashMap;

pub struct SlotTrades {
    pub slot: Slot,
    pub timestamp: Timestamp,
    pub trades: Vec<SlotTrade>,
}

#[derive(Debug)]
pub struct SlotTrade {
    pub mint: TokenMint,
    pub base_amount: Amount,
    pub quote_amount: Amount,
    pub is_buy: bool,
    pub wallet: PublicKey,
    pub virtual_base_reserves: Amount,
    pub virtual_quote_reserves: Amount,
    pub signature: Signature,
}

impl<L: LoadTokenInfo> TradeRepo<L> {
    pub async fn insert_trades<'a>(&self, tx: &mut Tx<'a>, slot: SlotTrades) -> RepoResult<Count> {
        if slot.trades.len() == 0 {
            return Ok(Count(0));
        }

        trace!("most likely inserts {} trades", slot.trades.len());

        let len = slot.trades.len();

        let keys = slot.trades.iter().map(|trade| trade.wallet.clone()).collect::<Vec<_>>();

        let addresses: HashMap<PublicKey, AddressId> = self
            .address_repo
            .list_or_populate_by_keys(tx, keys)
            .await?
            .into_iter()
            .map(|address| (address.address, address.id))
            .collect();

        let mut token_pairs = Vec::with_capacity(len);
        for trade in &slot.trades {
            let pair = (trade.mint.clone(), TokenMint::wsol());
            if !token_pairs.contains(&pair) {
                token_pairs.push(pair);
            }
        }

        let token_pairs: HashMap<TokenMint, TokenPairId> = self
            .token_pair_repo
            .list_or_populate(tx, token_pairs)
            .await?
            .into_iter()
            .map(|p| (p.base.mint, p.id))
            .collect();

        let mut slots = Vec::with_capacity(len);
        let mut address_ids = Vec::with_capacity(len);
        let mut token_pair_ids = Vec::with_capacity(len);
        let mut base_amounts: Vec<DecimalAmount> = Vec::with_capacity(len);
        let mut quote_amounts: Vec<DecimalAmount> = Vec::with_capacity(len);
        let mut prices = Vec::with_capacity(len);
        let mut is_buys = Vec::with_capacity(len);
        let mut timestamps = Vec::with_capacity(len);
        let mut quote_reserves: Vec<DecimalAmount> = Vec::with_capacity(len);
        let mut base_reserves: Vec<DecimalAmount> = Vec::with_capacity(len);
        let mut signatures = Vec::with_capacity(len);

        for trade in slot.trades {
            let base_amount = DecimalAmount::new(trade.base_amount, 6);
            let quote_amount = DecimalAmount::new(trade.quote_amount, 9);

            let base_reserve = DecimalAmount::new(trade.virtual_base_reserves, 6);
            let quote_reserve = DecimalAmount::new(trade.virtual_quote_reserves, 9);

            let price = PriceAvgQuote(quote_amount.0 as f64 / base_amount.0 as f64);

            slots.push(slot.slot);
            address_ids.push(addresses.get(&trade.wallet).unwrap());
            token_pair_ids.push(token_pairs.get(&trade.mint).unwrap());
            base_amounts.push(base_amount);
            quote_amounts.push(quote_amount);
            prices.push(price);
            is_buys.push(trade.is_buy);
            timestamps.push(slot.timestamp);
            base_reserves.push(base_reserve);
            quote_reserves.push(quote_reserve);
            signatures.push(trade.signature);
        }

        let result = Count(
            sqlx::query(
                r#"
                insert into pumpfun.trade (slot, address_id, token_pair_id, base_amount, quote_amount, price, is_buy, timestamp, virtual_base_reserves, virtual_quote_reserves, signature)
                select
                    unnest($1::bigint[]) as slot,
                    unnest($2::int[]) as address_id,
                    unnest($3::int[]) as token_pair_id,
                    unnest($4::double precision[]) as base_amount,
                    unnest($5::double precision[]) as quote_amount,
                    unnest($6::double precision[]) as price,
                    unnest($7::boolean[]) as is_buy,
                    unnest($8::timestamptz[]) as timestamp,
                    unnest($9::double precision[]) as virtual_base_reserves,
                    unnest($10::double precision[]) as virtual_quote_reserves,
                    unnest($11::text[]) as signature
            "#,
            )
                .bind(&slots)
                .bind(&address_ids)
                .bind(&token_pair_ids)
                .bind(&base_amounts)
                .bind(&quote_amounts)
                .bind(&prices)
                .bind(&is_buys)
                .bind(&timestamps)
                .bind(&quote_reserves)
                .bind(&base_reserves)
                .bind(&signatures)
                .execute(&mut **tx)
                .await?
                .rows_affected() as i64
        );

        trace!("inserted {} trades", result);

        Ok(result)
    }
}
