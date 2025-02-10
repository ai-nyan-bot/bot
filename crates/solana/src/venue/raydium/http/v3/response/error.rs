// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/0xcrust/raydium-swap (MIT License).
// Original MIT License Copyright (c) 0xcrust 2024.


use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ErrorResponse {
    pub id: String,
    pub success: bool,
    pub msg: String,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Received error response from API: {}",
            self.msg
        ))
    }
}

impl std::error::Error for ErrorResponse {}
