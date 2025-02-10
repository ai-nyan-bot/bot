// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct RpcUrl(pub String);

impl From<String> for RpcUrl {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for RpcUrl {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Display for RpcUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct WsUrl(pub String);

impl WsUrl {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for WsUrl {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for WsUrl {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Display for WsUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}
