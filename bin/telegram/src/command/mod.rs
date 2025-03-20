// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub(crate) use balance::balance;
pub(crate) use help::help;
pub(crate) use rules::rules;
pub(crate) use start::start;
pub(crate) use token::token;
pub(crate) use wallet::wallet;

mod balance;
mod help;
mod rules;
mod start;
mod token;
mod wallet;

pub type CommandResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
