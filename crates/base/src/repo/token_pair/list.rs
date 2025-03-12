// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{
    AddressId, DecimalAmount, Decimals, Description, Mint, Name, Symbol, Token, TokenId, TokenPair,
    TokenPairId, Uri,
};
use crate::repo::token_pair::shared::{find_missing_ids, find_missing_mints};
use crate::repo::token_pair::CachedTokenPair;
use crate::repo::TokenPairRepo;
use common::model::{BlockId, BlockTime};
use common::repo::{RepoResult, Tx};
use sqlx::Row;

impl TokenPairRepo {
    pub async fn list_all<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<Vec<TokenPair>> {
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
    base.creator_id as base_creator_id,
    base.block_id as base_block_id,
    base.block_time as base_block_time,
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
    quote.block_id as quopte_block_id,
    quote.block_time as quote_block_time
from solana.token_pair tp
join solana.token base on tp.base_id = base.id
join solana.token quote on tp.quote_id = quote.id
"#,
        )
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
                block_time: r.try_get::<BlockTime, _>("base_block_time").ok(),
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
                block_time: r.try_get::<BlockTime, _>("quote_block_time").ok(),
            },
        })
        .collect::<Vec<_>>())
    }

    pub async fn list_by_ids<'a>(
        &self,
        tx: &mut Tx<'a>,
        ids: impl IntoIterator<Item = impl Into<TokenPairId>> + Send,
    ) -> RepoResult<Vec<TokenPair>> {
        let ids = ids.into_iter().map(|id| id.into()).collect::<Vec<_>>();
        let mut result = self.read_token_pair_ids_from_cache(tx, &ids).await?;

        let to_read = find_missing_ids(&ids, &result);
        let mut read = self.read_token_pair_ids_from_db(tx, &to_read).await?;

        self.cache
            .put_all(read.iter().map(|pair| {
                (
                    pair.id,
                    (pair.base.mint.clone(), pair.quote.mint.clone()),
                    CachedTokenPair {
                        id: pair.id,
                        base_id: pair.base.id,
                        quote_id: pair.quote.id,
                    },
                )
            }))
            .await;

        result.append(&mut read);

        Ok(result)
    }

    pub async fn list_by_mints<'a>(
        &self,
        tx: &mut Tx<'a>,
        mints: impl IntoIterator<Item = (impl Into<Mint>, impl Into<Mint>)> + Send,
    ) -> RepoResult<Vec<TokenPair>> {
        let mints = mints
            .into_iter()
            .map(|mint| (mint.0.into(), mint.1.into()))
            .collect::<Vec<_>>();
        let mut result = self.read_token_pair_mints_from_cache(tx, &mints).await?;

        let to_read = find_missing_mints(&mints, &result);
        let mut read = self.read_token_pair_mints_from_db(tx, &to_read).await?;

        self.cache
            .put_all(read.iter().map(|pair| {
                (
                    pair.id,
                    (pair.base.mint.clone(), pair.quote.mint.clone()),
                    CachedTokenPair {
                        id: pair.id,
                        base_id: pair.base.id,
                        quote_id: pair.quote.id,
                    },
                )
            }))
            .await;

        result.append(&mut read);

        Ok(result)
    }
}
