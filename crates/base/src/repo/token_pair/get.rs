// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{TokenPair, TokenPairId};
use crate::repo::TokenPairRepo;
use common::repo::error::RepoError;
use common::repo::{RepoResult, Tx};

impl TokenPairRepo {
    pub async fn get_by_id<'a>(
        &self,
        tx: &mut Tx<'a>,
        id: impl Into<TokenPairId> + Send,
    ) -> RepoResult<TokenPair> {
        let mut result = self.list_by_ids(tx, vec![id]).await?;
        if result.is_empty() {
            return Err(RepoError::NotFound);
        }
        Ok(result.remove(0))
    }
}
