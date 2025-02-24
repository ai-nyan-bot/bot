// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::fmt::{Display, Formatter};

use solana_sdk::program_error::ProgramError;

#[derive(Debug)]
pub enum Error {
    ProgramError { message: String },
}

impl From<ProgramError> for Error {
    fn from(value: ProgramError) -> Self {
        Self::ProgramError {
            message: value.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ProgramError { message } => f.write_fmt(format_args!("ProgramError: {message}")),
        }
    }
}

impl std::error::Error for Error {}
