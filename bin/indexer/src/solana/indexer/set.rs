// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::indexer::IndexerRepo;
use common::repo::{RepoResult, Tx};
use solana::model::Slot;
use sqlx::query;

impl IndexerRepo {
    pub async fn set<'a>(&self, tx: &mut Tx<'a>, slot: impl Into<Slot> + Send) -> RepoResult<()> {
        query(
            r#"
insert into solana.indexer (id, slot) values (1, $1)
on conflict (id) do update set slot = $1
"#,
        )
        .bind(slot.into())
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
}
