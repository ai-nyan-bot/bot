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

    // It is not expected that those value ever change, hence those values will be hardcoded instead of
    // calling the rpc endpoint every time, enjoy it a lot if this test ever breaks :-*
    let result = test_instance.get_global_info().await;
    assert_eq!(result.initialized, true);
    assert_eq!(result.initial_virtual_token_reserves, 1073000000000000);
    assert_eq!(result.initial_virtual_sol_reserves, 30000000000);
    assert_eq!(result.initial_real_token_reserves, 793100000000000);
    assert_eq!(result.token_total_supply, 1000000000000000);
    assert_eq!(result.fee_basis_points, 100);
}
