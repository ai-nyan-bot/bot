// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Limit;
pub use create::*;

mod count;
mod create;
mod get;
mod list;

pub struct UserQueryAll {
    pub limit: Limit,
}

#[derive(Clone)]
#[derive(Default)]
pub struct UserRepo {}

