// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::token_info::{LoadTokenInfo, TokenInfo};
use async_trait::async_trait;
use common::model::RpcUrl;
use common::model::{Decimals, TokenMint, TokenName, TokenSymbol};
use log::{debug, error, info};
use mpl_token_metadata::accounts::Metadata;
use serde::{Deserialize, Serialize};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::account::ReadableAccount;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Mint;
use spl_token_2022::extension::{BaseStateWithExtensions, StateWithExtensions};
use spl_type_length_value::variable_len_pack::VariableLenPack;
use std::cmp::min;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Clone)]
pub struct RpcTokenInfoLoader {
    rpc_client: Arc<RpcClient>,
}

impl RpcTokenInfoLoader {
    pub fn new(rpc_url: impl Into<RpcUrl>) -> Self {
        Self {
            rpc_client: Arc::new(RpcClient::new(rpc_url.into().to_string())),
        }
    }
}

impl Debug for RpcTokenInfoLoader {
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
impl LoadTokenInfo for RpcTokenInfoLoader {
    async fn load(&self, mint: impl Into<TokenMint> + Send) -> Option<TokenInfo> {
        let mint = mint.into();
        debug!("Load token info: {mint}");

        let pubkey = mint.0.to_string().parse::<Pubkey>().unwrap();
        let keys: Vec<Pubkey> = vec![pubkey, Metadata::find_pda(&pubkey).0];

        match self
            .rpc_client
            .get_multiple_accounts_with_commitment(&keys, CommitmentConfig::processed())
            .await
        {
            Ok(response) => {
                let accounts = response.value;

                let Some(Some(account)) = accounts.get(0) else {
                    error!("account not found: {mint}");
                    return None;
                };

                match account.owner {
                    spl_token::ID => {
                        let Ok(unpacked_mint) = Mint::unpack_from_slice(account.data.as_slice()) else {
                            error!("unable to unpack mint: {mint}");
                            return None;
                        };

                        let Some(Some(pda)) = accounts.get(1) else {
                            error!("pda not found: {mint}");
                            return None;
                        };

                        let Ok(metadata) = Metadata::from_bytes(pda.data.as_slice()) else {
                            error!("metadata not found: {mint}");
                            return None;
                        };
                        Some(TokenInfo {
                            mint,
                            name: TokenName(sanitize_value(metadata.name.as_str())),
                            symbol: TokenSymbol(sanitize_value(metadata.symbol.as_str())),
                            decimals: Decimals(unpacked_mint.decimals as i16),
                        })
                    }
                    spl_token_2022::ID => {
                        let Ok(unpacked_mint) = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(account.data.as_slice()) else {
                            error!("unable to unpack mint: {mint}");
                            return None;
                        };

                        let Ok(metadata) = unpacked_mint
                            .get_extension_bytes::<spl_token_metadata_interface::state::TokenMetadata>()
                            .and_then(spl_token_metadata_interface::state::TokenMetadata::unpack_from_slice)
                        else {
                            info!("unable to unpack extension: {mint}");
                            return Some(TokenInfo {
                                mint,
                                name: "".into(),
                                symbol: "".into(),
                                decimals: Decimals(unpacked_mint.base.decimals as i16),
                            });
                        };

                        Some(TokenInfo {
                            mint,
                            name: TokenName(sanitize_value(metadata.name.as_str())),
                            symbol: TokenSymbol(sanitize_value(metadata.symbol.as_str())),
                            decimals: Decimals(unpacked_mint.base.decimals as i16),
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
}

fn sanitize_value(value: &str) -> String {
    value.replace("\0", "").trim().to_string()
}

#[cfg(test)]
mod tests {
	use crate::token_info::rpc::sanitize_value;

	#[test]
    fn sanitize_value_success() {
        assert_eq!(sanitize_value(" BOMBO \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"), "BOMBO")
    }
}
