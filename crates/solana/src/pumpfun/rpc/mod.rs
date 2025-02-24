// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::rpc::RpcClient;

pub use curve::{CurveInfo, LoadCurveInfo};

mod curve;
mod global;

#[derive(Clone)]
pub struct Rpc {
    client: RpcClient,
}

impl Rpc {
    pub fn new(client: RpcClient) -> Self {
        Self { client }
    }
}
