// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::{Mint, Token};
use crate::repo::TokenRepo;
use common::repo::error::RepoError;
use common::repo::{RepoResult, Tx};

impl TokenRepo {
    pub async fn get_by_mint<'a>(
        &self,
        tx: &mut Tx<'a>,
        mint: impl Into<Mint> + Send,
    ) -> RepoResult<Token> {
        let mut result = self.list_by_mints(tx, vec![mint.into()]).await?;
        if result.is_empty() {
            return Err(RepoError::NotFound);
        }
        Ok(result.remove(0))
    }
}
