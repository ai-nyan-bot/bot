// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{KeyPair, UserId, Wallet};
use common::model::{Count, Limit};
use common::repo::{RepoResult, Tx};
use std::convert::Into;

use base::repo::{WalletCreateCmd, WalletQueryAll, WalletRepo};
use common::crypt::{Nonce, SecretKey};

pub async fn create_wallet<'a>(tx: &mut Tx<'a>, user_id: impl Into<UserId>) -> RepoResult<Wallet> {
    let keypair = KeyPair::generate();
    WalletRepo {
        secret: SecretKey::from("3d7948d31771b3924dbeec3de83d905580d988c84964a6afd4c9cedd06776e91"),
    }
    .create(
        tx,
        WalletCreateCmd {
            user_id: user_id.into(),
            solana_public_key: keypair.public,
            solana_private_key: keypair.private,
            nonce: Nonce::generate(),
        },
    )
    .await
}

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    WalletRepo {
        secret: SecretKey::from("3d7948d31771b3924dbeec3de83d905580d988c84964a6afd4c9cedd06776e91"),
    }
    .count(tx)
    .await
    .unwrap()
}

pub async fn list_all<'a>(tx: &mut Tx<'a>) -> Box<[Wallet]> {
    WalletRepo {
        secret: SecretKey::from("3d7948d31771b3924dbeec3de83d905580d988c84964a6afd4c9cedd06776e91"),
    }
    .list(
        tx,
        WalletQueryAll {
            limit: Limit::max(),
        },
    )
    .await
    .unwrap()
}
