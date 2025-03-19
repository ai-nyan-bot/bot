// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::jupiter::repo::CandleRepo;
use common::model::Partition;
use common::repo::{RepoResult, Tx};

impl CandleRepo {
    pub async fn calculate_1s<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        let candle_table = format!("jupiter.candle_1s_{partition}");
        let swap_table = format!("jupiter.swap_{partition}");

        calculate_candles(tx, &candle_table, &swap_table).await?;
        calculate_duration(tx, &candle_table).await?;

        Ok(())
    }
}

async fn calculate_candles<'a>(
    tx: &mut Tx<'a>,
    candle_table: &str,
    swap_table: &str,
) -> RepoResult<()> {
    sqlx::query(
		format!(
			r#"
with last_timestamp as (
    select coalesce(
        (select date_trunc('second', timestamp) from {candle_table} order by timestamp desc limit 1),
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
        price,
        amount_base,
        amount_quote,
        is_buy
    from {swap_table}
    where
      timestamp >= (select start_ts from timestamp_range)
      and timestamp < (select end_ts from timestamp_range)
),
open_price as (
    select distinct on (token_pair_id, second)
        token_pair_id, second, price as open_price
    from swaps
    order by token_pair_id, second asc
),
close_price as (
    select distinct on (token_pair_id, second)
        token_pair_id, second, price as close_price
    from swaps
    order by token_pair_id, second desc
),
amount_base_buy as (
    select token_pair_id, second, sum(amount_base) as amount
    from swaps
    where is_buy = true
    group by token_pair_id, second
),
amount_quote_buy as (
    select token_pair_id, second, sum(amount_quote) as amount
    from swaps
    where is_buy = true
    group by token_pair_id, second
),
volume_buy as (
    select token_pair_id, second, sum(amount_base * price) as volume
    from swaps
    where is_buy = true
    group by token_pair_id, second
),
swap_buy as (
    select token_pair_id, second, count(*) as swaps
    from swaps
    where is_buy = true
    group by token_pair_id, second
),
amount_base_sell as (
    select token_pair_id, second, sum(amount_base) as amount
    from swaps
    where is_buy = false
    group by token_pair_id, second
),
amount_quote_sell as (
    select token_pair_id, second, sum(amount_quote) as amount
    from swaps
    where is_buy = false
    group by token_pair_id, second
),
volume_sell as (
    select token_pair_id, second, sum(amount_base * price) as volume
    from swaps
    where is_buy = false
    group by token_pair_id, second
),
swap_sell as (
    select token_pair_id, second, count(*) as swaps
    from swaps
    where is_buy = false
    group by token_pair_id, second
)
insert into {candle_table} (
    token_pair_id, timestamp, open, high, low, close, avg,
    amount_base_buy, amount_quote_buy, swap_buy, volume_buy,
    amount_base_sell, amount_quote_sell, swap_sell, volume_sell
)
    select
        s.token_pair_id,
        s.second,
        o.open_price,
        c.close_price,
        max(s.price) as high_price,
        min(s.price) as low_price,
        avg(s.price) as avg,
        coalesce(ab_buy.amount, 0) as amount_base_buy,
        coalesce(aq_buy.amount, 0) as amount_quote_buy,
        coalesce(swaps_buy.swaps, 0) as swap_buy,
        coalesce(vol_buy.volume, 0) as volume_buy,
        coalesce(ab_sell.amount, 0) as amount_base_sell,
        coalesce(aq_sell.amount, 0) as amount_quote_sell,
        coalesce(swaps_sell.swaps, 0) as swap_sell,
        coalesce(vol_sell.volume, 0) as volume_sell
    from swaps s
    join open_price o on s.token_pair_id = o.token_pair_id and s.second = o.second
    join close_price c on s.token_pair_id = c.token_pair_id and s.second = c.second
    left join amount_base_buy ab_buy on s.token_pair_id = ab_buy.token_pair_id and s.second = ab_buy.second
    left join amount_quote_buy aq_buy on s.token_pair_id = aq_buy.token_pair_id and s.second = aq_buy.second
    left join volume_buy vol_buy on s.token_pair_id = vol_buy.token_pair_id and s.second = vol_buy.second
    left join swap_buy swaps_buy on s.token_pair_id = swaps_buy.token_pair_id and s.second = swaps_buy.second
    left join amount_base_sell ab_sell on s.token_pair_id = ab_sell.token_pair_id and s.second = ab_sell.second
    left join amount_quote_sell aq_sell on s.token_pair_id = aq_sell.token_pair_id and s.second = aq_sell.second
    left join volume_sell vol_sell on s.token_pair_id = vol_sell.token_pair_id and s.second = vol_sell.second
    left join swap_sell swaps_sell on s.token_pair_id = swaps_sell.token_pair_id and s.second = swaps_sell.second
    group by
        s.token_pair_id, s.second,
        o.open_price, c.close_price,
        ab_buy.amount, aq_buy.amount, swaps_buy.swaps, vol_buy.volume,
        ab_sell.amount, aq_sell.amount, swaps_sell.swaps, vol_sell.volume
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
    {candle_table}.open is distinct from excluded.open or
    {candle_table}.high is distinct from excluded.high or
    {candle_table}.low is distinct from excluded.low or
    {candle_table}.close is distinct from excluded.close or
    {candle_table}.avg is distinct from excluded.avg or
    {candle_table}.amount_base_buy is distinct from excluded.amount_base_buy or
    {candle_table}.amount_quote_buy is distinct from excluded.amount_quote_buy or
    {candle_table}.volume_buy is distinct from excluded.volume_buy or
    {candle_table}.swap_buy is distinct from excluded.swap_buy or
    {candle_table}.amount_base_sell is distinct from excluded.amount_base_sell or
    {candle_table}.amount_quote_sell is distinct from excluded.amount_quote_sell or
    {candle_table}.volume_sell is distinct from excluded.volume_sell or
    {candle_table}.swap_sell is distinct from excluded.swap_sell
);
        "#).as_str())
		.execute(&mut **tx)
		.await?;
    Ok(())
}

async fn calculate_duration<'a>(tx: &mut Tx<'a>, candle_table: &str) -> RepoResult<()> {
    sqlx::query(
        format!(
            r#"
update {candle_table} c
set duration = extract(epoch from ((
    select timestamp from {candle_table} nc
    where nc.token_pair_id = c.token_pair_id and nc.timestamp > c.timestamp
    order by nc.timestamp limit 1
) - c.timestamp))
where 
    c.duration is null and exists (
    select 1 from {candle_table} nc
    where nc.token_pair_id = c.token_pair_id and nc.timestamp > c.timestamp
);
        "#
        )
        .as_str(),
    )
    .execute(&mut **tx)
    .await?;
    Ok(())
}
