// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::str::FromStr;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use common::model::PublicKey;
use solana::venue::raydium::{Raydium, RaydiumSwap, SwapExecutionMode};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "solana::dex=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let sol = PublicKey::from_str("So11111111111111111111111111111111111111112").unwrap();
    let usdc = PublicKey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();

    let swap_input = RaydiumSwap {
        input_token_mint: sol,
        output_token_mint: usdc,
        slippage_bps: 1000, // 10%
        amount: 1_000_000,  // 0.001 SOL
        mode: SwapExecutionMode::ExactIn,
        market: None,
    };

    let raydium = Raydium::default();

    let quote = raydium.quote(swap_input).await.unwrap();
    println!("Quote: {:#?}", quote);
}
