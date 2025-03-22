// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{AddressRepo, TokenRepo};
use base::test::NeverCalledTokenInfoLoader;
use common::model::Timestamp;
use common::repo::Tx;
use indexer::solana::block::index_block;
use indexer::solana::state::{State, StateInner};
use solana::repo::BalanceRepo;
use sqlx::Executor;
use std::sync::Arc;
use solana::convert::convert_block;
use solana::model::Slot;
use testing::{jupiter, pumpfun, run_test_with_pool_on_empty_db};

#[test_log::test(sqlx::test)]
async fn test_index_block_318124628() {
    run_test_with_pool_on_empty_db(|pool| async move {
		let block = serde_json::from_str(include_str!("./block_318124628.json")).unwrap();
		let block = convert_block(Slot::from(318124628), block).await.unwrap().unwrap();

		pool.acquire().await.unwrap().execute(r#"
insert into solana.token (mint, name, symbol, decimals, supply, metadata, description, image, website, creator_id, block_id, block_time) values
        ('FeR8VBqNRSUD5NtXAj2n3j1dAHkZHfyDktKuLXD4pump', 'jelly-my-jelly', 'jellyjelly', 6, 999976454.536214000000, 'https://ipfs.io/ipfs/QmRaah2aa24T3F2hGQCf8XefSuaNFZM2wGx2W2UnsxfLxM', null, null, null, null, null, null),
        ('9RjwNo6hBPkxayWHCqQD1VjaH8igSizEseNZNbddpump', 'StupidCoin', 'Stupid', 6, 994795729.276383000000, 'https://ipfs.io/ipfs/QmWhomQR3CWJ9CJtuc1pAeH9deE7V5Rh5n7Rnot2KHnmdY', null, null, null, null, null, null),
        ('41yoTfo3DzjYnwponfhznD9EGNEjfcVmADsaNELq2mzW', null, null, 0, 82190693199511552.000000000000, null, null, null, null, null, null, null),
        ('61V8vBaqAGMpgDQi4JcAwo1dmBGHsyhzodcPqnEVpump', 'AI Rig Complex', 'arc', 6, 999995246.110816000000, 'https://ipfs.io/ipfs/QmZf8F4hXkAma1fUY8ZLhbxtpdi1anGAof5FqVAjrKrYhh', null, null, null, null, null, null),
        ('CzLSujWBLFsSjncfkh59rUFqvafWcY5tzedWJSuypump', 'Goatseus Maximus', 'GOAT', 6, 999993932.201783000000, 'https://ipfs.io/ipfs/QmcGmF4tQxsQKdF3s1mSqYyg1vNqAXou32tbWztnsnEzth', null, null, null, null, null, null),
        ('HNg5PYJmtqcmzXrv6S9zP1CDKk5BgDuyFBxbvNApump', 'Alchemist AI', 'ALCH', 6, 999988163.003343000000, 'https://ipfs.io/ipfs/Qmbe4nA5bYwmGLJ6Bgcegf8tqUHBAVSNPQmT8gXYSb8rUN', null, null, null, null, null, null),
        ('49edhRJjQ7R9zRv9R72NdfNvQ9UDsXXaXm4r4Jrupump', 'Kick', 'KICK', 9, 995015884.604870783000, 'https://ipfs.io/ipfs/bafkreidysw44hinbrzzwxttflifqhtiqez7e3spvii2qo3p64frlmitw4a', null, null, null, null, null, null),
        ('Dfh5DzRgSvvCFDoYc2ciTkMrbDfRKybA4SoFbPmApump', 'Pippin', 'pippin', 6, 999946697.481608000000, 'https://ipfs.io/ipfs/QmWbM38Bhhcj4vQZpSyesRSotgoqicNjfaNoMmuDGtWZ2o', null, null, null, null, null, null),
        ('9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump', 'Fartcoin', 'Fartcoin', 6, 999993232.396262000000, 'https://ipfs.io/ipfs/QmYfe8zVGHA1heej47AkBX3Nnetg2h2kqj5yymz1xyKeHb', null, null, null, null, null, null),
        ('4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R', 'Raydium', 'RAY', 6, 554999065.508427000000, '', null, null, null, null, null, null),
        ('fueL3hBZjLLLJHiFH9cqZoozTG3XQZ53diwFPwbzNim', 'Fuel', 'FUEL', 0, 136453721717.000000000000, 'https://galaxy.staratlas.com/items/fueL3hBZjLLLJHiFH9cqZoozTG3XQZ53diwFPwbzNim', null, null, null, null, null, null),
        ('LSTo3PdLJmcm7r5gjN1RvR5nATg3UQbDstbXGL6xqQu', 'Eric the Goldfish', 'ERIC', 9, 999052932.180772279000, 'https://bafkreic3iehmj6tjkdqag2ke6v4d2om5m3l3nbvyq66vzu2hxsdx6k5dge.ipfs.nftstorage.link', null, null, null, null, null, null),
        ('EHPLVoaaQzpcUch6SiGWn8X5FX3YoxzE2YdPrtYF6dd4', 'WAGIE', 'WAGIE', 9, 69696877.542489867000, 'https://ipfs.io/ipfs/Qma68yUCHuamudyEiMmPGc5UPSmEB8YTCt7vi4TorLuF5x', null, null, null, null, null, null),
        ('6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN', 'OFFICIAL TRUMP', 'TRUMP', 6, 999999479.923616000000, 'https://arweave.net/cSCP0h2n1crjeSWE9KF-XtLciJalDNFs7Vf-Sm0NNY0', null, null, null, null, null, null),
        ('9w1rE2Jd9nW2E26vpmDpbrP1p3erwF47RNrdJDm31NnE', 'Migrating to McDonalds', 'MTM', 6, 999920832.714885000000, 'https://ipfs.io/ipfs/QmVGaWnWffMjmGoT7H9wAK9oD1BGUEqR2aQA9qyMDwRgBG', null, null, null, null, null, null),
        ('B2XHuZRCTowyeMpekfw1w1Ku6TdRc1N75WQ63896pump', 'GENERATIONAL BOTTOM', 'BOTTOM', 6, 999985997.000924000000, 'https://ipfs.io/ipfs/QmVphrZA8jWTinrnawLXkK3j2xTvoCEEcE5vY6FjfX5nNV', null, null, null, null, null, null),
        ('foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG', 'Food', 'FOOD', 0, 119099162958.000000000000, 'https://galaxy.staratlas.com/items/foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG', null, null, null, null, null, null),
        ('AdmrDJaLVSPaABXVfXfmvwNHiD7t4oRZ5hEF91H1moon', 'Dolos The Bully', '$BULLY', 6, 3423956.827490000000, 'https://ipfs.io/ipfs/QmV2rVtPr2QVFW25USbE9LPfsug9fEsCKCGZuFjXzX1htu', null, null, null, null, null, null),
        ('6uBHVHbEXESZ7UFcjd9wBJcsVGYj11WBGp2QRj9Epump', 'PEECOIN', '$PEE', 6, 999222477.095414000000, 'https://ipfs.io/ipfs/QmeRNbYnMny9QCHG9Xj334J2VJjqC7cjpVeBYZnVXxR8a5', null, null, null, null, null, null),
        ('EfgEGG9PxLhyk1wqtqgGnwgfVC7JYic3vC9BCWLvpump', 'Gyat Coin', 'GYAT', 6, 997528855.166366000000, 'https://ipfs.io/ipfs/QmcCs4RV1PcdPxkyCHHeSNdcLu6XR2CwwXvbdg11EC8u9r', null, null, null, null, null, null),
        ('EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm', 'dogwifhat', '$WIF', 6, 998840932.234952000000, 'https://bafkreihwqhounu3cdwgvk2gc2dqcinpntlccbo3xcy4xuerd24yndldl5q.ipfs.nftstorage.link', null, null, null, null, null, null),
        ('BHoiUQCAQJuGKVyGubjBSqtPn2tTme7sauAVw6H4pump', 'ISHOWMEAT', 'ISM', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmdecrFab42GHtRqXiH4xXtM2LHk41y1zkyKcKxJrP6Bf3', null, null, null, null, null, null);
        "#).await.unwrap();

		let pumpfun_swap_repo = solana::pumpfun::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));
		let jupiter_swap_repo = solana::jupiter::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));

		let state = State(Arc::new(StateInner {
			token_repo: TokenRepo::testing_no_token_info(),
			address_repo: AddressRepo::new(),
			balance_repo: BalanceRepo::new(),
			pool: pool.clone(),
			pumpfun_swap_repo,
			pumpfun_current_repo: Default::default(),
			jupiter_swap_repo,
		}));

		index_block(state, block).await;

		let mut tx = pool.begin().await.unwrap();
		let count = pumpfun::count_swaps(&mut tx).await;
		assert_eq!(count, 3);
		assert_pumpfun_swaps(&mut tx).await;

		let count = jupiter::count_swaps(&mut tx).await;
		assert_eq!(count, 3);
		assert_jupiter_swaps(&mut tx).await;

		let count = jupiter::count_micro_swaps(&mut tx).await;
		assert_eq!(count, 4);
		assert_jupiter_swaps(&mut tx).await;
	})
		.await
}

async fn assert_jupiter_swaps<'a>(tx: &mut Tx<'a>) {
    let swaps = jupiter::list_all_swaps(tx).await;
    assert_eq!(swaps.len(), 3);

    let swap = swaps.iter().find(|t| t.signature == "5Nf5fuXHg1WRYvDcNPeq8ciDQHPGdqUvG1FHU3nEXVfDMqCCP9pMAanGK6YyqDP515tQ4kZJaQVQX5w1NUJXkdoS").unwrap();

    assert_eq!(swap.address, 2266);
    assert_eq!(swap.amount_base, "920.381148");
    assert_eq!(swap.amount_quote, "0.503951337");

    assert_eq!(swap.price, "0.000547546349");
    assert!(swap.is_buy);
    assert_eq!(
        swap.timestamp,
        Timestamp::from_epoch_second(1738554016).unwrap()
    );

    // micro swaps
    let swaps = swaps
		.into_iter()
		.filter(|t| t.signature == "5tC86xHQJHj2oFd23P58bjNtkjQhAE3UDUAigLLJiKf3fmVhDP5YW9KzuZjSMxU5nzKf83njzcMNxoCbHDWNuv13")
		.collect::<Vec<_>>();
    assert_eq!(swaps.len(), 0);

    let mut swaps = jupiter::list_micro_with_signature(
        tx,
        "5tC86xHQJHj2oFd23P58bjNtkjQhAE3UDUAigLLJiKf3fmVhDP5YW9KzuZjSMxU5nzKf83njzcMNxoCbHDWNuv13",
    )
    .await;
    assert_eq!(swaps.len(), 3);

    let third = swaps.pop().unwrap();
    assert_eq!(third.address, 2760);
    assert_eq!(third.token_pair, 1008);
    assert_eq!(third.amount_base, "9906000e-12");
    assert_eq!(third.amount_quote, "207200e-8");
    assert_eq!(third.price, "209166161922067e-12");
    assert!(!third.is_buy);
    assert_eq!(
        third.timestamp,
        Timestamp::from_epoch_second(1738554016).unwrap()
    );

    let second = swaps.pop().unwrap();
    assert_eq!(second.address, 2760);
    assert_eq!(second.token_pair, 1007);
    assert_eq!(second.amount_base, "9906000e-12");
    assert_eq!(second.amount_quote, "8486000e-12");
    assert_eq!(second.price, "856652533818e-12");
    assert!(second.is_buy);
    assert_eq!(
        second.timestamp,
        Timestamp::from_epoch_second(1738554016).unwrap()
    );

    let first = swaps.pop().unwrap();
    assert_eq!(first.address, 2760);
    assert_eq!(first.token_pair, 1006);
    assert_eq!(first.amount_base, "8486000e-12");
    assert_eq!(first.amount_quote, "10697000e-12");
    assert_eq!(first.price, "1260546782937e-12");
    assert!(first.is_buy);
    assert_eq!(
        first.timestamp,
        Timestamp::from_epoch_second(1738554016).unwrap()
    );
}

async fn assert_pumpfun_swaps<'a>(tx: &mut Tx<'a>) {
    let swaps = pumpfun::list_all_swaps(tx).await;
    assert_eq!(swaps.len(), 3);

    // OKX transfer - 4ZpWPGRKmR4ChymvfWXabC4jbch6ryee9UJEQUbyfdmo2rMYyd9mYjt9AkRsz94p8ZqYwpTDTtYuaQHnLXxRvzys
    let first = swaps.iter().find(|t| t.token_pair == 1000).unwrap();
    assert_eq!(first.address, 2142);
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
    assert_eq!(second.address, 2786);
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
    assert_eq!(third.address, 2816);
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
