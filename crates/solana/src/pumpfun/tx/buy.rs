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
    /// Buys tokens from a bonding curve by spending SOL
    ///
    /// # Arguments
    ///
    /// * `mint` - Public key of the token mint to buy
    /// * `amount_sol` - Amount of SOL to spend in lamports
    /// * `slippage_basis_points` - Optional maximum acceptable slippage in basis points (1 bp = 0.01%). Defaults to 500
    /// * `priority_fee` - Optional priority fee configuration for compute units
    ///
    /// # Returns
    ///
    /// Returns the transaction signature if successful, or a ClientError if the operation fails
    pub async fn buy(
        &self,
        _payer: KeyPair,
        _mint: TokenMint,
        _amount_sol: u64,
        _slippage_basis_points: Option<u64>,
        // priority_fee: Option<PriorityFee>,
    ) -> pumpfun::PumpfunResult<Signature> {
        // let client = Arc::new(RpcClient::new("https://api.mainnet-beta.solana.com".to_string()));
        //
        // let rpc = Rpc::new(LocalRpcClient::new("https://api.mainnet-beta.solana.com"));
        //
        // // Get accounts and calculate buy amounts
        // // let global_account = self.get_global_account()?;
        // // let global_account = self.get_global_account().unwrap();
        //
        // // let bonding_curve_account = self.get_bonding_curve_account(mint)?;
        // // let bonding_curve_account = rpc.get_bonding_curve_account(mint).unwrap();
        // let bonding_curve_account = rpc.get_curve_account(mint.clone()).await.unwrap();
        //
        // // let amount_buy = bonding_curve_account.get_buy_price(amount_sol).map_err(error::ClientError::BondingCurveError)?;
        // let amount_buy = bonding_curve_account.get_buy_price(amount_sol)?;
        //
        // let amount_buy_with_slippage =
        //     calculate_with_slippage_buy(amount_sol, slippage_basis_points.unwrap_or(500));
        //
        // let client = Arc::new(RpcClient::new(
        //     "https://api.mainnet-beta.solana.com".to_string(),
        // ));
        //
        // // let keypair = Keypair::from_base58_string(keypair.as_str());
        //
        // // let mut transaction = self.swap_transaction(keypair.pubkey(), quote, overrides).await.unwrap();
        // let blockhash = client.get_latest_blockhash().await.unwrap();
        //
        // let mut instructions: Vec<Instruction> = vec![];
        //
        // // let kp: Keypair = payer.clone().into();
        // let kp: Keypair = payer.clone().into();
        // let mint: Pubkey = mint.into();
        //
        // let ata = create_ata_if_not_exists(&client, &kp, &kp.pubkey(), &mint).await;
        //
        // instructions.push(ComputeBudgetInstruction::set_compute_unit_price(100_000));
        // instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(300_000));
        //
        // // instructions.extend(setup_instructions);
        //
        // instructions.push(create_buy_instruction(
        //     &kp,
        //     &mint,
        //     &pubkey!("CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM"),
        //     BuyInstructionArgs {
        //         amount: amount_buy,
        //         max_sol_cost: amount_buy_with_slippage,
        //     },
        // ));
        // //
        // // if let Some(cleanup_instruction) = cleanup_instruction {
        // //     final_instructions.push(cleanup_instruction);
        // // }
        //
        // // let kp: Keypair = payer.clone().into();
        //
        // let mut message =
        //     VersionedMessage::Legacy(Message::new(&instructions, Some(&payer.public.into())));
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
        // // // Add priority fee if provided
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
        // // Create Associated Token Account if needed
        // // let ata: Pubkey = get_associated_token_address(&self.payer.pubkey(), mint);
        // // if client.get_account(&ata).is_err() {
        // //     request = request.instruction(create_associated_token_account(
        // //         &self.payer.pubkey(),
        // //         &self.payer.pubkey(),
        // //         mint,
        // //         &TOKEN_PROGRAM,
        // //     ));
        // // }
        // //
        // // // Add buy instruction
        // // request = request.instruction(create_buy_instruction(
        // //     self.payer,
        // //     mint,
        // //     &global_account.fee_recipient,
        // //     BuyInstructionArgs {
        // //         amount: amount_buy,
        // //         max_sol_cost: amount_buy_with_slippage,
        // //     },
        // // ));
        // //
        // // // Add signer
        // // request = request.signer(&self.payer);
        // //
        // // // Send transaction
        // // // let signature: Signature = request.send().await.map_err(error::ClientError::AnchorClientError)?;
        // // let signature: Signature = request.send().await.unwrap();
        // //
        // // Ok(signature)
        todo!()
    }
}

// async fn create_ata_if_not_exists(
//     client: &RpcClient,
//     payer: &Keypair,
//     wallet_address: &Pubkey,
//     token_mint: &Pubkey,
// ) -> Pubkey {
//     let token_program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
//
//     // Get the associated token address for the wallet and mint
//     let ata = get_associated_token_address(wallet_address, token_mint);
//
//     // Check if the associated token account already exists
//     if client.get_account(&ata).await.is_ok() {
//         println!("Associated Token Account already exists: {}", ata);
//         return ata;
//     }
//
//     // Create instruction to initialize the ATA
//     let instruction = instruction::create_associated_token_account(
//         &payer.pubkey(),
//         wallet_address,
//         token_mint,
//         &token_program_id,
//     );
//
//     // Create and send the transaction
//     let recent_blockhash = client.get_latest_blockhash().await.unwrap();
//     println!("{recent_blockhash}");
//
//     let transaction = Transaction::new_signed_with_payer(
//         &[instruction],
//         Some(&payer.pubkey()),
//         &[payer],
//         recent_blockhash,
//     );
//
//     println!("before sending and confirming");
//
//     client
//         .send_and_confirm_transaction(&transaction)
//         .await
//         .expect("Failed to create associated token account");
//
//     println!("Created Associated Token Account: {}", ata);
//     ata
// }
