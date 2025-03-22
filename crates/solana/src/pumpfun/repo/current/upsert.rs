// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::solana::Slot;
use crate::pumpfun::model::{Current, Swap};
use crate::pumpfun::repo::current::CurrentRepo;
use base::model::{Amount, TokenPairId};
use common::model::{
    AgeRelativeToLatestInSeconds, MarketCapQuote, MarketCapUsd, Percent, PriceQuote, PriceUsd,
};
use common::repo::{RepoResult, Tx};
use sqlx::Row;

impl CurrentRepo {
    pub async fn upsert<'a>(&self, tx: &mut Tx<'a>, swap: Swap) -> RepoResult<Current> {
        Ok(sqlx::query(
            r#"
with latest_sol_price as (
    select usd 
    from solana.sol_price_1m 
    order by updated_at desc 
    limit 1
), token_supply as (
    select tp.id as token_pair_id, t.supply 
    from solana.token_pair tp 
    join solana.token t on tp.base_id = t.id
    where tp.id = $1
)
insert into pumpfun.current (
    id, slot, virtual_base_reserves, virtual_quote_reserves, progress, complete, price, price_usd, market_cap, market_cap_usd, updated_at
)
values (
    $1, $2, $3, $4, $5, $6, $7, 
    case when (select usd from latest_sol_price) is null then null else $7 * (select usd from latest_sol_price) end, 
    case when (select supply from token_supply) is null then null else $7 * (select supply from token_supply) end, 
    case when (select usd from latest_sol_price) is null or (select supply from token_supply) is null  then null else $7 * (select supply from token_supply) * (select usd from latest_sol_price) 
    end, 
    now()
)
on conflict (id) do update set
    slot = excluded.slot,
    virtual_base_reserves = excluded.virtual_base_reserves,
    virtual_quote_reserves = excluded.virtual_quote_reserves,
    progress = excluded.progress,
    complete = excluded.complete,
    price = excluded.price,
    price_usd = case when (select usd from latest_sol_price) is null then null else excluded.price * (select usd from latest_sol_price) end,
    market_cap = case when (select supply from token_supply) is null then null else excluded.price * (select supply from token_supply) end,
    market_cap_usd = case when (select usd from latest_sol_price) is null or (select supply from token_supply) is null then null else excluded.market_cap * (select usd from latest_sol_price)                       end,
    updated_at = now()
returning 
    id, slot, virtual_base_reserves, virtual_quote_reserves, progress, complete, price, price_usd, market_cap, market_cap_usd, updated_at;


        "#
        )
            .bind(swap.token_pair)
            .bind(swap.slot)
            .bind(swap.virtual_base_reserves)
            .bind(swap.virtual_quote_reserves)
            .bind(swap.progress.clone())
            .bind(swap.progress >= 100.0)
            .bind(swap.price)
            .fetch_one(&mut **tx)
            .await
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
                age: AgeRelativeToLatestInSeconds(0),
            })?)
    }
}
