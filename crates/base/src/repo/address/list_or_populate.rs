// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original MIT License Copyright (c) blockworks-foundation 2024.

use crate::model::Address;
use crate::model::PublicKey;
use crate::repo::AddressRepo;
use common::repo::{RepoResult, Tx};

impl AddressRepo {
    pub async fn list_or_populate_by_keys<'a>(
        &self,
        tx: &mut Tx<'a>,
        keys: impl IntoIterator<Item = impl Into<PublicKey>> + Send,
    ) -> RepoResult<Vec<Address>> {
        let keys = keys.into_iter().map(|key| key.into()).collect::<Vec<_>>();

        let mut result = self.read_addresses_from_cache_keys(&keys).await?;

        let to_read = self.find_missing_keys(&keys, &result);
        let mut read = self.read_wallet_addresses_from_db(tx, &to_read).await?;
        self.cache.put_all(read.iter().map(|t| (t.id.clone(), t.address.clone(), t.clone()))).await;
        result.append(&mut read);

        let to_insert = self.find_missing_keys(&keys, &result);
        let mut inserted = self.insert_wallets(tx, &to_insert).await?;

        self.cache.put_all(read.iter().map(|t| (t.id.clone(), t.address.clone(), t.clone()))).await;
        result.append(&mut inserted);

        result.sort_by(|l, r| l.id.cmp(&r.id));
        Ok(result)
    }
}
