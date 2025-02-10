// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{UserId, Wallet};
use common::model::KeyPair;
use common::model::{Count, Limit};
use common::repo::{RepoResult, Tx};

use base::repo::{WalletCreateCmd, WalletQueryAll, WalletRepo};

const WALLET_REPO: WalletRepo = WalletRepo {};

pub async fn create_wallet<'a>(tx: &mut Tx<'a>, user_id: impl Into<UserId>) -> RepoResult<Wallet> {
    let keypair = KeyPair::generate();
    WALLET_REPO
        .create(
            tx,
            WalletCreateCmd {
                user_id: user_id.into(),
                solana_public_key: keypair.public,
                solana_private_key: keypair.private,
            },
        )
        .await
}

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    WALLET_REPO.count(tx, WalletQueryAll { limit: Limit::max() }).await.unwrap()
}

pub async fn list_all<'a>(tx: &mut Tx<'a>) -> Box<[Wallet]> {
    WALLET_REPO.list(tx, WalletQueryAll { limit: Limit::max() }).await.unwrap()
}
