// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use dotenv::dotenv;
use solana::token_info::rpc::RpcTokenInfoLoader;
use solana::token_info::LoadTokenInfo;
use std::env;

#[test_log::test(tokio::test)]
async fn test_load_token_info() {
    dotenv().ok();
    let test_instance = RpcTokenInfoLoader::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));
    let result = test_instance.load("CyUgNnKPQLqFcheyGV8wmypnJqojA7NzsdJjTS4nUT2j").await;
    assert!(result.is_some());

    let result = result.unwrap();
    assert_eq!(result.mint, "CyUgNnKPQLqFcheyGV8wmypnJqojA7NzsdJjTS4nUT2j");
    assert_eq!(result.name, "kith gil");
    assert_eq!(result.symbol, "gil");
    assert_eq!(result.decimals, 6);
}

#[test_log::test(tokio::test)]
async fn test_token_2022_without_extension() {
    dotenv().ok();
    let test_instance = RpcTokenInfoLoader::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));
    let token_info = test_instance.load("7atgF8KQo4wJrD5ATGX7t1V2zVvykPJbFfNeVf1icFv1").await.unwrap();

    assert_eq!(token_info.name, "");
    assert_eq!(token_info.symbol, "");
    assert_eq!(token_info.decimals, 2);
}

#[test_log::test(tokio::test)]
async fn test_token_2022_with_extension() {
    dotenv().ok();
    let test_instance = RpcTokenInfoLoader::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));
    let token_info = test_instance.load("2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo").await.unwrap();

    assert_eq!(token_info.mint, "2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo");
    assert_eq!(token_info.name, "PayPal USD");
    assert_eq!(token_info.symbol, "PYUSD");
    assert_eq!(token_info.decimals, 6);
}

#[test_log::test(tokio::test)]
async fn test_bug_decimal_mismatch() {
    dotenv().ok();
    let test_instance = RpcTokenInfoLoader::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));
    let token_info = test_instance.load("4h41QKUkQPd2pCAFXNNgZUyGUxQ6E7fMexaZZHziCvhh").await.unwrap();

    assert_eq!(token_info.decimals, 6);
}

#[test_log::test(tokio::test)]
async fn test_not_token() {
    dotenv().ok();
    let test_instance = RpcTokenInfoLoader::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));
    let result = test_instance.load("BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC").await;
    assert!(result.is_none());
}

#[test_log::test(tokio::test)]
async fn test_account_does_not_exists() {
    dotenv().ok();
    let test_instance = RpcTokenInfoLoader::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));
    let result = test_instance.load("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYA").await;
    assert!(result.is_none());
}
