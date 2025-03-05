// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::pumpfun::repo::candle::CandleRepo;
use common::model::Partition;
use common::repo::{RepoResult, Tx};

impl CandleRepo {
    pub async fn calculate_price_1s<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        let candle_price_table = format!("pumpfun.candle_price_1s_{partition}");
        let trade_table = format!("pumpfun.trade_{partition}");

        sqlx::query(
            format!(
                r#"
with last_timestamp as (
    select coalesce(
       (select date_trunc('second', timestamp) as ts from {candle_price_table} order by timestamp desc limit 1) ,
       (select timestamp - interval '1 second' as ts from {trade_table} order by timestamp limit 1),
       '1900-01-01 00:00:00'::timestamp
   ) as ts
),
next_trade_timestamp as (
    select timestamp as ts
    from {trade_table} trade
    where trade.timestamp > (select ts from last_timestamp)
    order by timestamp
    limit 1
),
timestamp as (
    select
        (select ts from next_trade_timestamp) as start_ts,
        (select ts from next_trade_timestamp) + interval '1 seconds' as end_ts
),
trades as (
    select
        token_pair_id,
        timestamp as second,
        price,
        base_amount as amount,
        is_buy
    from
        {trade_table}
    where
        -- to ensure that we get all trades, we are trailing the processing by one second
        timestamp <= date_trunc('second', now()) - interval '1 second' and
        timestamp >= (select start_ts from timestamp) and
        timestamp < (select end_ts from timestamp)
         -- limit drastically reduces the search space - there should not be more than 100 trades per second so
         -- so limiting it to 50k trades per second seems to be reasonable, which gives us a 500x speed up
        limit 50000
),
open_prices as (
    select distinct on (token_pair_id, second)
        token_pair_id,
        second,
        price as open_price
    from
        trades
    order by
        token_pair_id, second asc
),
close_prices as (
    select distinct on (token_pair_id, second)
        token_pair_id,
        second,
        price as close_price
    from
        trades
    order by
        token_pair_id, second desc
),
current_candles as (
    select
        t.token_pair_id,
        t.second,
        o.open_price,
        c.close_price,
        max(t.price) as high_price,
        min(t.price) as low_price,
        avg(t.price) as avg
    from
        trades t
    join open_prices o on t.token_pair_id = o.token_pair_id         and t.second = o.second
    join close_prices c on t.token_pair_id = c.token_pair_id        and t.second = c.second
    group by
        t.token_pair_id,
        t.second,
        o.open_price,
        c.close_price
),
previous_candles as (
    select r.* from pumpfun.candle_price_1s_most_recent r
             join current_candles c on
                 c.token_pair_id = r.token_pair_id and
                 c.second != r.timestamp
),
insert_current_candle as (
    insert into {candle_price_table} (
        token_pair_id,
        timestamp,
        open,
        high,
        low,
        close,
        avg,
        duration
    )
    select
        cur.token_pair_id,
        cur.second,
        cur.open_price,
        cur.high_price,
        cur.low_price,
        cur.close_price,
        cur.avg,
        null
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
        avg = excluded.avg
    where (
           {candle_price_table}.open != excluded.open or
           {candle_price_table}.high != excluded.high or
           {candle_price_table}.low != excluded.low or
           {candle_price_table}.close != excluded.close or
           {candle_price_table}.avg != excluded.avg
        )
    returning 1
),
previous_candles_to_update as (
    select
        token_pair_id,
        timestamp,
        extract(epoch from ((select start_ts from timestamp) - prev.timestamp)) as duration
    from previous_candles prev
),
update_previous_candles as (
    update {candle_price_table}
        set duration = prev.duration
    from previous_candles_to_update prev
    where
        {candle_price_table}.token_pair_id = prev.token_pair_id and
        {candle_price_table}.timestamp = prev.timestamp
    returning 1
)
select * from update_previous_candles
union all
select * from insert_current_candle            
            
        "#
            )
            .as_str(),
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
}

impl CandleRepo {
    pub async fn calculate_price_1m<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        aggregate_candle_price(
            tx,
            1,
            "minute",
            format!("pumpfun.candle_price_1s_{partition}").as_str(),
            format!("pumpfun.candle_price_1m_{partition}").as_str(),
        )
        .await
    }

    pub async fn calculate_price_5m<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        aggregate_candle_price(
            tx,
            5,
            "minute",
            format!("pumpfun.candle_price_1m_{partition}").as_str(),
            format!("pumpfun.candle_price_5m_{partition}").as_str(),
        )
        .await
    }

    pub async fn calculate_price_15m<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        aggregate_candle_price(
            tx,
            15,
            "minute",
            format!("pumpfun.candle_price_5m_{partition}").as_str(),
            format!("pumpfun.candle_price_15m_{partition}").as_str(),
        )
        .await
    }

    pub async fn calculate_price_1h<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        aggregate_candle_price(
            tx,
            1,
            "hour",
            format!("pumpfun.candle_price_15m_{partition}").as_str(),
            format!("pumpfun.candle_price_1h_{partition}").as_str(),
        )
        .await
    }

    pub async fn calculate_price_6h<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        aggregate_candle_price(
            tx,
            6,
            "hours",
            format!("pumpfun.candle_price_1h_{partition}").as_str(),
            format!("pumpfun.candle_price_6h_{partition}").as_str(),
        )
        .await
    }
    pub async fn calculate_price_1d<'a>(
        &self,
        tx: &mut Tx<'a>,
        partition: Partition,
    ) -> RepoResult<()> {
        aggregate_candle_price(
            tx,
            1,
            "day",
            format!("pumpfun.candle_price_6h_{partition}").as_str(),
            format!("pumpfun.candle_price_1d_{partition}").as_str(),
        )
        .await
    }
}

async fn aggregate_candle_price<'a>(
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
        coalesce(avg(avg),0) as avg
    from {source_table}
        where
            timestamp > (select start_ts from timestamp) and
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
    avg = excluded.avg
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
