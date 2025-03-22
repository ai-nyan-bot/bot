// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::rpc::{RpcClient, RpcResult};
use base::model::solana::Slot;

impl RpcClient {
    pub async fn slot(&self) -> RpcResult<Slot> {
        Ok(self.delegate.get_slot().await?.into())
    }
}
