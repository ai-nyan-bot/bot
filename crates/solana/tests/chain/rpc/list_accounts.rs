// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use dotenv::dotenv;
use solana::rpc::RpcClient;
use std::env;
use testing::hash_for_testing;

#[test_log::test(tokio::test)]
async fn test_one() {
    dotenv().ok();
    let test_instance =
        RpcClient::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let mut result = test_instance
        .list_accounts(["CpV7zK77DkyVvbCgGWtWrsqvh2VonFrqPg8cecrTpump"])
        .await
        .unwrap();

    assert_eq!(result.len(), 1);

    let result = result.pop().unwrap().unwrap();
    assert_eq!(result.lamports, 1461600);
    assert_eq!(result.data.len(), 82);
    assert_eq!(hash_for_testing(&result.data), 13739370046195003499);
    assert_eq!(result.owner, "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    assert_eq!(result.executable, false);
    assert_eq!(result.rent_epoch, 18446744073709551615);
}

#[test_log::test(tokio::test)]
async fn test_ok() {
    dotenv().ok();
    let test_instance =
        RpcClient::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let mut result = test_instance
        .list_accounts([
            "CpV7zK77DkyVvbCgGWtWrsqvh2VonFrqPg8cecrTpump",
            "HtjUkVqfhCSMKC5n6MDP3ErgKJZn62gAczsZXWGmK5jk",
        ])
        .await
        .unwrap();

    assert_eq!(result.len(), 2);

    let second = result.pop().unwrap().unwrap();
    assert_eq!(second.lamports, 2039280);
    assert_eq!(second.data.len(), 165);
    assert_eq!(hash_for_testing(&second.data), 17129445893295198644);
    assert_eq!(second.owner, "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    assert_eq!(second.executable, false);
    assert_eq!(second.rent_epoch, 18446744073709551615);

    let first = result.pop().unwrap().unwrap();
    assert_eq!(first.lamports, 1461600);
    assert_eq!(first.data.len(), 82);
    assert_eq!(hash_for_testing(&first.data), 13739370046195003499);
    assert_eq!(first.owner, "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    assert_eq!(first.executable, false);
    assert_eq!(first.rent_epoch, 18446744073709551615);
}

#[test_log::test(tokio::test)]
async fn test_base64_required() {
    dotenv().ok();
    let test_instance =
        RpcClient::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let mut result = test_instance
        .list_accounts(["JK7en6e1oFbCB6r5mQcEbsDnFixnj7SARa6Sed1kx7q"])
        .await
        .unwrap();

    let result = result.pop().unwrap().unwrap();
    assert_eq!(result.lamports, 2039280);
    assert_eq!(result.data.len(), 165);
    assert_eq!(hash_for_testing(&result.data), 16534873344988567128);
    assert_eq!(result.owner, "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    assert_eq!(result.executable, false);
    assert_eq!(result.rent_epoch, 18446744073709551615);
}

#[test_log::test(tokio::test)]
async fn test_partially_not_found() {
    dotenv().ok();
    let test_instance =
        RpcClient::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));

    let mut result = test_instance
        .list_accounts([
            "CpV7zK77DkyVvbCgGWtWrsqvh2VonFrqPg8cecrTpump",
            "22222222222222222222222222222222222222222222",
        ])
        .await
        .unwrap();

    assert_eq!(result.len(), 2);
    let second = result.pop().unwrap();
    assert!(second.is_none());

    let first = result.pop().unwrap().unwrap();
    assert_eq!(first.lamports, 1461600);
    assert_eq!(first.data.len(), 82);
    assert_eq!(hash_for_testing(&first.data), 13739370046195003499);
    assert_eq!(first.owner, "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    assert_eq!(first.executable, false);
    assert_eq!(first.rent_epoch, 18446744073709551615);
}
