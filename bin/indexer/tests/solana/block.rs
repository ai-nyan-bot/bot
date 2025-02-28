// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use dotenv::dotenv;
use indexer::solana::block::index_block;
use indexer::solana::state::{State, StateInner};
use solana::rpc::RpcClient;
use solana::token_info::rpc::TokenInfoRpcLoader;
use std::env;
use std::sync::Arc;
use testing::{jupiter, pumpfun, run_test_with_pool_on_empty_db};

#[test_log::test(sqlx::test)]
async fn test_index_block_318124628() {
    run_test_with_pool_on_empty_db(|pool| async move {
        dotenv().ok();
        let rpc_client =
            RpcClient::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));
        let block = rpc_client.get_block(318124628).await.unwrap().unwrap();

        let rpc_loader = TokenInfoRpcLoader::new(
            env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"),
        );

        let pumpfun_trade_repo = solana::pumpfun::repo::TradeRepo::testing(rpc_loader.clone());
        let jupiter_trade_repo = solana::jupiter::repo::TradeRepo::testing(rpc_loader.clone());

        let state = State(Arc::new(StateInner {
            pool: pool.clone(),
            pumpfun_trade_repo: pumpfun_trade_repo,
            pumpfun_curve_repo: Default::default(),
            jupiter_trade_repo: jupiter_trade_repo,
        }));

        index_block(state, block).await;

        let mut tx = pool.begin().await.unwrap();
        let count = pumpfun::count_all_trades(&mut tx).await;
        assert_eq!(count, 3);
        
        let trades = pumpfun::list_all_trades(&mut tx).await;
        assert_eq!(trades.len(), 3);
        
        dbg!(&trades);

        let count = jupiter::count_all_trades(&mut tx).await;
        assert_eq!(count, 5);
    })
    .await
}
