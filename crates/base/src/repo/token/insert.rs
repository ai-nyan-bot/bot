// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{Decimals, Description, Mint, Name, Supply, Symbol, Token, TokenId, Uri};
use crate::repo::TokenRepo;
use crate::{load_all, LoadTokenInfo};
use common::repo::error::RepoError;
use common::repo::{RepoResult, Tx};
use log::error;
use sqlx::Row;

impl<L: LoadTokenInfo> TokenRepo<L> {
    pub async fn insert_token<'a>(
        &self,
        tx: &mut Tx<'a>,
        token_mints: &[Mint],
    ) -> RepoResult<Vec<Token>> {
        if token_mints.is_empty() {
            return Ok(vec![]);
        }

        let mut mints = Vec::with_capacity(token_mints.len());
        let mut names = Vec::with_capacity(token_mints.len());
        let mut symbols = Vec::with_capacity(token_mints.len());
        let mut decimals = Vec::with_capacity(token_mints.len());
        let mut supplies = Vec::with_capacity(token_mints.len());
        let mut metadata = Vec::with_capacity(token_mints.len());
        let mut descriptions = Vec::with_capacity(token_mints.len());
        let mut images = Vec::with_capacity(token_mints.len());
        let mut websites = Vec::with_capacity(token_mints.len());

        for info in load_all(&self.info_loader, token_mints).await {
            if let Some(info) = info {
                mints.push(info.mint);
                names.push(info.name);
                symbols.push(info.symbol);
                decimals.push(info.decimals);
                supplies.push(info.supply);
                metadata.push(info.metadata.unwrap_or("null_value".into()));
                descriptions.push(info.description.unwrap_or("null_value".into()));
                images.push(info.image.unwrap_or("null_value".into()));
                websites.push(info.website.unwrap_or("null_value".into()));
            } else {
                error!("unable to load token info");
                return Err(RepoError::NotFound);
            }
        }

        Ok(sqlx::query(
            r#"with new_token as (
            insert into solana.token (mint,name,symbol,decimals)
            select
                unnest($1::text[]) as mint,
                unnest($2::text[]) as name,
                unnest($3::text[]) as symbol,
                unnest($4::int2[]) as decimals,
                unnest($5::int2[]) as supply,
                unnest(array_replace($6::varchar[], 'null_value', null)) as metadata,
                unnest(array_replace($7::varchar[], 'null_value', null)) as description,
                unnest(array_replace($8::varchar[], 'null_value', null)) as image,
                unnest(array_replace($9::varchar[], 'null_value', null)) as website
            on conflict (mint) do update set
                mint = excluded.mint,
                name = excluded.name,
                symbol = excluded.symbol,
                decimals = excluded.decimals,
                supply = excluded.supply,
                metadata = excluded.metadata,
                description = excluded.description,
                image = excluded.image,
                website = excluded.website
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
                website
            )
            select * from new_token"#,
        )
        .bind(&mints)
        .bind(&names)
        .bind(symbols)
        .bind(decimals)
        .bind(supplies)
        .bind(metadata)
        .bind(descriptions)
        .bind(images)
        .bind(websites)
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
