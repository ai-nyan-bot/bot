// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::PublicKey;
use crate::model::{Address, AddressId};
use common::model::Limit;
use crate::repo::cache::Cache;
pub use count::*;
pub use get::*;
pub use get_or_populate::*;
pub use insert::*;
pub use list::*;
pub use list_or_populate::*;
use std::ops::Deref;
use std::sync::Arc;

mod count;
mod get;
mod get_or_populate;
mod insert;
mod list;
mod list_or_populate;
mod shared;

pub struct AddressQuery {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct AddressRepo(pub Arc<AddressRepoInner>);

impl Deref for AddressRepo {
    type Target = AddressRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct AddressRepoInner {
    cache: Cache<AddressId, PublicKey, Address>,
}

impl AddressRepo {
    pub fn new() -> Self {
        Self(Arc::new(AddressRepoInner { cache: Cache::default() }))
    }
}
