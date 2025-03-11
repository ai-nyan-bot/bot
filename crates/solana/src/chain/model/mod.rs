// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use crate::pumpfun::model::summary::*;
pub use account::*;
use common::model::BlockTime;
pub use indexer::Indexer;
use serde::{Deserialize, Serialize};
pub use signature::*;
pub use slot::*;
pub use transaction::*;

mod account;
mod indexer;
mod signature;
mod slot;
mod transaction;

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    pub slot: Slot,
    pub timestamp: BlockTime,
    pub transactions: Vec<Transaction>,
}
