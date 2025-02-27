// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{Mint, TokenPair};
use crate::repo::TokenPairRepo;
use crate::LoadTokenInfo;
use common::repo::error::RepoError;
use common::repo::{RepoResult, Tx};

impl<L: LoadTokenInfo<Mint>> TokenPairRepo<L> {
    pub async fn get_or_populate<'a>(
		&self,
		tx: &mut Tx<'a>,
		base_mint: impl Into<Mint> + Send,
		quote_mint: impl Into<Mint> + Send,
    ) -> RepoResult<TokenPair> {
        let mut result = self.list_or_populate(tx, vec![(base_mint.into(), quote_mint.into())]).await?;
        if result.is_empty() {
            return Err(RepoError::NotFound);
        }
        Ok(result.remove(0))
    }
}
