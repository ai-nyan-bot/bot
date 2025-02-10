// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/0xcrust/raydium-swap (MIT License).
// Original MIT License Copyright (c) 0xcrust 2024.

use crate::venue::raydium;
use crate::venue::raydium::amm::*;
use crate::venue::raydium::http::v3::PoolType;
use crate::venue::raydium::math::{calc_total_without_take_pnl_no_orderbook, swap_with_slippage};
use crate::venue::raydium::{
    Error, ListPoolRequest, PoolSort, PoolSortOrder, Raydium, RaydiumQuote, RaydiumSwap, SwapDirection, RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_ID,
};
use common::model::PublicKey;
use log::debug;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use spl_token::state::Account;

// FIXME NO unwrap

impl Raydium {
    pub async fn quote(&self, input: impl Into<RaydiumSwap>) -> raydium::Result<RaydiumQuote> {
        let input = input.into();
        if input.input_token_mint == input.output_token_mint {
            return Err(Error::InputIsOutputTokenError);
        }

        let mut pool_id = input.market.map(|k| k.into());

        let input_mint: Pubkey = input.input_token_mint.into();
        let output_mint: Pubkey = input.output_token_mint.into();

        if pool_id.is_none() {
            let response = self
                .http_client
                .list_pools(ListPoolRequest {
                    pool_type: PoolType::Standard,
                    pool_sort: PoolSort::Liquidity,
                    sort_type: PoolSortOrder::Descending,
                    page_size: 100,
                    page: 1,
                    mint_one: input_mint,
                    mint_two: output_mint.into(),
                })
                .await?;

            debug!("Found {} pools", response.data.len());

            pool_id = response.data.iter().find_map(|pool| {
                if pool.mint_a.address == input_mint && pool.mint_b.address == output_mint
                    || pool.mint_a.address == output_mint && pool.mint_b.address == input_mint && pool.program_id == RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_ID
                {
                    debug!("Most liquid pool: {}", &pool.id);
                    Some(pool.id.clone())
                } else {
                    None
                }
            });
        }

        let Some(pool_id) = pool_id else {
            return Err(Error::MarketNotFoundError);
        };

        // let (amm_keys, market_keys) = if self.load_keys_by_api {
        let mut response = self
            .http_client
            .list_pool_keys([&pool_id].into_iter().map(|id| id.to_string()).collect())
            .await?;

        let keys = response.pop().ok_or(Error::PoolKeysNotFoundError { market: pool_id.into() })?;

        let amm_keys = AmmKeys::from(keys);
        // } else {
        //     let amm_keys = raydium_library::amm::utils::load_amm_keys(
        //         &self.client,
        //         &crate::venue::raydium::executor::RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_ID,
        //         &pool_id,
        //     )
        //         .await?;
        //
        //     let market_keys = MarketKeys::from(
        //         &raydium_library::amm::openbook::get_keys_for_market(
        //             &self.client,
        //             &amm_keys.market_program,
        //             &amm_keys.market,
        //         )
        //             .await?,
        //     );
        //
        //     (amm_keys, market_keys)
        // };

        // dbg!(&amm_keys);
        // dbg!(&market_keys);

        // reload accounts data to calculate amm pool vault amount
        // get multiple accounts at the same time to ensure data consistency
        let keys: Vec<PublicKey> = vec![
            pool_id,
            amm_keys.pc_vault,
            amm_keys.coin_vault,
            // amm_keys.amm_open_order,
            // amm_keys.market,
            // market_keys.event_queue,
        ]
        .into_iter()
        .map(|k| k.into())
        .collect();

        let accounts = self.rpc_client.get_multiple_accounts(&keys).await.unwrap();

        // let rsps = crate::utils::get_multiple_account_data(&self.client, &load_pubkeys).await?;
        // let accounts = array_ref![rsps, 0, 7];
        let [
        amm_account,
        amm_pc_vault_account,
        amm_coin_vault_account,
        // amm_open_orders_account,
        // market_account,
        // market_event_q_account,
        ] = accounts.as_slice() else { panic!() };

        let amm = Amm::decode(&amm_account.data).unwrap();
        // dbg!(&amm);

        // let _amm_target: raydium_amm::state::TargetOrders =
        //     transmute_one_pedantic::<raydium_amm::state::TargetOrders>(transmute_to_bytes(
        //         &amm_target_account.as_ref().unwrap().clone().data,
        //     ))
        //         .map_err(|e| e.without_src())?;

        let amm_pc_vault = Account::unpack(&amm_pc_vault_account.data).unwrap();

        let amm_coin_vault = Account::unpack(&amm_coin_vault_account.data).unwrap();

        // let (amm_pool_pc_vault_amount, amm_pool_coin_vault_amount) =
        // if raydium_amm::state::AmmStatus::from_u64(amm.status).orderbook_permission() {
        //     let amm_open_orders_account = &mut amm_open_orders_account.as_ref().unwrap().clone();
        //     let market_account = &mut market_account.as_ref().unwrap().clone();
        //     let market_event_q_account = &mut market_event_q_account.as_ref().unwrap().clone();
        //     let amm_open_orders_info = (&amm.open_orders, amm_open_orders_account).into_account_info();
        //     let market_account_info = (&amm.market, market_account).into_account_info();
        //     let market_event_queue_info = (&(market_keys.event_queue), market_event_q_account).into_account_info();
        //
        //     let amm_authority = Pubkey::find_program_address(
        //         &[raydium_amm::processor::AUTHORITY_AMM],
        //         &crate::venue::raydium::executor::RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_ID,
        //     ).0;
        //
        //     let lamports = &mut 0;
        //     let data = &mut [0u8];
        //
        //     let owner = Pubkey::default();
        //     let amm_authority_info = solana_program::account_info::AccountInfo::new(
        //         &amm_authority,
        //         false,
        //         false,
        //         lamports,
        //         data,
        //         &owner,
        //         false,
        //         0,
        //     );
        //
        //     let (market_state, open_orders) =
        //         raydium_amm::processor::Processor::load_serum_market_order(
        //             &market_account_info,
        //             &amm_open_orders_info,
        //             &amm_authority_info,
        //             &amm,
        //             false,
        //         )?;
        //
        //     let (amm_pool_pc_vault_amount, amm_pool_coin_vault_amount) =
        //         raydium_amm::math::Calculator::calc_total_without_take_pnl(
        //             amm_pc_vault.amount,
        //             amm_coin_vault.amount,
        //             &open_orders,
        //             &amm,
        //             &market_state,
        //             &market_event_queue_info,
        //             &amm_open_orders_info,
        //         )?;
        //
        //     (amm_pool_pc_vault_amount, amm_pool_coin_vault_amount)
        // } else {
        let (amm_pool_pc_vault_amount, amm_pool_coin_vault_amount) =
            calc_total_without_take_pnl_no_orderbook(amm_pc_vault.amount, amm_coin_vault.amount, &amm)?;
        //     (amm_pool_pc_vault_amount, amm_pool_coin_vault_amount)
        // };

        // dbg!(&amm_pool_pc_vault_amount);
        // dbg!(&amm_pool_coin_vault_amount);

        let direction = if input_mint == amm_keys.coin_mint && output_mint == amm_keys.pc_mint {
            SwapDirection::Coin2PC
        } else {
            SwapDirection::PC2Coin
        };

        let (output_amount, output_amount_with_slippage) = swap_with_slippage(
            amm_pool_pc_vault_amount,
            amm_pool_coin_vault_amount,
            amm.fees.swap_fee_numerator,
            amm.fees.swap_fee_denominator,
            direction,
            input.amount,
            input.mode,
            input.slippage_bps as u64,
        )?;

        debug!("output amount: {}", output_amount);
        debug!("output amount with slippage: {}", output_amount_with_slippage);

        // Ok(RaydiumAmmQuote {
        //     market: pool_id,
        //     input_mint: input.input_token_mint,
        //     output_mint: input.output_token_mint,
        //     amount: input.amount,
        //     other_amount,
        //     other_amount_threshold,
        //     amount_specified_is_input,
        //     input_mint_decimals: if coin_to_pc {
        //         amm.coin_decimals
        //     } else {
        //         amm.pc_decimals
        //     } as u8,
        //     output_mint_decimals: if coin_to_pc {
        //         amm.pc_decimals
        //     } else {
        //         amm.coin_decimals
        //     } as u8,
        //     amm_keys,
        //     market_keys,
        // })

        Ok(RaydiumQuote {
            market: pool_id,
            input_mint: input_mint.into(),
            output_mint: output_mint.into(),
            input_amount: input.amount,
            output_amount,
            output_amount_with_slippage,
            // input_mint_decimals: if coin_to_pc {
            //     amm.coin_decimals
            // } else {
            //     amm.pc_decimals
            // } as u8,
            // output_mint_decimals: if coin_to_pc {
            //     amm.pc_decimals
            // } else {
            //     amm.coin_decimals
            // } as u8,
            keys: amm_keys,
            // market_keys,
        })

        // todo!()
    }
}
