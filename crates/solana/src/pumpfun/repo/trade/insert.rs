// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{Signature, Slot};
use crate::pumpfun::model::{calculate_progress, Trade};
use crate::pumpfun::repo::TradeRepo;
use base::model::{
    AddressId, Amount, DecimalAmount, Mint, Percent, PriceQuote, PublicKey, TokenPairId,
};
use base::LoadTokenInfo;
use common::model::Timestamp;
use common::repo::{RepoResult, Tx};
use log::trace;
use sqlx::Row;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SlotTrades {
    pub slot: Slot,
    pub timestamp: Timestamp,
    pub trades: Vec<SlotTrade>,
}

#[derive(Debug, Clone)]
pub struct SlotTrade {
    pub mint: Mint,
    pub base_amount: Amount,
    pub quote_amount: Amount,
    pub is_buy: bool,
    pub wallet: PublicKey,
    pub virtual_base_reserves: Amount,
    pub virtual_quote_reserves: Amount,
    pub signature: Signature,
}

impl<L: LoadTokenInfo<Mint>> TradeRepo<L> {
    pub async fn insert_trades<'a>(
        &self,
        tx: &mut Tx<'a>,
        slot: SlotTrades,
    ) -> RepoResult<Vec<Trade>> {
        if slot.trades.is_empty() {
            return Ok(Vec::new());
        }

        trace!("most likely inserts {} trades", slot.trades.len());

        let len = slot.trades.len();

        let keys = slot
            .trades
            .iter()
            .map(|trade| trade.wallet.clone())
            .collect::<Vec<_>>();

        let addresses: HashMap<PublicKey, AddressId> = self
            .address_repo
            .list_or_populate_by_keys(tx, keys)
            .await?
            .into_iter()
            .map(|address| (address.address, address.id))
            .collect();

        let mut token_pairs = Vec::with_capacity(len);
        for trade in &slot.trades {
            let pair = (trade.mint.clone(), Mint::wsol());
            if !token_pairs.contains(&pair) {
                token_pairs.push(pair);
            }
        }

        let token_pairs: HashMap<Mint, TokenPairId> = self
            .token_pair_repo
            .list_or_populate(tx, token_pairs)
            .await?
            .into_iter()
            .map(|p| (p.base.mint, p.id))
            .collect();

        if token_pairs.is_empty() {
            return Ok(Vec::new());
        }

        let mut slots = Vec::with_capacity(len);
        let mut address_ids = Vec::with_capacity(len);
        let mut token_pair_ids = Vec::with_capacity(len);
        let mut base_amounts = Vec::with_capacity(len);
        let mut quote_amounts = Vec::with_capacity(len);
        let mut prices = Vec::with_capacity(len);
        let mut is_buys = Vec::with_capacity(len);
        let mut timestamps = Vec::with_capacity(len);
        let mut quote_reserves = Vec::with_capacity(len);
        let mut base_reserves = Vec::with_capacity(len);
        let mut progresses = Vec::with_capacity(len);
        let mut signatures = Vec::with_capacity(len);

        for trade in &slot.trades {
            let base_amount = DecimalAmount::new(trade.base_amount, 6);
            let quote_amount = DecimalAmount::new(trade.quote_amount, 9);

            let base_reserve = trade.virtual_base_reserves;
            let quote_reserve = trade.virtual_quote_reserves;

            let price = PriceQuote(quote_amount.0 / base_amount.0);

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
            progresses.push(calculate_progress(base_reserve));

            signatures.push(trade.signature.clone());
        }

        let rows = sqlx::query(
            r#"
insert into pumpfun.trade (
    slot, address_id, token_pair_id, base_amount, quote_amount, price,
    is_buy, timestamp, virtual_base_reserves, virtual_quote_reserves,progress, signature
)
select
    unnest($1::int8[]) as slot,
    unnest($2::int4[]) as address_id,
    unnest($3::int4[]) as token_pair_id,
    unnest($4::double precision[]) as base_amount,
    unnest($5::double precision[]) as quote_amount,
    unnest($6::double precision[]) as price,
    unnest($7::boolean[]) as is_buy,
    unnest($8::timestamptz[]) as timestamp,
    unnest($9::int8[]) as virtual_base_reserves,
    unnest($10::int8[]) as virtual_quote_reserves,
    unnest($11::real[]) as progress,
    unnest($12::text[]) as signature
on conflict (token_pair_id,signature) do nothing
returning slot, address_id, token_pair_id, base_amount, quote_amount, price, is_buy, timestamp, virtual_base_reserves, virtual_quote_reserves, progress;  "#,
        )
        .bind(&slots)
        .bind(&address_ids)
        .bind(&token_pair_ids)
        .bind(&base_amounts)
        .bind(&quote_amounts)
        .bind(&prices)
        .bind(&is_buys)
        .bind(&timestamps)
        .bind(&base_reserves)
        .bind(&quote_reserves)
        .bind(&progresses)
        .bind(&signatures)
        .fetch_all(&mut **tx)
        .await?;

        let inserted_trades = rows
            .into_iter()
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
            .collect::<Vec<_>>();

        trace!("inserted {} trades", inserted_trades.len());

        Ok(inserted_trades)
    }
}
