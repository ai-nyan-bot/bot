// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod global;
mod curve;

use crate::pumpfun::util::{get_curve_pda, get_global_pda};
use common::ByteReader;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use crate::pumpfun::rpc::global::GlobalAccount;

pub struct Rpc {}

impl Rpc {
    pub fn new() -> Self {
        Rpc {}
    }
}

