// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use dotenv::dotenv;
use solana::rpc::RpcClient;
use std::env;

#[test_log::test(tokio::test)]
async fn test_ok() {
    dotenv().ok();
    let test_instance =
        RpcClient::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let result = test_instance.get_block(315136165).await.unwrap();
    let result = result.unwrap();
    assert_eq!(result.slot, 315136165);
    assert_eq!(result.transactions.len(), 2012);
}

#[test_log::test(tokio::test)]
async fn test_block_does_not_exist() {
    dotenv().ok();
    let test_instance =
        RpcClient::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let result = test_instance.get_block(315136164).await.unwrap();
    assert!(result.is_none());
}
