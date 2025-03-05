// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::jupiter::repo::TwapRepo;
use common::model::{Partition, TimeUnit};
use common::repo::{RepoResult, Tx};

impl TwapRepo {
    pub async fn calculate_1m<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_twap(tx, 1, TimeUnit::Minute, "twap_1m", partition).await
    }

    pub async fn calculate_5m<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_twap(tx, 5, TimeUnit::Minute, "twap_5m", partition).await
    }

    pub async fn calculate_15m<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_twap(tx, 15, TimeUnit::Minute, "twap_15m", partition).await
    }

    pub async fn calculate_1h<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_twap(tx, 1, TimeUnit::Hour, "twap_1h", partition).await
    }

    pub async fn calculate_6h<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_twap(tx, 6, TimeUnit::Hour, "twap_6h", partition).await
    }

    pub async fn calculate_1d<'a>(&self, tx: &mut Tx<'a>, partition: Partition) -> RepoResult<()> {
        calculate_twap(tx, 1, TimeUnit::Day, "twap_1d", partition).await
    }
}

async fn calculate_twap<'a>(
    tx: &mut Tx<'a>,
    window: u32,
    time_unit: TimeUnit,
    destination_table: &str,
    partition: Partition,
) -> RepoResult<()> {
    let destination_table = format!("{}_{}", destination_table, partition);
    let total_window_seconds = time_unit.in_seconds() * window;

    let query_str = format!(
        r#"
with last_twap_ts as (
    select coalesce(
     (select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts
      from jupiter.{destination_table}
      order by timestamp desc
      limit 1),
     '1900-01-01 00:00:00'::timestamp) as ts
),
next_candle_ts as (
    select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts from jupiter.candle_1m_{partition}
        where timestamp >= (select ts from last_twap_ts) + interval '{window} {time_unit}'
        order by timestamp
        limit 1
),
timestamp as (
    select
        coalesce((select ts from  next_candle_ts), (select ts from  last_twap_ts))   as start_ts,
        coalesce((select ts from next_candle_ts), (select ts from last_twap_ts)) + interval '{window} {time_unit}' as end_ts
),
price_data as (
    select
        token_pair_id,
        timestamp,
        case
            when timestamp = max(timestamp) over (partition by token_pair_id)
                then extract(epoch from ((select end_ts from timestamp) - timestamp))
            else duration
        end as duration,
        avg
    from
        jupiter.candle_1s_{partition}
    where
        timestamp >=  (select start_ts from timestamp) and
        timestamp < (select end_ts from timestamp)
    order by
        token_pair_id, timestamp desc
),
start_offset as (
    select
        token_pair_id, min(timestamp), min(timestamp) - (select start_ts from timestamp) as start_offset
    from price_data
    group by token_pair_id
)
insert into jupiter.{destination_table} (
    token_pair_id,
    twap,
    timestamp
)
select
    pd.token_pair_id,
    sum(avg * duration) / ({total_window_seconds} - extract(epoch from so.start_offset)),
    date_trunc('{time_unit}', pd.timestamp) - (extract({time_unit} from pd.timestamp)::int % {window}) * interval '1 {time_unit}' as bucket
from
    price_data pd
join start_offset so on so.token_pair_id = pd.token_pair_id
group by
    pd.token_pair_id, bucket, so.start_offset
on conflict (token_pair_id, timestamp)
do update set twap = excluded.twap
where jupiter.{destination_table}.twap != excluded.twap;
        "#
    );

    let _ = sqlx::query(&query_str).execute(&mut **tx).await?;
    Ok(())
}
