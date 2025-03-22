// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use insert_sol::SolBalanceToInsert;
pub use insert_token::TokenBalanceToInsert;

mod insert_sol;
mod insert_token;

#[derive(Clone, Default)]
pub struct BalanceRepo {}

impl BalanceRepo {
    pub fn new() -> Self {
        Self {}
    }
}
