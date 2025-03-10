// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::token_info::{rewrite_ipfs, sanitize_value};
use base::model::Uri;
use base::TokenInfo;
use log::error;
use serde::Deserialize;
use solana_client::client_error::reqwest::{Client, StatusCode};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone)]
pub struct TokenInfoUriLoader {}

impl TokenInfoUriLoader {
    pub fn new() -> Self {
        Self {}
    }
}

impl TokenInfoUriLoader {
    pub async fn load(&self, uri: Uri) -> Option<TokenInfo> {
        let downloader = JsonDownloader::new();
        let metadata: ExternalMetadata = downloader.fetch(uri.into()).await?;

        Some(TokenInfo {
            mint: metadata.mint.map(|mint| sanitize_value(mint).into()),
            name: metadata.name.map(|name| sanitize_value(name).into()),
            symbol: metadata.symbol.map(|symbol| sanitize_value(symbol).into()),
            decimals: None,
            supply: None,
            metadata: None,
            image: metadata
                .image
                .map(|img| rewrite_ipfs(Uri::from(sanitize_value(img)))),
            description: metadata
                .description
                .map(|description| sanitize_value(description).into()),
            website: metadata
                .website
                .map(|web| rewrite_ipfs(Uri::from(sanitize_value(web)))),
        })
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ExternalMetadata {
    mint: Option<String>,
    name: Option<String>,
    symbol: Option<String>,
    description: Option<String>,
    image: Option<String>,
    website: Option<String>,
}

struct JsonDownloader {
    client: Client,
    max_retries: usize,
    retry_delay: Duration,
}

impl JsonDownloader {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .connect_timeout(Duration::from_secs(5))
                .timeout(Duration::from_secs(5))
                .build()
                .expect("Failed to build client"),
            max_retries: 5,
            retry_delay: Duration::from_millis(250),
        }
    }

    pub async fn fetch<T: for<'de> Deserialize<'de>>(&self, uri: Uri) -> Option<T> {
        let mut attempts = 0;
        loop {
            match self.client.get(&uri.0).send().await {
                Err(err) => {
                    attempts += 1;
                    if attempts >= self.max_retries {
                        error!("Failed to fetch external data from {uri} after {attempts} attempts: {err}");
                        return None;
                    }
                    sleep(self.retry_delay).await;
                    continue;
                }
                Ok(response) => {
                    if response.status().is_success() {
                        return match response.json::<T>().await {
                            Ok(result) => Some(result),
                            Err(err) => {
                                error!("Failed to parse JSON from {uri}: {err}");
                                None
                            }
                        };
                    } else if matches!(
                        response.status(),
                        StatusCode::FORBIDDEN
                            | StatusCode::TOO_MANY_REQUESTS
                            | StatusCode::BAD_GATEWAY
                            | StatusCode::GONE
                            | StatusCode::NOT_FOUND
                    ) {
                        error!(
                            "Giving up fetching data from {uri} - status: {}",
                            response.status()
                        );
                        return None;
                    }
                    attempts += 1;
                    if attempts >= self.max_retries {
                        error!(
                            "Failed to fetch external data from {uri} after {attempts} attempts"
                        );
                        return None;
                    }
                    sleep(self.retry_delay).await;
                }
            }
        }
    }
}
