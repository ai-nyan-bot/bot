// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{
    AddressId, DecimalAmount, Decimals, Description, Mint, Name, Symbol, Token, TokenId, TokenPair,
    TokenPairId, TokenPairMint, Uri,
};
use crate::repo::TokenPairRepo;
use common::model::BlockId;
use common::repo::{RepoResult, Tx};
use sqlx::Row;
use std::collections::HashMap;

impl TokenPairRepo {
    pub async fn insert_token_pairs<'a>(
        &self,
        tx: &mut Tx<'a>,
        mints: &[TokenPairMint],
    ) -> RepoResult<Vec<TokenPair>> {
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

        let tokens: Vec<Token> = self.token_repo.list_or_populate(tx, token_mints).await?;
        let tokens: HashMap<Mint, Token> = tokens
            .into_iter()
            .map(|token| (token.mint.clone(), token))
            .collect();

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
                base.supply as base_supply,
                base.metadata as base_metadata,
                base.description as base_description,
                base.image as base_image,
                base.website as base_website,
                base.creator_id as base_creator_id,
                base.block_id as base_block_id,
                quote.id as quote_id,
                quote.mint as quote_mint,
                quote.name as quote_name,
                quote.symbol as quote_symbol,
                quote.decimals as quote_decimals,
                quote.supply as quote_supply,
                quote.metadata as quote_metadata,
                quote.description as quote_description,
                quote.image as quote_image,
                quote.website as quote_website,
                quote.creator_id as quote_creator_id,
                quote.block_id as quote_block_id
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
                mint: r.get::<Mint, _>("base_mint"),
                name: r.try_get::<Name, _>("base_name").ok(),
                symbol: r.try_get::<Symbol, _>("base_symbol").ok(),
                decimals: r.get::<Decimals, _>("base_decimals"),
                supply: r.try_get::<DecimalAmount, _>("base_supply").ok(),
                description: r.try_get::<Description, _>("base_description").ok(),
                metadata: r.try_get::<Uri, _>("base_metadata").ok(),
                image: r.try_get::<Uri, _>("base_image").ok(),
                website: r.try_get::<Uri, _>("base_website").ok(),
                creator: r.try_get::<AddressId, _>("base_creator_id").ok(),
                block: r.try_get::<BlockId, _>("base_block_id").ok(),
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
                creator: r.try_get::<AddressId, _>("quote_creator_id").ok(),
                block: r.try_get::<BlockId, _>("quote_block_id").ok(),
            },
        })
        .collect::<Vec<_>>())
    }
}
