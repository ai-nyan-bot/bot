// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{Decimals, Description, Mint, Name, Supply, Symbol, Token, TokenId, Uri};
use crate::repo::cache::Cache;
use crate::repo::ReadTokenRepo;
use common::repo::{RepoResult, Tx};
use sqlx::Row;
use std::collections::HashSet;

pub fn find_missing_mints(mints: &[Mint], tokens: &[Token]) -> Vec<Mint> {
    let token_mints = tokens
        .iter()
        .map(|w| w.mint.clone())
        .collect::<HashSet<_>>();

    let mut result: Vec<Mint> = Vec::with_capacity(mints.len() - tokens.len());
    for mint in mints {
        if !token_mints.contains(mint) {
            result.push(mint.clone());
        }
    }

    result
}

pub fn find_missing_ids(ids: &[TokenId], tokens: &[Token]) -> Vec<TokenId> {
    let token_ids = tokens.iter().map(|w| w.id).collect::<HashSet<_>>();

    let mut result: Vec<TokenId> = Vec::with_capacity(ids.len() - tokens.len());
    for id in ids {
        if !token_ids.contains(id) {
            result.push(*id);
        }
    }

    result
}

impl ReadTokenRepo {
    pub async fn read_token_ids_from_cache(
        &self,
        cache: &Cache<TokenId, Mint, Token>,
        ids: &[TokenId],
    ) -> RepoResult<Vec<Token>> {
        let mut result = Vec::with_capacity(ids.len());

        for id in ids {
            if let Some(token) = cache.get_by_id(*id).await {
                result.push(token)
            }
        }

        Ok(result)
    }

    pub async fn read_token_mints_from_cache(&self, mints: &[Mint]) -> RepoResult<Vec<Token>> {
        let mut result = Vec::with_capacity(mints.len());

        for mint in mints {
            if let Some(token) = self.cache.get_by_key(mint.clone()).await {
                result.push(token)
            }
        }

        Ok(result)
    }

    pub async fn read_token_mints_from_db<'a>(
        &self,
        tx: &mut Tx<'a>,
        mints: &[Mint],
    ) -> RepoResult<Vec<Token>> {
        if mints.is_empty() {
            return Ok(vec![]);
        }

        Ok(sqlx::query(
            r#"select
                id,
                mint,
                name,
                symbol,
                decimals
              from solana.token
              where mint in (select unnest($1::varchar[]))"#,
        )
        .bind(mints)
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(|r| Token {
            id: r.get::<TokenId, _>("id"),
            mint: r.get::<Mint, _>("mint"),
            name: r.get::<Name, _>("name"),
            symbol: r.get::<Symbol, _>("symbol"),
            decimals: r.get::<Decimals, _>("decimals"),
            supply: r.get::<Supply, _>("supply"),
            description: r.try_get::<Description, _>("description").ok(),
            metadata: r.try_get::<Uri, _>("metadata").ok(),
            image: r.try_get::<Uri, _>("image").ok(),
            website: r.try_get::<Uri, _>("website").ok(),
        })
        .collect::<Vec<_>>())
    }

    pub async fn read_token_ids_from_db<'a>(
        &self,
        tx: &mut Tx<'a>,
        ids: &[TokenId],
    ) -> RepoResult<Vec<Token>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        Ok(sqlx::query(
            r#"select
                id,
                mint,
                name,
                symbol,
                decimals
              from solana.token
              where id in (select unnest($1::int4[]))"#,
        )
        .bind(ids)
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(|r| Token {
            id: r.get::<TokenId, _>("id"),
            mint: r.get::<Mint, _>("mint"),
            name: r.get::<Name, _>("name"),
            symbol: r.get::<Symbol, _>("symbol"),
            decimals: r.get::<Decimals, _>("decimals"),
            supply: r.get::<Supply, _>("supply"),
            description: r.try_get::<Description, _>("description").ok(),
            metadata: r.try_get::<Uri, _>("metadata").ok(),
            image: r.try_get::<Uri, _>("image").ok(),
            website: r.try_get::<Uri, _>("website").ok(),
        })
        .collect::<Vec<_>>())
    }
}
