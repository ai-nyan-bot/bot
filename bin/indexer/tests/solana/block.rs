// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use indexer::solana::block::index_block;
use indexer::solana::state::{State, StateInner};
use solana::rpc::RpcClient;
use solana::token_info::rpc::TokenInfoRpcLoader;
use std::sync::Arc;
use testing::run_test_with_pool_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_index_block_318124628() {
    run_test_with_pool_on_empty_db(|pool| async move {
        let rpc_client = RpcClient::new("http://api.mainnet-beta.solana.com");
        let block = rpc_client.get_block(318124628).await.unwrap().unwrap();

        let rpc_loader = TokenInfoRpcLoader::new("http://api.mainnet-beta.solana.com");

        let pumpfun_trade_repo = solana::pumpfun::repo::TradeRepo::testing(rpc_loader.clone());
        let jupiter_trade_repo = solana::jupiter::repo::TradeRepo::testing(rpc_loader.clone());

        let state = State(Arc::new(StateInner {
            pool: pool.clone(),
            pumpfun_trade_repo: pumpfun_trade_repo,
            pumpfun_curve_repo: Default::default(),
            jupiter_trade_repo: jupiter_trade_repo,
        }));

        index_block(state, block).await;
    })
    .await
}
