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
        let candle_progress_table = format!("pumpfun.candle_progress_1s_{partition}");
        let swap_table = format!("pumpfun.swap_{partition}");

        sqlx::query(
            format!(
                r#"
with last_timestamp as (
    select coalesce(
       (select date_trunc('second', timestamp) as ts from {candle_progress_table} order by timestamp desc limit 1) ,
       (select timestamp - interval '1 second' as ts from {swap_table} order by timestamp limit 1),
       '1900-01-01 00:00:00'::timestamp
   ) as ts
),
next_swap_timestamp as (
    select timestamp as ts
    from {swap_table} swap
    where swap.timestamp > (select ts from last_timestamp)
    order by timestamp
    limit 1
),
timestamp as (
    select
        (select ts from next_swap_timestamp) as start_ts,
        (select ts from next_swap_timestamp) + interval '1 seconds' as end_ts
),
swaps as (
    select
        token_pair_id,
        timestamp as second,
        progress
    from
        {swap_table}
    where
        -- to ensure that we get all swaps, we are trailing the processing by one second
        timestamp <= date_trunc('second', now()) - interval '1 second' and
        timestamp >= (select start_ts from timestamp) and
        timestamp < (select end_ts from timestamp)
         -- limit drastically reduces the search space - there should not be more than 100 swaps per second so
         -- so limiting it to 50k swaps per second seems to be reasonable, which gives us a 500x speed up
        limit 50000
),
open_progress as (
    select distinct on (token_pair_id, second)
        token_pair_id,
        second,
        progress as open_progress
    from
        swaps
    order by
        token_pair_id, second asc
),
close_progress as (
    select distinct on (token_pair_id, second)
        token_pair_id,
        second,
        progress as close_progress
    from
        swaps
    order by
        token_pair_id, second desc
),
current_candles as (
    select
        t.token_pair_id,
        t.second,
        o.open_progress,
        c.close_progress,
        max(t.progress) as high_progress,
        min(t.progress) as low_progress,
        avg(t.progress) as avg
    from
        swaps t
    join open_progress o on t.token_pair_id = o.token_pair_id and t.second = o.second
    join close_progress c on t.token_pair_id = c.token_pair_id and t.second = c.second
    group by
        t.token_pair_id,
        t.second,
        o.open_progress,
        c.close_progress
),
previous_candles as (
    select r.* from pumpfun.candle_progress_1s_most_recent r
             join current_candles c on
                 c.token_pair_id = r.token_pair_id and
                 c.second != r.timestamp
)
insert into {candle_progress_table} (
        token_pair_id,
        timestamp,
        open,
        high,
        low,
        close,
        avg
    )
    select
        cur.token_pair_id,
        cur.second,
        cur.open_progress,
        cur.high_progress,
        cur.low_progress,
        cur.close_progress,
        cur.avg
    from
        current_candles cur
    where
        not exists (
            select 1
            from previous_candles prev
            where cur.token_pair_id = prev.token_pair_id
              and cur.second = prev.timestamp
        )
    on conflict (token_pair_id, timestamp)
    do update set
        open = excluded.open,
        high = excluded.high,
        low = excluded.low,
        close = excluded.close,
        avg = excluded.avg,
        updated_at = now()
    where (
           {candle_progress_table}.open != excluded.open or
           {candle_progress_table}.high != excluded.high or
           {candle_progress_table}.low != excluded.low or
           {candle_progress_table}.close != excluded.close or
           {candle_progress_table}.avg != excluded.avg
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
       {destination_table}.open != excluded.open or
       {destination_table}.high != excluded.high or
       {destination_table}.low != excluded.low or
       {destination_table}.close != excluded.close or
       {destination_table}.avg != excluded.avg
    )
        "#
    );

    let _ = sqlx::query(&query_str).execute(&mut **tx).await?;
    Ok(())
}
