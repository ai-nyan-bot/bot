// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Signature;
use base::model::{DecimalAmount, Mint, PublicKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum TransactionStatus {
    Success,
    Error(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub signature: Signature,
    pub status: TransactionStatus,
    pub balance: TransactionBalance,
    pub instructions: Vec<CompiledInstruction>,
    pub inner_instructions: Vec<InnerInstructions>,
    pub log_messages: Vec<String>,
    pub keys: Keys,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionBalance {
    pub sol: Vec<TransactionSolBalance>,
    pub token: Vec<TransactionTokenBalance>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionSolBalance {
    pub address: PublicKey,
    pub pre: DecimalAmount,
    pub post: DecimalAmount,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionTokenBalance {
    pub mint: Mint,
    pub address: PublicKey,
    pub pre: DecimalAmount,
    pub post: DecimalAmount,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Keys {
    pub static_account: Vec<PublicKey>,
    pub log_account: Vec<PublicKey>,
}

impl Keys {
    pub fn contains(&self, public_key: &PublicKey) -> bool {
        self.static_account.contains(public_key) || self.log_account.contains(public_key)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompiledInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InnerInstruction {
    pub instruction: CompiledInstruction,
    pub stack_height: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InnerInstructions {
    pub index: u8,
    pub instructions: Vec<InnerInstruction>,
}
