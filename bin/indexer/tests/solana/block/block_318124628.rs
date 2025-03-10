// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::TokenRepo;
use base::test::NeverCalledTokenInfoLoader;
use common::model::Timestamp;
use common::repo::Tx;
use indexer::solana::block::index_block;
use indexer::solana::state::{State, StateInner};
use sqlx::Executor;
use std::sync::Arc;
use testing::{jupiter, pumpfun, run_test_with_pool_on_empty_db};

#[test_log::test(sqlx::test)]
async fn test_index_block_318124628() {
	run_test_with_pool_on_empty_db(|pool| async move {
		let block = serde_json::from_str(include_str!("./block_318124628.json")).unwrap();

		pool.acquire().await.unwrap().execute(r#"
        insert into solana.token (id, version, mint, name, symbol, decimals) values
            (1000, 0, '9w1rE2Jd9nW2E26vpmDpbrP1p3erwF47RNrdJDm31NnE', 'Migrating to McDonalds', 'MTM', 6),
            (1001, 0, 'B2XHuZRCTowyeMpekfw1w1Ku6TdRc1N75WQ63896pump', 'GENERATIONAL BOTTOM', 'BOTTOM', 6),
            (1002, 0, 'BHoiUQCAQJuGKVyGubjBSqtPn2tTme7sauAVw6H4pump', 'ISHOWMEAT', 'ISM', 6),
            (1003, 0, '9RjwNo6hBPkxayWHCqQD1VjaH8igSizEseNZNbddpump', 'StupidCoin', 'Stupid', 6),
            (1004, 0, 'EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm', 'dogwifhat', '$WIF', 6),
            (1005, 0, 'CzLSujWBLFsSjncfkh59rUFqvafWcY5tzedWJSuypump', 'Goatseus Maximus', 'GOAT', 6);
        "#).await.unwrap();

		let token_repo = TokenRepo::testing_no_token_info();
		let pumpfun_swap_repo = solana::pumpfun::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));
		let jupiter_swap_repo = solana::jupiter::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));

		let state = State(Arc::new(StateInner {
			// token_repo,
			pool: pool.clone(),
			pumpfun_swap_repo,
			pumpfun_curve_repo: Default::default(),
			jupiter_swap_repo,
		}));

		index_block(state, block).await;

		let mut tx = pool.begin().await.unwrap();
		let count = pumpfun::count_all_swaps(&mut tx).await;
		assert_eq!(count, 3);
		assert_pumpfun_swaps(&mut tx).await;

		let count = jupiter::count_all_swaps(&mut tx).await;
		assert_eq!(count, 7);
		assert_jupiter_swaps(&mut tx).await;
	})
		.await
}

async fn assert_jupiter_swaps<'a>(tx: &mut Tx<'a>) {
	let swaps = jupiter::list_all_swaps(tx).await;
	assert_eq!(swaps.len(), 7);

	let swap = swaps
		.iter()
		.find(|t| t.signature == "DYbMBUwwEdeMiQ6iCHRsKZuDRvkVLvKfkvdAF1rFbshRoNRb2cS5GQEVwwgGJG4vgTxB2TxmYtKX8jfrgmaQN44")
		.unwrap();

	assert_eq!(swap.address, 6);
	assert_eq!(swap.token_pair, 2);
	assert_eq!(swap.amount_base, "0.000010703");
	assert_eq!(swap.amount_quote, "0.002072");
	assert_eq!(swap.price, "193.590582079791");
	assert!(!swap.is_buy);
	assert_eq!(
		swap.timestamp,
		Timestamp::from_epoch_second(1738554016).unwrap()
	);

	let swap = swaps.iter().find(|t| t.signature == "5Nf5fuXHg1WRYvDcNPeq8ciDQHPGdqUvG1FHU3nEXVfDMqCCP9pMAanGK6YyqDP515tQ4kZJaQVQX5w1NUJXkdoS").unwrap();

	assert_eq!(swap.address, 5);
	assert_eq!(swap.amount_base, "920.381148");
	assert_eq!(swap.amount_quote, "0.503951337");

	assert_eq!(swap.price, "0.000547546349");
	assert!(swap.is_buy);
	assert_eq!(
		swap.timestamp,
		Timestamp::from_epoch_second(1738554016).unwrap()
	);

	let mut swaps = swaps
		.into_iter()
		.filter(|t| t.signature == "5tC86xHQJHj2oFd23P58bjNtkjQhAE3UDUAigLLJiKf3fmVhDP5YW9KzuZjSMxU5nzKf83njzcMNxoCbHDWNuv13")
		.collect::<Vec<_>>();

	assert_eq!(swaps.len(), 3);

	let first = swaps.pop().unwrap();
	assert_eq!(first.address, 6);
	assert_eq!(first.token_pair, 1006);
	assert_eq!(first.amount_base, "8486000e-12");
	assert_eq!(first.amount_quote, "10697000e-12");
	assert_eq!(first.price, "1260546782937e-12");
	assert!(first.is_buy);
	assert_eq!(
		first.timestamp,
		Timestamp::from_epoch_second(1738554016).unwrap()
	);


	let second = swaps.pop().unwrap();
	assert_eq!(second.address, 6);
	assert_eq!(second.token_pair, 1007);
	assert_eq!(second.amount_base, "9906000e-12");
	assert_eq!(second.amount_quote, "8486000e-12");
	assert_eq!(second.price, "856652533818e-12");
	assert!(second.is_buy);
	assert_eq!(
		second.timestamp,
		Timestamp::from_epoch_second(1738554016).unwrap()
	);


	let third = swaps.pop().unwrap();
	assert_eq!(third.address, 6);
	assert_eq!(third.token_pair, 1008);
	assert_eq!(third.amount_base, "9906000e-12");
	assert_eq!(third.amount_quote, "207200e-8");
	assert_eq!(third.price, "209166161922067e-12");
	assert!(!third.is_buy);
	assert_eq!(
		third.timestamp,
		Timestamp::from_epoch_second(1738554016).unwrap()
	);
}

async fn assert_pumpfun_swaps<'a>(tx: &mut Tx<'a>) {
	let swaps = pumpfun::list_all_swaps(tx).await;
	assert_eq!(swaps.len(), 3);

	// OKX transfer - 4ZpWPGRKmR4ChymvfWXabC4jbch6ryee9UJEQUbyfdmo2rMYyd9mYjt9AkRsz94p8ZqYwpTDTtYuaQHnLXxRvzys
	let first = swaps.iter().find(|t| t.token_pair == 1000).unwrap();
	assert_eq!(first.address, 1);
	assert_eq!(first.amount_base, "1950554.226272");
	assert_eq!(first.amount_quote, "0.1414091");
	assert_eq!(first.price, "72497e-12");
	assert!(first.is_buy);
	assert_eq!(
		first.timestamp,
		Timestamp::from_epoch_second(1738554016).unwrap()
	);
	assert_eq!(first.virtual_base_reserves, 665373017527792);
	assert_eq!(first.virtual_quote_reserves, 48378878041);

	// SELL - 57g7GeNn9j8J819XXd6eAqGThZsdwDHwNNVxMe6oJwAZwPYgN9G5R7co4B5SuyjpBCaf2nfNXmDJQuV98E6rGzpN
	let second = swaps.iter().find(|t| t.token_pair == 1001).unwrap();
	assert_eq!(second.address, 2);
	assert_eq!(second.amount_base, "456403.879924");
	assert_eq!(second.amount_quote, "0.014259323");
	assert_eq!(second.price, "31243e-12");
	assert!(!second.is_buy);
	assert_eq!(
		second.timestamp,
		Timestamp::from_epoch_second(1738554016).unwrap()
	);

	assert_eq!(second.virtual_base_reserves, 1015274165045491);
	assert_eq!(second.virtual_quote_reserves, 31705721601);

	// BUY 4FufGr26C7XNSMwqt8y51LYvf8UhgdDmmZicfQoRiGTiD549772Q54Q7GPeZ5EmnshMY4Sdb2Vph78cXiyJe8Bzo
	let third = swaps.iter().find(|t| t.token_pair == 1002).unwrap();
	assert_eq!(third.address, 3);
	assert_eq!(third.amount_base, "732322.630357");
	assert_eq!(third.amount_quote, "0.020550522");
	assert_eq!(third.price, "28062e-12");
	assert!(third.is_buy);
	assert_eq!(
		third.timestamp,
		Timestamp::from_epoch_second(1738554016).unwrap()
	);
	assert_eq!(third.virtual_base_reserves, 1070660588003693);
	assert_eq!(third.virtual_quote_reserves, 30065550522);
}
