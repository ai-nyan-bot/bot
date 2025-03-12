// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::pumpfun::repo::candle::CandleRepo;
use common::model::Partition;
use common::repo::{RepoResult, Tx};

impl CandleRepo {
    pub async fn calculate_1s<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        let candle_table = format!("pumpfun.candle_1s_{partition}");
        let swap_table = format!("pumpfun.swap_{partition}");

        sqlx::query(
            format!(
                r#"
with last_timestamp as (
    select coalesce(
       (select date_trunc('second', timestamp) as ts from {candle_table} order by timestamp desc limit 1) ,
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
        price,
        amount_base as amount_base,
        amount_quote as amount_quote,
        is_buy
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
open_price as (
    select distinct on (token_pair_id, second)
        token_pair_id,
        second,
        price as open_price
    from
        swaps
    order by
        token_pair_id, second asc
),
close_price as (
    select distinct on (token_pair_id, second)
        token_pair_id,
        second,
        price as close_price
    from
        swaps
    order by
        token_pair_id, second desc
),
amount_base_buy as (
    select
        token_pair_id,
        second,
        sum(amount_base) as amount
    from
        swaps
    where is_buy = true
    group by
        token_pair_id, second
),
amount_quote_buy as (
    select
        token_pair_id,
        second,
        sum(amount_quote) as amount
    from
        swaps
    where is_buy = true
    group by
        token_pair_id, second
),
volume_buy as (
    select
        token_pair_id,
        second,
        sum(amount_quote * price) as volume
    from
        swaps
    where is_buy = true
    group by
        token_pair_id, second
),
swap_buy as (
    select
        token_pair_id,
        second,
        count(*) as swaps
    from
        swaps
    where is_buy = true
    group by
        token_pair_id, second
),
amount_base_sell as (
    select
        token_pair_id,
        second,
        sum(amount_base) as amount
    from
        swaps
    where is_buy = false
    group by
        token_pair_id, second
),
amount_quote_sell as (
    select
        token_pair_id,
        second,
        sum(amount_quote) as amount
    from
        swaps
    where is_buy = false
    group by
        token_pair_id, second
),
volume_sell as (
    select
        token_pair_id,
        second,
        sum(amount_quote * price) as volume
    from
        swaps
    where is_buy = false
    group by
        token_pair_id, second
),
swap_sell as (
    select
        token_pair_id,
        second,
        count(*) as swaps
    from
        swaps
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
        coalesce(bab.amount,0) as amount_base_buy,
        coalesce(baq.amount,0) as amount_quote_buy,
        coalesce(bt.swaps,0) as swap_buy,
        coalesce(bv.volume,0) as volume_buy,
        coalesce(sab.amount,0) as amount_base_sell,
        coalesce(saq.amount,0) as amount_quote_sell,
        coalesce(st.swaps,0) as swap_sell,
        coalesce(sv.volume,0) as volume_sell
    from
        swaps t
    join open_price o on t.token_pair_id = o.token_pair_id and t.second = o.second
    join close_price c on t.token_pair_id = c.token_pair_id and t.second = c.second
    left join amount_base_buy bab on t.token_pair_id = bab.token_pair_id and t.second = bab.second
    left join amount_quote_buy baq on t.token_pair_id = baq.token_pair_id and t.second = baq.second
    left join volume_buy bv on t.token_pair_id = bv.token_pair_id  and t.second = bv.second
    left join swap_buy bt on t.token_pair_id = bt.token_pair_id  and t.second = bt.second
    left join amount_base_sell sab on t.token_pair_id = sab.token_pair_id  and t.second = sab.second
    left join amount_quote_sell saq on t.token_pair_id = saq.token_pair_id  and t.second = saq.second
    left join volume_sell sv on t.token_pair_id = sv.token_pair_id  and t.second = sv.second
    left join swap_sell st on t.token_pair_id = st.token_pair_id  and t.second = st.second
    group by
        t.token_pair_id,
        t.second,
        o.open_price,
        c.close_price,
        bab.amount,
        baq.amount,
        bt.swaps,
        bv.volume,
        sab.amount,
        saq.amount,
        st.swaps,
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
        amount_base_buy,
        amount_quote_buy,
        swap_buy,
        volume_buy,
        amount_base_sell,
        amount_quote_sell,
        swap_sell,
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
        cur.amount_base_buy,
        cur.amount_quote_buy,
        cur.swap_buy,
        cur.volume_buy,
        cur.amount_base_sell,
        cur.amount_quote_sell,
        cur.swap_sell,
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
        amount_base_buy = excluded.amount_base_buy,
        amount_quote_buy = excluded.amount_quote_buy,
        volume_buy = excluded.volume_buy,
        swap_buy = excluded.swap_buy,
        amount_base_sell = excluded.amount_base_sell,
        amount_quote_sell = excluded.amount_quote_sell,
        volume_sell = excluded.volume_sell,
        swap_sell = excluded.swap_sell,
        updated_at = now()
    where (
           {candle_table}.open != excluded.open or
           {candle_table}.high != excluded.high or
           {candle_table}.low != excluded.low or
           {candle_table}.close != excluded.close or
           {candle_table}.avg != excluded.avg or
           {candle_table}.amount_base_buy != excluded.amount_base_buy or
           {candle_table}.amount_quote_buy != excluded.amount_quote_buy or
           {candle_table}.volume_buy != excluded.volume_buy or
           {candle_table}.swap_buy != excluded.swap_buy or
           {candle_table}.amount_base_sell != excluded.amount_base_sell or
           {candle_table}.amount_quote_sell != excluded.amount_quote_sell or
           {candle_table}.volume_sell != excluded.volume_sell or
           {candle_table}.swap_sell != excluded.swap_sell
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
        "#).as_str())
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
            timestamp > (select start_ts from timestamp) and
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
       {destination_table}.open != excluded.open or
       {destination_table}.high != excluded.high or
       {destination_table}.low != excluded.low or
       {destination_table}.close != excluded.close or
       {destination_table}.avg != excluded.avg or
       {destination_table}.amount_base_buy != excluded.amount_base_buy or
       {destination_table}.amount_quote_buy != excluded.amount_quote_buy or
       {destination_table}.swap_buy != excluded.swap_buy or
       {destination_table}.volume_buy != excluded.volume_buy or
       {destination_table}.amount_base_sell != excluded.amount_base_sell or
       {destination_table}.amount_quote_sell != excluded.amount_quote_sell or
       {destination_table}.swap_sell != excluded.swap_sell or
       {destination_table}.volume_sell != excluded.volume_sell
    )
        "#
    );

    let _ = sqlx::query(&query_str).execute(&mut **tx).await?;
    Ok(())
}
