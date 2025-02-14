// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::repo::pumpfun::SummaryRepo;
use common::model::Partition;
use common::repo::{RepoResult, Tx};

impl SummaryRepo {
    pub async fn calculate_1m<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_summary(
            tx,
            2,
            "minutes",
            "pumpfun.candle_1m_most_recent",
            format!("pumpfun.candle_1m_{partition}").as_str(),
            "pumpfun.summary_1m",
        )
        .await
    }

    pub async fn calculate_5m<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_summary(
            tx,
            10,
            "minutes",
            "pumpfun.candle_5m_most_recent",
            format!("pumpfun.candle_5m_{partition}").as_str(),
            "pumpfun.summary_5m",
        )
        .await
    }

    pub async fn calculate_15m<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_summary(
            tx,
            30,
            "minutes",
            "pumpfun.candle_15m_most_recent",
            format!("pumpfun.candle_15m_{partition}").as_str(),
            "pumpfun.summary_15m",
        )
        .await
    }

    pub async fn calculate_1h<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_summary(
            tx,
            2,
            "hours",
            "pumpfun.candle_1h_most_recent",
            format!("pumpfun.candle_1h_{partition}").as_str(),
            "pumpfun.summary_1h",
        )
        .await
    }

    pub async fn calculate_4h<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_summary(
            tx,
            8,
            "hours",
            "pumpfun.candle_4h_most_recent",
            format!("pumpfun.candle_4h_{partition}").as_str(),
            "pumpfun.summary_4h",
        )
        .await
    }

    pub async fn calculate_1d<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_summary(
            tx,
            2,
            "days",
            "pumpfun.candle_1d_most_recent",
            format!("pumpfun.candle_1d_{partition}").as_str(),
            "pumpfun.summary_1d",
        )
        .await
    }
}

async fn calculate_summary<'a>(
    tx: &mut Tx<'a>,
    window: usize,
    time_unit: &str,
    candle_most_recent_table: &str,
    candle_source_table: &str,
    destination_table: &str,
) -> RepoResult<()> {
    let bucket_separator = window / 2;
    let query_str = format!(
        r#"
with
last_candle as (
    select timestamp from {candle_most_recent_table} order by timestamp desc limit 1
),
candles as (
    select *
    from {candle_source_table}
    where timestamp > (select timestamp from last_candle) - interval '{window} {time_unit}'
),
candles_with_prices as(
    select
       c.token_pair_id as token_pair_id,
       c.timestamp as timestamp,
       c.buy_amount as buy_amount,
       c.sell_amount as sell_amount,
       c.buy_trades as buy_trades,
       c.sell_trades as sell_trades,
       c.buy_volume as buy_volume,
       0 as buy_volume_usd,
       c.sell_volume as sell_volume,
       0 as sell_volume_usd,
       c.open as open,
       0 as open_usd,
       c.high as high,
       0 as high_usd,
       c.low as low,
       0 as low_usd,
       c.close as close,
       0 as close_usd,
       c.avg as avg,
       0 as avg_usd
    from candles c
),
aggregated as (
    select
        token_pair_id,
        case
            when timestamp > (select timestamp from last_candle) - interval '{bucket_separator} {time_unit}' then 'current'
            else 'previous'
        end as time_bucket,
        sum(buy_amount) as buy_amount,
        sum(sell_amount) as sell_amount,
        sum(buy_amount + sell_amount) as amount,
        sum(buy_trades) as buy_trades,
        sum(sell_trades) as sell_trades,
        sum(buy_trades + sell_trades) as trades,
        sum(buy_volume) as buy_volume,
        sum(buy_volume_usd) as buy_volume_usd,
        sum(sell_volume) as sell_volume,
        sum(sell_volume_usd) as sell_volume_usd,
        sum(buy_volume + sell_volume) as volume,
        sum(buy_volume_usd + sell_volume_usd) as volume_usd,
        max(high) as high,
        max(high_usd) as high_usd,
        min(low) as low,
        min(low_usd) as low_usd,
        avg(avg) as avg,
        avg(avg_usd) as avg_usd
    from candles_with_prices
    group by token_pair_id, time_bucket
    order by token_pair_id, time_bucket
),
current as (select * from aggregated where time_bucket = 'current'),
previous as (select * from aggregated where time_bucket = 'previous')
insert into {destination_table} (
    token_pair_id,
    amount, amount_change, buy_amount, buy_amount_change, sell_amount, sell_amount_change,
    trades, trades_change, buy_trades, buy_trades_change, sell_trades, sell_trades_change,
    volume, volume_change, buy_volume, buy_volume_change, sell_volume, sell_volume_change,
    volume_usd, volume_usd_change, buy_volume_usd, buy_volume_usd_change, sell_volume_usd, sell_volume_usd_change,
    high, high_change, high_usd, high_usd_change,
    low, low_change, low_usd, low_usd_change,
    avg, avg_change, avg_usd, avg_usd_change
)
select
    current.token_pair_id,
    (current.buy_amount + current.sell_amount) as amount,
    coalesce((current.amount - previous.amount) /
             nullif(previous.amount, 0) * 100, 0) as amount_change,
    current.buy_amount,
    coalesce((current.buy_amount - previous.buy_amount ) /
             nullif(previous.buy_amount, 0) * 100, 0) as buy_amount_change,

    current.sell_amount,
    coalesce((current.sell_amount - previous.sell_amount ) /
             nullif(previous.sell_amount, 0) * 100, 0) as sell_amount_change,

        (current.buy_trades + current.sell_trades) as trades,
    coalesce((current.trades::float8 - previous.trades::float8) /
             nullif(previous.trades::float8, 0) * 100, 0) as trades_change,

    current.buy_trades,
    coalesce((current.buy_trades::float8 - previous.buy_trades::float8 ) /
             nullif(previous.buy_trades::float8, 0) * 100, 0) as buy_trades_change,

    current.sell_trades,
    coalesce((current.sell_trades::float8 - previous.sell_trades::float8 ) /
             nullif(previous.sell_trades::float8, 0) * 100, 0) as sell_trades_change,

    (current.buy_volume + current.sell_volume) as volume,
    coalesce((current.volume - previous.volume) /
             nullif(previous.volume, 0) * 100, 0) as volume_change,

    current.buy_volume,
    coalesce((current.buy_volume - previous.buy_volume ) /
             nullif(previous.buy_volume, 0) * 100, 0) as buy_volume_change,

    current.sell_volume,
    coalesce((current.sell_volume - previous.sell_volume ) /
             nullif(previous.sell_volume, 0) * 100, 0) as sell_volume_change,

    (current.buy_volume_usd + current.sell_volume_usd) as volume_usd,
    coalesce((current.volume_usd - previous.volume_usd) /
             nullif(previous.volume_usd, 0) * 100, 0) as volume_usd_change,

    current.buy_volume_usd,
    coalesce((current.buy_volume_usd - previous.buy_volume_usd ) /
             nullif(previous.buy_volume_usd, 0) * 100, 0) as buy_volume_usd_change,

    current.sell_volume_usd,
    coalesce((current.sell_volume_usd - previous.sell_volume_usd ) /
             nullif(previous.sell_volume_usd, 0) * 100, 0) as sell_volume_usd_change,

    current.high,
    coalesce((current.high - previous.high ) /
             nullif(previous.high, 0) * 100, 0) as high_change,

    current.high_usd,
    coalesce((current.high_usd - previous.high_usd ) /
             nullif(previous.high_usd, 0) * 100, 0) as high_usd_change,

    current.low,
    coalesce((current.low - previous.low ) /
             nullif(previous.low, 0) * 100, 0) as low_change,

    current.low_usd,
    coalesce((current.low_usd - previous.low_usd ) /
             nullif(previous.low_usd, 0) * 100, 0) as low_usd_change,

    current.avg,
    coalesce((current.avg - previous.avg ) /
             nullif(previous.avg, 0) * 100, 0) as avg_change,

    current.avg_usd,
    coalesce((current.avg_usd - previous.avg_usd ) /
             nullif(previous.avg_usd, 0) * 100, 0) as avg_usd_change
from
    current
left join
    previous on current.token_pair_id = previous.token_pair_id
on conflict (token_pair_id) do update set
    amount = excluded.amount,
    amount_change = excluded.amount_change,
    buy_amount = excluded.buy_amount,
    buy_amount_change = excluded.buy_amount_change,
    sell_amount = excluded.sell_amount,
    sell_amount_change = excluded.sell_amount_change,
    trades = excluded.trades,
    trades_change = excluded.trades_change,
    buy_trades = excluded.buy_trades,
    buy_trades_change = excluded.buy_trades_change,
    sell_trades = excluded.sell_trades,
    sell_trades_change = excluded.sell_trades_change,
    volume = excluded.volume,
    volume_usd = excluded.volume_usd,
    volume_change = excluded.volume_change,
    volume_usd_change = excluded.volume_usd_change,
    buy_volume = excluded.buy_volume,
    buy_volume_usd = excluded.buy_volume_usd,
    buy_volume_change = excluded.buy_volume_change,
    buy_volume_usd_change = excluded.buy_volume_usd_change,
    sell_volume = excluded.sell_volume,
    sell_volume_usd = excluded.sell_volume_usd,
    sell_volume_change = excluded.sell_volume_change,
    sell_volume_usd_change = excluded.sell_volume_usd_change,
    high = excluded.high,
    high_change = excluded.high_change,
    high_usd = excluded.high_usd,
    high_usd_change = excluded.high_usd_change,
    low = excluded.low,
    low_change = excluded.low_change,
    low_usd = excluded.low_usd,
    low_usd_change = excluded.low_usd_change,
    avg = excluded.avg,
    avg_change = excluded.avg_change,
    avg_usd = excluded.avg_usd,
    avg_usd_change = excluded.avg_usd_change
where
    {destination_table}.amount != excluded.amount or
    {destination_table}.amount_change != excluded.amount_change or
    {destination_table}.buy_amount != excluded.buy_amount or
    {destination_table}.buy_amount_change != excluded.buy_amount_change or
    {destination_table}.sell_amount != excluded.sell_amount or
    {destination_table}.sell_amount_change != excluded.sell_amount_change or
    {destination_table}.trades != excluded.trades or
    {destination_table}.trades_change != excluded.trades_change or
    {destination_table}.buy_trades != excluded.buy_trades or
    {destination_table}.buy_trades_change != excluded.buy_trades_change or
    {destination_table}.sell_trades != excluded.sell_trades or
    {destination_table}.sell_trades_change != excluded.sell_trades_change or
    {destination_table}.volume != excluded.volume or
    {destination_table}.volume_usd != excluded.volume_usd or
    {destination_table}.volume_change != excluded.volume_change or
    {destination_table}.volume_usd_change != excluded.volume_usd_change or
    {destination_table}.buy_volume != excluded.buy_volume or
    {destination_table}.buy_volume_usd != excluded.buy_volume_usd or
    {destination_table}.buy_volume_change != excluded.buy_volume_change or
    {destination_table}.buy_volume_usd_change != excluded.buy_volume_usd_change or
    {destination_table}.sell_volume != excluded.sell_volume or
    {destination_table}.sell_volume_usd != excluded.sell_volume_usd or
    {destination_table}.sell_volume_change != excluded.sell_volume_change or
    {destination_table}.sell_volume_usd_change != excluded.sell_volume_usd_change or
    {destination_table}.high != excluded.high or
    {destination_table}.high_change != excluded.high_change or
    {destination_table}.high_usd != excluded.high_usd or
    {destination_table}.high_usd_change != excluded.high_usd_change or
    {destination_table}.low != excluded.low or
    {destination_table}.low_change != excluded.low_change or
    {destination_table}.low_usd != excluded.low_usd or
    {destination_table}.low_usd_change != excluded.low_usd_change or
    {destination_table}.avg != excluded.avg or
    {destination_table}.avg_change != excluded.avg_change or
    {destination_table}.avg_usd != excluded.avg_usd or
    {destination_table}.avg_usd_change != excluded.avg_usd_change;
"#
    );

    let _ = sqlx::query(&query_str).execute(&mut **tx).await?;
    Ok(())
}
