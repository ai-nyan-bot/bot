// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original MIT License Copyright (c) blockworks-foundation 2024.

use crate::repo::solana::TokenPairRepo;
use crate::token_info::LoadTokenInfo;
use common::model::{TokenMint, TokenPair};
use common::repo::error::RepoError;
use common::repo::{RepoResult, Tx};

impl<L: LoadTokenInfo> TokenPairRepo<L> {
    pub async fn get_or_populate_by_mint<'a>(
        &self,
        tx: &mut Tx<'a>,
        base_mint: impl Into<TokenMint> + Send,
        quote_mint: impl Into<TokenMint> + Send,
    ) -> RepoResult<TokenPair> {
        let mut result = self.list_or_populate_by_mints(tx, vec![(base_mint.into(), quote_mint.into())]).await?;
        if result.is_empty() {
            return Err(RepoError::NotFound);
        }
        Ok(result.remove(0))
    }
}
