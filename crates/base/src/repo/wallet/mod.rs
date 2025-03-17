// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::crypt::SecretKey;
use common::model::Limit;
pub use create::*;

mod count;
mod create;
mod get;
mod list;

pub struct WalletQueryAll {
    pub limit: Limit,
}

#[derive(Clone)]
pub struct WalletRepo {
    pub secret: SecretKey,
}

impl Default for WalletRepo {
    fn default() -> Self {
        Self {
            secret: SecretKey::from(
                "3d7948d31771b3924dbeec3de83d905580d988c84964a6afd4c9cedd06776e91",
            ),
        }
    }
}
