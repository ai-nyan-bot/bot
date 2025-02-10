// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Decimals, TokenMint, TokenName, TokenSymbol};
use crate::{LoadTokenInfo, TokenInfo};
use async_trait::async_trait;
use std::cell::UnsafeCell;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SuccessfulTokenInfoLoader(Arc<SuccessfulTokenInfoLoaderInner>);

pub struct SuccessfulTokenInfoLoaderInner {
    counter: Mutex<UnsafeCell<u16>>,
}

impl Default for SuccessfulTokenInfoLoader {
    fn default() -> Self {
        Self(Arc::new(SuccessfulTokenInfoLoaderInner {
            counter: Mutex::new(UnsafeCell::new(999)),
        }))
    }
}

#[async_trait]
impl LoadTokenInfo for SuccessfulTokenInfoLoader {
    async fn load(&self, mint: impl Into<TokenMint> + Send) -> Option<TokenInfo> {
        let mut lock = self.0.counter.lock().unwrap();
        let counter = lock.get_mut();
        *counter += 1;

        Some(TokenInfo {
            mint: mint.into(),
            name: TokenName::new(counter.to_string()),
            symbol: TokenSymbol::new(counter.to_string()),
            decimals: Decimals(*counter as i16),
        })
    }
}

pub struct FailingTokenInfoLoader {}

#[async_trait]
impl LoadTokenInfo for FailingTokenInfoLoader {
    async fn load(&self, _mint: impl Into<TokenMint> + Send) -> Option<TokenInfo> {
        None
    }
}

pub struct NeverCalledTokenInfoLoader {}

#[async_trait]
impl LoadTokenInfo for NeverCalledTokenInfoLoader {
    async fn load(&self, _mint: impl Into<TokenMint> + Send) -> Option<TokenInfo> {
        panic!("This function shall never be called")
    }
}
