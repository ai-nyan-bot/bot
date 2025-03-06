// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{DecimalAmount, Decimals, Description, Mint, Name, Symbol, Token, TokenId, Uri};
use crate::repo::TokenRepo;
use crate::{load_all, LoadTokenInfo};
use common::repo::error::RepoError;
use common::repo::{RepoResult, Tx};
use log::error;
use sqlx::Row;

impl<L: LoadTokenInfo<Mint>> TokenRepo<L> {
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
        let mut supplies: Vec<DecimalAmount> = Vec::with_capacity(token_mints.len());
        let mut metadata = Vec::with_capacity(token_mints.len());
        let mut descriptions = Vec::with_capacity(token_mints.len());
        let mut images = Vec::with_capacity(token_mints.len());
        let mut websites = Vec::with_capacity(token_mints.len());

        for info in load_all(&self.info_loader, token_mints).await {
            if let Some(info) = info {
                mints.push(info.mint.expect("token mint required"));
                names.push(info.name.unwrap_or("null_value".into()));
                symbols.push(info.symbol.unwrap_or("null_value".into()));
                decimals.push(info.decimals.expect("token decimals required"));

                if let Some(amount) = info.supply {
                    supplies.push(DecimalAmount::new(amount, info.decimals.unwrap()))
                } else {
                    supplies.push(DecimalAmount::from(-1.0))
                }

                metadata.push(info.metadata.unwrap_or("null_value".into()));
                descriptions.push(info.description.unwrap_or("null_value".into()));
                images.push(info.image.unwrap_or("null_value".into()));
                websites.push(info.website.unwrap_or("null_value".into()));
            } else {
                error!("unable to load token info");
                return Err(RepoError::NotFound);
            }
        }

        dbg!(&decimals);
        dbg!(&supplies);

        Ok(sqlx::query(
            r#"with new_token as (
            insert into solana.token (mint,name,symbol,decimals,supply,metadata,description,image,website)
            select
                unnest($1::text[]) as mint,
                unnest(array_replace($2::text[], 'null_value', null)) as name,
                unnest(array_replace($3::text[], 'null_value', null)) as symbol,
                unnest($4::int2[]) as decimals,
                unnest(array_replace($5::numeric(36, 12)[], -1, null)) as supply,
                unnest(array_replace($6::text[], 'null_value', null)) as metadata,
                unnest(array_replace($7::text[], 'null_value', null)) as description,
                unnest(array_replace($8::text[], 'null_value', null)) as image,
                unnest(array_replace($9::text[], 'null_value', null)) as website
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
        .bind(mints)
        .bind(names)
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
            name: r.try_get::<Name, _>("name").ok(),
            symbol: r.try_get::<Symbol, _>("symbol").ok(),
            decimals: r.get::<Decimals, _>("decimals"),
            supply: r.try_get::<DecimalAmount, _>("supply").ok(),
            description: r.try_get::<Description, _>("description").ok(),
            metadata: r.try_get::<Uri, _>("metadata").ok(),
            image: r.try_get::<Uri, _>("image").ok(),
            website: r.try_get::<Uri, _>("website").ok(),
        })
        .collect::<Vec<_>>())
    }
}
