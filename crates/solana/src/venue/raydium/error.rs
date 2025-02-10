// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::fmt::{Display, Formatter, Write};

use common::model::PublicKey;

use crate::venue::raydium::http::HttpError;

#[derive(Debug)]
pub enum Error {
    HttpError { message: String },
    InputIsOutputTokenError,
    MarketNotFoundError,
    MathError,
    PoolKeysNotFoundError { market: PublicKey },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InputIsOutputTokenError => f.write_str("Input and output token are the same"),
            Error::HttpError { message } => f.write_fmt(format_args!("{message}")),
            Error::MarketNotFoundError => f.write_str("Market not found"),
            Error::MathError => f.write_str("Math error"),
            Error::PoolKeysNotFoundError { market } => f.write_fmt(format_args!("Failed to get pool keys for {}", market)),
        }
    }
}

impl std::error::Error for Error {}

impl From<HttpError> for Error {
    fn from(value: HttpError) -> Self {
        Self::HttpError { message: value.to_string() }
    }
}
