// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::venue::pumpfun::util::{get_bonding_curve_pda, get_global_pda};
use crate::venue::pumpfun::{BondingCurveAccount, GlobalAccount};
use common::ByteReader;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

pub struct Rpc {}

impl Rpc {
    pub fn new() -> Self {
        Rpc {}
    }
}

impl Rpc {
    pub async fn get_global_account(&self) -> GlobalAccount {
        let global: Pubkey = get_global_pda();
        let client = Arc::new(RpcClient::new("https://api.mainnet-beta.solana.com".to_string()));

        let account = client.get_account(&global).await.unwrap();
        // .map_err(error::ClientError::SolanaClientError)?;

        let reader = ByteReader::new(&account.data);
        GlobalAccount::decode(&reader)
    }

    /// Gets a token's bonding curve account data containing pricing parameters
    ///
    /// # Arguments
    ///
    /// * `mint` - Public key of the token mint
    ///
    /// # Returns
    ///
    /// Returns the deserialized BondingCurveAccount if successful, or a ClientError if the operation fails
    pub async fn get_bonding_curve_account(&self, mint: impl Into<Pubkey>) -> BondingCurveAccount {
        let client = Arc::new(RpcClient::new("https://api.mainnet-beta.solana.com".to_string()));

        let mint = mint.into();
        let bonding_curve_pda =
			// Self::get_bonding_curve_pda(mint).ok_or(error::ClientError::BondingCurveNotFound)?;
			get_bonding_curve_pda(&mint).unwrap();

        let account = client.get_account(&bonding_curve_pda).await.unwrap();
        // .map_err(error::ClientError::SolanaClientError)?;

        // accounts::BondingCurveAccount::try_from_slice(&account.data)
        //     .map_err(error::ClientError::BorshError)

        dbg!(&account);

        let reader = ByteReader::new(&account.data);
        BondingCurveAccount::decode(&reader)
    }
}
