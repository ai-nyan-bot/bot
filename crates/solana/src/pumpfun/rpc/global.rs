// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/nhuxhr/pumpfun-rs (MIT License).
// Original MIT License Copyright (c) nhuxhr 2024.

use crate::pumpfun::util::get_global_pda;
use crate::pumpfun::Rpc;
use common::ByteReader;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

impl Rpc {
    pub async fn get_global_account(&self) -> GlobalAccount {
        let global: Pubkey = get_global_pda();
        let client = Arc::new(RpcClient::new(
            "https://api.mainnet-beta.solana.com".to_string(),
        ));

        let account = client.get_account(&global).await.unwrap();
        // .map_err(error::ClientError::SolanaClientError)?;

        let reader = ByteReader::new(&account.data);
        GlobalAccount::decode(&reader)
    }
}

#[derive(Debug, Clone)]
pub struct GlobalAccount {
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

impl GlobalAccount {
    pub fn decode(reader: &ByteReader) -> Self {
        Self {
            discriminator: reader.read_u64().unwrap(),
            initialized: reader.read_u8().unwrap() > 1,
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
