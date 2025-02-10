// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original MIT License Copyright (c) blockworks-foundation 2024.

use crate::repo::solana::token_pair::shared::find_missing_mints;
use crate::repo::solana::TokenPairRepo;
use crate::token_info::LoadTokenInfo;
use common::model::{TokenMint, TokenPair};
use common::repo::{RepoResult, Tx};

impl<L: LoadTokenInfo> TokenPairRepo<L> {
    pub async fn list_or_populate_by_mints<'a>(
        &self,
        tx: &mut Tx<'a>,
        mints: impl IntoIterator<Item = (impl Into<TokenMint>, impl Into<TokenMint>)> + Send,
    ) -> RepoResult<Vec<TokenPair>> {
        let mints = mints.into_iter().map(|(base, quote)| (base.into(), quote.into())).collect::<Vec<_>>();
        // let mut result = self.read_token_pair_mints_from_cache(tx, &mints).await?;
        //
        // let to_read = self.find_missing_mints(&mints, &result);
        // let mut read = self.read_token_pair_mints_from_db(tx, &to_read).await?;
        //
        // self.cache
        //     .put_all(read.iter().map(|pair| {
        //         (
        //             pair.id.clone(),
        //             (pair.base.mint.clone(), pair.quote.mint.clone()),
        //             CachedTokenPair {
        //                 id: pair.id.clone(),
        //                 mint: (pair.base.mint.clone(), pair.quote.mint.clone()),
        //                 base_id: pair.base.id,
        //                 quote_id: pair.quote.id,
        //             },
        //         )
        //     }))
        //     .await;
        // result.append(&mut read);
        // let mints = mints.into_iter().map(|mint| mint.into()).collect::<Vec<_>>();

        let mut result = vec![];
        result.extend(self.read.list_by_mints(tx, mints.clone()).await?);

        let to_insert = find_missing_mints(&mints, &result);
        let mut inserted = self.insert_token_pairs(tx, &to_insert).await?;
        self.read.populate_cache(inserted.iter()).await;
        // self.cache
        //     .put_all(inserted.iter().map(|pair| {
        //         (
        //             pair.id.clone(),
        //             (pair.base.mint.clone(), pair.quote.mint.clone()),
        //             CachedTokenPair {
        //                 id: pair.id.clone(),
        //                 mint: (pair.base.mint.clone(), pair.quote.mint.clone()),
        //                 base_id: pair.base.id,
        //                 quote_id: pair.quote.id,
        //             },
        //         )
        //     }))
        //     .await;

        result.append(&mut inserted);
        result.sort_by(|l, r| l.id.cmp(&r.id));
        Ok(result)
    }
}
