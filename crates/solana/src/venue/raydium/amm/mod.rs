// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/raydium-io/raydium-amm (Apache 2.0 License).
// Original Apache 2.0 License Copyright (c) raydium.io 2024.


use solana_sdk::pubkey::Pubkey;

pub use key::AmmKeys;

mod key;
mod status;

// #[repr(C)]
// #[derive(Copy, Clone, Debug)]
pub struct Amm {
    /// Initialized status.
    pub status: u64,
    /// Nonce used in program address.
    /// The program address is created deterministically with the nonce,
    /// amm program id, and amm account Pubkey.  This program address has
    /// authority over the amm's token coin account, token pc account, and pool
    /// token mint.
    pub nonce: u64,
    /// max order count
    pub order_num: u64,
    /// within this range, 5 => 5% range
    pub depth: u64,
    /// coin decimal
    pub coin_decimals: u64,
    /// pc decimal
    pub pc_decimals: u64,
    /// amm machine state
    pub state: u64,
    /// amm reset_flag
    pub reset_flag: u64,
    /// min size 1->0.000001
    pub min_size: u64,
    /// vol_max_cut_ratio numerator, sys_decimal_value as denominator
    pub vol_max_cut_ratio: u64,
    /// amount wave numerator, sys_decimal_value as denominator
    pub amount_wave: u64,
    /// coinLotSize 1 -> 0.000001
    pub coin_lot_size: u64,
    /// pcLotSize 1 -> 0.000001
    pub pc_lot_size: u64,
    /// min_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier
    pub min_price_multiplier: u64,
    /// max_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier
    pub max_price_multiplier: u64,
    /// system decimal value, used to normalize the value of coin and pc amount
    pub sys_decimal_value: u64,
    /// All fee information
    pub fees: Fees,
    /// Statistical data
    pub state_data: StateData,
    /// Coin vault
    pub coin_vault: Pubkey,
    /// Pc vault
    pub pc_vault: Pubkey,
    /// Coin vault mint
    pub coin_vault_mint: Pubkey,
    /// Pc vault mint
    pub pc_vault_mint: Pubkey,
    /// lp mint
    pub lp_mint: Pubkey,
    /// open_orders key
    pub open_orders: Pubkey,
    /// market key
    pub market: Pubkey,
    /// market program key
    pub market_program: Pubkey,
    /// target_orders key
    pub target_orders: Pubkey,
    /// padding
    pub padding1: [u64; 8],
    /// amm owner key
    pub amm_owner: Pubkey,
    /// pool lp amount
    pub lp_amount: u64,
    /// client order id
    pub client_order_id: u64,
    /// padding
    pub padding2: [u64; 2],
}

// unsafe impl Zeroable for Amm {}
//
// unsafe impl Pod for Amm {}

impl Amm {
    pub fn decode(account: &[u8]) -> Result<&Self, ()> {
        // if account.len() != size_of::<Self>() {
        //     return Err(AmmError::ExpectedAccount.into());
        // }
        // let data = try_from_bytes(account)?;
        // let data = from_bytes(account); // FIXME

        // if data.status == AmmStatus::Uninitialized as u64 {
        //     return Err(AmmError::InvalidStatus.into());
        // }
        // Ok(data)
        todo!()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct StateData {
    /// delay to take pnl coin
    pub need_take_pnl_coin: u64,
    /// delay to take pnl pc
    pub need_take_pnl_pc: u64,
    /// total pnl pc
    pub total_pnl_pc: u64,
    /// total pnl coin
    pub total_pnl_coin: u64,
    /// ido pool open time
    pub pool_open_time: u64,
    /// padding for future updates
    pub padding: [u64; 2],
    /// switch from orderbookonly to init
    pub orderbook_to_init_time: u64,

    /// swap coin in amount
    pub swap_coin_in_amount: [u8; 16],
    /// swap pc out amount
    pub swap_pc_out_amount: [u8; 16],
    /// charge pc as swap fee while swap pc to coin
    pub swap_acc_pc_fee: u64,

    /// swap pc in amount
    pub swap_pc_in_amount: [u8; 16],
    /// swap coin out amount
    pub swap_coin_out_amount: [u8; 16],
    /// charge coin as swap fee while swap coin to pc
    pub swap_acc_coin_fee: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct Fees {
    /// numerator of the min_separate
    pub min_separate_numerator: u64,
    /// denominator of the min_separate
    pub min_separate_denominator: u64,
    /// numerator of the fee
    pub trade_fee_numerator: u64,
    /// denominator of the fee
    /// and 'trade_fee_denominator' must be equal to 'min_separate_denominator'
    pub trade_fee_denominator: u64,
    /// numerator of the pnl
    pub pnl_numerator: u64,
    /// denominator of the pnl
    pub pnl_denominator: u64,
    /// numerator of the swap_fee
    pub swap_fee_numerator: u64,
    /// denominator of the swap_fee
    pub swap_fee_denominator: u64,
}
