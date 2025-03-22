// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/nhuxhr/pumpfun-rs (MIT License).
// Original MIT License Copyright (c) nhuxhr 2024.

use crate::model::solana::Signature;
use crate::pumpfun;
use crate::pumpfun::Pumpfun;
use base::model::KeyPair;
use base::model::TokenMint;

impl Pumpfun {
    /// Sells tokens back to the bonding curve in exchange for SOL
    ///
    /// # Arguments
    ///
    /// * `mint` - Public key of the token mint to sell
    /// * `amount_token` - Optional amount of tokens to sell in base units. If None, sells entire balance
    /// * `slippage_basis_points` - Optional maximum acceptable slippage in basis points (1 bp = 0.01%). Defaults to 500
    /// * `priority_fee` - Optional priority fee configuration for compute units
    ///
    /// # Returns
    ///
    /// Returns the transaction signature if successful, or a ClientError if the operation fails
    pub async fn sell(
        &self,
        _keypair: KeyPair,
        _mint: TokenMint,
        _amount: Option<u64>,
        _slippage_basis_points: Option<u64>,
        // priority_fee: Option<PriorityFee>,
        // ) -> Result<Signature, error::ClientError> {
    ) -> pumpfun::PumpfunResult<Signature> {
        // // let ata: Pubkey = get_associated_token_address(&self.payer.pubkey(), mint);
        // // let balance = self.rpc.get_token_account_balance(&ata).unwrap();
        // //
        // // let balance_u64: u64 = balance.amount.parse::<u64>().unwrap();
        //
        // let rpc = Rpc::new(LocalRpcClient::new("https://api.mainnet-beta.solana.com"));
        //
        // // let _amount = amount_token.unwrap_or(balance_u64);
        // let amount = amount.unwrap();
        // let client = Arc::new(RpcClient::new(
        //     "https://api.mainnet-beta.solana.com".to_string(),
        // ));
        //
        // let global_account = rpc.get_global_info().await;
        //
        // let bonding_curve_account = rpc.get_curve_account(mint.clone()).await.unwrap();
        //
        // let min_sol_output = bonding_curve_account
        //     // .get_sell_price(_amount, global_account.fee_basis_points)
        //     .get_sell_price(amount, global_account.fee_basis_points)?;
        // // .unwrap();
        // // .map_err(error::ClientError::BondingCurveError)?;
        //
        // let _min_sol_output =
        //     calculate_with_slippage_sell(min_sol_output, slippage_basis_points.unwrap_or(500));
        //
        // // let mut request = self.program.request();
        //
        // // Add priority fee if provided
        // // if let Some(fee) = priority_fee {
        // //     if let Some(limit) = fee.limit {
        // //         let limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(limit);
        // //         request = request.instruction(limit_ix);
        // //     }
        // //
        // //     if let Some(price) = fee.price {
        // //         let price_ix = ComputeBudgetInstruction::set_compute_unit_price(price);
        // //         request = request.instruction(price_ix);
        // //     }
        // // }
        //
        // // Add sell instruction
        // // request = request.instruction(instruction::sell(
        // //     self.payer,
        // //     mint,
        // //     &global_account.fee_recipient,
        // //     cpi::instruction::Sell { _amount, _min_sol_output },
        // // ));
        // // let kp: Keypair = Keypair::from_base58_string(env!("PRIVATE_KEY"));
        // let kp: Keypair = keypair.into();
        // let mint: Pubkey = mint.into();
        //
        // let mut instructions: Vec<Instruction> = vec![];
        //
        // instructions.push(ComputeBudgetInstruction::set_compute_unit_price(100_000));
        // instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(300_000));
        //
        // // instructions.extend(setup_instructions);
        // instructions.push(create_sell_instruction(
        //     &kp,
        //     &mint,
        //     &pubkey!("CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM"),
        //     SellInstructionArgs {
        //         amount,
        //         min_sol_output: _min_sol_output,
        //     },
        // ));
        // //
        // // if let Some(cleanup_instruction) = cleanup_instruction {
        // //     final_instructions.push(cleanup_instruction);
        // // }
        //
        // // let kp: Keypair = payer.clone().into();
        //
        // let blockhash = client.get_latest_blockhash().await.unwrap();
        //
        // let mut message = VersionedMessage::Legacy(Message::new(&instructions, Some(&kp.pubkey())));
        // // if let Some(hash) = blockhash {
        // message.set_recent_blockhash(blockhash);
        // // }
        // let mut transaction = VersionedTransaction {
        //     signatures: vec![solana_sdk::signature::Signature::default()],
        //     message,
        // };
        //
        // transaction.message.set_recent_blockhash(blockhash);
        //
        // let tx = VersionedTransaction::try_new(transaction.message, &[&kp]).unwrap();
        //
        // let result = client
        //     .send_transaction_with_config(
        //         &tx,
        //         RpcSendTransactionConfig {
        //             skip_preflight: false,
        //             preflight_commitment: None,
        //             encoding: None,
        //             max_retries: None,
        //             min_context_slot: None,
        //         },
        //     )
        //     .await
        //     .expect("Failed to swap");
        //
        // // println!("{result}");
        // Ok(Signature(result.to_string()))
        //
        todo!()
    }
}
