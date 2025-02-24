// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use dotenv::dotenv;
use solana::rpc::RpcClient;
use std::env;
use testing::hash_for_testing;

#[test_log::test(tokio::test)]
async fn test_ok() {
    dotenv().ok();
    let test_instance =
        RpcClient::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let result = test_instance
        .get_account("CpV7zK77DkyVvbCgGWtWrsqvh2VonFrqPg8cecrTpump")
        .await
        .unwrap();

    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.slot > 322765891);

    let account = result.account;
    assert_eq!(account.lamports, 1461600);
    assert_eq!(account.data.len(), 82);
    assert_eq!(hash_for_testing(&account.data), 13739370046195003499);
    assert_eq!(account.owner, "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    assert_eq!(account.executable, false);
    assert_eq!(account.rent_epoch, 18446744073709551615);
}

#[test_log::test(tokio::test)]
async fn test_base64_required() {
    dotenv().ok();
    let test_instance =
        RpcClient::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let result = test_instance
        .get_account("JK7en6e1oFbCB6r5mQcEbsDnFixnj7SARa6Sed1kx7q")
        .await
        .unwrap();

    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.slot > 322765891);

    let account = result.account;
    assert_eq!(account.lamports, 2039280);
    assert_eq!(account.data.len(), 165);
    assert_eq!(hash_for_testing(&account.data), 16534873344988567128);
    assert_eq!(account.owner, "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    assert_eq!(account.executable, false);
    assert_eq!(account.rent_epoch, 18446744073709551615);
}

#[test_log::test(tokio::test)]
async fn test_account_not_found() {
    dotenv().ok();
    let test_instance =
        RpcClient::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let result = test_instance
        .get_account("22222222222222222222222222222222222222222222")
        .await
        .unwrap();

    assert!(result.is_none());
}
