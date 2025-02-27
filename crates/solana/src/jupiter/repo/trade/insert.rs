// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::jupiter::model::Trade;
use crate::jupiter::repo::TradeRepo;
use crate::model::{Signature, Slot};
use base::model::{
    determine_mints, AddressId, Amount, DecimalAmount, Mint, PriceAvgQuote, PriceQuote, PublicKey,
    Token, TokenPair, TokenPairId, TokenPairMint,
};
use base::LoadTokenInfo;
use common::model::Timestamp;
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
    pub input_mint: Mint,
    pub input_amount: Amount,
    pub output_mint: Mint,
    pub output_amount: Amount,
    pub wallet: PublicKey,
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

        let addresses = slot
            .trades
            .iter()
            .map(|trade| trade.wallet.clone())
            .collect::<Vec<_>>();

        let keys: HashMap<PublicKey, AddressId> = self
            .address_repo
            .list_or_populate_by_keys(tx, addresses)
            .await?
            .into_iter()
            .map(|address| (address.address, address.id))
            .collect();

        let mut token_pairs = Vec::with_capacity(len);
        for trade in &slot.trades {
            if let Some((base_mint, quote_mint)) =
                determine_mints(trade.input_mint.clone(), trade.output_mint.clone())
            {
                let pair = (base_mint, quote_mint);
                if !token_pairs.contains(&pair) {
                    token_pairs.push(pair);
                }
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
        let mut base_amounts = Vec::with_capacity(len);
        let mut quote_amounts = Vec::with_capacity(len);
        let mut prices = Vec::with_capacity(len);
        let mut is_buys = Vec::with_capacity(len);
        let mut timestamps = Vec::with_capacity(len);
        let mut signatures = Vec::with_capacity(len);

        for trade in slot.trades {
            if let Some((base_mint, quote_mint)) =
                determine_mints(trade.input_mint.clone(), trade.output_mint.clone())
            {
                let token_pair = token_pairs.get(&(base_mint, quote_mint)).unwrap();

                let (price, base_amount, quote_amount, is_buy) =
                    calculate_price_amount_and_side(&trade, &token_pair.base, &token_pair.quote);

                slots.push(slot.slot);
                address_ids.push(keys.get(&trade.wallet).unwrap());
                token_pair_ids.push(token_pair.id);
                base_amounts.push(base_amount);
                quote_amounts.push(quote_amount);
                prices.push(price);
                is_buys.push(is_buy);
                timestamps.push(slot.timestamp);
                signatures.push(trade.signature);
            }
        }

        let rows = sqlx::query(
                r#"
                insert into jupiter.trade (slot, address_id, token_pair_id, base_amount, quote_amount, price, is_buy, timestamp, signature)
                select
                    unnest($1::int8[]) as slot,
                    unnest($2::int4[]) as address_id,
                    unnest($3::int4[]) as token_pair_id,
                    unnest($4::double precision[]) as base_amount,
                    unnest($5::double precision[]) as quote_amount,
                    unnest($6::double precision[]) as price,
                    unnest($7::boolean[]) as is_buy,
                    unnest($8::timestamptz[]) as timestamp,
                    unnest($9::text[]) as signature
on conflict (token_pair_id,signature) do nothing
returning slot, address_id, token_pair_id, base_amount, quote_amount, price, is_buy, timestamp;
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
            })
            .collect::<Vec<_>>();

        trace!("inserted {} trades", inserted_trades.len());

        Ok(inserted_trades)
    }
}

fn calculate_price_amount_and_side(
    trade: &SlotTrade,
    base_token: &Token,
    quote_token: &Token,
) -> (PriceAvgQuote, DecimalAmount, DecimalAmount, bool) {
    let input_decimals = if trade.input_mint == base_token.mint {
        &base_token.decimals
    } else {
        &quote_token.decimals
    };
    let output_decimals = if trade.output_mint == base_token.mint {
        &base_token.decimals
    } else {
        &quote_token.decimals
    };

    let input_amount = DecimalAmount::new(trade.input_amount, input_decimals.clone());
    let output_amount = DecimalAmount::new(trade.output_amount, output_decimals.clone());

    let base_amount = if trade.input_mint == base_token.mint {
        input_amount.clone()
    } else {
        output_amount.clone()
    };

    let quote_amount = if trade.output_mint == base_token.mint {
        input_amount.clone()
    } else {
        output_amount.clone()
    };

    if trade.input_mint == base_token.mint {
        assert!(input_amount > 0.0, "Input amount must not be 0");
        (
            PriceAvgQuote(output_amount.0 / input_amount.0),
            base_amount,
            quote_amount,
            false,
        )
    } else {
        assert!(output_amount > 0.0, "Output amount must not be 0");
        (
            PriceAvgQuote(input_amount.0 / output_amount.0),
            base_amount,
            quote_amount,
            true,
        )
    }
}
