// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original MIT License Copyright (c) blockworks-foundation 2024.

use crate::model::PublicKey;
use crate::model::{Address, AddressId};
use crate::repo::AddressRepo;
use crate::repo::cache::Cache;
use common::repo::{RepoResult, Tx};
use sqlx::Row;
use std::collections::HashSet;

impl AddressRepo {
    pub fn find_missing_keys(&self, keys: &[PublicKey], addresses: &[Address]) -> Vec<PublicKey> {
        let wallet_addresses = addresses.iter().map(|w| w.address.clone()).collect::<HashSet<_>>();

        let mut result: Vec<PublicKey> = Vec::with_capacity(keys.len() - addresses.len());
        for key in keys {
            if !wallet_addresses.contains(key) {
                result.push(key.clone());
            }
        }

        result
    }

    pub fn find_missing_ids(&self, ids: &[AddressId], addresses: &[Address]) -> Vec<AddressId> {
        let address_ids = addresses.iter().map(|w| w.id).collect::<HashSet<_>>();

        let mut result: Vec<AddressId> = Vec::with_capacity(ids.len() - addresses.len());
        for id in ids {
            if !address_ids.contains(id) {
                result.push(*id);
            }
        }

        result
    }

    pub async fn read_addresses_from_cache_ids(&self, cache: &Cache<AddressId, PublicKey, Address>, ids: &[AddressId]) -> RepoResult<Vec<Address>> {
        let mut result = Vec::with_capacity(ids.len());

        for id in ids {
            if let Some(wallet) = cache.get_by_id(id.clone()).await {
                result.push(wallet)
            }
        }

        Ok(result)
    }

    pub async fn read_addresses_from_cache_keys(&self, keys: &[PublicKey]) -> RepoResult<Vec<Address>> {
        let mut result = Vec::with_capacity(keys.len());

        for key in keys {
            if let Some(wallet) = self.cache.get_by_key(key.clone()).await {
                result.push(wallet)
            }
        }

        Ok(result)
    }

    pub async fn read_wallet_addresses_from_db<'a>(&self, tx: &mut Tx<'a>, keys: &[PublicKey]) -> RepoResult<Vec<Address>> {
        if keys.is_empty() {
            return Ok(vec![]);
        }

        Ok(sqlx::query(
            r#"select
                id,
                address
              from solana.address
              where address in (select unnest($1::varchar[]))"#,
        )
        .bind(&keys)
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(|r| Address {
            id: r.get::<AddressId, _>("id"),
            address: r.get::<PublicKey, _>("address"),
        })
        .collect::<Vec<_>>())
    }

    pub async fn read_address_ids_from_db<'a>(&self, tx: &mut Tx<'a>, ids: &[AddressId]) -> RepoResult<Vec<Address>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        Ok(sqlx::query(
            r#"select
                id,
                address
              from solana.address
              where id in (select unnest($1::int4[]))"#,
        )
        .bind(&ids)
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
