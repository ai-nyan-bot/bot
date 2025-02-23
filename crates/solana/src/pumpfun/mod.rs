// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/nhuxhr/pumpfun-rs (MIT License).
// Original MIT License Copyright (c) nhuxhr 2024.

pub mod model;
pub mod repo;
mod rpc;

mod buy;
pub(crate) mod constant;
mod ix;
pub mod parse;
mod sell;
pub(crate) mod util;




use common::ByteReader;
pub use crate::pumpfun::parse::PumpFunParser;
pub use rpc::Rpc;
use solana_sdk::pubkey::Pubkey;
use std::fmt::{Debug, Display, Formatter, Write};
use std::ops::{Div, Mul, Sub};


pub struct Pumpfun {}

#[derive(Debug)]
pub enum PumpfunError {
	CurveCompleted,
}

impl Display for PumpfunError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			PumpfunError::CurveCompleted => f.write_str("Curve already completed"),
		}
	}
}

impl std::error::Error for PumpfunError {}

pub type PumpfunResult<T> = std::result::Result<T, PumpfunError>;

#[derive(Debug, Clone)]
pub struct GlobalAccount {
	/// Unique identifier for the global account
	pub discriminator: u64,
	/// Whether the global account has been initialized
	pub initialized: bool,
	/// Authority that can modify global settings
	pub authority: Pubkey,
	/// Account that receives fees
	pub fee_recipient: Pubkey,
	/// Initial virtual token reserves for price calculations
	pub initial_virtual_token_reserves: u64,
	/// Initial virtual SOL reserves for price calculations
	pub initial_virtual_sol_reserves: u64,
	/// Initial actual token reserves available for trading
	pub initial_real_token_reserves: u64,
	/// Total supply of tokens
	pub token_total_supply: u64,
	/// Fee in basis points (1/100th of a percent)
	pub fee_basis_points: u64,
}

impl GlobalAccount {
	pub fn decode(reader: &ByteReader) -> Self {
		Self {
			discriminator: reader.read_u64().unwrap(),
			initialized: reader.read_u8().unwrap() > 1,
			authority: Pubkey::try_from(reader.read_range(32).unwrap())
				.unwrap()
				.into(),
			fee_recipient: Pubkey::try_from(reader.read_range(32).unwrap())
				.unwrap()
				.into(),
			initial_virtual_token_reserves: reader.read_u64().unwrap(),
			initial_virtual_sol_reserves: reader.read_u64().unwrap(),
			initial_real_token_reserves: reader.read_u64().unwrap(),
			token_total_supply: reader.read_u64().unwrap(),
			fee_basis_points: reader.read_u64().unwrap(),
		}
	}
}

/// Represents a bonding curve for token pricing and liquidity management
#[derive(Debug, Clone)]
pub struct BondingCurveAccount {
	/// Unique identifier for the bonding curve
	pub discriminator: u64,
	/// Virtual token reserves used for price calculations
	pub virtual_token_reserves: u64,
	/// Virtual SOL reserves used for price calculations
	pub virtual_sol_reserves: u64,
	/// Actual token reserves available for trading
	pub real_token_reserves: u64,
	/// Actual SOL reserves available for trading
	pub real_sol_reserves: u64,
	/// Total supply of tokens
	pub token_total_supply: u64,
	/// Whether the bonding curve is complete/finalized
	pub complete: bool,
}

impl BondingCurveAccount {
	pub fn decode(reader: &ByteReader) -> Self {
		Self {
			discriminator: reader.read_u64().unwrap(),
			virtual_token_reserves: reader.read_u64().unwrap(),
			virtual_sol_reserves: reader.read_u64().unwrap(),
			real_token_reserves: reader.read_u64().unwrap(),
			real_sol_reserves: reader.read_u64().unwrap(),
			token_total_supply: reader.read_u64().unwrap(),
			complete: reader.read_u8().unwrap() != 0,
		}
	}
}

impl BondingCurveAccount {
	/// Creates a new bonding curve instance
	///
	/// # Arguments
	/// * `discriminator` - Unique identifier for the curve
	/// * `virtual_token_reserves` - Virtual token reserves for price calculations
	/// * `virtual_sol_reserves` - Virtual SOL reserves for price calculations
	/// * `real_token_reserves` - Actual token reserves available
	/// * `real_sol_reserves` - Actual SOL reserves available
	/// * `token_total_supply` - Total supply of tokens
	/// * `complete` - Whether the curve is complete
	pub fn new(
		discriminator: u64,
		virtual_token_reserves: u64,
		virtual_sol_reserves: u64,
		real_token_reserves: u64,
		real_sol_reserves: u64,
		token_total_supply: u64,
		complete: bool,
	) -> Self {
		Self {
			discriminator,
			virtual_token_reserves,
			virtual_sol_reserves,
			real_token_reserves,
			real_sol_reserves,
			token_total_supply,
			complete,
		}
	}

	/// Calculates the amount of tokens received for a given SOL amount
	///
	/// # Arguments
	/// * `amount` - Amount of SOL to spend
	///
	/// # Returns
	/// * `Ok(u64)` - Amount of tokens that would be received
	/// * `Err(&str)` - Error message if curve is complete
	pub fn get_buy_price(&self, amount: u64) -> PumpfunResult<u64> {
		if self.complete {
			return Err(PumpfunError::CurveCompleted);
		}

		if amount == 0 {
			return Ok(0);
		}

		// Calculate the product of virtual reserves using u128 to avoid overflow
		let n: u128 = (self.virtual_sol_reserves as u128) * (self.virtual_token_reserves as u128);

		// Calculate the new virtual sol reserves after the purchase
		let i: u128 = (self.virtual_sol_reserves as u128) + (amount as u128);

		// Calculate the new virtual token reserves after the purchase
		let r: u128 = n / i + 1;

		// Calculate the amount of tokens to be purchased
		let s: u128 = (self.virtual_token_reserves as u128) - r;

		// Convert back to u64 and return the minimum of calculated tokens and real reserves
		let s_u64 = s as u64;
		Ok(if s_u64 < self.real_token_reserves {
			s_u64
		} else {
			self.real_token_reserves
		})
	}

	/// Calculates the amount of SOL received for selling tokens
	///
	/// # Arguments
	/// * `amount` - Amount of tokens to sell
	/// * `fee_basis_points` - Fee in basis points (1/100th of a percent)
	///
	/// # Returns
	/// * `Ok(u64)` - Amount of SOL that would be received after fees
	/// * `Err(&str)` - Error message if curve is complete
	pub fn get_sell_price(&self, amount: u64, fee_basis_points: u64) -> PumpfunResult<u64> {
		if self.complete {
			return Err(PumpfunError::CurveCompleted);
		}

		if amount == 0 {
			return Ok(0);
		}

		// Calculate the proportional amount of virtual sol reserves to be received using u128
		let n: u128 = ((amount as u128) * (self.virtual_sol_reserves as u128))
			/ ((self.virtual_token_reserves as u128) + (amount as u128));

		// Calculate the fee amount in the same units
		let a: u128 = (n * (fee_basis_points as u128)) / 10000;

		// Return the net amount after deducting the fee, converting back to u64
		Ok((n - a) as u64)
	}

	/// Calculates the current market cap in SOL
	pub fn get_market_cap_sol(&self) -> u64 {
		if self.virtual_token_reserves == 0 {
			return 0;
		}

		((self.token_total_supply as u128) * (self.virtual_sol_reserves as u128)
			/ (self.virtual_token_reserves as u128)) as u64
	}

	/// Calculates the final market cap in SOL after all tokens are sold
	///
	/// # Arguments
	/// * `fee_basis_points` - Fee in basis points (1/100th of a percent)
	pub fn get_final_market_cap_sol(&self, fee_basis_points: u64) -> u64 {
		let total_sell_value: u128 =
			self.get_buy_out_price(self.real_token_reserves, fee_basis_points) as u128;
		let total_virtual_value: u128 = (self.virtual_sol_reserves as u128) + total_sell_value;
		let total_virtual_tokens: u128 =
			(self.virtual_token_reserves as u128) - (self.real_token_reserves as u128);

		if total_virtual_tokens == 0 {
			return 0;
		}

		((self.token_total_supply as u128) * total_virtual_value / total_virtual_tokens) as u64
	}

	/// Calculates the price to buy out all remaining tokens
	///
	/// # Arguments
	/// * `amount` - Amount of tokens to buy
	/// * `fee_basis_points` - Fee in basis points (1/100th of a percent)
	pub fn get_buy_out_price(&self, amount: u64, fee_basis_points: u64) -> u64 {
		// Get the effective amount of sol tokens
		let sol_tokens: u128 = if amount < self.real_sol_reserves {
			self.real_sol_reserves as u128
		} else {
			amount as u128
		};

		// Calculate total sell value
		let total_sell_value: u128 = (sol_tokens * (self.virtual_sol_reserves as u128))
			/ ((self.virtual_token_reserves as u128) - sol_tokens)
			+ 1;

		// Calculate fee
		let fee: u128 = (total_sell_value * (fee_basis_points as u128)) / 10000;

		// Return total including fee, converting back to u64
		(total_sell_value + fee) as u64
	}

	pub fn progress(&self) -> u64 {
		// totalSupply: 1,000,000,000 (Pump Fun Token)
		// reservedTokens: 206,900,000
		// Therefore, initialRealTokenReserves: 793,100,000

		// const reservedTokens=new BN(206900000).mul(new BN(1000_000));
		let reserved_tokens: u64 = 206900000 * 1000_000;
		println!("reserved_tokens: {reserved_tokens}");
		// const initialRealTokenReserves=data.tokenTotalSupply.sub(reservedTokens);
		let initial_real_token_reserves = self.token_total_supply.sub(reserved_tokens);
		println!("irtr: {initial_real_token_reserves}");
		// const bondingCurveProgress= new BN(100).sub(data.realTokenReserves.mul(new BN(100)).div(initialRealTokenReserves))
		let progress = 100.sub(
			self.real_token_reserves
				.mul(100)
				.div(initial_real_token_reserves),
		);
		println!("progress {progress}");
		// return bondingCurveProgress.toString(10)

		// Formula: BondingCurveProgress = 100 - ((leftTokens * 100) / initialRealTokenReserves)
		// Where:
		//     leftTokens = realTokenReserves - reservedTokens
		// initialRealTokenReserves = totalSupply - reservedTokens
		//
		// Definitions:
		//     initialRealTokenReserves = totalSupply - reservedTokens

		// leftTokens = realTokenReserves - reservedTokens
		// realTokenReserves: Token balance at the market address.

		// Simplified Formula: BondingCurveProgress = 100 - (((balance - 206900000) * 100) / 793100000)

		// println!("{}", self.real_token_reserves);
		// let temp = ((((self.virtual_token_reserves - self.real_token_reserves) / 1000000) - 206900000) * 100) / 793100000;
		// println!("{}", temp);

		// let progress = 100 - temp;
		return progress;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn get_bonding_curve() -> BondingCurveAccount {
		BondingCurveAccount::new(
			1,     // discriminator
			1000,  // virtual_token_reserves
			1000,  // virtual_sol_reserves
			500,   // real_token_reserves
			500,   // real_sol_reserves
			1000,  // token_total_supply
			false, // complete
		)
	}

	fn get_large_bonding_curve() -> BondingCurveAccount {
		BondingCurveAccount::new(
			1,            // discriminator
			u64::MAX / 2, // virtual_token_reserves
			u64::MAX / 2, // virtual_sol_reserves
			u64::MAX / 4, // real_token_reserves
			u64::MAX / 4, // real_sol_reserves
			u64::MAX / 2, // token_total_supply
			false,        // complete
		)
	}

	#[test]
	fn test_bonding_curve_account() {
		let bonding_curve: BondingCurveAccount = get_bonding_curve();

		// Test buy price calculation
		assert_eq!(bonding_curve.get_buy_price(0).unwrap(), 0);

		let buy_price = bonding_curve.get_buy_price(100).unwrap();
		assert!(buy_price > 0);
		assert!(buy_price <= bonding_curve.real_token_reserves);

		// Test sell price calculation
		assert_eq!(bonding_curve.get_sell_price(0, 250).unwrap(), 0);

		let sell_price = bonding_curve.get_sell_price(100, 250).unwrap();
		assert!(sell_price > 0);
	}

	#[test]
	fn test_bonding_curve_complete() {
		let mut bonding_curve: BondingCurveAccount = get_bonding_curve();

		// Test operations work when not complete
		assert!(bonding_curve.get_buy_price(100).is_ok());
		assert!(bonding_curve.get_sell_price(100, 250).is_ok());

		// Set curve to complete
		bonding_curve.complete = true;

		// Test operations fail when complete
		assert!(bonding_curve.get_buy_price(100).is_err());
		assert!(bonding_curve.get_sell_price(100, 250).is_err());
	}

	#[test]
	fn test_market_cap_calculations() {
		let bonding_curve: BondingCurveAccount = get_bonding_curve();

		// Test market cap calculations
		let market_cap = bonding_curve.get_market_cap_sol();
		assert!(market_cap > 0);

		let final_market_cap = bonding_curve.get_final_market_cap_sol(250);
		assert!(final_market_cap > 0);
	}

	#[test]
	fn test_buy_out_price() {
		let bonding_curve: BondingCurveAccount = get_bonding_curve();

		let buy_out_price = bonding_curve.get_buy_out_price(100, 250);
		assert!(buy_out_price > 0);

		// Test with amount less than real_sol_reserves
		let small_buy_out = bonding_curve.get_buy_out_price(400, 250);
		assert!(small_buy_out > 0);
	}

	#[test]
	fn test_overflow_buy_price() {
		let bonding_curve = get_large_bonding_curve();

		// Test buying with large SOL amount
		let buy_price = bonding_curve.get_buy_price(u64::MAX).unwrap();
		assert!(buy_price > 0);
		assert!(buy_price <= bonding_curve.real_token_reserves);
	}

	#[test]
	fn test_overflow_sell_price() {
		let bonding_curve = get_large_bonding_curve();

		// Test selling large token amount
		let sell_price = bonding_curve.get_sell_price(u64::MAX / 4, 250).unwrap();
		assert!(sell_price > 0);
	}

	#[test]
	fn test_overflow_market_cap() {
		let bonding_curve = get_large_bonding_curve();

		// Test market cap with large values
		let market_cap = bonding_curve.get_market_cap_sol();
		assert!(market_cap > 0);

		let final_market_cap = bonding_curve.get_final_market_cap_sol(250);
		assert!(final_market_cap > 0);
	}

	#[test]
	fn test_overflow_buy_out_price() {
		let bonding_curve = get_large_bonding_curve();

		// Test buy out with large token amount
		let buy_out_price = bonding_curve.get_buy_out_price(u64::MAX / 4, 250);
		assert!(buy_out_price > 0);
	}
}
