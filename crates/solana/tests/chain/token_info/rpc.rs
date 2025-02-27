// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::LoadTokenInfo;
use dotenv::dotenv;
use solana::token_info::rpc::TokenInfoRpcLoader;
use std::env;

#[test_log::test(tokio::test)]
async fn test_ok() {
    dotenv().ok();
    let test_instance =
        TokenInfoRpcLoader::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));
    let result = test_instance
        .load("CyUgNnKPQLqFcheyGV8wmypnJqojA7NzsdJjTS4nUT2j")
        .await;
    assert!(result.is_some());

    let result = result.unwrap();
    assert_eq!(
        result.mint.unwrap(),
        "CyUgNnKPQLqFcheyGV8wmypnJqojA7NzsdJjTS4nUT2j"
    );
    assert_eq!(result.name.unwrap(), "kith gil");
    assert_eq!(result.symbol.unwrap(), "gil");
    assert_eq!(result.decimals.unwrap(), 6);
    assert!(result.supply.unwrap().0 > 0, "has supply");
    assert_eq!(
        result.metadata.unwrap(),
        "https://ipfs.io/ipfs/QmSb77j7oyJrzwbJTc3EtXA6rCNcwrMAFMVQm3yqcRXZ4y"
    );
}

#[test_log::test(tokio::test)]
async fn test_no_pda() {
    dotenv().ok();
    let test_instance =
        TokenInfoRpcLoader::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));
    let result = test_instance
        .load("SLNDpmoWTVADgEdndyvWzroNL7zSi1dF9PC3xHGtPwp")
        .await;
    assert!(result.is_some());

    let result = result.unwrap();
    dbg!(&result);


    assert_eq!(result.mint.unwrap(), "SLNDpmoWTVADgEdndyvWzroNL7zSi1dF9PC3xHGtPwp");
    assert_eq!(result.name, None);
    assert_eq!(result.symbol, None);
    assert_eq!(result.decimals.unwrap(), 6);
    assert!(result.supply.unwrap().0 > 0, "has supply");
    assert_eq!(result.metadata, None);
}

#[test_log::test(tokio::test)]
async fn test_token_2022_without_extension() {
    dotenv().ok();
    let test_instance =
        TokenInfoRpcLoader::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let result = test_instance
        .load("7atgF8KQo4wJrD5ATGX7t1V2zVvykPJbFfNeVf1icFv1")
        .await
        .unwrap();

    assert_eq!(result.name, None);
    assert_eq!(result.symbol, None);
    assert_eq!(result.decimals.unwrap(), 2);
    assert!(result.supply.unwrap().0 > 0, "has supply");
}

#[test_log::test(tokio::test)]
async fn test_token_2022_with_extension() {
    dotenv().ok();
    let test_instance =
        TokenInfoRpcLoader::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let result = test_instance
        .load("2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo")
        .await
        .unwrap();

    assert_eq!(
        result.mint.unwrap(),
        "2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo"
    );
    assert_eq!(result.name.unwrap(), "PayPal USD");
    assert_eq!(result.symbol.unwrap(), "PYUSD");
    assert_eq!(result.decimals.unwrap(), 6);
    assert!(result.supply.unwrap().0 > 0, "has supply");
    assert_eq!(
        result.metadata.unwrap(),
        "https://token-metadata.paxos.com/pyusd_metadata/prod/solana/pyusd_metadata.json"
    )
}

#[test_log::test(tokio::test)]
async fn test_bug_decimal_mismatch() {
    dotenv().ok();
    let test_instance =
        TokenInfoRpcLoader::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let result = test_instance
        .load("4h41QKUkQPd2pCAFXNNgZUyGUxQ6E7fMexaZZHziCvhh")
        .await
        .unwrap();

    assert_eq!(result.decimals.unwrap(), 6);
}

#[test_log::test(tokio::test)]
async fn test_not_token() {
    dotenv().ok();
    let test_instance =
        TokenInfoRpcLoader::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let result = test_instance
        .load("BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC")
        .await;
    assert!(result.is_none());
}

#[test_log::test(tokio::test)]
async fn test_account_does_not_exists() {
    dotenv().ok();
    let test_instance =
        TokenInfoRpcLoader::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let result = test_instance
        .load("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYA")
        .await;
    assert!(result.is_none());
}
