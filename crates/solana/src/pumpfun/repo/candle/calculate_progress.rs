// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::pumpfun::repo::candle::CandleRepo;
use common::model::Partition;
use common::repo::{RepoResult, Tx};

impl CandleRepo {
    pub async fn calculate_progress_1s<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        let progress_table = format!("pumpfun.candle_progress_1s_{partition}");
        let swap_table = format!("pumpfun.swap_{partition}");

        sqlx::query(
            format!(
                r#"
with last_timestamp as (
    select coalesce(
        (select date_trunc('second', timestamp) from {progress_table} order by timestamp desc limit 1),
        (select timestamp - interval '1 second' from {swap_table} order by timestamp limit 1),
        '1900-01-01 00:00:00'::timestamp
    ) as ts
),
next_swap_timestamp as (
    select timestamp as ts
    from {swap_table}
    where timestamp > (select ts from last_timestamp)
    order by timestamp
    limit 1
),
timestamp_range as (
    select
        (select ts from next_swap_timestamp) - interval '10 second'as start_ts,
        (select ts from next_swap_timestamp) + interval '1 minute' as end_ts
),
swaps as (
    select
        token_pair_id,
        date_trunc('second', timestamp) as second,
        progress,
        amount_base,
        amount_quote,
        is_buy
    from {swap_table}
    where
      timestamp >= (select start_ts from timestamp_range)
      and timestamp < (select end_ts from timestamp_range)
),
open as (
    select distinct on (token_pair_id, second)
        token_pair_id, second, progress as open
    from swaps
    order by token_pair_id, second asc
),
close as (
    select distinct on (token_pair_id, second)
        token_pair_id, second, progress as close
    from swaps
    order by token_pair_id, second desc
)
insert into {progress_table} (
    token_pair_id, timestamp, open, high, low, close, avg
)
    select
        s.token_pair_id,
        s.second,
        o.open,
        max(s.progress) as high,
        min(s.progress) as low,
        c.close,
        avg(s.progress) as avg
    from swaps s
    join open o on s.token_pair_id = o.token_pair_id and s.second = o.second
    join close c on s.token_pair_id = c.token_pair_id and s.second = c.second
    group by
        s.token_pair_id, s.second,
        o.open, c.close
on conflict (token_pair_id, timestamp)
do update set
    open = excluded.open,
    high = excluded.high,
    low = excluded.low,
    close = excluded.close,
    avg = excluded.avg,
    updated_at = now()
where (
    {progress_table}.open is distinct from excluded.open or
    {progress_table}.high is distinct from excluded.high or
    {progress_table}.low is distinct from excluded.low or
    {progress_table}.close is distinct from excluded.close or
    {progress_table}.avg is distinct from excluded.avg
)
        "#).as_str())
            .execute(&mut **tx)
            .await?;
        Ok(())
    }
}

impl CandleRepo {
    pub async fn calculate_progress_1m<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        aggregate_candle(
            tx,
            1,
            "minute",
            format!("pumpfun.candle_progress_1s_{partition}").as_str(),
            format!("pumpfun.candle_progress_1m_{partition}").as_str(),
        )
        .await
    }

    pub async fn calculate_progress_5m<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        aggregate_candle(
            tx,
            5,
            "minute",
            format!("pumpfun.candle_progress_1m_{partition}").as_str(),
            format!("pumpfun.candle_progress_5m_{partition}").as_str(),
        )
        .await
    }

    pub async fn calculate_progress_15m<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        aggregate_candle(
            tx,
            15,
            "minute",
            format!("pumpfun.candle_progress_5m_{partition}").as_str(),
            format!("pumpfun.candle_progress_15m_{partition}").as_str(),
        )
        .await
    }

    pub async fn calculate_progress_1h<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        aggregate_candle(
            tx,
            1,
            "hour",
            format!("pumpfun.candle_progress_15m_{partition}").as_str(),
            format!("pumpfun.candle_progress_1h_{partition}").as_str(),
        )
        .await
    }

    pub async fn calculate_progress_6h<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        aggregate_candle(
            tx,
            6,
            "hours",
            format!("pumpfun.candle_progress_1h_{partition}").as_str(),
            format!("pumpfun.candle_progress_6h_{partition}").as_str(),
        )
        .await
    }
    pub async fn calculate_progress_1d<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        aggregate_candle(
            tx,
            1,
            "day",
            format!("pumpfun.candle_progress_6h_{partition}").as_str(),
            format!("pumpfun.candle_progress_1d_{partition}").as_str(),
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
with last_candle_progress_ts as (
    select coalesce(
     (select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts
      from {destination_table}
      order by timestamp desc
      limit 1),
     '1900-01-01 00:00:00'::timestamp) as ts
),
next_candle_progress_ts as (
  select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts
  from {source_table}
  where timestamp > (select ts from last_candle_progress_ts)
  order by timestamp
  limit 1
),
timestamp as (
    select
        (coalesce((select ts from next_candle_progress_ts), (select ts from last_candle_progress_ts))) - interval '{window} {time_unit}' as start_ts,
        (coalesce((select ts from next_candle_progress_ts), (select ts from last_candle_progress_ts))) + interval '3 days' as end_ts
),
aggregated_candles as (
    select
        token_pair_id,
        date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as timestamp,
        coalesce((array_agg(open order by timestamp))[1],0) as open,
        coalesce(max(high),0) as high,
        coalesce(min(low),0)  as low,
        coalesce((array_agg(close order by timestamp desc))[1],0)  as close,
        coalesce(avg(avg),0) as avg
    from {source_table}
        where
            timestamp >= (select start_ts from timestamp) and
            timestamp < (select end_ts from timestamp)
    group by token_pair_id, date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}'
)
insert into {destination_table} (
    token_pair_id,
    timestamp,
    open,
    high,
    low,
    close,
    avg
)
select
    token_pair_id,
    timestamp,
    open,
    high,
    low,
    close,
    avg
from aggregated_candles
on conflict (token_pair_id, timestamp)
do update set
    open = excluded.open,
    high = excluded.high,
    low = excluded.low,
    close = excluded.close,
    avg = excluded.avg,
    updated_at = now()
where (
       {destination_table}.open is distinct from excluded.open or
       {destination_table}.high is distinct from excluded.high or
       {destination_table}.low is distinct from excluded.low or
       {destination_table}.close is distinct from excluded.close or
       {destination_table}.avg is distinct from excluded.avg
    )
        "#
    );

    let _ = sqlx::query(&query_str).execute(&mut **tx).await?;
    Ok(())
}
