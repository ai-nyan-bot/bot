// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Limit;
pub use create::*;

mod count;
mod create;
mod get;
mod list;

pub struct AuthQueryAll {
    pub limit: Limit,
}

#[derive(Clone)]
#[derive(Default)]
pub struct AuthRepo {}

impl AuthRepo {
    pub fn new() -> Self {
        Self {}
    }
}

