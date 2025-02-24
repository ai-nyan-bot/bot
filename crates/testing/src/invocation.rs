// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::InvocationRepo;
use common::model::Count;
use common::repo::Tx;

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    InvocationRepo::new().count_all(tx).await.unwrap()
}
