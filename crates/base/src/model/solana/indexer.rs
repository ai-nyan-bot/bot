// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::solana::Slot;
use common::model::UpdatedAt;

pub struct Indexer {
    pub slot: Slot,
    pub updated_at: UpdatedAt,
}
