// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Signature, Slot, Transaction};
use solana_sdk::bs58;
use solana_sdk::instruction::CompiledInstruction;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::UiInstruction::{Compiled, Parsed};
use solana_transaction_status::{EncodedTransactionWithStatusMeta, InnerInstruction, InnerInstructions};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    DecodingFailed(String),
    UnsupportedTokenPair,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::DecodingFailed(e) => f.write_fmt(format_args!("decoding failed: {}", e)),
            ParseError::UnsupportedTokenPair => f.write_str("unsupported token pair"),
        }
    }
}

impl std::error::Error for ParseError {}

pub type ParseResult<T> = Result<Vec<T>, ParseError>;

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

impl TryFrom<(Slot, EncodedTransactionWithStatusMeta)> for TransactionToParse {
    type Error = ParseError;

    // FIXME no unwrap in here
    fn try_from(value: (Slot, EncodedTransactionWithStatusMeta)) -> Result<Self, Self::Error> {
        let (slot, value) = value;
        let vtx = value.transaction.decode().unwrap();
        let meta = value.meta.unwrap();
        let instructions = vtx
            .message
            .instructions()
            .iter()
            .map(|i| CompiledInstruction {
                program_id_index: i.program_id_index,
                accounts: i.accounts.clone(),
                data: i.data.clone(),
            })
            .collect();

        let inner_instructions = match meta.inner_instructions {
            OptionSerializer::Some(inners) => inners
                .into_iter()
                .map(|i| {
                    let inner_ixs: Vec<InnerInstruction> = i
                        .instructions
                        .into_iter()
                        .map(|ii| match ii {
                            Compiled(c) => InnerInstruction {
                                instruction: CompiledInstruction {
                                    program_id_index: c.program_id_index,
                                    accounts: c.accounts,
                                    data: bs58::decode(c.data).into_vec().unwrap(),
                                },
                                stack_height: c.stack_height,
                            },
                            Parsed(_) => unimplemented!(),
                        })
                        .collect::<Vec<_>>();

                    InnerInstructions {
                        index: i.index,
                        instructions: inner_ixs,
                    }
                })
                .collect(),
            OptionSerializer::None => vec![],
            OptionSerializer::Skip => vec![],
        };

        let log_messages = match meta.log_messages {
            OptionSerializer::Some(l) => l,
            OptionSerializer::None => vec![],
            OptionSerializer::Skip => vec![],
        };

        let account_keys = vtx.message.static_account_keys().to_vec();

        Ok(TransactionToParse {
            slot: slot.into(),
            signature: Signature(vtx.signatures[0].to_string()),
            account_keys,
            instructions,
            inner_instructions,
            log_messages,
        })
    }
}
