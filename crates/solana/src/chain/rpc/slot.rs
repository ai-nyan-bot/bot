// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Slot;
use crate::rpc::{RpcClient, RpcResult};
use log::trace;

impl RpcClient {
    pub async fn slot(&self) -> RpcResult<Slot> {
        trace!("fetch rpc slot");
        Ok(self.client.get_slot().await?.into())
    }
}
