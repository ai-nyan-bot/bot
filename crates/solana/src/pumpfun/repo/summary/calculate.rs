// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::pumpfun::repo::SummaryRepo;
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

	pub async fn calculate_6h<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
		calculate_summary(
			tx,
			8,
			"hours",
			"pumpfun.candle_6h_most_recent",
			format!("pumpfun.candle_6h_{partition}").as_str(),
			"pumpfun.summary_6h",
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
       c.amount_buy as amount_buy,
       c.amount_sell as amount_sell,
       c.trades_buy as trades_buy,
       c.trades_sell as trades_sell,
       c.volume_buy as volume_buy,
       0 as volume_buy_usd,
       c.volume_sell as volume_sell,
       0 as volume_sell_usd,
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
        sum(amount_buy) as amount_buy,
        sum(amount_sell) as amount_sell,
        sum(amount_buy + amount_sell) as amount,
        sum(trades_buy) as trades_buy,
        sum(trades_sell) as trades_sell,
        sum(trades_buy + trades_sell) as trades,
        sum(volume_buy) as volume_buy,
        sum(volume_buy_usd) as volume_buy_usd,
        sum(volume_sell) as volume_sell,
        sum(volume_sell_usd) as volume_sell_usd,
        sum(volume_buy + volume_sell) as volume,
        sum(volume_buy_usd + volume_sell_usd) as volume_usd,
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
    trades, trades_change, trades_change_percent,
    trades_buy, trades_buy_change, trades_buy_change_percent, 
    trades_sell, trades_sell_change, trades_sell_change_percent
)
select
    current.token_pair_id,
    current.trades_buy + current.trades_sell as trades,
    current.trades - previous.trades as trades_change,
    (current.trades::float8 - previous.trades::float8) / nullif(previous.trades::float8, 0) * 100 as trades_change_percent,

    current.trades_buy,
    current.trades_buy - previous.trades_buy as trades_buy_change,
    (current.trades_buy::float8 - previous.trades_buy::float8 ) / nullif(previous.trades_buy::float8, 0) * 100 as trades_buy_change_percent,

    current.trades_sell,
    current.trades_sell - previous.trades_sell as trades_sell_change,
    (current.trades_sell::float8 - previous.trades_sell::float8 ) / nullif(previous.trades_sell::float8, 0) * 100 as trades_sell_change_percent
from
    current
left join
    previous on current.token_pair_id = previous.token_pair_id
on conflict (token_pair_id) do update set
    trades = excluded.trades,
    trades_change = excluded.trades_change,
    trades_change_percent = excluded.trades_change_percent,
    trades_buy = excluded.trades_buy,
    trades_buy_change = excluded.trades_buy_change,
    trades_buy_change_percent = excluded.trades_buy_change_percent,
    trades_sell = excluded.trades_sell,
    trades_sell_change = excluded.trades_sell_change,
    trades_sell_change_percent = excluded.trades_sell_change_percent
where
    {destination_table}.trades != excluded.trades or
    {destination_table}.trades_change != excluded.trades_change or
    {destination_table}.trades_change_percent != excluded.trades_change_percent or
    {destination_table}.trades_buy != excluded.trades_buy or
    {destination_table}.trades_buy_change != excluded.trades_buy_change or
    {destination_table}.trades_buy_change_percent != excluded.trades_buy_change_percent or
    {destination_table}.trades_sell != excluded.trades_sell or
    {destination_table}.trades_sell_change != excluded.trades_sell_change or
    {destination_table}.trades_sell_change_percent != excluded.trades_sell_change_percent
"#
	);

	let _ = sqlx::query(&query_str).execute(&mut **tx).await?;
	Ok(())
}
