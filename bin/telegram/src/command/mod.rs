// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub(crate) use balance::balance;
pub(crate) use rules::rules;
pub(crate) use start::start;
pub(crate) use token::token;

mod balance;
mod rules;
mod start;
mod token;

pub type CommandResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
