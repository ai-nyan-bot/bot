// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/jupiter (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::jupiter::repo::CandleRepo;
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
			format!("twap_1m_{}", partition),
			"sol_price_1m",
			"candle_mcap_1m",
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
			format!("twap_5m_{}", partition),
			"sol_price_5m",
			"candle_mcap_5m",
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
			format!("twap_15m_{}", partition),
			"sol_price_15m",
			"candle_mcap_15m",
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
			format!("twap_1h_{}", partition),
			"sol_price_1h",
			"candle_mcap_1h",
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
			format!("twap_6h_{}", partition),
			"sol_price_6h",
			"candle_mcap_6h",
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
			format!("twap_1d_{}", partition),
			"sol_price_1d",
			"candle_mcap_1d",
		)
			.await
	}
}

async fn calculate<'a>(
	tx: &mut Tx<'a>,
	window: usize,
	time_unit: impl AsRef<str>,
	candle_source_table: impl AsRef<str>,
	twap_source_table: impl AsRef<str>,
	sol_price_mcap_table: impl AsRef<str>,
	destination_table: impl AsRef<str>,
) -> RepoResult<()> {
	let time_unit = time_unit.as_ref();
	let candle_source_table = candle_source_table.as_ref();
	let twap_source_table = twap_source_table.as_ref();
	let sol_price_mcap_table = sol_price_mcap_table.as_ref();
	let destination_table = destination_table.as_ref();

	let query_str = format!(
		r#"
with
last_candle_ts as (
    select coalesce(
        (select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts
         from jupiter.{destination_table}
         order by timestamp desc
         limit 1),
        '1900-01-01 00:00:00'::timestamp) as ts
),
next_candle_ts as (
    select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts
    from jupiter.{candle_source_table}
    where timestamp > (select ts from last_candle_ts)
    order by timestamp
    limit 1
),
timestamp as (
    select
        (coalesce((select ts from next_candle_ts), (select ts from next_twap_ts)), (select ts from last_candle_ts))) - interval '{window} {time_unit}' as start_ts,
        (coalesce((select ts from next_candle_ts), (select ts from next_twap_ts)), (select ts from last_candle_ts))) + interval '3 days' as end_ts
),
twaps as (
    select token_pair_id, timestamp, twap from jupiter.{twap_source_table} tw
    join solana.token_pair tp on tp.id = tw.token_pair_id
    where tp.quote_id = 1 and
          timestamp >= (select start_ts from timestamp) and
          timestamp < (select end_ts from timestamp)
)
insert into jupiter.{destination_table}
(
    token_pair_id,
    timestamp,
    open,
    high,
    low,
    close,
    avg
)
select
    c.token_pair_id,
    c.timestamp,
    c.open * sp.mcap,
    c.high * sp.mcap,
    c.low * sp.mcap,
    c.close * sp.mcap,
    c.avg * sp.mcap
from jupiter.{candle_source_table} c
join lateral (
    select mcap from jupiter.{sol_price_mcap_table}
    where timestamp <= c.timestamp
    order by timestamp desc
    limit 1
) sp on true
join twaps tw on tw.timestamp = c.timestamp and tw.token_pair_id = c.token_pair_id
join solana.token_pair tp on tp.id = tw.token_pair_id    where tp.quote_id = 1
on conflict (token_pair_id, timestamp)
do update set
    open = excluded.open,
    high = excluded.high,
    low = excluded.low,
    close = excluded.close
where
    {destination_table}.open != excluded.open or
    {destination_table}.high != excluded.high or
    {destination_table}.low != excluded.low or
    {destination_table}.close != excluded.close or
    {destination_table}.avg != excluded.avg
"#
	);

	let _ = sqlx::query(&query_str).execute(&mut **tx).await?;
	Ok(())
}
