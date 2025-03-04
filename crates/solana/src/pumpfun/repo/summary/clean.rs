// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::pumpfun::repo::SummaryRepo;
use common::repo::{RepoResult, Tx};

impl SummaryRepo {
	pub async fn clean_1m<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<()> {
		let query_str = r#"
with last_timestamp as (select updated_at from pumpfun.summary_1m order by updated_at desc limit 1)
delete from pumpfun.summary_1m
where updated_at < (select * from last_timestamp) - interval '1 minutes';
        "#;
		let _ = sqlx::query(query_str).execute(&mut **tx).await?;
		Ok(())
	}

	pub async fn clean_5m<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<()> {
		let query_str = r#"
with last_timestamp as (select updated_at from pumpfun.summary_5m order by updated_at desc limit 1)
delete from pumpfun.summary_5m
where updated_at < (select * from last_timestamp) - interval '5 minutes';
        "#;
		let _ = sqlx::query(query_str).execute(&mut **tx).await?;
		Ok(())
	}

	pub async fn clean_15m<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<()> {
		let query_str = r#"
with last_timestamp as (select updated_at from pumpfun.summary_15m order by updated_at desc limit 1)
delete from pumpfun.summary_15m
where updated_at < (select * from last_timestamp) - interval '15 minutes';
        "#;
		let _ = sqlx::query(query_str).execute(&mut **tx).await?;
		Ok(())
	}

	pub async fn clean_1h<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<()> {
		let query_str = r#"
with last_timestamp as (select updated_at from pumpfun.summary_1h order by updated_at desc limit 1)
delete from pumpfun.summary_1h
where updated_at < (select * from last_timestamp) - interval '1 hour';
        "#;
		let _ = sqlx::query(query_str).execute(&mut **tx).await?;
		Ok(())
	}

	pub async fn clean_6h<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<()> {
		let query_str = r#"
with last_timestamp as (select updated_at from pumpfun.summary_6h order by updated_at desc limit 1)
delete from pumpfun.summary_6h
where updated_at < (select * from last_timestamp) - interval '4 hour';
        "#;
		let _ = sqlx::query(query_str).execute(&mut **tx).await?;
		Ok(())
	}

	pub async fn clean_1d<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<()> {
		let query_str = r#"
with last_timestamp as (select updated_at from pumpfun.summary_1d order by updated_at desc limit 1)
delete from pumpfun.summary_1d
where updated_at < (select * from last_timestamp) - interval '1 day';
        "#;
		let _ = sqlx::query(query_str).execute(&mut **tx).await?;
		Ok(())
	}
}
