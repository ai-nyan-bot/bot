// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::token_info::{rewrite_ipfs, sanitize_value};
use async_trait::async_trait;
use base::model::{Amount, Decimals, Mint as TokenMint, Name, Symbol};
use base::{LoadTokenInfo, TokenInfo};
use common::model::RpcUrl;
use log::{debug, error, info};
use mpl_token_metadata::accounts::Metadata;
use serde::{Deserialize, Serialize};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Mint;
use spl_token_2022::extension::{BaseStateWithExtensions, StateWithExtensions};
use spl_type_length_value::variable_len_pack::VariableLenPack;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone)]
pub struct TokenInfoRpcLoader {
    rpc_client: Arc<RpcClient>,
}

impl TokenInfoRpcLoader {
    pub fn new(rpc_url: impl Into<RpcUrl>) -> Self {
        Self {
            rpc_client: Arc::new(RpcClient::new(rpc_url.into().to_string())),
        }
    }
}

impl Debug for TokenInfoRpcLoader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("RpcTokenInfoLoader")
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct KeyedMint {
    pub key: Pubkey,
    pub mint: Mint,
}

impl KeyedMint {
    pub fn new(key: Pubkey, mint: Mint) -> Self {
        KeyedMint { key, mint }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct TokenWithMetadata {
    pub mint: String,
    pub decimals: u8,
    pub metadata: TokenMetadata,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image: String,
}

#[async_trait]
impl LoadTokenInfo for TokenInfoRpcLoader {
    async fn load(&self, mint: TokenMint) -> Option<TokenInfo> {
        debug!("load token info: {mint}");
        
        let base_delay = Duration::from_millis(100);

        for iteration in 0..4 {
            let result = download_and_process(self.rpc_client.as_ref(), mint.clone()).await;
            if result.is_some() {
                return result;
            }

            let delay = base_delay * 2_u32.pow(iteration);
            debug!(
                "retry {} load token info {}: waiting for {:?}",
                iteration + 1,
                mint,
                delay
            );
            
            sleep(delay).await;
        }
        None
    }
}

async fn download_and_process(rpc_client: &RpcClient, mint: TokenMint) -> Option<TokenInfo> {
    let pubkey = mint.0.to_string().parse::<Pubkey>().unwrap();
    let keys: Vec<Pubkey> = vec![pubkey, Metadata::find_pda(&pubkey).0];

    match rpc_client
        .get_multiple_accounts_with_commitment(&keys, CommitmentConfig::processed())
        .await
    {
        Ok(response) => {
            let accounts = response.value;

            let Some(Some(account)) = accounts.first() else {
                error!("account not found: {mint}");
                return None;
            };

            match account.owner {
                spl_token::ID => {
                    let Ok(unpacked_mint) = Mint::unpack_from_slice(account.data.as_slice()) else {
                        error!("unable to unpack mint: {mint}");
                        return None;
                    };

                    if let Some(Some(pda)) = accounts.get(1) {
                        if let Ok(metadata) = Metadata::from_bytes(pda.data.as_slice()) {
                            return Some(TokenInfo {
                                mint: Some(mint),
                                name: Some(Name(sanitize_value(metadata.name.as_str()))),
                                symbol: Some(Symbol(sanitize_value(metadata.symbol.as_str()))),
                                decimals: Some(Decimals(unpacked_mint.decimals as i16)),
                                supply: Some(Amount(unpacked_mint.supply as i64)),
                                description: None,
                                metadata: Some(rewrite_ipfs(sanitize_value(metadata.uri)).into()),
                                image: None,
                                website: None,
                            });
                        };
                    }

                    Some(TokenInfo {
                        mint: Some(mint),
                        name: None,
                        symbol: None,
                        decimals: Some(Decimals(unpacked_mint.decimals as i16)),
                        supply: Some(Amount(unpacked_mint.supply as i64)),
                        description: None,
                        metadata: None,
                        image: None,
                        website: None,
                    })
                }
                spl_token_2022::ID => {
                    let Ok(unpacked_mint) =
                        StateWithExtensions::<spl_token_2022::state::Mint>::unpack(
                            account.data.as_slice(),
                        )
                    else {
                        error!("unable to unpack mint: {mint}");
                        return None;
                    };

                    let Ok(metadata) = unpacked_mint
                        .get_extension_bytes::<spl_token_metadata_interface::state::TokenMetadata>()
                        .and_then(
                            spl_token_metadata_interface::state::TokenMetadata::unpack_from_slice,
                        )
                    else {
                        info!("unable to unpack extension: {mint}");
                        return Some(TokenInfo {
                            mint: Some(mint),
                            name: None,
                            symbol: None,
                            decimals: Some(Decimals(unpacked_mint.base.decimals as i16)),
                            supply: Some(Amount(unpacked_mint.base.supply as i64)),
                            description: None,
                            metadata: None,
                            image: None,
                            website: None,
                        });
                    };

                    Some(TokenInfo {
                        mint: Some(mint),
                        name: Some(Name(sanitize_value(metadata.name.as_str()))),
                        symbol: Some(Symbol(sanitize_value(metadata.symbol.as_str()))),
                        decimals: Some(Decimals(unpacked_mint.base.decimals as i16)),
                        supply: Some(Amount(unpacked_mint.base.supply as i64)),
                        description: None,
                        metadata: Some(rewrite_ipfs(sanitize_value(metadata.uri)).into()),
                        image: None,
                        website: None,
                    })
                }
                _ => {
                    error!("token owner not supported {mint}");
                    None
                }
            }
        }
        Err(err) => {
            error!("failed to get accounts: {err}");
            None
        }
    }
}
