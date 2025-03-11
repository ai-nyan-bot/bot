// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{AddressRepo, TokenRepo};
use base::test::NeverCalledTokenInfoLoader;
use indexer::solana::block::index_block;
use indexer::solana::state::{State, StateInner};
use sqlx::Executor;
use std::ops::Deref;
use std::sync::Arc;
use testing::run_test_with_pool_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_index_block_326027759() {
    // DnLM31nU9K4kczFoAfhbU4hrK7XaAxSJfkieUqnPpump contains invalid utf-8 characters
    run_test_with_pool_on_empty_db(|pool| async move {
		let block = serde_json::from_str(include_str!("./block_326027759.json")).unwrap();

		pool.acquire().await.unwrap().execute(r#"
		insert into solana.token (id, version, mint, name, symbol, decimals, supply, metadata, description, image, website, creator_id, block_id, block_time, created_at, updated_at) values
        (1001, 0, '8zErU24XxdtywoUzCELbd6RKcGT1hTUWX4xCjyjDpump', 'AI Trader Agent', 'AIT', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmfFM4RACpRMzkpbPHZRioZthvGNW3U2d4A59wv3ZmvfVC', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1002, 0, '8VbR1WBaJXxCGmcK9fwUUPEfPjcMymWbj5SjXF9ppump', 'SMOG', 'SMOG', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmRK9JRCcpardHoJLFwfHi7Jx1M52KLBQBMWdwp78H7kV9', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1003, 0, 'GcTiQBMLQjFPK9LLKuCBchFbP2VLaQv6YABpDF6mpump', 'Justice for Wheesung', 'Wheesung', 6, 999797824.572475000000, 'https://ipfs.io/ipfs/QmWYm27h9NceuctPRsDDkrcqPbHUoHmH6P2KTZojjbdsCv', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1004, 0, '2GCTWvAEdtnjMh7tDS9cre7en98QiHYQK16yP6hQpump', 'wanksy', 'wanksy', 6, 999803955.438615000000, 'https://ipfs.io/ipfs/QmSTN8CBGTkgBRvt1Q4FubzUWpuxfc68xdvas56d4eyTdH', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1006, 0, 'D8bxLkUqwQVopFY4mn78E16wDbgy6BgjFjyMJBFnpump', 'INDIA X COINBASE', 'JEETBASE', 6, 999623384.040572000000, 'https://ipfs.io/ipfs/QmPhCm3dgNYETXRnHkbm4JHtqGmC84WhPmnQ9WGGyW4PiH', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1007, 0, 'FK5teYaPuVAQUStCoWTfY966WEx1vf8S14eNmbS9pump', 'wetard ye', 'wye', 6, 999999835.862988000000, 'https://ipfs.io/ipfs/QmQrFf8LfdPDERyrKGTVu4pp1puJSDKDp9WyrS5ZuSGTKv', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1008, 0, 'JDME4c1i9YdCCqwG65r4u5yCTNF8rJqm2tMuntN3pump', 'dwumstick', 'dwumstick', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmPUsLJJdHvDaPxWPUjK11n8b8vXaPHJs7D81qR4X1eFft', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1009, 0, '4QPC1PDwhroryitdpGLa5kucGaVc7QLkoVkkNA1dpump', 'billy the ai fish', 'billy', 6, 999911684.343967000000, 'https://ipfs.io/ipfs/QmXuzLTbSYsozXUrY8Y6NDpanxCKzVaxkk75uVfQniF4ku', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1010, 0, 'G1pcv7hEvUavFKHmespp3KsJXrrmrnpSG9qour87pump', 'SPACES MENTAL HEALTH TALK', 'SPACES', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmbDkFPsff3Leh11PCjpxS6MKdNzfVa4TxnBuLkYLLts8L', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1011, 0, 'BZy2VXHdSEtDjy8ku95Gx8ngxLruaDbEohx1ijvX7NhM', 'Pi Network', 'PI', 6, 99999874497.435251000000, 'https://ipfs.io/ipfs/QmRk5ZvFkP95kuGRz6HGnHb9w8EMGdUhvv9TarKGFUtSrE', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1012, 0, 'HTwjtGdooLeNPVkcZFrd2D4dgWo7GVvU9wYg7tpKX9t9', 'PAWS Labs', 'PAWS', 6, 99999999962.789451000000, 'https://ipfs.io/ipfs/QmR38gRQLn8sV8TBY3C5dwdLmRkDLavZ9iCNc81QN64NzX', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1013, 0, 'hntyVP6YFm1Hg25TN9WGLqM12b8TQmcknKrdu1oxWux', 'Helium Network Token', 'HNT', 8, 179283241.602982180000, 'https://shdw-drive.genesysgo.net/6tcnBSybPG7piEDShBcrVtYJDPSvGrDbVvXmXKpzBvWP/hnt.json', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1014, 0, 'AR2SmDfEbV838SXgDitHmBEdYUTxGCBs8GubSCWfpump', 'wetard twump', 'wetard', 6, 999987720.992088000000, 'https://ipfs.io/ipfs/QmemWMVkXuGW17Yu35jpsofQhcdpTq1wSGGk23nTLSLL7L', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1015, 0, '8s1tWaoroV3wdee1MkQ4BoEYFPG8RVppujtJFwTTQQZZ', 'BRIDGE', 'BRIDGE', 6, 999218088.252395000000, 'https://ipfs.io/ipfs/QmWqxSYLfKB627RAUtAzDAYg8iSCdM4u3oP51JJJ4AKhMk', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
        (1016, 0, '2bW2fdEzuGhGFvcmxGUaHuTB7LPYWNmioacuSrweYGX8', 'Bubblemaps', 'BMT', 6, 99999999996.333207000000, 'https://ipfs.io/ipfs/Qme9J9fnZokiv3AvZzAQ3WiUTY5C22ZBEjm1EcxHH5emzH', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00'),
		(1017, 0, 'WGRXSegEZiEghqbSJJAWTnvL3uZnavUPyf3seZhpump', 'PENGU X PEPE', 'PENGU', 6, 999999898.568578000000, 'https://ipfs.io/ipfs/QmbDCtw3s23FkSoD2Ju4B62xWimAxEKtTiuUkdhUXwxT92', null, null, null, null, null, null, '2025-03-11 22:56:44.388315 +00:00', '2025-03-11 22:56:44.388315 +00:00');
        "#).await.unwrap();

		let token_repo = TokenRepo::testing_no_token_info();
		let pumpfun_swap_repo = solana::pumpfun::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));
		let jupiter_swap_repo = solana::jupiter::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));

		let state = State(Arc::new(StateInner {
			token_repo: token_repo.clone(),
			address_repo: AddressRepo::new(),
			pool: pool.clone(),
			pumpfun_swap_repo,
			pumpfun_curve_repo: Default::default(),
			jupiter_swap_repo,
		}));

		index_block(state, block).await;

		let mut tx = pool.begin().await.unwrap();

		let token = token_repo.get_by_mint(&mut tx, "DnLM31nU9K4kczFoAfhbU4hrK7XaAxSJfkieUqnPpump").await.unwrap();
		assert_eq!(token.id, 1000);
		assert_eq!(token.mint, "DnLM31nU9K4kczFoAfhbU4hrK7XaAxSJfkieUqnPpump");
		assert_eq!(token.name.unwrap(), "The forbidden pear");
		assert_eq!(token.symbol.unwrap(), "PEAR");
		assert_eq!(token.decimals, 6);
		assert_eq!(token.supply.unwrap(), 1_000_000_000);
		assert_eq!(token.block.unwrap(), 326027759);
		assert_eq!(token.block_time.unwrap().deref(), "2025-03-11 10:13:53.0 +00:00:00");
	})
		.await
}
