// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::pumpfun::repo::CandleRepo;
use common::model::Partition;
use common::repo::{RepoResult, Tx};

impl CandleRepo {
    pub async fn calculate_mcap_1m<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        calculate(
            tx,
            1,
            "minute",
            format!("candle_1m_{}", partition),
            "sol_price_1m",
            "candle_market_cap_1m",
        )
        .await
    }

    pub async fn calculate_mcap_5m<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        calculate(
            tx,
            5,
            "minute",
            format!("candle_5m_{}", partition),
            "sol_price_5m",
            "candle_market_cap_5m",
        )
        .await
    }

    pub async fn calculate_mcap_15m<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        calculate(
            tx,
            15,
            "minute",
            format!("candle_15m_{}", partition),
            "sol_price_15m",
            "candle_market_cap_15m",
        )
        .await
    }

    pub async fn calculate_mcap_1h<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        calculate(
            tx,
            1,
            "hour",
            format!("candle_1h_{}", partition),
            "sol_price_1h",
            "candle_market_cap_1h",
        )
        .await
    }

    pub async fn calculate_mcap_6h<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        calculate(
            tx,
            6,
            "hours",
            format!("candle_6h_{}", partition),
            "sol_price_6h",
            "candle_market_cap_6h",
        )
        .await
    }

    pub async fn calculate_mcap_1d<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        calculate(
            tx,
            1,
            "day",
            format!("candle_1d_{}", partition),
            "sol_price_1d",
            "candle_market_cap_1d",
        )
        .await
    }
}

async fn calculate<'a>(
    tx: &mut Tx<'a>,
    window: usize,
    time_unit: impl AsRef<str>,
    candle_source_table: impl AsRef<str>,
    sol_price_usd_table: impl AsRef<str>,
    destination_table: impl AsRef<str>,
) -> RepoResult<()> {
    let time_unit = time_unit.as_ref();
    let candle_source_table = candle_source_table.as_ref();
    let sol_price_usd_table = sol_price_usd_table.as_ref();
    let destination_table = destination_table.as_ref();

    let query_str = format!(
        r#"
with
last_candle_cte as (
    select coalesce(
        (select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts
         from pumpfun.{destination_table}
         order by timestamp desc
         limit 1),
        '1900-01-01 00:00:00'::timestamp) as ts
),
next_candle_cte as (
    select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts
    from pumpfun.{candle_source_table}
    where timestamp > (select ts from last_candle_cte)
    order by timestamp
    limit 1
),
range_cte as (
    select
        (coalesce((select ts from next_candle_cte), (select ts from last_candle_cte))) - interval '{window} {time_unit}' as start_ts,
        (coalesce((select ts from next_candle_cte), (select ts from last_candle_cte))) + interval '3 days' as end_ts
)
insert into pumpfun.{destination_table}
(
    token_pair_id,
    timestamp,
    open,
    open_usd,
    high,
    high_usd,
    low,
    low_usd,
    close,
    close_usd,
    avg,
    avg_usd
)
select
    c.token_pair_id,
    c.timestamp,
    c.open * base.supply as open,
    c.open * base.supply * sp.usd as open_usd,
    c.high * base.supply as high,
    c.high * base.supply * sp.usd as high_usd,
    c.low * base.supply as low,
    c.low * base.supply * sp.usd as low_usd,
    c.close * base.supply as close,
    c.close * base.supply * sp.usd as close_usd,    
    c.avg * base.supply as avg,
    c.avg * base.supply * sp.usd as avg_usd
from pumpfun.{candle_source_table} c
join lateral (
    select usd from solana.{sol_price_usd_table}
    where timestamp = c.timestamp
    limit 1
) sp on true
join solana.token_pair tp on tp.id = c.token_pair_id
join solana.token base on base.id = tp.base_id
join range_cte r on true
where
    base.supply is not null and
    c.timestamp between r.start_ts and r.end_ts
on conflict (token_pair_id, timestamp)
do update set
    open = excluded.open,
    open_usd = excluded.open_usd,
    high = excluded.high,
    high_usd = excluded.high_usd,
    low = excluded.low,
    low_usd = excluded.low_usd,
    close = excluded.close,
    close_usd = excluded.close_usd,
    avg = excluded.avg,
    avg_usd = excluded.avg_usd,
    updated_at = now()
where
    {destination_table}.open is distinct from excluded.open or
    {destination_table}.high is distinct from excluded.high or
    {destination_table}.low is distinct from excluded.low or
    {destination_table}.close is distinct from excluded.close or
    {destination_table}.avg is distinct from excluded.avg
"#
    );

    let _ = sqlx::query(&query_str).execute(&mut **tx).await?;
    Ok(())
}
