// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/raydium-io/raydium-amm (Apache 2.0 License).
// Original Apache 2.0 License Copyright (c) raydium.io 2024.

#[repr(u64)]
#[allow(unused)]
pub enum AmmStatus {
    Uninitialized = 0u64,
    Initialized = 1u64,
    Disabled = 2u64,
    WithdrawOnly = 3u64,
    // pool only can add or remove liquidity, can't swap and plan orders
    LiquidityOnly = 4u64,
    // pool only can add or remove liquidity and plan orders, can't swap
    OrderBookOnly = 5u64,
    // pool only can add or remove liquidity and swap, can't plan orders
    SwapOnly = 6u64,
    // pool status after created and will auto update to SwapOnly during swap after open_time
    WaitingTrade = 7u64,
}

#[allow(unused)]
impl AmmStatus {
    pub fn from_u64(status: u64) -> Self {
        match status {
            0u64 => AmmStatus::Uninitialized,
            1u64 => AmmStatus::Initialized,
            2u64 => AmmStatus::Disabled,
            3u64 => AmmStatus::WithdrawOnly,
            4u64 => AmmStatus::LiquidityOnly,
            5u64 => AmmStatus::OrderBookOnly,
            6u64 => AmmStatus::SwapOnly,
            7u64 => AmmStatus::WaitingTrade,
            _ => unreachable!(),
        }
    }

    pub fn into_u64(self) -> u64 {
        match self {
            AmmStatus::Uninitialized => 0u64,
            AmmStatus::Initialized => 1u64,
            AmmStatus::Disabled => 2u64,
            AmmStatus::WithdrawOnly => 3u64,
            AmmStatus::LiquidityOnly => 4u64,
            AmmStatus::OrderBookOnly => 5u64,
            AmmStatus::SwapOnly => 6u64,
            AmmStatus::WaitingTrade => 7u64,
        }
    }
    pub fn valid_status(status: u64) -> bool {
        matches!(status, 1u64..=7u64)
    }

    pub fn deposit_permission(&self) -> bool {
        match self {
            AmmStatus::Uninitialized => false,
            AmmStatus::Initialized => true,
            AmmStatus::Disabled => false,
            AmmStatus::WithdrawOnly => false,
            AmmStatus::LiquidityOnly => true,
            AmmStatus::OrderBookOnly => true,
            AmmStatus::SwapOnly => true,
            AmmStatus::WaitingTrade => true,
        }
    }

    pub fn withdraw_permission(&self) -> bool {
        match self {
            AmmStatus::Uninitialized => false,
            AmmStatus::Initialized => true,
            AmmStatus::Disabled => false,
            AmmStatus::WithdrawOnly => true,
            AmmStatus::LiquidityOnly => true,
            AmmStatus::OrderBookOnly => true,
            AmmStatus::SwapOnly => true,
            AmmStatus::WaitingTrade => true,
        }
    }

    pub fn swap_permission(&self) -> bool {
        match self {
            AmmStatus::Uninitialized => false,
            AmmStatus::Initialized => true,
            AmmStatus::Disabled => false,
            AmmStatus::WithdrawOnly => false,
            AmmStatus::LiquidityOnly => false,
            AmmStatus::OrderBookOnly => false,
            AmmStatus::SwapOnly => true,
            AmmStatus::WaitingTrade => true,
        }
    }

    pub fn orderbook_permission(&self) -> bool {
        match self {
            AmmStatus::Uninitialized => false,
            AmmStatus::Initialized => true,
            AmmStatus::Disabled => false,
            AmmStatus::WithdrawOnly => false,
            AmmStatus::LiquidityOnly => false,
            AmmStatus::OrderBookOnly => true,
            AmmStatus::SwapOnly => false,
            AmmStatus::WaitingTrade => false,
        }
    }
}
