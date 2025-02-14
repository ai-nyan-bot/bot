// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::str::FromStr;

use crate::http::error::HttpError;
use crate::http::json::JsonReq;
use crate::http::state::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use base::model::AuthenticatedUser;
use base::model::PublicKey;
use log::debug;
use serde::{Deserialize, Serialize};
use solana::venue::raydium::{ComputeUnitLimits, PriorityFeeConfig, Raydium, RaydiumSwap, SwapConfigOverrides, SwapExecutionMode};

#[derive(Deserialize, Debug)]
pub struct SwapRequest {
    // from: String,
    // to: String,
    // amount: u64,
    // slippage: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SwapResponse {
    from: String,
    to: String,
    amount: u64,
    amount_with_slippage: u64,
    signature: String,
}

pub async fn swap(
    Path(id): Path<String>,
    Extension(_user): Extension<AuthenticatedUser>,
    State(_state): State<AppState>,
    JsonReq(req): JsonReq<SwapRequest>,
) -> Result<Json<SwapResponse>, HttpError> {
    debug!("POST /v1/wallets/{}/swap {:?}", id, req);

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

    let overrides = SwapConfigOverrides {
        // priority_fee: Some(PriorityFeeConfig::DynamicMultiplier(1)),
        priority_fee: Some(PriorityFeeConfig::FixedCuPrice(200_000)),
        cu_limits: Some(ComputeUnitLimits::Fixed(60_000)),
        wrap_and_unwrap_sol: None,
        destination_token_account: None,
        as_legacy_transaction: None,
    };

    let signature = raydium
        .swap(
            "3ZSqgN5FTNTto4vqZgEzFrCwqcuq3b6hXFeAajE31Tp6pwi85y6izHG9TpRGF1p5S5AasuPD9b7kDkEqCGxeoySb".to_string(),
            quote.clone(),
            Some(overrides),
        )
        .await
        .unwrap();
    // println!("{}", signature)
    Ok(Json(SwapResponse {
        from: quote.input_mint.to_string(),
        to: quote.output_mint.to_string(),
        amount: quote.output_amount,
        amount_with_slippage: quote.output_amount_with_slippage,
        signature: signature.0,
    }))
}
