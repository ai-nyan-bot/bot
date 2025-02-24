// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::PublicKey;
use base::model::{Amount, TokenMint};

#[derive(Debug)]
pub enum Instruction {
	Trade { swaps: Vec<Jupiter6Swap>, signer: PublicKey },
}

#[derive(Debug)]
pub struct Jupiter6Swap {
	pub amm: PublicKey,
	pub input_mint: TokenMint,
	pub input_amount: Amount,
	pub output_mint: TokenMint,
	pub output_amount: Amount,
}
