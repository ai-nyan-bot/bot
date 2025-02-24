// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use solana_client::client_error::ClientErrorKind;
use std::fmt::Write;

#[derive(Debug)]
pub enum RpcClientError {
    Io(String),
    Reqwest(String),
    Middleware(String),
    Rpc(String),
    Serde(String),
    Signing(String),
    Transaction(String),
    Custom(String),
}

impl std::fmt::Display for RpcClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcClientError::Io(msg) => f.write_fmt(format_args!("rpc io error: {msg}")),
            RpcClientError::Reqwest(msg) => f.write_fmt(format_args!("rpc reqwest error: {msg}")),
            RpcClientError::Middleware(msg) => f.write_fmt(format_args!("rpc middleware error: {msg}")),
            RpcClientError::Rpc(msg) => f.write_fmt(format_args!("rpc error: {msg}")),
            RpcClientError::Serde(msg) => f.write_fmt(format_args!("rpc serde error: {msg}")),
            RpcClientError::Signing(msg) => f.write_fmt(format_args!("rpc signing error: {msg}")),
            RpcClientError::Transaction(msg) => f.write_fmt(format_args!("rpc transaction error: {msg}")),
            RpcClientError::Custom(msg) => f.write_fmt(format_args!("rpc custom error: {msg}")),
        }
    }
}

impl From<solana_client::client_error::ClientError> for RpcClientError {
    fn from(value: solana_client::client_error::ClientError) -> Self {
        match value.kind {
            ClientErrorKind::Io(err) => RpcClientError::Io(err.to_string()),
            ClientErrorKind::Reqwest(err) => RpcClientError::Reqwest(err.to_string()),
            ClientErrorKind::Middleware(err) => RpcClientError::Middleware(err.to_string()),
            ClientErrorKind::RpcError(err) => RpcClientError::Rpc(err.to_string()),
            ClientErrorKind::SerdeJson(err) => RpcClientError::Serde(err.to_string()),
            ClientErrorKind::SigningError(err) => RpcClientError::Signing(err.to_string()),
            ClientErrorKind::TransactionError(err) => RpcClientError::Transaction(err.to_string()),
            ClientErrorKind::Custom(err) => RpcClientError::Custom(err.to_string()),
        }
    }
}

impl std::error::Error for RpcClientError {}
