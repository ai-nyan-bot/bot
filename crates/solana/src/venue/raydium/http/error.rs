// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::venue::raydium::http::v3::response::error::ErrorResponse;
use solana_client::client_error::reqwest;
use solana_client::client_error::reqwest::Error;

#[derive(Debug)]
pub enum HttpError {
    DeserializationError { message: String },
    NetworkError { message: String },
    ResponseError { id: String, message: String },
    UnexpectedResponse,
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpError::DeserializationError { message } => f.write_fmt(format_args!("Deserialization failed: {}", message)),
            HttpError::NetworkError { message } => f.write_fmt(format_args!("Network request failed: {}", message)),
            HttpError::ResponseError { id, message } => f.write_fmt(format_args!("Received error response raydium: {} - {}", id, message)),
            HttpError::UnexpectedResponse => f.write_str("Unexpected response"),
        }
    }
}

impl std::error::Error for HttpError {}

impl From<ErrorResponse> for HttpError {
    fn from(value: ErrorResponse) -> Self {
        HttpError::ResponseError {
            id: value.id,
            message: value.msg,
        }
    }
}

impl From<reqwest::Error> for HttpError {
    fn from(value: Error) -> Self {
        HttpError::NetworkError { message: value.to_string() }
    }
}
