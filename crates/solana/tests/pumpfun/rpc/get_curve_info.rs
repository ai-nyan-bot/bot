// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use dotenv::dotenv;
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
        .get_curve_info("CpV7zK77DkyVvbCgGWtWrsqvh2VonFrqPg8cecrTpump")
        .await
        .unwrap();

    assert_eq!(result.virtual_base_reserves, 1071762963469721);
    assert_eq!(result.virtual_quote_reserves, 30034626924);
    assert_eq!(result.real_base_reserves, 791862963469721);
    assert_eq!(result.real_quote_reserves, 34626924);
    assert_eq!(result.total_supply, 1000000000000000);
    assert!(!result.complete);
}

#[test_log::test(tokio::test)]
async fn test_not_found() {
    dotenv().ok();
    let test_instance = Rpc::new(RpcClient::new(
        env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"),
    ));

    let result = test_instance
        .get_curve_info("22222222222222222222222222222222222222222222")
        .await;

    assert!(result.is_none());
}
