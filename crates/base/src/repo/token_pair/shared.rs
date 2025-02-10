// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original MIT License Copyright (c) blockworks-foundation 2024.

use crate::model::{Decimals, Token, TokenId, TokenMint, TokenName, TokenPair, TokenPairId, TokenPairMint, TokenSymbol};
use crate::repo::ReadTokenPairRepo;
use common::repo::{RepoResult, Tx};
use sqlx::Row;
use std::collections::{HashMap, HashSet};
use std::vec;

pub fn find_missing_mints(mints: &[TokenPairMint], token_pairs: &[TokenPair]) -> Vec<TokenPairMint> {
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

// pub(crate) fn find_missing_ids(&self, ids: &[TokenId], tokens: &[Token]) -> Vec<TokenId> {
// 	let token_ids = tokens.iter().map(|w| w.id).collect::<HashSet<_>>();
//
// 	let mut result: Vec<TokenId> = Vec::with_capacity(ids.len() - tokens.len());
// 	for id in ids {
// 		if !token_ids.contains(id) {
// 			result.push(*id);
// 		}
// 	}
//
// 	result
// }
//

impl ReadTokenPairRepo {
    // pub(crate) async fn read_token_ids_from_cache(&self, cache: &Cache<TokenId, TokenMint, Token>, ids: &[TokenId]) -> RepoResult<Vec<Token>> {
    // 	let mut result = Vec::with_capacity(ids.len());
    //
    // 	for id in ids {
    // 		if let Some(token) = cache.get_by_id(id.clone()).await {
    // 			result.push(token)
    // 		}
    // 	}
    //
    // 	Ok(result)
    // }
    //

    pub async fn read_token_pair_mints_from_cache<'a>(&self, tx: &mut Tx<'a>, mints: &[TokenPairMint]) -> RepoResult<Vec<TokenPair>> {
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
        let tokens: HashMap<TokenId, Token> = tokens.into_iter().map(|t| (t.id.clone(), t)).collect();

        Ok(result
            .into_iter()
            .map(|pair| TokenPair {
                id: pair.id,
                base: tokens[&pair.base_id].clone(),
                quote: tokens[&pair.quote_id].clone(),
            })
            .collect())
    }

    pub async fn read_token_pair_mints_from_db<'a>(&self, tx: &mut Tx<'a>, mints: &[TokenPairMint]) -> RepoResult<Vec<TokenPair>> {
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
    quote.id as quote_id,
    quote.mint as quote_mint,
    quote.name as quote_name,
    quote.symbol as quote_symbol,
    quote.decimals as quote_decimals
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
                mint: r.get::<TokenMint, _>("base_mint"),
                name: r.get::<TokenName, _>("base_name"),
                symbol: r.get::<TokenSymbol, _>("base_symbol"),
                decimals: r.get::<Decimals, _>("base_decimals"),
            },
            quote: Token {
                id: r.get::<TokenId, _>("quote_id"),
                mint: r.get::<TokenMint, _>("quote_mint"),
                name: r.get::<TokenName, _>("quote_name"),
                symbol: r.get::<TokenSymbol, _>("quote_symbol"),
                decimals: r.get::<Decimals, _>("quote_decimals"),
            },
        })
        .collect::<Vec<_>>())
    }

    // pub(crate) async fn read_token_ids_from_db<'a>(&self, tx: &mut Tx<'a>, ids: &[TokenId]) -> RepoResult<Vec<Token>> {
    // 	if ids.is_empty() {
    // 		return Ok(vec![]);
    // 	}
    //
    // 	Ok(sqlx::query(
    // 		r#"select
    //             id,
    //             mint,
    //             name,
    //             symbol,
    //             decimals
    //           from solana.token
    //           where id in (select unnest($1::int4[]))"#,
    // 	)
    // 		.bind(&ids)
    // 		.fetch_all(&mut **tx)
    // 		.await?
    // 		.into_iter()
    // 		.map(|r| Token {
    // 			id: r.get::<TokenId, _>("id"),
    // 			mint: r.get::<TokenMint, _>("mint"),
    // 			name: r.get::<TokenName, _>("name"),
    // 			symbol: r.get::<TokenSymbol, _>("symbol"),
    // 			decimals: r.get::<TokenDecimals, _>("decimals"),
    // 		})
    // 		.collect::<Vec<_>>())
    // }
}
