// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{
    DecimalAmount, Decimals, Description, Mint, Name, Symbol, Token, TokenId, TokenPair,
    TokenPairId, TokenPairMint, Uri,
};
use crate::repo::ReadTokenPairRepo;
use common::repo::{RepoResult, Tx};
use sqlx::Row;
use std::collections::{HashMap, HashSet};
use std::vec;

pub fn find_missing_mints(
    mints: &[TokenPairMint],
    token_pairs: &[TokenPair],
) -> Vec<TokenPairMint> {
    let pair_mints = token_pairs
        .iter()
        .map(|pair| (pair.base.mint.clone(), pair.quote.mint.clone()))
        .collect::<HashSet<_>>();

    let mut result: Vec<TokenPairMint> = Vec::with_capacity(mints.len() - token_pairs.len());
    for mint in mints {
        if !pair_mints.contains(mint) {
            result.push(mint.clone());
        }
    }

    result
}

pub(crate) fn find_missing_ids(ids: &[TokenPairId], token_pairs: &[TokenPair]) -> Vec<TokenPairId> {
    let token_ids = token_pairs.iter().map(|w| w.id).collect::<HashSet<_>>();

    let mut result: Vec<TokenPairId> = Vec::with_capacity(ids.len() - token_pairs.len());
    for id in ids {
        if !token_ids.contains(id) {
            result.push(*id);
        }
    }

    result
}

impl ReadTokenPairRepo {
    pub(crate) async fn read_token_pair_ids_from_cache<'a>(
        &self,
        tx: &mut Tx<'a>,
        ids: &[TokenPairId],
    ) -> RepoResult<Vec<TokenPair>> {
        let mut result = Vec::with_capacity(ids.len());

        for id in ids {
            if let Some(token) = self.cache.get_by_id(*id).await {
                result.push(token)
            }
        }

        let mut all_ids = Vec::with_capacity(result.len() * 2);
        for pair in &result {
            all_ids.push(pair.base_id);
            all_ids.push(pair.quote_id);
        }

        let tokens = self.token_repo.list_by_ids(tx, all_ids).await?;
        let tokens: HashMap<TokenId, Token> = tokens.into_iter().map(|t| (t.id, t)).collect();

        Ok(result
            .into_iter()
            .map(|pair| TokenPair {
                id: pair.id,
                base: tokens[&pair.base_id].clone(),
                quote: tokens[&pair.quote_id].clone(),
            })
            .collect())
    }

    pub async fn read_token_pair_mints_from_cache<'a>(
        &self,
        tx: &mut Tx<'a>,
        mints: &[TokenPairMint],
    ) -> RepoResult<Vec<TokenPair>> {
        let mut result = Vec::with_capacity(mints.len());
        for mint in mints {
            if let Some(token) = self.cache.get_by_key(mint.clone()).await {
                result.push(token)
            }
        }

        let mut all_ids = Vec::with_capacity(result.len() * 2);
        for pair in &result {
            all_ids.push(pair.base_id);
            all_ids.push(pair.quote_id);
        }

        let tokens = self.token_repo.list_by_ids(tx, all_ids).await?;
        let tokens: HashMap<TokenId, Token> = tokens.into_iter().map(|t| (t.id, t)).collect();

        Ok(result
            .into_iter()
            .map(|pair| TokenPair {
                id: pair.id,
                base: tokens[&pair.base_id].clone(),
                quote: tokens[&pair.quote_id].clone(),
            })
            .collect())
    }

    pub async fn read_token_pair_mints_from_db<'a>(
        &self,
        tx: &mut Tx<'a>,
        mints: &[TokenPairMint],
    ) -> RepoResult<Vec<TokenPair>> {
        if mints.is_empty() {
            return Ok(vec![]);
        }

        let mut base_mints = Vec::with_capacity(mints.len());
        let mut quote_mints = Vec::with_capacity(mints.len());

        for (base, quote) in mints {
            base_mints.push(base.clone());
            quote_mints.push(quote.clone());
        }

        Ok(sqlx::query(
            r#"
with base_token as (
    select * from solana.token 
    where mint = any($1::text[])
),
quote_token as (
    select * from solana.token 
    where mint = any($2::text[])
),
input_pairs as (
    select unnest($1::text[]) as base_mint, unnest($2::text[]) as quote_mint
)
select
    tp.id as id,
    base.id as base_id,
    base.mint as base_mint,
    base.name as base_name,
    base.symbol as base_symbol,
    base.decimals as base_decimals,
    base.supply as base_supply,
    base.metadata as base_metadata,
    base.description as base_description,
    base.image as base_image,
    base.website as base_website,
    quote.id as quote_id,
    quote.mint as quote_mint,
    quote.name as quote_name,
    quote.symbol as quote_symbol,
    quote.decimals as quote_decimals,
    quote.supply as quote_supply,
    quote.metadata as quote_metadata,
    quote.description as quote_description,
    quote.image as quote_image,
    quote.website as quote_website
from solana.token_pair tp
join base_token base on tp.base_id = base.id
join quote_token quote on tp.quote_id = quote.id
join input_pairs ip on base.mint = ip.base_mint and quote.mint = ip.quote_mint;
"#,
        )
        .bind(base_mints)
        .bind(quote_mints)
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(|r| TokenPair {
            id: r.get::<TokenPairId, _>("id"),
            base: Token {
                id: r.get::<TokenId, _>("base_id"),
                mint: r.get::<Mint, _>("base_mint"),
                name: r.try_get::<Name, _>("base_name").ok(),
                symbol: r.try_get::<Symbol, _>("base_symbol").ok(),
                decimals: r.get::<Decimals, _>("base_decimals"),
                supply: r.try_get::<DecimalAmount, _>("base_supply").ok(),
                description: r.try_get::<Description, _>("base_description").ok(),
                metadata: r.try_get::<Uri, _>("base_metadata").ok(),
                image: r.try_get::<Uri, _>("base_image").ok(),
                website: r.try_get::<Uri, _>("base_website").ok(),
            },
            quote: Token {
                id: r.get::<TokenId, _>("quote_id"),
                mint: r.get::<Mint, _>("quote_mint"),
                name: r.try_get::<Name, _>("quote_name").ok(),
                symbol: r.try_get::<Symbol, _>("quote_symbol").ok(),
                decimals: r.get::<Decimals, _>("quote_decimals"),
                supply: r.try_get::<DecimalAmount, _>("quote_supply").ok(),
                description: r.try_get::<Description, _>("quote_description").ok(),
                metadata: r.try_get::<Uri, _>("quote_metadata").ok(),
                image: r.try_get::<Uri, _>("quote_image").ok(),
                website: r.try_get::<Uri, _>("quote_website").ok(),
            },
        })
        .collect::<Vec<_>>())
    }

    pub(crate) async fn read_token_pair_ids_from_db<'a>(
        &self,
        tx: &mut Tx<'a>,
        ids: &[TokenPairId],
    ) -> RepoResult<Vec<TokenPair>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        Ok(sqlx::query(
            r#"
select
    tp.id as id,
    base.id as base_id,
    base.mint as base_mint,
    base.name as base_name,
    base.symbol as base_symbol,
    base.decimals as base_decimals,
    base.supply as base_supply,
    base.metadata as base_metadata,
    base.description as base_description,
    base.image as base_image,
    base.website as base_website,
    quote.id as quote_id,
    quote.mint as quote_mint,
    quote.name as quote_name,
    quote.symbol as quote_symbol,
    quote.decimals as quote_decimals,
    quote.supply as quote_supply,
    quote.metadata as quote_metadata,
    quote.description as quote_description,
    quote.image as quote_image,
    quote.website as quote_website
from solana.token_pair tp
join solana.token base on tp.base_id = base.id
join solana.token quote on tp.quote_id = quote.id
where tp.id in (select unnest($1::int8[]));
"#,
        )
        .bind(ids)
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(|r| TokenPair {
            id: r.get::<TokenPairId, _>("id"),
            base: Token {
                id: r.get::<TokenId, _>("base_id"),
                mint: r.get::<Mint, _>("base_mint"),
                name: r.try_get::<Name, _>("base_name").ok(),
                symbol: r.try_get::<Symbol, _>("base_symbol").ok(),
                decimals: r.get::<Decimals, _>("base_decimals"),
                supply: r.try_get::<DecimalAmount, _>("base_supply").ok(),
                description: r.try_get::<Description, _>("base_description").ok(),
                metadata: r.try_get::<Uri, _>("base_metadata").ok(),
                image: r.try_get::<Uri, _>("base_image").ok(),
                website: r.try_get::<Uri, _>("base_website").ok(),
            },
            quote: Token {
                id: r.get::<TokenId, _>("quote_id"),
                mint: r.get::<Mint, _>("quote_mint"),
                name: r.try_get::<Name, _>("quote_name").ok(),
                symbol: r.try_get::<Symbol, _>("quote_symbol").ok(),
                decimals: r.get::<Decimals, _>("quote_decimals"),
                supply: r.try_get::<DecimalAmount, _>("quote_supply").ok(),
                description: r.try_get::<Description, _>("quote_description").ok(),
                metadata: r.try_get::<Uri, _>("quote_metadata").ok(),
                image: r.try_get::<Uri, _>("quote_image").ok(),
                website: r.try_get::<Uri, _>("quote_website").ok(),
            },
        })
        .collect::<Vec<_>>())
    }
}
