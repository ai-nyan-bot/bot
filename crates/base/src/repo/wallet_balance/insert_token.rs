// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::solana::Slot;
use crate::repo::balance::BalanceRepo;
use base::model::{AddressId, DecimalAmount, TokenId};
use common::model::Timestamp;
use common::repo::{RepoResult, Tx};
use std::collections::HashMap;

pub struct TokenBalanceToInsert {
    pub slot: Slot,
    pub timestamp: Timestamp,
    pub address: AddressId,
    pub token: TokenId,
    pub pre: DecimalAmount,
    pub post: DecimalAmount,
}

impl BalanceRepo {
    pub async fn insert_token_balances<'a>(
        &self,
        tx: &mut Tx<'a>,
        to_insert: impl IntoIterator<Item = TokenBalanceToInsert> + Send,
    ) -> RepoResult<()> {
        use std::collections::hash_map::Entry;

        // (first_pre, latest_post, block_id, timestamp)
        let mut aggregates: HashMap<
            (AddressId, TokenId),
            (DecimalAmount, DecimalAmount, Slot, Timestamp),
        > = HashMap::new();

        for b in to_insert {
            let key = (b.address, b.token);
            match aggregates.entry(key) {
                Entry::Occupied(mut entry) => {
                    let (_first_pre, post, block_id, ts) = entry.get_mut();
                    *post = b.post;
                    assert_eq!(*block_id, b.slot);
                    assert_eq!(*ts, b.timestamp);
                }
                Entry::Vacant(entry) => {
                    entry.insert((b.pre, b.post, b.slot, b.timestamp));
                }
            }
        }

        if aggregates.is_empty() {
            return Ok(());
        }

        let mut addresses = Vec::with_capacity(aggregates.len());
        let mut tokens = Vec::with_capacity(aggregates.len());
        let mut balances = Vec::with_capacity(aggregates.len());
        let mut deltas = Vec::with_capacity(aggregates.len());
        let mut slots = Vec::with_capacity(aggregates.len());
        let mut timestamps = Vec::with_capacity(aggregates.len());

        for ((address, token), (pre, post, block_id, timestamp)) in aggregates {
            let balance = post.clone();
            let delta = post - pre;
            addresses.push(address);
            tokens.push(token);
            balances.push(balance);
            deltas.push(delta);
            slots.push(block_id);
            timestamps.push(timestamp);
        }

        sqlx::query(
            r#"
            insert into solana.token_balance (
                address_id, token_id, balance, delta, slot, timestamp
            )
            select *
            from (
                select
                    unnest($1::int8[]) as address_id,
                    unnest($2::int8[]) as token_id,
                    unnest($3::numeric(36, 12)[]) as balance,
                    unnest($4::numeric(36, 12)[]) as delta,
                    unnest($5::int8[]) as slot,
                    unnest($6::timestamptz[]) as timestamp
            ) as rows
            where delta != 0
            on conflict (address_id, token_id, slot) do update
            set
                balance = excluded.balance,
                delta = excluded.delta
            "#,
        )
        .bind(addresses)
        .bind(tokens)
        .bind(balances)
        .bind(deltas)
        .bind(slots)
        .bind(timestamps)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
