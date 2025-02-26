// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use solana_client::pubsub_client::PubsubClientError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum WsClientError {
    UrlParseError(String),
    ConnectionError(String),
    WsError(String),
    ConnectionClosed(String),
    JsonParseError(String),
    SubscribeFailed(String),
    UnexpectedMessageError(String),
    RequestFailed(String),
    RequestError(String),
    UnexpectedSubscriptionResponse(String),
    UnexpectedGetVersionResponse(String),
}

impl Display for WsClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WsClientError::UrlParseError(msg) => {
                f.write_fmt(format_args!("url parse error: {msg}"))
            }
            WsClientError::ConnectionError(msg) => {
                f.write_fmt(format_args!("connection error: {msg}"))
            }
            WsClientError::WsError(msg) => f.write_fmt(format_args!("ws error: {msg}")),
            WsClientError::ConnectionClosed(msg) => {
                f.write_fmt(format_args!("connection closed error: {msg}"))
            }
            WsClientError::JsonParseError(msg) => {
                f.write_fmt(format_args!("json parse error: {msg}"))
            }
            WsClientError::SubscribeFailed(msg) => {
                f.write_fmt(format_args!("subscribe failed error: {msg}"))
            }
            WsClientError::UnexpectedMessageError(msg) => {
                f.write_fmt(format_args!("unexpected message error: {msg}"))
            }
            WsClientError::RequestFailed(msg) => {
                f.write_fmt(format_args!("request failed error: {msg}"))
            }
            WsClientError::RequestError(msg) => f.write_fmt(format_args!("request error: {msg}")),
            WsClientError::UnexpectedSubscriptionResponse(msg) => {
                f.write_fmt(format_args!("unexpected response error: {msg}"))
            }
            WsClientError::UnexpectedGetVersionResponse(msg) => {
                f.write_fmt(format_args!("unexpected response error: {msg}"))
            }
        }
    }
}

impl std::error::Error for WsClientError {}

impl From<PubsubClientError> for WsClientError {
    fn from(value: PubsubClientError) -> Self {
        match value {
            PubsubClientError::UrlParseError(err) => WsClientError::UrlParseError(err.to_string()),
            PubsubClientError::ConnectionError(err) => {
                WsClientError::ConnectionError(err.to_string())
            }
            PubsubClientError::WsError(err) => WsClientError::WsError(err.to_string()),
            PubsubClientError::ConnectionClosed(err) => {
                WsClientError::ConnectionClosed(err.to_string())
            }
            PubsubClientError::JsonParseError(err) => {
                WsClientError::JsonParseError(err.to_string())
            }
            PubsubClientError::SubscribeFailed { reason, message } => {
                WsClientError::SubscribeFailed(format!("{reason}: {message}"))
            }
            PubsubClientError::UnexpectedMessageError(err) => {
                WsClientError::UnexpectedMessageError(err.to_string())
            }
            PubsubClientError::RequestFailed { reason, message } => {
                WsClientError::RequestFailed(format!("{reason}: {message}"))
            }
            PubsubClientError::RequestError(err) => WsClientError::RequestError(err.to_string()),
            PubsubClientError::UnexpectedSubscriptionResponse(err) => {
                WsClientError::UnexpectedSubscriptionResponse(err.to_string())
            }
            PubsubClientError::UnexpectedGetVersionResponse(err) => {
                WsClientError::UnexpectedGetVersionResponse(err.to_string())
            }
        }
    }
}
