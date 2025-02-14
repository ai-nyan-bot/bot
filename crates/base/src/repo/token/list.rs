// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{Token, TokenId, TokenMint};
use crate::repo::token::shared::{find_missing_ids, find_missing_mints};
use crate::repo::ReadTokenRepo;
use common::repo::{RepoResult, Tx};

impl ReadTokenRepo {
    pub async fn list_by_ids<'a>(&self, tx: &mut Tx<'a>, ids: impl IntoIterator<Item = impl Into<TokenId>> + Send) -> RepoResult<Vec<Token>> {
        let ids = ids.into_iter().map(|id| id.into()).collect::<Vec<_>>();
        let mut result = self.read_token_ids_from_cache(&self.cache, &ids).await?;

        let to_read = find_missing_ids(&ids, &result);
        let mut read = self.read_token_ids_from_db(tx, &to_read).await?;
        self.cache.put_all(read.iter().map(|t| (t.id.clone(), t.mint.clone(), t.clone()))).await;
        result.append(&mut read);
        Ok(result)
    }
}

impl ReadTokenRepo {
    pub async fn list_by_mints<'a>(&self, tx: &mut Tx<'a>, mints: impl IntoIterator<Item = impl Into<TokenMint>> + Send) -> RepoResult<Vec<Token>> {
        let mints = mints.into_iter().map(|mint| mint.into()).collect::<Vec<_>>();
        let mut result = self.read_token_mints_from_cache(&mints).await?;

        let to_read = find_missing_mints(&mints, &result);
        let mut read = self.read_token_mints_from_db(tx, &to_read).await?;
        self.cache.put_all(read.iter().map(|t| (t.id.clone(), t.mint.clone(), t.clone()))).await;
        result.append(&mut read);

        Ok(result)
    }
}
