// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original MIT License Copyright (c) blockworks-foundation 2024.

use crate::model::Address;
use crate::model::PublicKey;
use crate::repo::AddressRepo;
use common::repo::error::RepoError;
use common::repo::{RepoResult, Tx};

impl AddressRepo {
    pub async fn get_or_populate_by_key<'a>(&self, tx: &mut Tx<'a>, keys: impl Into<PublicKey> + Send) -> RepoResult<Address> {
        let mut result = self.list_or_populate_by_keys(tx, vec![keys.into()]).await?;
        if result.is_empty() {
            return Err(RepoError::NotFound);
        }
        Ok(result.remove(0))
    }
}
