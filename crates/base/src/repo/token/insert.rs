// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{
    AddressId, DecimalAmount, Decimals, Description, Mint, Name, Symbol, Token, TokenId, Uri,
};
use crate::repo::TokenRepo;
use common::model::{BlockId, BlockTime, Timestamp};
use common::repo::{RepoResult, Tx};
use sqlx::Row;

#[derive(Debug)]
pub struct TokenToInsert {
    pub block: Option<BlockId>,
    pub block_time: Option<BlockTime>,
    pub mint: Mint,
    pub name: Option<Name>,
    pub symbol: Option<Symbol>,
    pub decimals: Decimals,
    pub supply: Option<DecimalAmount>,
    pub metadata: Option<Uri>,
    pub description: Option<Description>,
    pub image: Option<Uri>,
    pub website: Option<Uri>,
    pub creator: Option<AddressId>,
}

impl TokenRepo {
    pub async fn insert_token<'a>(
        &self,
        tx: &mut Tx<'a>,
        to_insert: impl IntoIterator<Item = TokenToInsert> + Send,
    ) -> RepoResult<Vec<Token>> {
        let token_list = to_insert.into_iter().collect::<Vec<_>>();
        if token_list.is_empty() {
            return Ok(vec![]);
        }

        let mut mints = Vec::with_capacity(token_list.len());
        let mut names = Vec::with_capacity(token_list.len());
        let mut symbols = Vec::with_capacity(token_list.len());
        let mut decimals = Vec::with_capacity(token_list.len());
        let mut supplies: Vec<DecimalAmount> = Vec::with_capacity(token_list.len());
        let mut metadata = Vec::with_capacity(token_list.len());
        let mut descriptions = Vec::with_capacity(token_list.len());
        let mut images = Vec::with_capacity(token_list.len());
        let mut websites = Vec::with_capacity(token_list.len());
        let mut creators = Vec::with_capacity(token_list.len());
        let mut blocks = Vec::with_capacity(token_list.len());
        let mut block_times = Vec::with_capacity(token_list.len());

        for to_insert in token_list {
            mints.push(to_insert.mint);
            names.push(to_insert.name.unwrap_or("null_value".into()));
            symbols.push(to_insert.symbol.unwrap_or("null_value".into()));
            decimals.push(to_insert.decimals);

            if let Some(supply) = to_insert.supply {
                supplies.push(supply)
            } else {
                supplies.push(DecimalAmount::from(-1i64))
            }

            metadata.push(to_insert.metadata.unwrap_or("null_value".into()));
            descriptions.push(to_insert.description.unwrap_or("null_value".into()));
            images.push(to_insert.image.unwrap_or("null_value".into()));
            websites.push(to_insert.website.unwrap_or("null_value".into()));
            creators.push(to_insert.creator.unwrap_or(AddressId::from(-1)));
            blocks.push(to_insert.block.unwrap_or(BlockId::from(-1)));
            block_times.push(
                to_insert
                    .block_time
                    .unwrap_or(BlockTime(Timestamp::from_epoch_second(0).unwrap())),
            );
        }

        Ok(sqlx::query(
            r#"with new_token as (
            insert into solana.token (mint, name, symbol, decimals, supply, metadata, description, image, website, creator_id, block_id, block_time)
            select
                unnest($1::text[]) as mint,
                unnest(array_replace($2::text[], 'null_value', null)) as name,
                unnest(array_replace($3::text[], 'null_value', null)) as symbol,
                unnest($4::int2[]) as decimals,
                unnest(array_replace($5::numeric(36, 12)[], -1, null)) as supply,
                unnest(array_replace($6::text[], 'null_value', null)) as metadata,
                unnest(array_replace($7::text[], 'null_value', null)) as description,
                unnest(array_replace($8::text[], 'null_value', null)) as image,
                unnest(array_replace($9::text[], 'null_value', null)) as website,
                unnest(array_replace($10::int8[], -1, null)) as creator_id,
                unnest(array_replace($11::int8[], -1, null)) as block_id,
                unnest(array_replace($12::timestamptz[], '1970-01-01 00:00:00+00'::timestamptz, null)) as block_time
            on conflict (mint) do update set
                mint = excluded.mint,
                name = excluded.name,
                symbol = excluded.symbol,
                decimals = excluded.decimals,
                supply = excluded.supply,
                metadata = excluded.metadata,
                description = excluded.description,
                image = excluded.image,
                website = excluded.website,
                creator_id = excluded.creator_id,
                block_id = excluded.block_id,
                block_time = excluded.block_time
            returning
                id,
                mint,
                name,
                symbol,
                decimals,
                supply,
                metadata,
                description,
                image,
                website,
                creator_id,
                block_id,
                block_time
            )
            select * from new_token"#,
        )
        .bind(mints)
        .bind(names)
        .bind(symbols)
        .bind(decimals)
        .bind(supplies)
        .bind(metadata)
        .bind(descriptions)
        .bind(images)
        .bind(websites)
        .bind(creators)
        .bind(blocks)
        .bind(block_times)
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(|r| Token {
            id: r.get::<TokenId, _>("id"),
            mint: r.get::<Mint, _>("mint"),
            name: r.try_get::<Name, _>("name").ok(),
            symbol: r.try_get::<Symbol, _>("symbol").ok(),
            decimals: r.get::<Decimals, _>("decimals"),
            supply: r.try_get::<DecimalAmount, _>("supply").ok(),
            description: r.try_get::<Description, _>("description").ok(),
            metadata: r.try_get::<Uri, _>("metadata").ok(),
            image: r.try_get::<Uri, _>("image").ok(),
            website: r.try_get::<Uri, _>("website").ok(),
            creator: r.try_get::<AddressId, _>("creator_id").ok(),
            block: r.try_get::<BlockId, _>("block_id").ok(),
            block_time: r.try_get::<BlockTime, _>("block_time").ok()
        })
        .collect::<Vec<_>>())
    }
}
