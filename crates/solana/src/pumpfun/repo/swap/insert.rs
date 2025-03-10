// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{Signature, Slot};
use crate::pumpfun::model::{calculate_progress, Swap};
use crate::pumpfun::repo::SwapRepo;
use base::model::{AddressId, Amount, DecimalAmount, Mint, PublicKey, SwapId, TokenPairId};
use common::model::{Percent, PriceQuote, Timestamp};
use common::repo::{RepoResult, Tx};
use log::trace;
use sqlx::Row;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SlotSwaps {
    pub slot: Slot,
    pub timestamp: Timestamp,
    pub swaps: Vec<SlotSwap>,
}

#[derive(Debug, Clone)]
pub struct SlotSwap {
    pub mint: Mint,
    pub amount_base: Amount,
    pub amount_quote: Amount,
    pub is_buy: bool,
    pub wallet: PublicKey,
    pub virtual_base_reserves: Amount,
    pub virtual_quote_reserves: Amount,
    pub signature: Signature,
}

impl SwapRepo {
    pub async fn insert_swaps<'a>(
        &self,
        tx: &mut Tx<'a>,
        slot: SlotSwaps,
    ) -> RepoResult<Vec<Swap>> {
        if slot.swaps.is_empty() {
            return Ok(Vec::new());
        }

        trace!("most likely inserts {} swaps", slot.swaps.len());

        let len = slot.swaps.len();

        let keys = slot
            .swaps
            .iter()
            .map(|swap| swap.wallet.clone())
            .collect::<Vec<_>>();

        let addresses: HashMap<PublicKey, AddressId> = self
            .address_repo
            .list_or_populate_by_keys(tx, keys)
            .await?
            .into_iter()
            .map(|address| (address.address, address.id))
            .collect();

        let mut token_pairs = Vec::with_capacity(len);
        for swap in &slot.swaps {
            let pair = (swap.mint.clone(), Mint::wsol());
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
        let mut amount_bases = Vec::with_capacity(len);
        let mut amount_quotes = Vec::with_capacity(len);
        let mut prices = Vec::with_capacity(len);
        let mut is_buys = Vec::with_capacity(len);
        let mut timestamps = Vec::with_capacity(len);
        let mut quote_reserves = Vec::with_capacity(len);
        let mut base_reserves = Vec::with_capacity(len);
        let mut progresses = Vec::with_capacity(len);
        let mut signatures = Vec::with_capacity(len);

        for swap in &slot.swaps {
            let amount_base = DecimalAmount::new(swap.amount_base, 6);
            let amount_quote = DecimalAmount::new(swap.amount_quote, 9);

            assert!(amount_base > 0, "base amount required");
            assert!(amount_quote > 0, "quote amount required");

            let base_reserve = swap.virtual_base_reserves;
            let quote_reserve = swap.virtual_quote_reserves;

            let price = PriceQuote(amount_quote.0.clone() / amount_base.0.clone());

            slots.push(slot.slot);
            address_ids.push(addresses.get(&swap.wallet).unwrap());
            token_pair_ids.push(token_pairs.get(&swap.mint).unwrap());
            amount_bases.push(amount_base);
            amount_quotes.push(amount_quote);
            prices.push(price);
            is_buys.push(swap.is_buy);
            timestamps.push(slot.timestamp);
            base_reserves.push(base_reserve);
            quote_reserves.push(quote_reserve);
            progresses.push(calculate_progress(base_reserve));

            signatures.push(swap.signature.clone());
        }

        let rows = sqlx::query(
            r#"
insert into pumpfun.swap (
    slot, address_id, token_pair_id, amount_base, amount_quote, price,
    is_buy, timestamp, virtual_base_reserves, virtual_quote_reserves,progress, signature
)
select
    unnest($1::int8[]) as slot,
    unnest($2::int8[]) as address_id,
    unnest($3::int8[]) as token_pair_id,
    unnest($4::numeric(36, 12)[]) as amount_base,
    unnest($5::numeric(36, 12)[]) as amount_quote,
    unnest($6::numeric(36, 12)[]) as price,
    unnest($7::boolean[]) as is_buy,
    unnest($8::timestamptz[]) as timestamp,
    unnest($9::int8[]) as virtual_base_reserves,
    unnest($10::int8[]) as virtual_quote_reserves,
    unnest($11::real[]) as progress,
    unnest($12::text[]) as signature
on conflict (token_pair_id,signature) do nothing
returning id, slot, address_id, token_pair_id, amount_base, amount_quote, price, is_buy, timestamp, virtual_base_reserves, virtual_quote_reserves, progress, signature;  "#,
        )
        .bind(&slots)
        .bind(&address_ids)
        .bind(&token_pair_ids)
        .bind(&amount_bases)
        .bind(&amount_quotes)
        .bind(&prices)
        .bind(&is_buys)
        .bind(&timestamps)
        .bind(&base_reserves)
        .bind(&quote_reserves)
        .bind(&progresses)
        .bind(&signatures)
        .fetch_all(&mut **tx)
        .await?;

        let inserted_swaps = rows
            .into_iter()
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
                virtual_base_reserves: r.get::<Amount, _>("virtual_base_reserves"),
                virtual_quote_reserves: r.get::<Amount, _>("virtual_quote_reserves"),
                progress: r.get::<Percent, _>("progress"),
                signature: r.get::<Signature, _>("signature"),
            })
            .collect::<Vec<_>>();

        trace!("inserted {} swaps", inserted_swaps.len());

        Ok(inserted_swaps)
    }
}
