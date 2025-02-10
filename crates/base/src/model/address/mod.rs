// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::PublicKey;
pub use id::*;

mod id;

#[derive(Debug, Clone)]
pub struct Address {
    pub id: AddressId,
    pub address: PublicKey,
}
