// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use account::*;
use common::model::Timestamp;
pub use indexer::Indexer;
pub use signature::*;
pub use slot::*;
pub use summary::*;
pub use transaction::*;

mod account;
mod indexer;
pub mod jupiter;
mod signature;
mod slot;
mod summary;
mod transaction;

#[derive(Debug)]
pub struct Block {
    pub slot: Slot,
    pub timestamp: Timestamp,
    pub transactions: Vec<Transaction>,
}
