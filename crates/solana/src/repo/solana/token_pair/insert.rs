// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original MIT License Copyright (c) blockworks-foundation 2024.

use crate::repo::solana::TokenPairRepo;
use crate::token_info::LoadTokenInfo;
use common::model::{Decimals, Token, TokenId, TokenMint, TokenName, TokenPair, TokenPairId, TokenPairMint, TokenSymbol};
use common::repo::{RepoResult, Tx};
use sqlx::Row;
use std::collections::HashMap;

impl<L: LoadTokenInfo> TokenPairRepo<L> {
    pub async fn insert_token_pairs<'a>(&self, tx: &mut Tx<'a>, mints: &[TokenPairMint]) -> RepoResult<Vec<TokenPair>> {
        if mints.is_empty() {
            return Ok(vec![]);
        }
        
        let mut token_mints = Vec::with_capacity(mints.len() * 2);
        for (base, quote) in mints {
            if !token_mints.contains(base) {
                token_mints.push(base.clone());
            }

            if !token_mints.contains(quote) {
                token_mints.push(quote.clone());
            }
        }

        let tokens: Vec<Token> = self.token_repo.list_or_populate_by_mints(tx, token_mints).await?;
        let tokens: HashMap<TokenMint, Token> = tokens.into_iter().map(|token| (token.mint.clone(), token)).collect();

        let mut base_ids = Vec::with_capacity(mints.len());
        let mut quote_ids = Vec::with_capacity(mints.len());

        let mut token_pairs = Vec::with_capacity(mints.len());
        for (base, quote) in mints {
            if !token_pairs.contains(&(base, quote)) {
                base_ids.push(tokens[base].id);
                quote_ids.push(tokens[quote].id);
                token_pairs.push((base, quote))
            }
        }

        Ok(sqlx::query(
            r#"with new_token_pairs as (
            insert into solana.token_pair (base_id,quote_id)
            select
                unnest($1::int[]) as base_id,
                unnest($2::int[]) as quote_id
            on conflict (base_id, quote_id) do update set
                base_id = excluded.base_id,
                quote_id = excluded.quote_id
            returning
                id,
                base_id,
                quote_id
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
            from new_token_pairs tp
            left join solana.token base on tp.base_id = base.id
            left join solana.token quote on tp.quote_id = quote.id
            where tp.id in (select id from new_token_pairs)
        "#,
        )
        .bind(&base_ids)
        .bind(&quote_ids)
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
}
