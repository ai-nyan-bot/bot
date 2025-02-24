// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/nhuxhr/pumpfun-rs (MIT License).
// Original MIT License Copyright (c) nhuxhr 2024.

use crate::pumpfun::util::global_pda;
use crate::pumpfun::Rpc;
use common::ByteReader;
use solana_sdk::pubkey::Pubkey;

impl Rpc {
    pub async fn get_global_info(&self) -> GlobalInfo {
        let global = global_pda();
        let account = self.client.get_account(global).await.unwrap().unwrap();
        let reader = ByteReader::new(&account.account.data);
        GlobalInfo::decode(&reader)
    }
}

#[derive(Debug, Clone)]
pub struct GlobalInfo {
    pub discriminator: u64,
    pub initialized: bool,
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
}

impl GlobalInfo {
    pub fn decode(reader: &ByteReader) -> Self {
        Self {
            discriminator: reader.read_u64().unwrap(),
            initialized: reader.read_u8().unwrap() > 0,
            authority: Pubkey::try_from(reader.read_range(32).unwrap())
                .unwrap()
                .into(),
            fee_recipient: Pubkey::try_from(reader.read_range(32).unwrap())
                .unwrap()
                .into(),
            initial_virtual_token_reserves: reader.read_u64().unwrap(),
            initial_virtual_sol_reserves: reader.read_u64().unwrap(),
            initial_real_token_reserves: reader.read_u64().unwrap(),
            token_total_supply: reader.read_u64().unwrap(),
            fee_basis_points: reader.read_u64().unwrap(),
        }
    }
}
