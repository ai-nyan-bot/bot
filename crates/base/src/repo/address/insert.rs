// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original MIT License Copyright (c) blockworks-foundation 2024.

use crate::model::PublicKey;
use crate::model::{Address, AddressId};
use crate::repo::AddressRepo;
use common::repo::{RepoResult, Tx};
use sqlx::Row;

impl AddressRepo {
    pub async fn insert_wallets<'a>(&self, tx: &mut Tx<'a>, keys: &[PublicKey]) -> RepoResult<Vec<Address>> {
        let mut to_insert = Vec::with_capacity(keys.len());

        for address in keys {
            if !to_insert.contains(address) {
                to_insert.push(address.clone());
            }
        }

        if to_insert.is_empty() {
            return Ok(Vec::new());
        }

        Ok(sqlx::query(
            r#"with new_wallets AS (
                    insert into solana.address (address)
                    select * from unnest($1::varchar[])
                    on conflict (address) do update
                        set address = excluded.address
                    returning
                        id,
                        address
                )
                select * from new_wallets"#,
        )
        .bind(&to_insert)
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(|r| Address {
            id: r.get::<AddressId, _>("id"),
            address: r.get::<PublicKey, _>("address"),
        })
        .collect::<Vec<_>>())
    }
}
