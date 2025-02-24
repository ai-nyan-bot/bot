// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Count;
use common::repo::Tx;
use solana::pumpfun::repo::ReadTradeRepo;

pub async fn count_all_trades<'a>(tx: &mut Tx<'a>) -> Count {
    ReadTradeRepo::new().count_all(tx).await.unwrap()
}
