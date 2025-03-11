// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::str::FromStr;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::{get_associated_token_address, instruction};

const USDC: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

#[tokio::main]
async fn main() {
    let keypair = Keypair::new();
    // Get the public key to share for receiving funds
    println!("Keep this secret! Private Key: {:?}", keypair.to_bytes());
    println!("{}", keypair.to_base58_string());

    // let keypair = Keypair::from_base58_string(env!("PRIVATE_KEY"));
    let public_key = keypair.pubkey();
    println!("Wallet Public Key: {}", public_key);
    //
    // let rpc_url = "https://api.mainnet-beta.solana.com";
    // let client = RpcClient::new(rpc_url.to_string());
    //
    // let balance = client.get_balance(&public_key).await.expect("Failed to fetch balance");
    // println!("Wallet balance: {} lamports", balance);
    //
    // let ata = create_ata_if_not_exists(&client, &keypair, &public_key, &Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB").unwrap()).await;
    // println!("{ata}")

    // let c = venue::raydium::HttpClient::default();
    //
    // let tokens = c.list_token().await.unwrap();
}

async fn create_ata_if_not_exists(
    client: &RpcClient,
    payer: &Keypair,
    wallet_address: &Pubkey,
    token_mint: &Pubkey,
) -> Pubkey {
    let token_program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();

    // Get the associated token address for the wallet and mint
    let ata = get_associated_token_address(wallet_address, token_mint);

    // Check if the associated token account already exists
    if client.get_account(&ata).await.is_ok() {
        println!("Associated Token Account already exists: {}", ata);
        return ata;
    }

    // Create instruction to initialize the ATA
    let instruction = instruction::create_associated_token_account(
        &payer.pubkey(),
        wallet_address,
        token_mint,
        &token_program_id,
    );

    // Create and send the transaction
    let recent_blockhash = client.get_latest_blockhash().await.unwrap();
    println!("{recent_blockhash}");

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    println!("before sending and confirming");

    client
        .send_and_confirm_transaction(&transaction)
        .await
        .expect("Failed to create associated token account");

    println!("Created Associated Token Account: {}", ata);
    ata
}
