// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::indexer::IndexerRepo;
use base::model::solana::{Indexer, Slot};
use common::model::UpdatedAt;
use common::repo::{RepoResult, Tx};
use sqlx::{query, Row};

impl IndexerRepo {
    pub async fn get<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<Indexer> {
        Ok(query("select * from solana.indexer where id = 1;")
            .fetch_one(&mut **tx)
            .await
            .map(|r| Indexer {
                slot: r.get::<Slot, _>("slot"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })?)
    }
}
