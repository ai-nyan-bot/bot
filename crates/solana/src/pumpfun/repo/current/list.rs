// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::solana::Slot;
use crate::pumpfun::model::Current;
use crate::pumpfun::repo::current::CurrentQuery;
use crate::pumpfun::repo::CurrentRepo;
use base::model::{Amount, TokenPairId};
use common::model::{
    AgeRelativeToLatestInSeconds, MarketCapQuote, MarketCapUsd, Percent, PriceQuote, PriceUsd,
};
use common::repo::{RepoResult, Tx};
use sqlx::Row;

impl CurrentRepo {
    pub async fn list<'a>(&self, tx: &mut Tx<'a>, query: CurrentQuery) -> RepoResult<Vec<Current>> {
        Ok(sqlx::query(
            r#"
        with latest as (
            select updated_at 
            from pumpfun.current 
            order by updated_at desc 
            limit 1
        )
        select 
            c.*,
            extract(epoch from (latest.updated_at - c.updated_at))::int8 as age_seconds
        from 
            pumpfun.current c,
            latest
        limit $1;
        "#,
        )
        .bind(query.limit)
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(|r| Current {
            id: r.get::<TokenPairId, _>("id"),
            slot: r.get::<Slot, _>("slot"),
            virtual_base_reserves: r.get::<Amount, _>("virtual_base_reserves"),
            virtual_quote_reserves: r.get::<Amount, _>("virtual_quote_reserves"),
            progress: r.get::<Percent, _>("progress"),
            complete: r.get::<bool, _>("complete"),
            price: r.get::<PriceQuote, _>("price"),
            price_usd: r.try_get::<PriceUsd, _>("price_usd").ok(),
            market_cap: r.try_get::<MarketCapQuote, _>("market_cap").ok(),
            market_cap_usd: r.try_get::<MarketCapUsd, _>("market_cap_usd").ok(),
            age: r.get::<AgeRelativeToLatestInSeconds, _>("age_seconds"),
        })
        .collect::<Vec<_>>())
    }
}
