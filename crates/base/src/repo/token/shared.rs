// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original MIT License Copyright (c) blockworks-foundation 2024.

use crate::repo::ReadTokenRepo;
use common::model::{Decimals, Token, TokenId, TokenMint, TokenName, TokenSymbol};
use common::repo::cache::Cache;
use common::repo::{RepoResult, Tx};
use sqlx::Row;
use std::collections::HashSet;

pub fn find_missing_mints(mints: &[TokenMint], tokens: &[Token]) -> Vec<TokenMint> {
    let token_mints = tokens.iter().map(|w| w.mint.clone()).collect::<HashSet<_>>();

    let mut result: Vec<TokenMint> = Vec::with_capacity(mints.len() - tokens.len());
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
    pub async fn read_token_ids_from_cache(&self, cache: &Cache<TokenId, TokenMint, Token>, ids: &[TokenId]) -> RepoResult<Vec<Token>> {
        let mut result = Vec::with_capacity(ids.len());

        for id in ids {
            if let Some(token) = cache.get_by_id(id.clone()).await {
                result.push(token)
            }
        }

        Ok(result)
    }

    pub async fn read_token_mints_from_cache(&self, mints: &[TokenMint]) -> RepoResult<Vec<Token>> {
        let mut result = Vec::with_capacity(mints.len());

        for mint in mints {
            if let Some(token) = self.cache.get_by_key(mint.clone()).await {
                result.push(token)
            }
        }

        Ok(result)
    }

    pub async fn read_token_mints_from_db<'a>(&self, tx: &mut Tx<'a>, mints: &[TokenMint]) -> RepoResult<Vec<Token>> {
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
        .bind(&mints)
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(|r| Token {
            id: r.get::<TokenId, _>("id"),
            mint: r.get::<TokenMint, _>("mint"),
            name: r.get::<TokenName, _>("name"),
            symbol: r.get::<TokenSymbol, _>("symbol"),
            decimals: r.get::<Decimals, _>("decimals"),
        })
        .collect::<Vec<_>>())
    }

    pub async fn read_token_ids_from_db<'a>(&self, tx: &mut Tx<'a>, ids: &[TokenId]) -> RepoResult<Vec<Token>> {
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
        .bind(&ids)
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(|r| Token {
            id: r.get::<TokenId, _>("id"),
            mint: r.get::<TokenMint, _>("mint"),
            name: r.get::<TokenName, _>("name"),
            symbol: r.get::<TokenSymbol, _>("symbol"),
            decimals: r.get::<Decimals, _>("decimals"),
        })
        .collect::<Vec<_>>())
    }
}
