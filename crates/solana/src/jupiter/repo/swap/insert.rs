// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::jupiter::model::Swap;
use crate::jupiter::repo::SwapRepo;
use crate::model::{Signature, Slot};
use base::model::{
    determine_mints, AddressId, Amount, DecimalAmount, Mint, PublicKey, SwapId, Token, TokenPair,
    TokenPairId, TokenPairMint,
};
use bigdecimal::{BigDecimal, Zero};
use common::model::{PriceQuote, Timestamp};
use common::repo::{RepoResult, Tx};
use log::trace;
use sqlx::Row;
use std::collections::HashMap;

pub struct SlotSwaps {
    pub slot: Slot,
    pub timestamp: Timestamp,
    pub swaps: Vec<SlotSwap>,
}

#[derive(Debug)]
pub struct SlotSwap {
    pub input_mint: Mint,
    pub input_amount: Amount,
    pub output_mint: Mint,
    pub output_amount: Amount,
    pub wallet: PublicKey,
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

        let addresses = slot
            .swaps
            .iter()
            .map(|swap| swap.wallet.clone())
            .collect::<Vec<_>>();

        let keys: HashMap<PublicKey, AddressId> = self
            .address_repo
            .list_or_populate_by_keys(tx, addresses)
            .await?
            .into_iter()
            .map(|address| (address.address, address.id))
            .collect();

        let mut token_pairs = Vec::with_capacity(len);
        for swap in &slot.swaps {
            if let Some((base_mint, quote_mint)) =
                determine_mints(swap.input_mint.clone(), swap.output_mint.clone())
            {
                let pair = (base_mint, quote_mint);
                if !token_pairs.contains(&pair) {
                    token_pairs.push(pair);
                }
            } else {
                // warn!(
                //     "unable to determine mints for {} and {}",
                //     swap.input_mint, swap.output_mint
                // )
            }
        }

        let token_pairs: HashMap<TokenPairMint, TokenPair> = self
            .token_pair_repo
            .list_or_populate(tx, token_pairs)
            .await?
            .into_iter()
            .map(|p| ((p.base.mint.clone(), p.quote.mint.clone()), p))
            .collect();

        let mut slots = Vec::with_capacity(len);
        let mut address_ids = Vec::with_capacity(len);
        let mut token_pair_ids = Vec::with_capacity(len);
        let mut amount_bases = Vec::with_capacity(len);
        let mut amount_quotes = Vec::with_capacity(len);
        let mut prices = Vec::with_capacity(len);
        let mut is_buys = Vec::with_capacity(len);
        let mut timestamps = Vec::with_capacity(len);
        let mut signatures = Vec::with_capacity(len);

        for swap in slot.swaps {
            if let Some((base_mint, quote_mint)) =
                determine_mints(swap.input_mint.clone(), swap.output_mint.clone())
            {
                let token_pair = token_pairs.get(&(base_mint, quote_mint)).unwrap();

                let (price, amount_base, amount_quote, is_buy) =
                    calculate_amount_and_side(&swap, &token_pair.base, &token_pair.quote);

                assert!(amount_base > 0, "base amount required");
                assert!(amount_quote > 0, "quote amount required");

                slots.push(slot.slot);
                address_ids.push(keys.get(&swap.wallet).unwrap());
                token_pair_ids.push(token_pair.id);
                amount_bases.push(amount_base);
                amount_quotes.push(amount_quote);
                prices.push(price);
                is_buys.push(is_buy);
                timestamps.push(slot.timestamp);
                signatures.push(swap.signature);
            }
        }

        let rows = sqlx::query(
                r#"
                insert into jupiter.swap (slot, address_id, token_pair_id, amount_base, amount_quote, price, is_buy, timestamp, signature)
                select
                    unnest($1::int8[]) as slot,
                    unnest($2::int8[]) as address_id,
                    unnest($3::int8[]) as token_pair_id,
                    unnest($4::numeric(36, 12)[]) as amount_base,
                    unnest($5::numeric(36, 12)[]) as amount_quote,
                    unnest($6::numeric(36, 12)[]) as price,
                    unnest($7::boolean[]) as is_buy,
                    unnest($8::timestamptz[]) as timestamp,
                    unnest($9::text[]) as signature
on conflict (token_pair_id,signature) do nothing
returning id, slot, address_id, token_pair_id, amount_base, amount_quote, price, is_buy, timestamp, signature;
            "#,
            )
            .bind(&slots)
            .bind(&address_ids)
            .bind(&token_pair_ids)
            .bind(&amount_bases)
            .bind(&amount_quotes)
            .bind(&prices)
            .bind(&is_buys)
            .bind(&timestamps)
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
                signature: r.get::<Signature, _>("signature"),
            })
            .collect::<Vec<_>>();

        trace!("inserted {} swaps", inserted_swaps.len());

        Ok(inserted_swaps)
    }
}

fn calculate_amount_and_side(
    swap: &SlotSwap,
    base_token: &Token,
    quote_token: &Token,
) -> (PriceQuote, DecimalAmount, DecimalAmount, bool) {
    let input_decimals = if swap.input_mint == base_token.mint {
        &base_token.decimals
    } else {
        &quote_token.decimals
    };
    let output_decimals = if swap.output_mint == base_token.mint {
        &base_token.decimals
    } else {
        &quote_token.decimals
    };

    let input_amount = DecimalAmount::new(swap.input_amount, input_decimals.clone());
    let output_amount = DecimalAmount::new(swap.output_amount, output_decimals.clone());

    let amount_base = if swap.input_mint == base_token.mint {
        input_amount.clone()
    } else {
        output_amount.clone()
    };

    let amount_quote = if swap.output_mint == base_token.mint {
        input_amount.clone()
    } else {
        output_amount.clone()
    };

    if swap.input_mint == base_token.mint {
        assert!(
            input_amount > BigDecimal::zero(),
            "Input amount must not be 0"
        );
        (
            PriceQuote(output_amount.0 / input_amount.0),
            amount_base,
            amount_quote,
            false,
        )
    } else {
        assert!(
            output_amount > BigDecimal::zero(),
            "Output amount must not be 0"
        );
        (
            PriceQuote(input_amount.0 / output_amount.0),
            amount_base,
            amount_quote,
            true,
        )
    }
}
