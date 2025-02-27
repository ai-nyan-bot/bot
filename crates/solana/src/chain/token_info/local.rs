// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::token_info::{rewrite_ipfs, sanitize_value};
use async_trait::async_trait;
use base::model::{Decimals, Mint, Uri};
use base::{LoadTokenInfo, TokenInfo};
use log::warn;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

/// Legacy token list for Solana from solana-labs repo and kana, merge data if required
pub struct TokenInfoLocalLoader {
    metadata: HashMap<Mint, TokenInfo>,
}

#[async_trait]
impl LoadTokenInfo<Mint> for TokenInfoLocalLoader {
    async fn load(&self, mint: impl Into<Mint> + Send) -> Option<TokenInfo> {
        self.metadata.get(&mint.into()).map(|t| t.clone())
    }
}

#[derive(Debug, Deserialize)]
struct SolanaLegacyTokenListWrap {
    pub tokens: Vec<SolanaLegacyTokenMetadata>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct SolanaLegacyTokenMetadata {
    #[serde(rename = "address")]
    pub mint: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
    pub extensions: Option<LegacyTokenMetadataExtension>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct LegacyTokenMetadataExtension {
    pub description: Option<String>,
    pub twitter: Option<String>,
    #[serde(rename = "coingeckoId")]
    pub coingecko_id: Option<String>,
    pub website: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct KanaTokenMetadata {
    // mint address
    pub address: String,
    pub decimals: i64,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
}

impl TokenInfoLocalLoader {
    pub fn new() -> Self {
        let token_info_solana = Self::load_legacy_file();
        let token_info_kana = Self::load_kana_file();

        let mut merged_map = HashMap::new();
        for mint_addr in itertools::merge(token_info_solana.keys(), token_info_kana.keys()) {
            let solana_labs = token_info_solana.get(mint_addr);
            let kana = token_info_kana.get(mint_addr);
            let merged = merge_token_info(solana_labs, kana);
            merged_map.insert(mint_addr.clone(), merged.into_owned());
        }

        Self {
            metadata: merged_map,
        }
    }

    fn load_legacy_file() -> HashMap<Mint, TokenInfo> {
        let json = include_bytes!("legacy.json");
        let tokens = serde_json::from_slice::<SolanaLegacyTokenListWrap>(json).unwrap();

        let mut result = HashMap::new();
        for token in tokens.tokens {
            let mint: Mint = token.mint.into();
            let mapped = TokenInfo {
                mint: Some(mint.clone()),
                symbol: Some(sanitize_value(token.symbol).into()),
                name: Some(sanitize_value(token.name).into()),
                decimals: Some(token.decimals.into()),
                supply: None,
                metadata: None,
                image: Some(rewrite_ipfs(Uri::from(sanitize_value(token.logo_uri)))),
                description: token.extensions.as_ref().and_then(|e| {
                    e.description
                        .clone()
                        .map(|desc| sanitize_value(desc).into())
                }),
                website: token.extensions.as_ref().and_then(|e| {
                    e.website
                        .clone()
                        .map(|website| rewrite_ipfs(Uri::from(sanitize_value(website))))
                }),
            };

            result.insert(mint, mapped);
        }

        result
    }

    fn load_kana_file() -> HashMap<Mint, TokenInfo> {
        let json = include_bytes!("kana.json");
        let tokens = serde_json::from_slice::<Vec<KanaTokenMetadata>>(json).unwrap();

        let mut result = HashMap::new();
        for token in tokens {
            let mint: Mint = token.address.into();
            let mapped = TokenInfo {
                mint: Some(mint.clone()),
                symbol: Some(sanitize_value(token.symbol).into()),
                name: Some(sanitize_value(token.name).into()),
                decimals: Some(token.decimals.into()),
                supply: None,
                metadata: None,
                image: Some(rewrite_ipfs(sanitize_value(token.logo_uri))),
                // not provided in token list json
                description: None,
                // not provided in token list json
                website: None,
            };

            result.insert(mint, mapped);
        }

        result
    }

    pub fn load(&self, mint: impl Into<Mint>) -> Option<TokenInfo> {
        self.metadata.get(&mint.into()).map(|t| t.clone())
    }
}

fn merge_token_info<'a>(
    solana_labs: Option<&'a TokenInfo>,
    kana: Option<&'a TokenInfo>,
) -> Cow<'a, TokenInfo> {
    match (solana_labs, kana) {
        (Some(from_labs), Some(from_kana)) => {
            let same_values = from_labs.symbol == from_kana.symbol
                && from_labs.name == from_kana.name
                && from_labs.decimals == from_kana.decimals
                && from_labs.image == from_kana.image
                && from_labs.description == from_kana.description
                && from_labs.website == from_kana.website;

            let merged_decimals = merge_decimals(
                from_labs.decimals.unwrap(),
                from_kana.decimals.unwrap(),
                &from_labs.mint.as_ref().unwrap().0 .0.as_str(),
            );

            if !same_values {
                let merged_meta = TokenInfo {
                    mint: from_labs.mint.clone(),
                    symbol: from_labs.symbol.clone(),
                    name: from_kana.name.clone(),
                    // be careful, some from_labs data is wrong
                    decimals: Some(merged_decimals),
                    supply: None,
                    metadata: None,
                    image: from_labs.image.clone(),
                    description: from_labs.description.clone(),
                    website: from_labs.website.clone(),
                };
                Cow::Owned(merged_meta)
            } else {
                Cow::Borrowed(from_labs)
            }
        }
        (Some(from_labs), None) => Cow::Borrowed(from_labs),
        (None, Some(from_kana)) => Cow::Borrowed(from_kana),
        _ => unreachable!(),
    }
}

pub(crate) fn merge_decimals(from_legacy: Decimals, from_kana: Decimals, mint: &str) -> Decimals {
    let from_legacy = from_legacy.0;
    let from_kana = from_kana.0;

    if from_legacy != from_kana {
        // looking at 4h41QKUkQPd2pCAFXNNgZUyGUxQ6E7fMexaZZHziCvhh the solana-labs data is wrong
        warn!(
            "Decimals mismatch for token: {} (labs:{} kana:{}))",
            mint, from_legacy, from_kana
        );

        // prefer kana over labs
        if from_kana > 0 {
            from_kana.into()
        } else if from_legacy > 0 {
            from_legacy.into()
        } else {
            warn!("Decimals from both sources are zeroes for token: {}", mint);
            0i16.into()
        }
    } else {
        if from_legacy == 0 {
            warn!("Decimals are zero for token: {}", mint);
        }

        from_legacy.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::token_info::local::{merge_decimals, TokenInfoLocalLoader};

    #[test]
    pub fn load_legacy_token_list() {
        let test_instance = TokenInfoLocalLoader::new();
        let token_info = test_instance
            .load("HCXXtXPasqcF4BVsrPQPfHMQPUofoCbDbjsTUANFSHDR")
            .unwrap();
        assert_eq!(token_info.symbol.unwrap(), "MONKE");
        assert_eq!(token_info.name.unwrap(), "MONKE TOKEN");
        assert_eq!(token_info.image.unwrap(), "https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/HCXXtXPasqcF4BVsrPQPfHMQPUofoCbDbjsTUANFSHDR/logo.png");
    }

    #[test]
    pub fn token_only_in_kana() {
        let test_instance = TokenInfoLocalLoader::new();
        let token_info = test_instance
            .load("7atgF8KQo4wJrD5ATGX7t1V2zVvykPJbFfNeVf1icFv1")
            .unwrap();
        assert_eq!(token_info.symbol.unwrap(), "$CWIF");
        assert_eq!(token_info.name.unwrap(), "catwifhat");
        assert_eq!(
            token_info.image.unwrap(),
            "https://i.postimg.cc/d1QD417z/200x200logo-copy.jpg"
        );
    }

    #[test]
    pub fn merge_coingecko_description() {
        let test_instance = TokenInfoLocalLoader::new();
        let token_info = test_instance
            .load("PRiME7gDoiG1vGr95a3CRMv9xHY7UGjd4JKvfSkmQu2")
            .unwrap();
        assert_eq!(token_info.symbol.unwrap(), "PRIME");

        assert_eq!(token_info.description.unwrap(), "SolanaPrime utility token");
    }

    #[test]
    fn test_merge_decimals() {
        assert_eq!(merge_decimals(0.into(), 0.into(), "mint"), 0);
        assert_eq!(merge_decimals(0.into(), 2.into(), "mint"), 2);
        assert_eq!(merge_decimals(2.into(), 0.into(), "mint"), 2);
        assert_eq!(merge_decimals(2.into(), 3.into(), "mint"), 3);
    }
}
