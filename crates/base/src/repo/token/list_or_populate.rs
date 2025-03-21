// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{Mint, Token};
use crate::repo::token::shared::find_missing_mints;
use crate::repo::TokenRepo;
use common::repo::{RepoResult, Tx};

impl TokenRepo {
    pub async fn list_or_populate<'a>(
        &self,
        tx: &mut Tx<'a>,
        mints: impl IntoIterator<Item = impl Into<Mint>> + Send,
    ) -> RepoResult<Vec<Token>> {
        let mints = mints
            .into_iter()
            .map(|mint| mint.into())
            .collect::<Vec<_>>();

        let mut result = vec![];
        result.extend(self.list_by_mints(tx, mints.clone()).await?);

        let to_insert = find_missing_mints(&mints, &result);
        if !to_insert.is_empty() {
            let mut inserted = self.populate_token(tx, &to_insert).await?;

            self.populate_cache(inserted.iter()).await;
            result.append(&mut inserted);
        }

        result.sort_by(|l, r| l.id.cmp(&r.id));
        Ok(result)
    }
}
