// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{AddressQuery, AddressRepo};
use common::model::{Count, Limit};
use common::repo::Tx;

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    let repo = AddressRepo::new();
    repo.count(tx, AddressQuery { limit: Limit::max() }).await.unwrap()
}
