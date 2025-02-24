// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use dotenv::dotenv;
use solana::pumpfun::rpc::LoadCurveInfo;
use solana::pumpfun::Rpc;
use solana::rpc::RpcClient;
use std::env;

#[test_log::test(tokio::test)]
async fn test_ok() {
    dotenv().ok();
    let test_instance = Rpc::new(RpcClient::new(
        env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"),
    ));

    let result = test_instance
        .load_curve_info("CpV7zK77DkyVvbCgGWtWrsqvh2VonFrqPg8cecrTpump")
        .await
        .unwrap();

    assert_eq!(result.virtual_token_reserves, 1071443444605882);
    assert_eq!(result.virtual_sol_reserves, 30043583654);
    assert_eq!(result.real_token_reserves, 791543444605882);
    assert_eq!(result.real_sol_reserves, 43583654);
    assert_eq!(result.token_total_supply, 1000000000000000);
    assert_eq!(result.complete, false);
}

#[test_log::test(tokio::test)]
async fn test_not_found() {
    dotenv().ok();
    let test_instance = Rpc::new(RpcClient::new(
        env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"),
    ));

    let result = test_instance
        .load_curve_info("22222222222222222222222222222222222222222222")
        .await;

    assert!(result.is_none());
}
