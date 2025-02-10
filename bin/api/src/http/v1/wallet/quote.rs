// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::str::FromStr;

use crate::http::error::HttpError;
use crate::http::json::JsonReq;
use crate::http::state::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use base::model::AuthenticatedUser;
use common::model::PublicKey;
use log::debug;
use serde::{Deserialize, Serialize};
use solana::venue::raydium::{Raydium, RaydiumSwap, SwapExecutionMode};

#[derive(Deserialize, Debug)]
pub struct QuoteRequest {
    // from: String,
    // to: String,
    // amount: u64,
    // slippage: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteResponse {
    from: String,
    to: String,
    amount: u64,
    amount_with_slippage: u64,
}

pub async fn quote(
    Path(id): Path<String>,
    Extension(user): Extension<AuthenticatedUser>,
    State(_state): State<AppState>,
    JsonReq(req): JsonReq<QuoteRequest>,
) -> Result<Json<QuoteResponse>, HttpError> {
    debug!("POST /v1/wallets/{}/quote {:?}", id, req);

    println!("{:#?}", user);

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

    Ok(Json(QuoteResponse {
        from: quote.input_mint.to_string(),
        to: quote.output_mint.to_string(),
        amount: quote.output_amount,
        amount_with_slippage: quote.output_amount_with_slippage,
    }))
}
