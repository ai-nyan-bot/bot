// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Signature, Slot, Transaction};
use common::ReaderError;
use log::error;
use solana_sdk::instruction::CompiledInstruction;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::InnerInstructions;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    DecodingFailed,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::DecodingFailed => f.write_str("decoding failed"),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<ReaderError> for ParseError {
    fn from(_value: ReaderError) -> Self {
        Self::DecodingFailed
    }
}

pub(crate) fn log_and_return_parse_error<'a>(
    err: ParseError,
    signature: &'a Signature,
    name: &'a str,
) -> ParseError {
    error!("Failed to parse {name} of {}: {err}", signature);
    err
}

pub type ParseResult<T> = Result<T, ParseError>;

pub trait Parser<T> {
    fn parse(&self, tx: &Transaction) -> ParseResult<T>;
}

#[derive(Debug)]
pub struct TransactionToParse {
    pub slot: Slot,
    pub signature: Signature,
    pub account_keys: Vec<Pubkey>,
    pub instructions: Vec<CompiledInstruction>,
    pub inner_instructions: Vec<InnerInstructions>,
    pub log_messages: Vec<String>,
}
