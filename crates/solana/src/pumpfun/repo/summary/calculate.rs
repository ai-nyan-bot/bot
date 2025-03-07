// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::pumpfun::repo::SummaryRepo;
use common::model::Partition;
use common::repo::{RepoResult, Tx};

impl SummaryRepo {
    pub async fn calculate_1m<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_summary(
            tx,
            2,
            "minutes",
            format!("pumpfun.candle_1m_{partition}"),
            format!("pumpfun.candle_market_cap_1m_{partition}"),
            format!("pumpfun.candle_progress_1m_{partition}"),
            format!("pumpfun.candle_usd_1m_{partition}"),
            "pumpfun.summary_1m",
        )
        .await
    }

    pub async fn calculate_5m<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_summary(
            tx,
            10,
            "minutes",
            format!("pumpfun.candle_5m_{partition}"),
            format!("pumpfun.candle_market_cap_5m_{partition}"),
            format!("pumpfun.candle_progress_5m_{partition}"),
            format!("pumpfun.candle_usd_5m_{partition}"),
            "pumpfun.summary_5m",
        )
        .await
    }

    pub async fn calculate_15m<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_summary(
            tx,
            30,
            "minutes",
            format!("pumpfun.candle_15m_{partition}"),
            format!("pumpfun.candle_market_cap_15m_{partition}"),
            format!("pumpfun.candle_progress_15m_{partition}"),
            format!("pumpfun.candle_usd_15m_{partition}"),
            "pumpfun.summary_15m",
        )
        .await
    }

    pub async fn calculate_1h<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_summary(
            tx,
            2,
            "hours",
            format!("pumpfun.candle_1h_{partition}"),
            format!("pumpfun.candle_market_cap_1h_{partition}"),
            format!("pumpfun.candle_progress_1h_{partition}"),
            format!("pumpfun.candle_usd_1h_{partition}"),
            "pumpfun.summary_1h",
        )
        .await
    }

    pub async fn calculate_6h<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_summary(
            tx,
            12,
            "hours",
            format!("pumpfun.candle_6h_{partition}"),
            format!("pumpfun.candle_market_cap_6h_{partition}"),
            format!("pumpfun.candle_progress_6h_{partition}"),
            format!("pumpfun.candle_usd_6h_{partition}"),
            "pumpfun.summary_6h",
        )
        .await
    }

    pub async fn calculate_1d<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_summary(
            tx,
            2,
            "days",
            format!("pumpfun.candle_1d_{partition}"),
            format!("pumpfun.candle_market_cap_1d_{partition}"),
            format!("pumpfun.candle_progress_1d_{partition}"),
            format!("pumpfun.candle_usd_1d_{partition}"),
            "pumpfun.summary_1d",
        )
        .await
    }
}

async fn calculate_summary<'a>(
    tx: &mut Tx<'a>,
    window: usize,
    time_unit: &str,
    candle_table: impl AsRef<str>,
    candle_market_cap_table: impl AsRef<str>,
    candle_progress_table: impl AsRef<str>,
    candle_usd_table: impl AsRef<str>,
    destination_table: impl AsRef<str>,
) -> RepoResult<()> {
    let candle_table = candle_table.as_ref();
    let candle_market_cap_table = candle_market_cap_table.as_ref();
    let candle_progress_table = candle_progress_table.as_ref();
    let candle_usd_table = candle_usd_table.as_ref();
    let destination_table = destination_table.as_ref();
    let bucket_separator = window / 2;

    let aggregate = r#"
		token_pair_id,

		amount_base_buy,
		amount_base_sell,
		amount_base_buy + amount_base_sell as amount_base,
		
		amount_quote_buy,
		amount_quote_sell,
		amount_quote_buy + amount_quote_sell as amount_quote,
	
		curve_progress_open,
		curve_progress_high,
		curve_progress_low,
		curve_progress_close,
		curve_progress_avg,
	
		market_cap_open,
		market_cap_open_usd,
		market_cap_high,
		market_cap_high_usd,
		market_cap_low,
		market_cap_low_usd,
		market_cap_close,
		market_cap_close_usd,
		market_cap_avg,
		market_cap_avg_usd,
	
		price_open,
		price_open_usd,
		price_high,
		price_high_usd,
		price_low,
		price_low_usd,
		price_close,
		price_close_usd,
		price_avg,
		price_avg_usd,
	
		trade_buy,
		trade_sell,
		trade_buy + trade_sell as trade,
	
		volume_buy,
		volume_buy_usd,
		volume_sell,
		volume_sell_usd,
		volume_buy + volume_sell as volume,
		volume_buy_usd + volume_sell_usd as volume_usd
	"#;

    let query_str = format!(
        r#"
with
last_candle as (
    select timestamp from {candle_table} order by timestamp desc limit 1
),
candles as(
	select c.token_pair_id     as token_pair_id,
		   c.timestamp         as timestamp,
	
		   c.amount_base_buy   as amount_base_buy,
		   c.amount_quote_buy  as amount_quote_buy,
		   c.amount_base_sell  as amount_base_sell,
		   c.amount_quote_sell as amount_quote_sell,
	
		   cp.open             as curve_progress_open,
		   cp.high             as curve_progress_high,
		   cp.low              as curve_progress_low,
		   cp.close            as curve_progress_close,
		   cp.avg              as curve_progress_avg,

		   cm.open             as market_cap_open,
		   cm.open_usd         as market_cap_open_usd,
		   cm.high             as market_cap_high,
		   cm.high_usd         as market_cap_high_usd,
		   cm.low              as market_cap_low,
		   cm.low_usd          as market_cap_low_usd,
		   cm.close            as market_cap_close,
		   cm.close_usd        as market_cap_close_usd,
		   cm.avg              as market_cap_avg,
		   cm.avg_usd          as market_cap_avg_usd,

		   c.open              as price_open,
		   cu.open             as price_open_usd,
		   c.high              as price_high,
		   cu.high             as price_high_usd,
		   c.low               as price_low,
		   cu.low              as price_low_usd,
		   c.close             as price_close,
		   cu.close            as price_close_usd,
		   c.avg               as price_avg,
		   cu.avg              as price_avg_usd,
	
		   c.trade_buy         as trade_buy,
		   c.trade_sell        as trade_sell,
	
		   c.volume_buy        as volume_buy,
		   cu.volume_buy       as volume_buy_usd,
		   c.volume_sell       as volume_sell,
		   cu.volume_sell      as volume_sell_usd

	from {candle_table} c
			 join {candle_progress_table} cp on cp.token_pair_id = c.token_pair_id and cp.timestamp = c.timestamp
			 join {candle_usd_table} cu on cu.token_pair_id = c.token_pair_id and cu.timestamp = c.timestamp
			 join {candle_market_cap_table} cm on cm.token_pair_id = c.token_pair_id and cm.timestamp = c.timestamp
	where c.timestamp > (select timestamp from last_candle) - interval '{window} {time_unit}'
),
current as (
    select
		{aggregate}
    from candles
    where
	    timestamp > (select timestamp from last_candle) - interval '{bucket_separator} {time_unit}'
),
previous as (
    select
		{aggregate}
    from candles
    where
	    timestamp <= (select timestamp from last_candle) - interval '{bucket_separator} {time_unit}'
)
insert into {destination_table} (
    token_pair_id,
    amount_base,
    amount_base_change,
    amount_base_percent,
    amount_base_buy,
    amount_base_buy_change,
    amount_base_buy_percent,
    amount_base_sell,
    amount_base_sell_change,
    amount_base_sell_percent,
    amount_quote,
    amount_quote_change,
    amount_quote_percent,
    amount_quote_buy,
    amount_quote_buy_change,
    amount_quote_buy_percent,
    amount_quote_sell,
    amount_quote_sell_change,
    amount_quote_sell_percent,
    curve_progress_open,
    curve_progress_open_change,
    curve_progress_high,
    curve_progress_high_change,
    curve_progress_low,
    curve_progress_low_change,
    curve_progress_close,
    curve_progress_close_change,
    curve_progress_avg,
    curve_progress_avg_change,
    market_cap_open,
    market_cap_open_usd,
    market_cap_open_change,
    market_cap_open_usd_change,
    market_cap_open_percent,
    market_cap_high,
    market_cap_high_usd,
    market_cap_high_change,
    market_cap_high_usd_change,
    market_cap_high_percent,
    market_cap_low,
    market_cap_low_usd,
    market_cap_low_change,
    market_cap_low_usd_change,
    market_cap_low_percent,
    market_cap_close,
    market_cap_close_usd,
    market_cap_close_change,
    market_cap_close_usd_change,
    market_cap_close_percent,
    market_cap_avg,
    market_cap_avg_usd,
    market_cap_avg_change,
    market_cap_avg_usd_change,
    market_cap_avg_percent,
    price_open,
    price_open_usd,
    price_open_change,
    price_open_usd_change,
    price_open_percent,
    price_high,
    price_high_usd,
    price_high_change,
    price_high_usd_change,
    price_high_percent,
    price_low,
    price_low_usd,
    price_low_change,
    price_low_usd_change,
    price_low_percent,
    price_close,
    price_close_usd,
    price_close_change,
    price_close_usd_change,
    price_close_percent,
    price_avg,
    price_avg_usd,
    price_avg_change,
    price_avg_usd_change,
    price_avg_percent,
    trade,
    trade_change,
    trade_percent,
    trade_buy,
    trade_buy_change,
    trade_buy_percent,
    trade_sell,
    trade_sell_change,
    trade_sell_percent,
    volume,
    volume_usd,
    volume_change,
    volume_usd_change,
    volume_percent,
    volume_buy,
    volume_buy_usd,
    volume_buy_change,
    volume_buy_usd_change,
    volume_buy_percent,
    volume_sell,
    volume_sell_usd,
    volume_sell_change,
    volume_sell_usd_change,
    volume_sell_percent
)
select
    current.token_pair_id,
    
    current.amount_base as amount_base,
    current.amount_base - previous.amount_base as amount_base_change,
    (current.amount_base - previous.amount_base) / nullif(previous.amount_base, 0) * 100 as amount_base_percent,

    current.amount_base_buy as amount_base_buy,
    current.amount_base_buy - previous.amount_base_buy as amount_base_buy_change,
    (current.amount_base_buy - previous.amount_base_buy) / nullif(previous.amount_base_buy, 0) * 100 as amount_base_buy_percent,

    current.amount_base_sell as amount_base_sell,
    current.amount_base_sell - previous.amount_base_sell as amount_base_sell_change,
    (current.amount_base_sell - previous.amount_base_sell) / nullif(previous.amount_base_sell, 0) * 100 as amount_base_sell_percent,

    current.amount_quote as amount_quote,
    current.amount_quote - previous.amount_quote as amount_quote_change,
    (current.amount_quote - previous.amount_quote) / nullif(previous.amount_quote, 0) * 100 as amount_quote_percent,

    current.amount_quote_buy as amount_quote_buy,
    current.amount_quote_buy - previous.amount_quote_buy as amount_quote_buy_change,
    (current.amount_quote_buy - previous.amount_quote_buy) / nullif(previous.amount_quote_buy, 0) * 100 as amount_quote_buy_percent,

    current.amount_quote_sell as amount_quote_sell,
    current.amount_quote_sell - previous.amount_quote_sell as amount_quote_sell_change,
    (current.amount_quote_sell - previous.amount_quote_sell) / nullif(previous.amount_quote_sell, 0) * 100 as amount_quote_sell_percent,

    current.curve_progress_open as curve_progress_open,
    current.curve_progress_open - previous.curve_progress_open as curve_progress_open_change,

    current.curve_progress_high as curve_progress_high,
    current.curve_progress_high - previous.curve_progress_high as curve_progress_high_change,

    current.curve_progress_low as curve_progress_low,
    current.curve_progress_low - previous.curve_progress_low as curve_progress_low_change,

    current.curve_progress_close as curve_progress_close,
    current.curve_progress_close - previous.curve_progress_close as curve_progress_close_change,

    current.curve_progress_avg as curve_progress_avg,
    current.curve_progress_avg - previous.curve_progress_avg as curve_progress_avg_change,

    current.market_cap_open as market_cap_open,
    current.market_cap_open_usd as market_cap_open_usd,
    current.market_cap_open - previous.market_cap_open as market_cap_open_change,
    current.market_cap_open_usd - previous.market_cap_open_usd as market_cap_open_usd_change,
    (current.market_cap_open - previous.market_cap_open) / nullif(previous.market_cap_open, 0) * 100 as market_cap_open_percent,

    current.market_cap_high as market_cap_high,
    current.market_cap_high_usd as market_cap_high_usd,
    current.market_cap_high - previous.market_cap_high as market_cap_high_change,
    current.market_cap_high_usd - previous.market_cap_high_usd as market_cap_high_usd_change,
    (current.market_cap_high - previous.market_cap_high) / nullif(previous.market_cap_high, 0) * 100 as market_cap_high_percent,

    current.market_cap_low as market_cap_low,
    current.market_cap_low_usd as market_cap_low_usd,
    current.market_cap_low - previous.market_cap_low as market_cap_low_change,
    current.market_cap_low_usd - previous.market_cap_low_usd as market_cap_low_usd_change,
    (current.market_cap_low - previous.market_cap_low) / nullif(previous.market_cap_low, 0) * 100 as market_cap_low_percent,

    current.market_cap_close as market_cap_close,
    current.market_cap_close_usd as market_cap_close_usd,
    current.market_cap_close - previous.market_cap_close as market_cap_close_change,
    current.market_cap_close_usd - previous.market_cap_close_usd as market_cap_close_usd_change,
    (current.market_cap_close - previous.market_cap_close) / nullif(previous.market_cap_close, 0) * 100 as market_cap_close_percent,

    current.market_cap_avg as market_cap_avg,
    current.market_cap_avg_usd as market_cap_avg_usd,
    current.market_cap_avg - previous.market_cap_avg as market_cap_avg_change,
    current.market_cap_avg_usd - previous.market_cap_avg_usd as market_cap_avg_usd_change,
    (current.market_cap_avg - previous.market_cap_avg) / nullif(previous.market_cap_avg, 0) * 100 as market_cap_avg_percent,

    current.price_open as price_open,
    current.price_open_usd as price_open_usd,
    current.price_open - previous.price_open as price_open_change,
    current.price_open_usd - previous.price_open_usd as price_open_usd_change,
    (current.price_open - previous.price_open) / nullif(previous.price_open, 0) * 100 as price_open_percent,

    current.price_high as price_high,
    current.price_high_usd as price_high_usd,
    current.price_high - previous.price_high as price_high_change,
    current.price_high_usd - previous.price_high_usd as price_high_usd_change,
    (current.price_high - previous.price_high) / nullif(previous.price_high, 0) * 100 as price_high_percent,

    current.price_low as price_low,
    current.price_low_usd as price_low_usd,
    current.price_low - previous.price_low as price_low_change,
    current.price_low_usd - previous.price_low_usd as price_low_usd_change,
    (current.price_low - previous.price_low) / nullif(previous.price_low, 0) * 100 as price_low_percent,

    current.price_close as price_close,
    current.price_close_usd as price_close_usd,
    current.price_close - previous.price_close as price_close_change,
    current.price_close_usd - previous.price_close_usd as price_close_usd_change,
    (current.price_close - previous.price_close) / nullif(previous.price_close, 0) * 100 as price_close_percent,

    current.price_avg as price_avg,
    current.price_avg_usd as price_avg_usd,
    current.price_avg - previous.price_avg as price_avg_change,
    current.price_avg_usd - previous.price_avg_usd as price_avg_usd_change,
    (current.price_avg - previous.price_avg) / nullif(previous.price_avg, 0) * 100 as price_avg_percent,

    current.trade as trade,
    current.trade - previous.trade as trade_change,
    (current.trade - previous.trade) / nullif(previous.trade, 0) * 100 as trade_percent,

    current.trade_buy as trade_buy,
    current.trade_buy - previous.trade_buy as trade_buy_change,
    (current.trade_buy - previous.trade_buy) / nullif(previous.trade_buy, 0) * 100 as trade_buy_percent,

    current.trade_sell as trade_sell,
    current.trade_sell - previous.trade_sell as trade_sell_change,
    (current.trade_sell - previous.trade_sell) / nullif(previous.trade_sell, 0) * 100 as trade_sell_percent,

    current.volume as volume,
    current.volume_usd as volume_usd,
    current.volume - previous.volume as volume_change,
    current.volume_usd - previous.volume_usd as volume_usd_change,
    (current.volume - previous.volume) / nullif(previous.volume, 0) * 100 as volume_percent,

    current.volume_buy as volume_buy,
    current.volume_buy_usd as volume_buy_usd,
    current.volume_buy - previous.volume_buy as volume_buy_change,
    current.volume_buy_usd - previous.volume_buy_usd as volume_buy_usd_change,
    (current.volume_buy - previous.volume_buy) / nullif(previous.volume_buy, 0) * 100 as volume_buy_percent,

    current.volume_sell as volume_sell,
    current.volume_sell_usd as volume_sell_usd,
    current.volume_sell - previous.volume_sell as volume_sell_change,
    current.volume_sell_usd - previous.volume_sell_usd as volume_sell_usd_change,
    (current.volume_sell - previous.volume_sell) / nullif(previous.volume_sell, 0) * 100 as volume_sell_percent
from
    current
left join
    previous on current.token_pair_id = previous.token_pair_id
on conflict (token_pair_id) do update set
	amount_base = excluded.amount_base,
	amount_base_change = excluded.amount_base_change,
	amount_base_percent = excluded.amount_base_percent,
	amount_base_buy = excluded.amount_base_buy,
	amount_base_buy_change = excluded.amount_base_buy_change,
	amount_base_buy_percent = excluded.amount_base_buy_percent,
	amount_base_sell = excluded.amount_base_sell,
	amount_base_sell_change = excluded.amount_base_sell_change,
	amount_base_sell_percent = excluded.amount_base_sell_percent,
	
	amount_quote = excluded.amount_quote,
	amount_quote_change = excluded.amount_quote_change,
	amount_quote_percent = excluded.amount_quote_percent,
	amount_quote_buy = excluded.amount_quote_buy,
	amount_quote_buy_change = excluded.amount_quote_buy_change,
	amount_quote_buy_percent = excluded.amount_quote_buy_percent,
	amount_quote_sell = excluded.amount_quote_sell,
	amount_quote_sell_change = excluded.amount_quote_sell_change,
	amount_quote_sell_percent = excluded.amount_quote_sell_percent,
	
	curve_progress_open = excluded.curve_progress_open,
	curve_progress_open_change = excluded.curve_progress_open_change,
	
	curve_progress_high = excluded.curve_progress_high,
	curve_progress_high_change = excluded.curve_progress_high_change,
	
	curve_progress_low = excluded.curve_progress_low,
	curve_progress_low_change = excluded.curve_progress_low_change,
	
	curve_progress_close = excluded.curve_progress_close,
	curve_progress_close_change = excluded.curve_progress_close_change,
	
	curve_progress_avg = excluded.curve_progress_avg,
	curve_progress_avg_change = excluded.curve_progress_avg_change,
	
	market_cap_open = excluded.market_cap_open,
	market_cap_open_usd = excluded.market_cap_open_usd,
	market_cap_open_change = excluded.market_cap_open_change,
	market_cap_open_usd_change = excluded.market_cap_open_usd_change,
	market_cap_open_percent = excluded.market_cap_open_percent,
	
	market_cap_high = excluded.market_cap_high,
	market_cap_high_usd = excluded.market_cap_high_usd,
	market_cap_high_change = excluded.market_cap_high_change,
	market_cap_high_usd_change = excluded.market_cap_high_usd_change,
	market_cap_high_percent = excluded.market_cap_high_percent,
	
	market_cap_low = excluded.market_cap_low,
	market_cap_low_usd = excluded.market_cap_low_usd,
	market_cap_low_change = excluded.market_cap_low_change,
	market_cap_low_usd_change = excluded.market_cap_low_usd_change,
	market_cap_low_percent = excluded.market_cap_low_percent,
	
	market_cap_close = excluded.market_cap_close,
	market_cap_close_usd = excluded.market_cap_close_usd,
	market_cap_close_change = excluded.market_cap_close_change,
	market_cap_close_usd_change = excluded.market_cap_close_usd_change,
	market_cap_close_percent = excluded.market_cap_close_percent,
	
	market_cap_avg = excluded.market_cap_avg,
	market_cap_avg_usd = excluded.market_cap_avg_usd,
	market_cap_avg_change = excluded.market_cap_avg_change,
	market_cap_avg_usd_change = excluded.market_cap_avg_usd_change,
	market_cap_avg_percent = excluded.market_cap_avg_percent,
	
	price_open = excluded.price_open,
	price_open_usd = excluded.price_open_usd,
	price_open_change = excluded.price_open_change,
	price_open_usd_change = excluded.price_open_usd_change,
	price_open_percent = excluded.price_open_percent,
	
	price_high = excluded.price_high,
	price_high_usd = excluded.price_high_usd,
	price_high_change = excluded.price_high_change,
	price_high_usd_change = excluded.price_high_usd_change,
	price_high_percent = excluded.price_high_percent,
	
	price_low = excluded.price_low,
	price_low_usd = excluded.price_low_usd,
	price_low_change = excluded.price_low_change,
	price_low_usd_change = excluded.price_low_usd_change,
	price_low_percent = excluded.price_low_percent,
	
	price_close = excluded.price_close,
	price_close_usd = excluded.price_close_usd,
	price_close_change = excluded.price_close_change,
	price_close_usd_change = excluded.price_close_usd_change,
	price_close_percent = excluded.price_close_percent,
	
	price_avg = excluded.price_avg,
	price_avg_usd = excluded.price_avg_usd,
	price_avg_change = excluded.price_avg_change,
	price_avg_usd_change = excluded.price_avg_usd_change,
	price_avg_percent = excluded.price_avg_percent,
	
	trade = excluded.trade,
	trade_change = excluded.trade_change,
	trade_percent = excluded.trade_percent,
	trade_buy = excluded.trade_buy,
	trade_buy_change = excluded.trade_buy_change,
	trade_buy_percent = excluded.trade_buy_percent,
	trade_sell = excluded.trade_sell,
	trade_sell_change = excluded.trade_sell_change,
	trade_sell_percent = excluded.trade_sell_percent,
	
	volume = excluded.volume,
	volume_usd = excluded.volume_usd,
	volume_change = excluded.volume_change,
	volume_usd_change = excluded.volume_usd_change,
	volume_percent = excluded.volume_percent,
	volume_buy = excluded.volume_buy,
	volume_buy_usd = excluded.volume_buy_usd,
	volume_buy_change = excluded.volume_buy_change,
	volume_buy_usd_change = excluded.volume_buy_usd_change,
	volume_buy_percent = excluded.volume_buy_percent,
	volume_sell = excluded.volume_sell,
	volume_sell_usd = excluded.volume_sell_usd,
	volume_sell_change = excluded.volume_sell_change,
	volume_sell_usd_change = excluded.volume_sell_usd_change,
	volume_sell_percent = excluded.volume_sell_percent
where
    {destination_table}.amount_base != excluded.amount_base or
    {destination_table}.amount_base_change != excluded.amount_base_change or
    {destination_table}.amount_base_percent != excluded.amount_base_percent or
    {destination_table}.amount_base_buy != excluded.amount_base_buy or
    {destination_table}.amount_base_buy_change != excluded.amount_base_buy_change or
    {destination_table}.amount_base_buy_percent != excluded.amount_base_buy_percent or
    {destination_table}.amount_base_sell != excluded.amount_base_sell or
    {destination_table}.amount_base_sell_change != excluded.amount_base_sell_change or
    {destination_table}.amount_base_sell_percent != excluded.amount_base_sell_percent or

    {destination_table}.amount_quote != excluded.amount_quote or
    {destination_table}.amount_quote_change != excluded.amount_quote_change or
    {destination_table}.amount_quote_percent != excluded.amount_quote_percent or
    {destination_table}.amount_quote_buy != excluded.amount_quote_buy or
    {destination_table}.amount_quote_buy_change != excluded.amount_quote_buy_change or
    {destination_table}.amount_quote_buy_percent != excluded.amount_quote_buy_percent or
    {destination_table}.amount_quote_sell != excluded.amount_quote_sell or
    {destination_table}.amount_quote_sell_change != excluded.amount_quote_sell_change or
    {destination_table}.amount_quote_sell_percent != excluded.amount_quote_sell_percent or

    {destination_table}.curve_progress_open != excluded.curve_progress_open or
    {destination_table}.curve_progress_open_change != excluded.curve_progress_open_change or

    {destination_table}.curve_progress_high != excluded.curve_progress_high or
    {destination_table}.curve_progress_high_change != excluded.curve_progress_high_change or

    {destination_table}.curve_progress_low != excluded.curve_progress_low or
    {destination_table}.curve_progress_low_change != excluded.curve_progress_low_change or

    {destination_table}.curve_progress_close != excluded.curve_progress_close or
    {destination_table}.curve_progress_close_change != excluded.curve_progress_close_change or

    {destination_table}.curve_progress_avg != excluded.curve_progress_avg or
    {destination_table}.curve_progress_avg_change != excluded.curve_progress_avg_change or

    {destination_table}.market_cap_open != excluded.market_cap_open or
    {destination_table}.market_cap_open_usd != excluded.market_cap_open_usd or
    {destination_table}.market_cap_open_change != excluded.market_cap_open_change or
    {destination_table}.market_cap_open_usd_change != excluded.market_cap_open_usd_change or
    {destination_table}.market_cap_open_percent != excluded.market_cap_open_percent or

    {destination_table}.market_cap_high != excluded.market_cap_high or
    {destination_table}.market_cap_high_usd != excluded.market_cap_high_usd or
    {destination_table}.market_cap_high_change != excluded.market_cap_high_change or
    {destination_table}.market_cap_high_usd_change != excluded.market_cap_high_usd_change or
    {destination_table}.market_cap_high_percent != excluded.market_cap_high_percent or

    {destination_table}.market_cap_low != excluded.market_cap_low or
    {destination_table}.market_cap_low_usd != excluded.market_cap_low_usd or
    {destination_table}.market_cap_low_change != excluded.market_cap_low_change or
    {destination_table}.market_cap_low_usd_change != excluded.market_cap_low_usd_change or
    {destination_table}.market_cap_low_percent != excluded.market_cap_low_percent or

    {destination_table}.market_cap_close != excluded.market_cap_close or
    {destination_table}.market_cap_close_usd != excluded.market_cap_close_usd or
    {destination_table}.market_cap_close_change != excluded.market_cap_close_change or
    {destination_table}.market_cap_close_usd_change != excluded.market_cap_close_usd_change or
    {destination_table}.market_cap_close_percent != excluded.market_cap_close_percent or

    {destination_table}.market_cap_avg != excluded.market_cap_avg or
    {destination_table}.market_cap_avg_usd != excluded.market_cap_avg_usd or
    {destination_table}.market_cap_avg_change != excluded.market_cap_avg_change or
    {destination_table}.market_cap_avg_usd_change != excluded.market_cap_avg_usd_change or
    {destination_table}.market_cap_avg_percent != excluded.market_cap_avg_percent or

    {destination_table}.price_open != excluded.price_open or
    {destination_table}.price_open_usd != excluded.price_open_usd or
    {destination_table}.price_open_change != excluded.price_open_change or
    {destination_table}.price_open_usd_change != excluded.price_open_usd_change or
    {destination_table}.price_open_percent != excluded.price_open_percent or

    {destination_table}.price_high != excluded.price_high or
    {destination_table}.price_high_usd != excluded.price_high_usd or
    {destination_table}.price_high_change != excluded.price_high_change or
    {destination_table}.price_high_usd_change != excluded.price_high_usd_change or
    {destination_table}.price_high_percent != excluded.price_high_percent or

    {destination_table}.price_low != excluded.price_low or
    {destination_table}.price_low_usd != excluded.price_low_usd or
    {destination_table}.price_low_change != excluded.price_low_change or
    {destination_table}.price_low_usd_change != excluded.price_low_usd_change or
    {destination_table}.price_low_percent != excluded.price_low_percent or

    {destination_table}.price_close != excluded.price_close or
    {destination_table}.price_close_usd != excluded.price_close_usd or
    {destination_table}.price_close_change != excluded.price_close_change or
    {destination_table}.price_close_usd_change != excluded.price_close_usd_change or
    {destination_table}.price_close_percent != excluded.price_close_percent or

    {destination_table}.price_avg != excluded.price_avg or
    {destination_table}.price_avg_usd != excluded.price_avg_usd or
    {destination_table}.price_avg_change != excluded.price_avg_change or
    {destination_table}.price_avg_usd_change != excluded.price_avg_usd_change or
    {destination_table}.price_avg_percent != excluded.price_avg_percent or
    
    {destination_table}.trade != excluded.trade or
    {destination_table}.trade_change != excluded.trade_change or
    {destination_table}.trade_percent != excluded.trade_percent or
    {destination_table}.trade_buy != excluded.trade_buy or
    {destination_table}.trade_buy_change != excluded.trade_buy_change or
    {destination_table}.trade_buy_percent != excluded.trade_buy_percent or
    {destination_table}.trade_sell != excluded.trade_sell or
    {destination_table}.trade_sell_change != excluded.trade_sell_change or
    {destination_table}.trade_sell_percent != excluded.trade_sell_percent or

    {destination_table}.volume != excluded.volume or
    {destination_table}.volume_usd != excluded.volume_usd or
    {destination_table}.volume_change != excluded.volume_change or
    {destination_table}.volume_usd_change != excluded.volume_usd_change or
    {destination_table}.volume_percent != excluded.volume_percent or
    {destination_table}.volume_buy != excluded.volume_buy or
    {destination_table}.volume_buy_usd != excluded.volume_buy_usd or
    {destination_table}.volume_buy_change != excluded.volume_buy_change or
    {destination_table}.volume_buy_usd_change != excluded.volume_buy_usd_change or
    {destination_table}.volume_buy_percent != excluded.volume_buy_percent or
    {destination_table}.volume_sell != excluded.volume_sell or
    {destination_table}.volume_sell_usd != excluded.volume_sell_usd or
    {destination_table}.volume_sell_change != excluded.volume_sell_change or
    {destination_table}.volume_sell_usd_change != excluded.volume_sell_usd_change or
    {destination_table}.volume_sell_percent != excluded.volume_sell_percent
"#
    );

    let _ = sqlx::query(&query_str).execute(&mut **tx).await?;
    Ok(())
}
