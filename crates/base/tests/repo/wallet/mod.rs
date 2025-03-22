// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{PrivateKey, PublicKey, UserId, Wallet, WalletId};
use base::repo::{WalletCreateCmd, WalletRepo};
use common::crypt::{Nonce, SecretKey};
use common::repo::{RepoResult, Tx};
use lazy_static::lazy_static;
use std::str::FromStr;

mod create;
mod get_by_id;
mod get_by_user_id;

lazy_static! {
    static ref PUBLIC_KEY: PublicKey =
        PublicKey::from_str("3HqBrkdtMwDzBWb5aGYX3R2e7pjxHvGDt2MLHcVG1ufp").unwrap();
    static ref PRIVATE_KEY: PrivateKey = PrivateKey::from_str(
        "2UDkPfh42ccnyW6WgahbwU7ck7zUM2xmFAJUQJPt1Cvhqqe49N2rapHagpimTPReqQhAagH6chAVxUoD7wY1Psio"
    )
    .unwrap();
}

pub(crate) async fn create_wallet(
    tx: &mut Tx<'_>,
    user_id: impl Into<UserId>,
    public_key: impl Into<PublicKey>,
    private_key: impl Into<PrivateKey>,
    nonce: Nonce,
) -> RepoResult<Wallet> {
    WalletRepo {
        secret: SecretKey::from("3d7948d31771b3924dbeec3de83d905580d988c84964a6afd4c9cedd06776e91"),
    }
    .create(
        tx,
        WalletCreateCmd {
            user_id: user_id.into(),
            public_key: public_key.into(),
            private_key: private_key.into(),
            nonce,
        },
    )
    .await
}

pub(crate) async fn get_solana_private(
    tx: &mut Tx<'_>,
    wallet_id: impl Into<WalletId> + Send,
) -> RepoResult<PrivateKey> {
    WalletRepo {
        secret: SecretKey::from("3d7948d31771b3924dbeec3de83d905580d988c84964a6afd4c9cedd06776e91"),
    }
    .get_private_key(tx, wallet_id)
    .await
}
