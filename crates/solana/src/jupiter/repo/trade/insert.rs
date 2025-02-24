// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{Signature, Slot};
use crate::jupiter::repo::TradeRepo;
use base::model::{determine_mints, AddressId, Amount, DecimalAmount, PriceAvgQuote, PublicKey, Token, TokenMint, TokenPair, TokenPairMint};
use base::LoadTokenInfo;
use common::model::{Count, Timestamp};
use common::repo::{RepoResult, Tx};
use log::trace;
use std::collections::HashMap;

pub struct SlotTrades {
    pub slot: Slot,
    pub timestamp: Timestamp,
    pub trades: Vec<SlotTrade>,
}

#[derive(Debug)]
pub struct SlotTrade {
    pub input_mint: TokenMint,
    pub input_amount: Amount,
    pub output_mint: TokenMint,
    pub output_amount: Amount,
    pub wallet: PublicKey,
    pub signature: Signature,
}

impl<L: LoadTokenInfo> TradeRepo<L> {
    pub async fn insert_trades<'a>(&self, tx: &mut Tx<'a>, slot: SlotTrades) -> RepoResult<Count> {
        if slot.trades.len() == 0 {
            return Ok(Count(0));
        }

        trace!("most likely inserts {} trades", slot.trades.len());

        let len = slot.trades.len();

        let addresses = slot.trades.iter().map(|trade| trade.wallet.clone()).collect::<Vec<_>>();

        let keys: HashMap<PublicKey, AddressId> = self
            .address_repo
            .list_or_populate_by_keys(tx, addresses)
            .await?
            .into_iter()
            .map(|address| (address.address, address.id))
            .collect();

        let mut token_pairs = Vec::with_capacity(len);
        for trade in &slot.trades {
            if let Some((base_mint, quote_mint)) = determine_mints(trade.input_mint.clone(), trade.output_mint.clone()) {
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
        let mut amounts = Vec::with_capacity(len);
        let mut prices = Vec::with_capacity(len);
        let mut is_buys = Vec::with_capacity(len);
        let mut timestamps = Vec::with_capacity(len);
        let mut signatures = Vec::with_capacity(len);

        for trade in slot.trades {
            if let Some((base_mint, quote_mint)) = determine_mints(trade.input_mint.clone(), trade.output_mint.clone()) {
                let token_pair = token_pairs.get(&(base_mint, quote_mint)).unwrap();

                let (price, amount, is_buy) = calculate_price_amount_and_side(&trade, &token_pair.base, &token_pair.base);
                slots.push(slot.slot);
                address_ids.push(keys.get(&trade.wallet).unwrap());
                token_pair_ids.push(token_pair.id);
                amounts.push(amount);
                prices.push(price);
                is_buys.push(is_buy);
                timestamps.push(slot.timestamp);
                signatures.push(trade.signature);
            }
        }

        let result = Count(
            sqlx::query(
                r#"
                insert into jupiter.trade (slot, address_id, token_pair_id, amount, price, is_buy, timestamp, signature)
                select
                    unnest($1::bigint[]) as slot,
                    unnest($2::int[]) as address_id,
                    unnest($3::int[]) as token_pair_id,
                    unnest($4::double precision[]) as amount,
                    unnest($5::double precision[]) as price,
                    unnest($6::boolean[]) as is_buy,
                    unnest($7::timestamptz[]) as timestamp,
                    unnest($8::text[]) as signature
            "#,
            )
            .bind(&slots)
            .bind(&address_ids)
            .bind(&token_pair_ids)
            .bind(&amounts)
            .bind(&prices)
            .bind(&is_buys)
            .bind(&timestamps)
            .bind(&signatures)
            .execute(&mut **tx)
            .await?
            .rows_affected() as i64,
        );

        trace!("inserted {} trades", result);

        Ok(result)
    }
}

fn calculate_price_amount_and_side(trade: &SlotTrade, base_token: &Token, quote_token: &Token) -> (PriceAvgQuote, DecimalAmount, bool) {
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

    let input_amount = DecimalAmount::new(trade.input_amount.clone(), input_decimals.clone());
    let output_amount = DecimalAmount::new(trade.output_amount.clone(), output_decimals.clone());

    if trade.input_mint == base_token.mint {
        assert!(input_amount > 0.0, "Input amount must not be 0");
        (PriceAvgQuote(output_amount.0 as f64 / input_amount.0 as f64), input_amount, false)
    } else {
        assert!(output_amount > 0.0, "Output amount must not be 0");
        (PriceAvgQuote(input_amount.0 as f64 / output_amount.0 as f64), output_amount, true)
    }
}
