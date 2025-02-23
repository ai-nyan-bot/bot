// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::pumpfun::repo::CandleRepo;
use common::model::Partition;
use common::repo::{RepoResult, Tx};

impl CandleRepo {
    pub async fn calculate_1s<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        let candle_table = format!("pumpfun.candle_1s_{partition}");
        let trade_table = format!("pumpfun.trade_{partition}");

        sqlx::query(
            format!(
                r#"
with last_timestamp as (
    select coalesce(
       (select date_trunc('second', timestamp) as ts from {candle_table} order by timestamp desc limit 1) ,
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
amount_buy as (
    select
        token_pair_id,
        second,
        sum(amount) as amount
    from
        trades
    where is_buy = true
    group by
        token_pair_id, second
),
volume_buys as (
    select
        token_pair_id,
        second,
        sum(amount * price) as volume
    from
        trades
    where is_buy = true
    group by
        token_pair_id, second
),
trades_buy as (
    select
        token_pair_id,
        second,
        count(*) as trades
    from
        trades
    where is_buy = true
    group by
        token_pair_id, second
),
amount_sell as (
    select
        token_pair_id,
        second,
        sum(amount) as amount
    from
        trades
    where is_buy = false
    group by
        token_pair_id, second
),
volume_sells as (
    select
        token_pair_id,
        second,
        sum(amount * price) as volume
    from
        trades
    where is_buy = false
    group by
        token_pair_id, second
),
trades_sell as (
    select
        token_pair_id,
        second,
        count(*) as trades
    from
        trades
    where is_buy = false
    group by
        token_pair_id, second
),
current_candles as (
    select
        t.token_pair_id,
        t.second,
        o.open_price,
        c.close_price,
        max(t.price) as high_price,
        min(t.price) as low_price,
        avg(t.price) as avg,
        coalesce(ba.amount,0) as amount_buy,
        coalesce(bt.trades,0) as trades_buy,
        coalesce(bv.volume,0) as volume_buy,
        coalesce(sa.amount,0) as amount_sell,
        coalesce(st.trades,0) as trades_sell,
        coalesce(sv.volume,0) as volume_sell
    from
        trades t
    join open_prices o on t.token_pair_id = o.token_pair_id         and t.second = o.second
    join close_prices c on t.token_pair_id = c.token_pair_id        and t.second = c.second
    left join amount_buy ba on t.token_pair_id = ba.token_pair_id    and t.second = ba.second
    left join volume_buys bv on t.token_pair_id = bv.token_pair_id  and t.second = bv.second
    left join trades_buy bt on t.token_pair_id = bt.token_pair_id   and t.second = bt.second
    left join amount_sell sa on t.token_pair_id = sa.token_pair_id  and t.second = sa.second
    left join volume_sells sv on t.token_pair_id = sv.token_pair_id and t.second = sv.second
    left join trades_sell st on t.token_pair_id = st.token_pair_id  and t.second = st.second
    group by
        t.token_pair_id,
        t.second,
        o.open_price,
        c.close_price,
        ba.amount,
        bt.trades,
        bv.volume,
        sa.amount,
        st.trades,
        sv.volume
),
previous_candles as (
    select r.* from pumpfun.candle_1s_most_recent r
             join current_candles c on
                 c.token_pair_id = r.token_pair_id and
                 c.second != r.timestamp
),
insert_current_candle as (
    insert into {candle_table} (
        token_pair_id,
        timestamp,
        open,
        high,
        low,
        close,
        avg,
        amount_buy,
        trades_buy,
        volume_buy,
        amount_sell,
        trades_sell,
        volume_sell,
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
        cur.amount_buy,
        cur.trades_buy,
        cur.volume_buy,
        cur.amount_sell,
        cur.trades_sell,
        cur.volume_sell,
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
        avg = excluded.avg,
        amount_buy = excluded.amount_buy,
        volume_buy = excluded.volume_buy,
        trades_buy = excluded.trades_buy,
        amount_sell = excluded.amount_sell,
        volume_sell = excluded.volume_sell,
        trades_sell = excluded.trades_sell
    where (
           {candle_table}.open != excluded.open or
           {candle_table}.high != excluded.high or
           {candle_table}.low != excluded.low or
           {candle_table}.close != excluded.close or
           {candle_table}.avg != excluded.avg or
           {candle_table}.amount_buy != excluded.amount_buy or
           {candle_table}.volume_buy != excluded.volume_buy or
           {candle_table}.trades_buy != excluded.trades_buy or
           {candle_table}.amount_sell != excluded.amount_sell or
           {candle_table}.volume_sell != excluded.volume_sell or
           {candle_table}.trades_sell != excluded.trades_sell
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
    update {candle_table}
        set duration = prev.duration
    from previous_candles_to_update prev
    where
        {candle_table}.token_pair_id = prev.token_pair_id and
        {candle_table}.timestamp = prev.timestamp
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

    pub async fn calculate_4h<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        aggregate_candle(
            tx,
            4,
            "hours",
            format!("pumpfun.candle_1h_{partition}").as_str(),
            format!("pumpfun.candle_4h_{partition}").as_str(),
        )
        .await
    }
    pub async fn calculate_1d<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        aggregate_candle(
            tx,
            1,
            "day",
            format!("pumpfun.candle_4h_{partition}").as_str(),
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
with last_candle_ts as (
    select coalesce(
     (select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts
      from {destination_table}
      order by timestamp desc
      limit 1),
     '1900-01-01 00:00:00'::timestamp) as ts
),
next_candle_ts as (
  select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts
  from {source_table}
  where timestamp > (select ts from last_candle_ts)
  order by timestamp
  limit 1
),
timestamp as (
    select
        (coalesce((select ts from next_candle_ts), (select ts from last_candle_ts))) - interval '{window} {time_unit}' as start_ts,
        (coalesce((select ts from next_candle_ts), (select ts from last_candle_ts))) + interval '3 days' as end_ts
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
        sum(amount_buy) as amount_buy,
        sum(trades_buy) as trades_buy,
        sum(volume_buy) as volume_buy,
        sum(amount_sell) as amount_sell,
        sum(trades_sell) as trades_sell,
        sum(volume_sell) as volume_sell
    from {source_table}
        where
            timestamp > (select start_ts from timestamp) and
            timestamp < (select end_ts from timestamp) and
            (trades_buy + trades_sell) > 0
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
    amount_buy,
    trades_buy,
    volume_buy,
    amount_sell,
    trades_sell,
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
    amount_buy,
    trades_buy,
    volume_buy,
    amount_sell,
    trades_sell,
    volume_sell
from aggregated_candles
on conflict (token_pair_id, timestamp)
do update set
    open = excluded.open,
    high = excluded.high,
    low = excluded.low,
    close = excluded.close,
    avg = excluded.avg,
    amount_buy = excluded.amount_buy,
    trades_buy = excluded.trades_buy,
    volume_buy = excluded.volume_buy,
    amount_sell = excluded.amount_sell,
    trades_sell = excluded.trades_sell,
    volume_sell = excluded.volume_sell
where (
       {destination_table}.open != excluded.open or
       {destination_table}.high != excluded.high or
       {destination_table}.low != excluded.low or
       {destination_table}.close != excluded.close or
       {destination_table}.avg != excluded.avg or
       {destination_table}.amount_buy != excluded.amount_buy or
       {destination_table}.trades_buy != excluded.trades_buy or
       {destination_table}.volume_buy != excluded.volume_buy or
       {destination_table}.amount_sell != excluded.amount_sell or
       {destination_table}.trades_sell != excluded.trades_sell or
       {destination_table}.volume_sell != excluded.volume_sell
    )
        "#
    );

    let _ = sqlx::query(&query_str).execute(&mut **tx).await?;
    Ok(())
}
