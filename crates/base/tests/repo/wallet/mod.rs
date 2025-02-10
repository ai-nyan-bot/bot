// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{PrivateKey, PublicKey, UserId, Wallet};
use base::repo::{WalletCreateCmd, WalletRepo};
use common::repo::{RepoResult, Tx};
use lazy_static::lazy_static;
use std::str::FromStr;

mod create;
mod get;

lazy_static! {
    static ref PUBLIC_KEY: PublicKey = PublicKey::from_str("3HqBrkdtMwDzBWb5aGYX3R2e7pjxHvGDt2MLHcVG1ufp").unwrap();
    static ref PRIVATE_KEY: PrivateKey =
        PrivateKey::from_str("2UDkPfh42ccnyW6WgahbwU7ck7zUM2xmFAJUQJPt1Cvhqqe49N2rapHagpimTPReqQhAagH6chAVxUoD7wY1Psio").unwrap();
}

pub(crate) async fn create_wallet(
    tx: &mut Tx<'_>,
    user_id: impl Into<UserId>,
    solana_public_key: impl Into<PublicKey>,
    solana_private_key: impl Into<PrivateKey>,
) -> RepoResult<Wallet> {
    WalletRepo::default()
        .create(
            tx,
            WalletCreateCmd {
                user_id: user_id.into(),
                solana_public_key: solana_public_key.into(),
                solana_private_key: solana_private_key.into(),
            },
        )
        .await
}
