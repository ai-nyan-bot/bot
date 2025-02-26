// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::PublicKey;
use crate::model::{Address, AddressId};
use crate::repo::AddressRepo;
use common::repo::{RepoResult, Tx};

impl AddressRepo {
    pub async fn list_by_ids<'a>(&self, tx: &mut Tx<'a>, ids: impl IntoIterator<Item = impl Into<AddressId>> + Send) -> RepoResult<Vec<Address>> {
        let ids = ids.into_iter().map(|id| id.into()).collect::<Vec<_>>();
        let mut result = self.read_addresses_from_cache_ids(&self.cache, &ids).await?;

        let to_read = self.find_missing_ids(&ids, &result);
        let mut read = self.read_address_ids_from_db(tx, &to_read).await?;
        self.cache.put_all(read.iter().map(|t| (t.id, t.address.clone(), t.clone()))).await;
        result.append(&mut read);
        Ok(result)
    }

    pub async fn list_by_keys<'a>(&self, tx: &mut Tx<'a>, keys: impl IntoIterator<Item = impl Into<PublicKey>> + Send) -> RepoResult<Vec<Address>> {
        let keys = keys.into_iter().map(|address| address.into()).collect::<Vec<_>>();
        let mut result = self.read_addresses_from_cache_keys(&keys).await?;

        let to_read = self.find_missing_keys(&keys, &result);
        let mut read = self.read_wallet_addresses_from_db(tx, &to_read).await?;
        self.cache.put_all(read.iter().map(|t| (t.id, t.address.clone(), t.clone()))).await;
        result.append(&mut read);

        Ok(result)
    }
}
