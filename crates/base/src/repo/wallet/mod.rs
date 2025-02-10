// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Limit;
pub use count::*;
pub use create::*;
pub use get::*;
pub use list::*;

mod count;
mod create;
mod get;
mod list;

pub struct WalletQueryAll {
    pub limit: Limit,
}

#[derive(Clone)]
pub struct WalletRepo {}

impl Default for WalletRepo {
    fn default() -> Self {
        Self {}
    }
}
