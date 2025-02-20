// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{TokenMint, TokenPair, TokenPairId};
use crate::repo::token_pair::shared::{find_missing_ids, find_missing_mints};
use crate::repo::token_pair::CachedTokenPair;
use crate::repo::ReadTokenPairRepo;
use common::repo::{RepoResult, Tx};

impl ReadTokenPairRepo {
    pub async fn list_by_ids<'a>(
        &self,
        tx: &mut Tx<'a>,
        ids: impl IntoIterator<Item = impl Into<TokenPairId>> + Send,
    ) -> RepoResult<Vec<TokenPair>> {
        let ids = ids.into_iter().map(|id| id.into()).collect::<Vec<_>>();
        let mut result = self.read_token_pair_ids_from_cache(tx, &ids).await?;

        let to_read = find_missing_ids(&ids, &result);
        let mut read = self.read_token_pair_ids_from_db(tx, &to_read).await?;

        self.cache
            .put_all(read.iter().map(|pair| {
                (
                    pair.id.clone(),
                    (pair.base.mint.clone(), pair.quote.mint.clone()),
                    CachedTokenPair {
                        id: pair.id.clone(),
                        mint: (pair.base.mint.clone(), pair.quote.mint.clone()),
                        base_id: pair.base.id,
                        quote_id: pair.quote.id,
                    },
                )
            }))
            .await;

        result.append(&mut read);

        Ok(result)
    }

    pub async fn list_by_mints<'a>(
        &self,
        tx: &mut Tx<'a>,
        mints: impl IntoIterator<Item = (impl Into<TokenMint>, impl Into<TokenMint>)> + Send,
    ) -> RepoResult<Vec<TokenPair>> {
        let mints = mints
            .into_iter()
            .map(|mint| (mint.0.into(), mint.1.into()))
            .collect::<Vec<_>>();
        let mut result = self.read_token_pair_mints_from_cache(tx, &mints).await?;

        let to_read = find_missing_mints(&mints, &result);
        let mut read = self.read_token_pair_mints_from_db(tx, &to_read).await?;

        self.cache
            .put_all(read.iter().map(|pair| {
                (
                    pair.id.clone(),
                    (pair.base.mint.clone(), pair.quote.mint.clone()),
                    CachedTokenPair {
                        id: pair.id.clone(),
                        mint: (pair.base.mint.clone(), pair.quote.mint.clone()),
                        base_id: pair.base.id,
                        quote_id: pair.quote.id,
                    },
                )
            }))
            .await;

        result.append(&mut read);

        Ok(result)
    }
}
