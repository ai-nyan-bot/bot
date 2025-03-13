// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::chain::repo::SolRepo;
use common::repo::{RepoResult, Tx};

impl SolRepo {
    pub async fn calculate_1m<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<()> {
        calculate(tx, 1, "minute", "twap_1m", "sol_price_1m").await
    }
    pub async fn calculate_5m<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<()> {
        calculate(tx, 5, "minute", "twap_5m", "sol_price_5m").await
    }
    pub async fn calculate_15m<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<()> {
        calculate(tx, 15, "minute", "twap_15m", "sol_price_15m").await
    }
    pub async fn calculate_1h<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<()> {
        calculate(tx, 1, "hour", "twap_1h", "sol_price_1h").await
    }
    pub async fn calculate_6h<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<()> {
        calculate(tx, 6, "hour", "twap_6h", "sol_price_6h").await
    }
    pub async fn calculate_1d<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<()> {
        calculate(tx, 1, "day", "twap_1d", "sol_price_1d").await
    }
}

async fn calculate<'a>(
    tx: &mut Tx<'a>,
    window: usize,
    time_unit: &str,
    twap_source_table: &str,
    destination_table: &str,
) -> RepoResult<()> {
    let query_str = format!(
        r#"
with
last_price_ts as (
    select coalesce(
        (select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts
         from solana.{destination_table}
         order by timestamp desc
         limit 1),
        '1900-01-01 00:00:00'::timestamp) as ts
),
next_twap_ts as (
    select date_trunc('{time_unit}', timestamp) - (extract({time_unit} from timestamp)::int % {window}) * interval '1 {time_unit}' as ts
    from jupiter.{twap_source_table}
    where timestamp > (select ts from last_price_ts)
    order by timestamp
    limit 1
),
timestamp as (
    select
        coalesce((select ts from next_twap_ts), (select ts from last_price_ts)) - interval '{window} {time_unit}' as start_ts,
        coalesce((select ts from next_twap_ts), (select ts from last_price_ts)) + interval '3 days' as end_ts
)
insert into solana.{destination_table} (
    timestamp,
    usd
)
select timestamp, avg(twap) as usd from jupiter.{twap_source_table}
-- 1,2 wsol/usdt wsol/usdc
where token_pair_id in(1,2) and
      timestamp >= (select start_ts from timestamp) and
      timestamp < (select end_ts from timestamp)
group by timestamp
on conflict (timestamp)
do update set
    usd = excluded.usd
where (solana.{destination_table}.usd is distinct from excluded.usd );
"#
    );
    let _ = sqlx::query(&query_str).execute(&mut **tx).await?;
    Ok(())
}
