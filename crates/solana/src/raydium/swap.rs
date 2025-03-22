// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/0xcrust/raydium-swap (MIT License).
// Original MIT License Copyright (c) 0xcrust 2024.

use std::sync::Arc;

use crate::raydium;
use crate::raydium::amm::AmmKeys;
use crate::raydium::ix::SwapInstructionsBuilder;
use crate::raydium::{
    Raydium, RaydiumQuote, SwapConfigOverrides, RAYDIUM_AUTHORITY,
    RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_ID,
};
use base::model::solana::Signature;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::VersionedTransaction;
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::solana_program::pubkey;

impl Raydium {
    pub async fn swap(
        &self,
        keypair: String,
        quote: RaydiumQuote,
        overrides: Option<SwapConfigOverrides>,
    ) -> raydium::Result<Signature> {
        let client = Arc::new(RpcClient::new(
            "https://api.mainnet-beta.solana.com".to_string(),
        ));

        let keypair = Keypair::from_base58_string(keypair.as_str());

        let mut transaction = self
            .swap_transaction(keypair.pubkey(), quote, overrides)
            .await
            .unwrap();

        let blockhash = client.get_latest_blockhash().await.unwrap();
        transaction.message.set_recent_blockhash(blockhash);

        let swap_tx = VersionedTransaction::try_new(transaction.message, &[&keypair]).unwrap();

        let result = client
            .send_transaction_with_config(
                &swap_tx,
                RpcSendTransactionConfig {
                    skip_preflight: false,
                    preflight_commitment: None,
                    encoding: None,
                    max_retries: None,
                    min_context_slot: None,
                },
            )
            .await
            .expect("Failed to swap");

        // println!("{result}");
        Ok(Signature(result.to_string()))
    }

    pub async fn swap_instructions(
        &self,
        input_pubkey: Pubkey,
        output: RaydiumQuote,
        overrides: Option<SwapConfigOverrides>,
    ) -> raydium::Result<Vec<solana_sdk::instruction::Instruction>> {
        let builder = self.make_swap(input_pubkey, output, overrides).await?;
        builder.build_instructions()
    }

    pub async fn swap_transaction(
        &self,
        input_pubkey: Pubkey,
        output: RaydiumQuote,
        overrides: Option<SwapConfigOverrides>,
    ) -> raydium::Result<VersionedTransaction> {
        let builder = self.make_swap(input_pubkey, output, overrides).await?;
        builder.build_transaction(Some(&input_pubkey), None)
    }

    async fn make_swap(
        &self,
        input_pubkey: Pubkey,
        quote: RaydiumQuote,
        overrides: Option<SwapConfigOverrides>,
    ) -> raydium::Result<SwapInstructionsBuilder> {
        let client = Arc::new(RpcClient::new(
            "https://api.mainnet-beta.solana.com".to_string(),
        ));

        let priority_fee = overrides
            .clone()
            .and_then(|o| o.priority_fee)
            .or(self.config.priority_fee);

        let cu_limits = overrides
            .clone()
            .and_then(|o| o.cu_limits)
            .or(self.config.cu_limits);

        let wrap_and_unwrap_sol = overrides
            .and_then(|o| o.wrap_and_unwrap_sol)
            .or(self.config.wrap_and_unwrap_sol)
            .unwrap_or(true);

        let mut builder = SwapInstructionsBuilder::default();
        let _associated_accounts = builder.handle_token_wrapping_and_accounts_creation(
            input_pubkey,
            wrap_and_unwrap_sol,
            quote.input_amount,
            quote.input_mint,
            quote.output_mint,
            spl_token::ID,
            spl_token::ID,
            None,
        )?;

        let instruction = build_swap(
            &RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_ID,
            &quote.keys,
            &input_pubkey,
            &get_associated_token_address(&input_pubkey, &quote.input_mint),
            &get_associated_token_address(&input_pubkey, &quote.output_mint),
            quote.input_amount,
            quote.output_amount_with_slippage,
        )?;
        builder.swap_instruction = Some(instruction);

        let compute_units = builder
            .handle_compute_units_params(cu_limits, client.as_ref(), input_pubkey)
            .await?;

        builder.handle_priority_fee_params(priority_fee, compute_units)?;

        Ok(builder)
    }
}

pub fn build_swap(
    amm_program: &Pubkey,
    keys: &AmmKeys,
    user_owner: &Pubkey,
    user_source: &Pubkey,
    user_destination: &Pubkey,
    input_amount: u64,
    output_amount_with_slippage: u64,
) -> raydium::Result<Instruction> {
    let expected_size = 1 + 8 + 8;
    let mut data = Vec::with_capacity(expected_size);
    data.extend_from_slice(&[9u8]);
    data.extend_from_slice(&input_amount.to_le_bytes());
    data.extend_from_slice(&output_amount_with_slippage.to_le_bytes());
    assert_eq!(data.len(), expected_size);

    Ok(Instruction {
        program_id: RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_ID,
        data,
        accounts: vec![
            // spl token
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            // amm
            AccountMeta::new(keys.pool, false), // Amm Info
            AccountMeta::new_readonly(RAYDIUM_AUTHORITY, false), // Amm authority
            AccountMeta::new(*amm_program, false), // oo
            AccountMeta::new(keys.coin_vault, false), // coin vault
            AccountMeta::new(keys.pc_vault, false), // pc vault
            // market
            AccountMeta::new(*amm_program, false), // ob program
            AccountMeta::new(*amm_program, false), // ob market
            AccountMeta::new(*amm_program, false), // ob bids
            AccountMeta::new(*amm_program, false), // ob asks
            AccountMeta::new(*amm_program, false), // ob events queue
            AccountMeta::new(*amm_program, false), // ob coin
            AccountMeta::new(*amm_program, false), // ob pc
            AccountMeta::new(*amm_program, false), // ob signer
            // user
            AccountMeta::new(*user_source, false), // user source token account
            AccountMeta::new(*user_destination, false), // user destination token account
            AccountMeta::new(*user_owner, true),
        ],
    })
}
