// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{TokenMint, TokenPair};
use crate::repo::token_pair::shared::find_missing_mints;
use crate::repo::TokenPairRepo;
use crate::LoadTokenInfo;
use common::repo::{RepoResult, Tx};

impl<L: LoadTokenInfo> TokenPairRepo<L> {
    pub async fn list_or_populate<'a>(
        &self,
        tx: &mut Tx<'a>,
        mints: impl IntoIterator<Item = (impl Into<TokenMint>, impl Into<TokenMint>)> + Send,
    ) -> RepoResult<Vec<TokenPair>> {
        let mints = mints.into_iter().map(|(base, quote)| (base.into(), quote.into())).collect::<Vec<_>>();

        let mut result = vec![];
        result.extend(self.read.list_by_mints(tx, mints.clone()).await?);

        let to_insert = find_missing_mints(&mints, &result);
        if !to_insert.is_empty() {
            let mut inserted = self.insert_token_pairs(tx, &to_insert).await?;
            self.read.populate_cache(inserted.iter()).await;
            result.append(&mut inserted);
        }
        result.sort_by(|l, r| l.id.cmp(&r.id));
        Ok(result)
    }
}
