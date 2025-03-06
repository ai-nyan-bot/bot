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
            "pumpfun.candle_1m_most_recent",
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
			"pumpfun.candle_5m_most_recent",
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
			"pumpfun.candle_15m_most_recent",
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
			"pumpfun.candle_1h_most_recent",
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
            8,
            "hours",
			"pumpfun.candle_6h_most_recent",
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
			"pumpfun.candle_1d_most_recent",
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
    candle_most_recent_table: impl AsRef<str>,
    candle_table: impl AsRef<str>,
    candle_market_cap_table: impl AsRef<str>,
    candle_progress_table: impl AsRef<str>,
    candle_usd_table: impl AsRef<str>,
    destination_table: impl AsRef<str>,
) -> RepoResult<()> {
    let candle_most_recent_table = candle_most_recent_table.as_ref();
    let candle_table = candle_table.as_ref();
    let candle_market_cap_table = candle_market_cap_table.as_ref();
    let candle_progress_table = candle_progress_table.as_ref();
    let candle_usd_table = candle_usd_table.as_ref();
    let destination_table = destination_table.as_ref();

    let bucket_separator = window / 2;
    let query_str = format!(
        r#"
with
last_candle as (
    select timestamp from {candle_most_recent_table} order by timestamp desc limit 1
),
candles as(
	select c.token_pair_id     as token_pair_id,
		   c.timestamp         as timestamp,
	
		   c.amount_base_buy   as amount_base_buy,
		   c.amount_quote_buy  as amount_quote_buy,
		   c.amount_base_sell  as amount_base_sell,
		   c.amount_quote_sell as amount_quote_sell,
	
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
aggregated as (
    select
        token_pair_id,
        case
            when timestamp > (select timestamp from last_candle) - interval '{bucket_separator} {time_unit}' then 'current'
            else 'previous'
        end as time_bucket,
        sum(amount_base_buy) as amount_base_buy,
        sum(amount_quote_buy) as amount_quote_buy,
        sum(amount_base_buy + amount_base_sell) as amount_base,
        
        sum(amount_base_sell) as amount_base_sell,
        sum(amount_quote_sell) as amount_quote_sell,
        sum(amount_quote_buy + amount_quote_sell) as amount_quote,

		max(curve_progress_high) as curve_progress_high,
        min(curve_progress_low) as curve_progress_low,
        avg(curve_progress_avg) as curve_progress_avg,

        max(price_high) as price_high,
        max(price_high_usd) as price_high_usd,
        min(price_low) as price_low,
        min(price_low_usd) as price_low_usd,
        avg(price_avg) as price_avg,
        avg(price_avg_usd) as price_avg_usd,
        
        max(price_high) as price_high,
        max(price_high_usd) as price_high_usd,
        min(price_low) as price_low,
        min(price_low_usd) as price_low_usd,
        avg(price_avg) as price_avg,
        avg(price_avg_usd) as price_avg_usd,
        
        sum(trade_buy) as trade_buy,
        sum(trade_sell) as trade_sell,
        sum(trade_buy + trade_sell) as trade,
        
        sum(volume_buy) as volume_buy,
        sum(volume_buy_usd) as volume_buy_usd,
        sum(volume_sell) as volume_sell,
        sum(volume_sell_usd) as volume_sell_usd,
        sum(volume_buy + volume_sell) as volume,
        sum(volume_buy_usd + volume_sell_usd) as volume_usd
        
    from candles
    group by token_pair_id, time_bucket
    order by token_pair_id, time_bucket
),
current as (select * from aggregated where time_bucket = 'current'),
previous as (select * from aggregated where time_bucket = 'previous')
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
    holder_open,
    holder_open_change,
    holder_open_percent,
    holder_high,
    holder_high_change,
    holder_high_percent,
    holder_low,
    holder_low_change,
    holder_low_percent,
    holder_close,
    holder_close_change,
    holder_close_percent,
    holder_avg,
    holder_avg_change,
    holder_avg_percent,
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
    -- amount_base
    null,
    -- amount_base_change
    null,
    -- amount_base_percent
    null,
    -- amount_base_buy
    null,
    -- amount_base_buy_change
    null,
    -- amount_base_buy_percent
    null,
    -- amount_base_sell
    null,
    -- amount_base_sell_change
    null,
    -- amount_base_sell_percent
    null,

    -- amount_quote
    null,
    -- amount_quote_change
    null,
    -- amount_quote_percent
    null,
    -- amount_quote_buy
    null,
    -- amount_quote_buy_change
    null,
    -- amount_quote_buy_percent
    null,
    -- amount_quote_sell
    null,
    -- amount_quote_sell_change
    null,
    -- amount_quote_sell_percent
    null,

    -- curve_progress_open
    null,
    -- curve_progress_open_change
    null,

    -- curve_progress_high
    null,
    -- curve_progress_high_change
    null,

    -- curve_progress_low
    null,
    -- curve_progress_low_change
    null,

    -- curve_progress_close
    null,
    -- curve_progress_close_change
    null,

    -- curve_progress_avg
    null,
    -- curve_progress_avg_change
    null,

    -- holder_open
    null,
    -- holder_open_change
    null,
    -- holder_open_percent
    null,

    -- holder_high
    null,
    -- holder_high_change
    null,
    -- holder_high_percent
    null,

    -- holder_low
    null,
    -- holder_low_change
    null,
    -- holder_low_percent
    null,

    -- holder_close
    null,
    -- holder_close_change
    null,
    -- holder_close_percent
    null,

    -- holder_avg
    null,
    -- holder_avg_change
    null,
    -- holder_avg_percent
    null,

    -- market_cap_open
    null,
    -- market_cap_open_usd
    null,
    -- market_cap_open_change
    null,
    -- market_cap_open_usd_change
    null,
    -- market_cap_open_percent
    null,

    -- market_cap_high
    null,
    -- market_cap_high_usd
    null,
    -- market_cap_high_change
    null,
    -- market_cap_high_usd_change
    null,
    -- market_cap_high_percent
    null,

    -- market_cap_low
    null,
    -- market_cap_low_usd
    null,
    -- market_cap_low_change
    null,
    -- market_cap_low_usd_change
    null,
    -- market_cap_low_percent
    null,

    -- market_cap_close
    null,
    -- market_cap_close_usd
    null,
    -- market_cap_close_change
    null,
    -- market_cap_close_usd_change
    null,
    -- market_cap_close_percent
    null,

    -- market_cap_avg
    null,
    -- market_cap_avg_usd
    null,
    -- market_cap_avg_change
    null,
    -- market_cap_avg_usd_change
    null,
    -- market_cap_avg_percent
    null,

    -- price_open
    null,
    -- price_open_usd
    null,
    -- price_open_change
    null,
    -- price_open_usd_change
    null,
    -- price_open_percent
    null,

    -- price_high
    null,
    -- price_high_usd
    null,
    -- price_high_change
    null,
    -- price_high_usd_change
    null,
    -- price_high_percent
    null,

    -- price_low
    null,
    -- price_low_usd
    null,
    -- price_low_change
    null,
    -- price_low_usd_change
    null,
    -- price_low_percent
    null,

    -- price_close
    null,
    -- price_close_usd
    null,
    -- price_close_change
    null,
    -- price_close_usd_change
    null,
    -- price_close_percent
    null,

    -- price_avg
    null,
    -- price_avg_usd
    null,
    -- price_avg_change
    null,
    -- price_avg_usd_change
    null,
    -- price_avg_percent
    null,

    -- trade
    null,
    -- trade_change
    null,
    -- trade_percent
    null,
    -- trade_buy
    null,
    -- trade_buy_change
    null,
    -- trade_buy_percent
    null,
    -- trade_sell
    null,
    -- trade_sell_change
    null,
    -- trade_sell_percent
    null,

    -- volume
    null,
    -- volume_usd
    null,
    -- volume_change
    null,
    -- volume_usd_change
    null,
    -- volume_percent
    null,
    -- volume_buy
    null,
    -- volume_buy_usd
    null,
    -- volume_buy_change
    null,
    -- volume_buy_usd_change
    null,
    -- volume_buy_percent
    null,
    -- volume_sell
    null,
    -- volume_sell_usd
    null,
    -- volume_sell_change
    null,
    -- volume_sell_usd_change
    null,
    -- volume_sell_percent
    null
    
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
	
	holder_open = excluded.holder_open,
	holder_open_change = excluded.holder_open_change,
	holder_open_percent = excluded.holder_open_percent,
	
	holder_high = excluded.holder_high,
	holder_high_change = excluded.holder_high_change,
	holder_high_percent = excluded.holder_high_percent,
	
	holder_low = excluded.holder_low,
	holder_low_change = excluded.holder_low_change,
	holder_low_percent = excluded.holder_low_percent,
	
	holder_close = excluded.holder_close,
	holder_close_change = excluded.holder_close_change,
	holder_close_percent = excluded.holder_close_percent,
	
	holder_avg = excluded.holder_avg,
	holder_avg_change = excluded.holder_avg_change,
	holder_avg_percent = excluded.holder_avg_percent,
	
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

    {destination_table}.holder_open != excluded.holder_open or
    {destination_table}.holder_open_change != excluded.holder_open_change or
    {destination_table}.holder_open_percent != excluded.holder_open_percent or

    {destination_table}.holder_high != excluded.holder_high or
    {destination_table}.holder_high_change != excluded.holder_high_change or
    {destination_table}.holder_high_percent != excluded.holder_high_percent or

    {destination_table}.holder_low != excluded.holder_low or
    {destination_table}.holder_low_change != excluded.holder_low_change or
    {destination_table}.holder_low_percent != excluded.holder_low_percent or

    {destination_table}.holder_close != excluded.holder_close or
    {destination_table}.holder_close_change != excluded.holder_close_change or
    {destination_table}.holder_close_percent != excluded.holder_close_percent or

    {destination_table}.holder_avg != excluded.holder_avg or
    {destination_table}.holder_avg_change != excluded.holder_avg_change or
    {destination_table}.holder_avg_percent != excluded.holder_avg_percent or

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
