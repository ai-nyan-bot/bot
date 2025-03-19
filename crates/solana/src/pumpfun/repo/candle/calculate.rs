// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::pumpfun::repo::candle::CandleRepo;
use common::model::Partition;
use common::repo::{RepoResult, Tx};

impl CandleRepo {
    pub async fn calculate_1m<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        aggregate_candle(
            tx,
            1,
            "minute",
            format!("pumpfun.candle_1s_{partition}").as_str(),
            format!("pumpfun.candle_1m_{partition}").as_str(),
        )
        .await
    }

    pub async fn calculate_5m<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        aggregate_candle(
            tx,
            5,
            "minute",
            format!("pumpfun.candle_1m_{partition}").as_str(),
            format!("pumpfun.candle_5m_{partition}").as_str(),
        )
        .await
    }

    pub async fn calculate_15m<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        aggregate_candle(
            tx,
            15,
            "minute",
            format!("pumpfun.candle_5m_{partition}").as_str(),
            format!("pumpfun.candle_15m_{partition}").as_str(),
        )
        .await
    }

    pub async fn calculate_1h<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        aggregate_candle(
            tx,
            1,
            "hour",
            format!("pumpfun.candle_15m_{partition}").as_str(),
            format!("pumpfun.candle_1h_{partition}").as_str(),
        )
        .await
    }

    pub async fn calculate_6h<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        aggregate_candle(
            tx,
            6,
            "hours",
            format!("pumpfun.candle_1h_{partition}").as_str(),
            format!("pumpfun.candle_6h_{partition}").as_str(),
        )
        .await
    }
    pub async fn calculate_1d<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        aggregate_candle(
            tx,
            1,
            "day",
            format!("pumpfun.candle_6h_{partition}").as_str(),
            format!("pumpfun.candle_1d_{partition}").as_str(),
        )
        .await
    }
}

async fn aggregate_candle<'a>(
    tx: &mut Tx<'a>,
    window: usize,
    time_unit: &str,
    source_table: &str,
    destination_table: &str,
) -> RepoResult<()> {
    let query_str = format!(
        r#"
with last_candle_price_ts as (
    select coalesce(
     (select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts
      from {destination_table}
      order by timestamp desc
      limit 1),
     '1900-01-01 00:00:00'::timestamp) as ts
),
next_candle_price_ts as (
  select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts
  from {source_table}
  where timestamp > (select ts from last_candle_price_ts)
  order by timestamp
  limit 1
),
timestamp as (
    select
        (coalesce((select ts from next_candle_price_ts), (select ts from last_candle_price_ts))) - interval '{window} {time_unit}' as start_ts,
        (coalesce((select ts from next_candle_price_ts), (select ts from last_candle_price_ts))) + interval '3 days' as end_ts
),
aggregated_candles as (
    select
        token_pair_id,
        date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as timestamp,
        coalesce((array_agg(open order by timestamp))[1],0) as open,
        coalesce(max(high),0) as high,
        coalesce(min(low),0)  as low,
        coalesce((array_agg(close order by timestamp desc))[1],0)  as close,
        coalesce(avg(avg),0) as avg,
        sum(amount_base_buy) as amount_base_buy,
        sum(amount_quote_buy) as amount_quote_buy,
        sum(swap_buy) as swap_buy,
        sum(volume_buy) as volume_buy,
        sum(amount_base_sell) as amount_base_sell,
        sum(amount_quote_sell) as amount_quote_sell,
        sum(swap_sell) as swap_sell,
        sum(volume_sell) as volume_sell
    from {source_table}
        where
            timestamp >= (select start_ts from timestamp) and
            timestamp < (select end_ts from timestamp) and
            (swap_buy + swap_sell) > 0
    group by token_pair_id, date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}'
)
insert into {destination_table} (
    token_pair_id,
    timestamp,
    open,
    high,
    low,
    close,
    avg,
    amount_base_buy,
    amount_quote_buy,
    swap_buy,
    volume_buy,
    amount_base_sell,
    amount_quote_sell,
    swap_sell,
    volume_sell
)
select
    token_pair_id,
    timestamp,
    open,
    high,
    low,
    close,
    avg,
    amount_base_buy,
    amount_quote_buy,
    swap_buy,
    volume_buy,
    amount_base_sell,
    amount_quote_sell,
    swap_sell,
    volume_sell
from aggregated_candles
on conflict (token_pair_id, timestamp)
do update set
    open = excluded.open,
    high = excluded.high,
    low = excluded.low,
    close = excluded.close,
    avg = excluded.avg,
    amount_base_buy = excluded.amount_base_buy,
    amount_quote_buy = excluded.amount_quote_buy,
    swap_buy = excluded.swap_buy,
    volume_buy = excluded.volume_buy,
    amount_base_sell = excluded.amount_base_sell,
    amount_quote_sell = excluded.amount_quote_sell,
    swap_sell = excluded.swap_sell,
    volume_sell = excluded.volume_sell,
    updated_at = now()
where (
       {destination_table}.open is distinct from excluded.open or
       {destination_table}.high is distinct from excluded.high or
       {destination_table}.low is distinct from excluded.low or
       {destination_table}.close is distinct from excluded.close or
       {destination_table}.avg is distinct from excluded.avg or
       {destination_table}.amount_base_buy is distinct from excluded.amount_base_buy or
       {destination_table}.amount_quote_buy is distinct from excluded.amount_quote_buy or
       {destination_table}.swap_buy is distinct from excluded.swap_buy or
       {destination_table}.volume_buy is distinct from excluded.volume_buy or
       {destination_table}.amount_base_sell is distinct from excluded.amount_base_sell or
       {destination_table}.amount_quote_sell is distinct from excluded.amount_quote_sell or
       {destination_table}.swap_sell is distinct from excluded.swap_sell or
       {destination_table}.volume_sell is distinct from excluded.volume_sell
    )
        "#
    );

    let _ = sqlx::query(&query_str).execute(&mut **tx).await?;
    Ok(())
}
