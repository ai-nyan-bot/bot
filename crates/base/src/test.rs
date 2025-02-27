// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Decimals, Description, Mint, Name, Supply, Symbol, Uri};
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
impl LoadTokenInfo<Mint> for SuccessfulTokenInfoLoader {
    async fn load(&self, mint: impl Into<Mint> + Send) -> Option<TokenInfo> {
        let mut lock = self.0.counter.lock().unwrap();
        let counter = lock.get_mut();
        *counter += 1;

        Some(TokenInfo {
            mint: Some(mint.into()),
            name: Some(Name::new(counter.to_string())),
            symbol: Some(Symbol::new(counter.to_string())),
            decimals: Some(Decimals(*counter as i16)),
            supply: Some(Supply(*counter as i64)),
            description: Some(Description(format!("Token-Description-{counter}"))),
            metadata: Some(Uri(format!("http://metadata-{counter}"))),
            image: Some(Uri(format!("http://image-{counter}"))),
            website: Some(Uri(format!("http://website-{counter}"))),
        })
    }
}

#[derive(Default)]
pub struct FailingTokenInfoLoader {}

#[async_trait]
impl LoadTokenInfo<Mint> for FailingTokenInfoLoader {
    async fn load(&self, _mint: impl Into<Mint> + Send) -> Option<TokenInfo> {
        None
    }
}

pub struct NeverCalledTokenInfoLoader {}

#[async_trait]
impl LoadTokenInfo<Mint> for NeverCalledTokenInfoLoader {
    async fn load(&self, _mint: impl Into<Mint> + Send) -> Option<TokenInfo> {
        panic!("This function shall never be called")
    }
}
