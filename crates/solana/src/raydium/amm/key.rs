// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::raydium::http::v3::response::pool_keys::PoolKey;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub struct MarketKeys {
    pub event_queue: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub coin_vault: Pubkey,
    pub pc_vault: Pubkey,
    pub vault_signer_key: Pubkey,
}
//
// impl From<PoolKey> for MarketKeys {
//     fn from(value: PoolKey) -> Self {
//         // let keys = value
//         //     .keys
//         //     .market
//         //     .expect("market keys should be present for amm");
//
//         MarketKeys {
//             event_queue: value.market_event_queue,
//             bids: keys.market_bids,
//             asks: keys.market_asks,
//             coin_vault: keys.market_base_vault,
//             pc_vault: keys.market_quote_vault,
//             vault_signer_key: keys.market_authority,
//         }
//     }
// }

#[derive(Clone, Debug)]
pub struct AmmKeys {
    pub pool: Pubkey,
    pub coin_mint: Pubkey,
    pub pc_mint: Pubkey,
    // pub amm_authority: Pubkey,
    // pub amm_target: Pubkey,
    pub coin_vault: Pubkey,
    pub pc_vault: Pubkey,
    // pub amm_lp_mint: Pubkey,
    // pub amm_open_order: Pubkey,
    // pub market_program: Pubkey,
    // pub market: Pubkey,
    pub market: Option<MarketKeys>,
}

impl From<PoolKey> for AmmKeys {
    fn from(value: PoolKey) -> Self {
        // let market_keys = value
        //     .keys
        //     .market
        //     .expect("market keys should be present for amm");

        AmmKeys {
            pool: value.id,
            coin_mint: value.mint_a.address,
            pc_mint: value.mint_b.address,
            // amm_authority: value.keys.authority.clone(),
            // amm_target: value
            //     .keys
            //     .target_orders
            //     .expect("target orders should be present for amm"),
            coin_vault: value.vault.a,
            pc_vault: value.vault.b,
            // amm_lp_mint: value.keys.mint_lp.address.clone(),
            // amm_open_order: value
            //     .keys
            //     .open_orders
            //     .expect("open orders should be present for amm"),

            // market_program: market_keys.market_program_id.clone(),
            // market: market_keys.market_id.clone(),
            market: None,
        }
    }
}
