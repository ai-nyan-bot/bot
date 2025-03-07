// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod block_318124628;
mod block_323481688;

//
// use dotenv::dotenv;
// use indexer::solana::block::index_block;
// use indexer::solana::state::{State, StateInner};
// use solana::rpc::RpcClient;
// use solana::token_info::rpc::TokenInfoRpcLoader;
// use std::{env, fs};
// use std::sync::Arc;
// use testing::{run_test_with_pool_on_empty_db};
//
// #[test_log::test(sqlx::test)]
// async fn prepare_test() {
// 	run_test_with_pool_on_empty_db(|pool| async move {
// 		dotenv().ok();
// 		let rpc_client =
// 			RpcClient::new(env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"));
// 		let block = rpc_client.get_block(323481688).await.unwrap().unwrap();
//
// 		fs::write("/tmp/block.json", serde_json::to_string(&block).unwrap()).unwrap();
//
// 		let rpc_loader = TokenInfoRpcLoader::new(
// 			env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"),
// 		);
//
// 		let pumpfun_swap_repo = solana::pumpfun::repo::SwapRepo::testing(rpc_loader.clone());
// 		let jupiter_swap_repo = solana::jupiter::repo::SwapRepo::testing(rpc_loader.clone());
//
// 		let state = State(Arc::new(StateInner {
// 			pool: pool.clone(),
// 			pumpfun_swap_repo: pumpfun_swap_repo,
// 			pumpfun_curve_repo: Default::default(),
// 			jupiter_swap_repo: jupiter_swap_repo,
// 		}));
//
// 		index_block(state, block).await;
//
// 	}).await;
// }
