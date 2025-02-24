// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::pumpfun::model::CurveInfo;
use crate::pumpfun::util::curve_pda;
use crate::pumpfun::{PumpfunError, PumpfunResult, Rpc};
use async_trait::async_trait;
use base::model::{Amount, PublicKey};
use common::ByteReader;
use log::error;
use std::ops::{Div, Mul, Sub};

#[async_trait]
pub trait LoadCurveInfo: Send + Sync {
    async fn load_curve_info(&self, key: impl Into<PublicKey> + Send) -> Option<CurveInfo>;
}

#[async_trait]
impl LoadCurveInfo for Rpc {
    async fn load_curve_info(&self, key: impl Into<PublicKey> + Send) -> Option<CurveInfo> {
        let key = key.into();
        let curve_pda = match curve_pda(key.clone()) {
            None => {
                error!("unable to resolve pda for curve {key}");
                return None;
            }
            Some(pda) => pda,
        };

        if let Ok(Some(curve_account)) = self.client.get_account(curve_pda).await {
            let reader = ByteReader::new(&curve_account.account.data);
            let curve = CurveAccount::decode(&reader);

            Some(CurveInfo {
                virtual_base_reserves: curve.virtual_base_reserves.into(),
                virtual_quote_reserves: curve.virtual_quote_reserves.into(),
                real_base_reserves: curve.real_base_reserves.into(),
                real_quote_reserves: curve.real_quote_reserves.into(),
                total_supply: curve.total_supply.into(),
                complete: curve.complete,
            })
        } else {
            error!("unable to retrieve curve info {key}");
            None
        }
    }
}

#[derive(Debug, Clone)]
struct CurveAccount {
    pub discriminator: u64,
    pub virtual_base_reserves: u64,
    pub virtual_quote_reserves: u64,
    pub real_base_reserves: u64,
    pub real_quote_reserves: u64,
    pub total_supply: u64,
    pub complete: bool,
}

impl CurveAccount {
    fn decode(reader: &ByteReader) -> Self {
        Self {
            discriminator: reader.read_u64().unwrap(),
            virtual_base_reserves: reader.read_u64().unwrap(),
            virtual_quote_reserves: reader.read_u64().unwrap(),
            real_base_reserves: reader.read_u64().unwrap(),
            real_quote_reserves: reader.read_u64().unwrap(),
            total_supply: reader.read_u64().unwrap(),
            complete: reader.read_u8().unwrap() != 0,
        }
    }
}

// FIXME not for curve account
impl CurveAccount {
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
            virtual_base_reserves: virtual_token_reserves,
            virtual_quote_reserves: virtual_sol_reserves,
            real_base_reserves: real_token_reserves,
            real_quote_reserves: real_sol_reserves,
            total_supply: token_total_supply,
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
        let n: u128 = (self.virtual_quote_reserves as u128) * (self.virtual_base_reserves as u128);

        // Calculate the new virtual sol reserves after the purchase
        let i: u128 = (self.virtual_quote_reserves as u128) + (amount as u128);

        // Calculate the new virtual token reserves after the purchase
        let r: u128 = n / i + 1;

        // Calculate the amount of tokens to be purchased
        let s: u128 = (self.virtual_base_reserves as u128) - r;

        // Convert back to u64 and return the minimum of calculated tokens and real reserves
        let s_u64 = s as u64;
        Ok(if s_u64 < self.real_base_reserves {
            s_u64
        } else {
            self.real_base_reserves
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
        let n: u128 = ((amount as u128) * (self.virtual_quote_reserves as u128))
            / ((self.virtual_base_reserves as u128) + (amount as u128));

        // Calculate the fee amount in the same units
        let a: u128 = (n * (fee_basis_points as u128)) / 10000;

        // Return the net amount after deducting the fee, converting back to u64
        Ok((n - a) as u64)
    }

    /// Calculates the current market cap in SOL
    pub fn get_market_cap_sol(&self) -> u64 {
        if self.virtual_base_reserves == 0 {
            return 0;
        }

        ((self.total_supply as u128) * (self.virtual_quote_reserves as u128)
            / (self.virtual_base_reserves as u128)) as u64
    }

    /// Calculates the final market cap in SOL after all tokens are sold
    ///
    /// # Arguments
    /// * `fee_basis_points` - Fee in basis points (1/100th of a percent)
    pub fn get_final_market_cap_sol(&self, fee_basis_points: u64) -> u64 {
        let total_sell_value: u128 =
            self.get_buy_out_price(self.real_base_reserves, fee_basis_points) as u128;
        let total_virtual_value: u128 = (self.virtual_quote_reserves as u128) + total_sell_value;
        let total_virtual_tokens: u128 =
            (self.virtual_base_reserves as u128) - (self.real_base_reserves as u128);

        if total_virtual_tokens == 0 {
            return 0;
        }

        ((self.total_supply as u128) * total_virtual_value / total_virtual_tokens) as u64
    }

    /// Calculates the price to buy out all remaining tokens
    ///
    /// # Arguments
    /// * `amount` - Amount of tokens to buy
    /// * `fee_basis_points` - Fee in basis points (1/100th of a percent)
    pub fn get_buy_out_price(&self, amount: u64, fee_basis_points: u64) -> u64 {
        // Get the effective amount of sol tokens
        let sol_tokens: u128 = if amount < self.real_quote_reserves {
            self.real_quote_reserves as u128
        } else {
            amount as u128
        };

        // Calculate total sell value
        let total_sell_value: u128 = (sol_tokens * (self.virtual_quote_reserves as u128))
            / ((self.virtual_base_reserves as u128) - sol_tokens)
            + 1;

        // Calculate fee
        let fee: u128 = (total_sell_value * (fee_basis_points as u128)) / 10000;

        // Return total including fee, converting back to u64
        (total_sell_value + fee) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_bonding_curve() -> CurveAccount {
        CurveAccount::new(
            1,     // discriminator
            1000,  // virtual_token_reserves
            1000,  // virtual_sol_reserves
            500,   // real_token_reserves
            500,   // real_sol_reserves
            1000,  // token_total_supply
            false, // complete
        )
    }

    fn get_large_bonding_curve() -> CurveAccount {
        CurveAccount::new(
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
        let bonding_curve: CurveAccount = get_bonding_curve();

        // Test buy price calculation
        assert_eq!(bonding_curve.get_buy_price(0).unwrap(), 0);

        let buy_price = bonding_curve.get_buy_price(100).unwrap();
        assert!(buy_price > 0);
        assert!(buy_price <= bonding_curve.real_base_reserves);

        // Test sell price calculation
        assert_eq!(bonding_curve.get_sell_price(0, 250).unwrap(), 0);

        let sell_price = bonding_curve.get_sell_price(100, 250).unwrap();
        assert!(sell_price > 0);
    }

    #[test]
    fn test_bonding_curve_complete() {
        let mut bonding_curve: CurveAccount = get_bonding_curve();

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
        let bonding_curve: CurveAccount = get_bonding_curve();

        // Test market cap calculations
        let market_cap = bonding_curve.get_market_cap_sol();
        assert!(market_cap > 0);

        let final_market_cap = bonding_curve.get_final_market_cap_sol(250);
        assert!(final_market_cap > 0);
    }

    #[test]
    fn test_buy_out_price() {
        let bonding_curve: CurveAccount = get_bonding_curve();

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
        assert!(buy_price <= bonding_curve.real_base_reserves);
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
