// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

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

		let pumpfun_trade_repo = solana::pumpfun::repo::TradeRepo::testing(NeverCalledTokenInfoLoader{});
		let jupiter_trade_repo = solana::jupiter::repo::TradeRepo::testing(NeverCalledTokenInfoLoader{});

		let state = State(Arc::new(StateInner {
			pool: pool.clone(),
			pumpfun_trade_repo,
			pumpfun_curve_repo: Default::default(),
			jupiter_trade_repo,
		}));

		index_block(state, block).await;

		let mut tx = pool.begin().await.unwrap();
		let count = pumpfun::count_all_trades(&mut tx).await;
		assert_eq!(count, 3);
		assert_pumpfun_trades(&mut tx).await;

		let count = jupiter::count_all_trades(&mut tx).await;
		assert_eq!(count, 5);
		assert_jupiter_trades(&mut tx).await;
	})
		.await
}

async fn assert_jupiter_trades<'a>(tx: &mut Tx<'a>) {
    let trades = jupiter::list_all_trades(tx).await;
    assert_eq!(trades.len(), 5);

    // DYbMBUwwEdeMiQ6iCHRsKZuDRvkVLvKfkvdAF1rFbshRoNRb2cS5GQEVwwgGJG4vgTxB2TxmYtKX8jfrgmaQN44
    let trade = trades
        .iter()
        .find(|t| t.base_amount == 0.000010703)
        .unwrap();

    assert_eq!(trade.address, 6);
    assert_eq!(trade.token_pair, 2);
    assert_eq!(trade.base_amount, 0.000010703);
    assert_eq!(trade.quote_amount, 0.002072);
    assert_eq!(trade.price, 193.5905820797907);
    assert!(!trade.is_buy);
    assert_eq!(
        trade.timestamp,
        Timestamp::from_epoch_second(1738554016).unwrap()
    );

    // 5tC86xHQJHj2oFd23P58bjNtkjQhAE3UDUAigLLJiKf3fmVhDP5YW9KzuZjSMxU5nzKf83njzcMNxoCbHDWNuv13
    let trade = trades
        .iter()
        .find(|t| t.base_amount == 0.000010697)
        .unwrap();

    assert_eq!(trade.address, 6);
    assert_eq!(trade.token_pair, 2);
    assert_eq!(trade.base_amount, 0.000010697);
    assert_eq!(trade.quote_amount, 0.002072);
    assert_eq!(trade.price, 193.69916799102555);
    assert!(!trade.is_buy);
    assert_eq!(
        trade.timestamp,
        Timestamp::from_epoch_second(1738554016).unwrap()
    );

    // iMLZWF2Y26uvJL49yAqWBS2kQnC6D14Jr6Vyn2LpeYFZqWR5vsFMfiXm4pm7b8yobdEqoRTpT9VpADo9RWNQ3pB
    let trade = trades.iter().find(|t| t.token_pair == 1004).unwrap();

    assert_eq!(trade.address, 5);
    assert_eq!(trade.base_amount, 274.241866);
    assert_eq!(trade.quote_amount, 1.0);
    assert_eq!(trade.price, 0.0036464162623514236);
    assert!(trade.is_buy);
    assert_eq!(
        trade.timestamp,
        Timestamp::from_epoch_second(1738554016).unwrap()
    );

    // 5Nf5fuXHg1WRYvDcNPeq8ciDQHPGdqUvG1FHU3nEXVfDMqCCP9pMAanGK6YyqDP515tQ4kZJaQVQX5w1NUJXkdoS
    let trade = trades.iter().find(|t| t.token_pair == 1005).unwrap();

    assert_eq!(trade.address, 5);
    assert_eq!(trade.base_amount, 920.381148);
    assert_eq!(trade.quote_amount, 0.503951337);
    assert_eq!(trade.price, 0.0005475463487003105);
    assert!(trade.is_buy);
    assert_eq!(
        trade.timestamp,
        Timestamp::from_epoch_second(1738554016).unwrap()
    );
}

async fn assert_pumpfun_trades<'a>(tx: &mut Tx<'a>) {
    let trades = pumpfun::list_all_trades(tx).await;
    assert_eq!(trades.len(), 3);

    // OKX transfer - 4ZpWPGRKmR4ChymvfWXabC4jbch6ryee9UJEQUbyfdmo2rMYyd9mYjt9AkRsz94p8ZqYwpTDTtYuaQHnLXxRvzys
    let first = trades.iter().find(|t| t.token_pair == 1000).unwrap();
    assert_eq!(first.address, 1);
    assert_eq!(first.base_amount, 1950554.226272);
    assert_eq!(first.quote_amount, 0.1414091);
    assert_eq!(first.price, 0.00000007249688221704474);
    assert!(first.is_buy);
    assert_eq!(
        first.timestamp,
        Timestamp::from_epoch_second(1738554016).unwrap()
    );
    assert_eq!(first.virtual_base_reserves, 665373017527792);
    assert_eq!(first.virtual_quote_reserves, 48378878041);

    // SELL - 57g7GeNn9j8J819XXd6eAqGThZsdwDHwNNVxMe6oJwAZwPYgN9G5R7co4B5SuyjpBCaf2nfNXmDJQuV98E6rGzpN
    let second = trades.iter().find(|t| t.token_pair == 1001).unwrap();
    assert_eq!(second.address, 2);
    assert_eq!(second.base_amount, 456403.879924);
    assert_eq!(second.quote_amount, 0.014259323);
    assert_eq!(second.price, 0.000000031242773401432195);
    assert!(!second.is_buy);
    assert_eq!(
        second.timestamp,
        Timestamp::from_epoch_second(1738554016).unwrap()
    );
    assert_eq!(second.virtual_base_reserves, 1015274165045491);
    assert_eq!(second.virtual_quote_reserves, 31705721601);

    // BUY 4FufGr26C7XNSMwqt8y51LYvf8UhgdDmmZicfQoRiGTiD549772Q54Q7GPeZ5EmnshMY4Sdb2Vph78cXiyJe8Bzo
    let third = trades.iter().find(|t| t.token_pair == 1002).unwrap();
    assert_eq!(third.address, 3);
    assert_eq!(third.base_amount, 732322.630357);
    assert_eq!(third.quote_amount, 0.020550522);
    assert_eq!(third.price, 0.00000002806211517727074);
    assert!(third.is_buy);
    assert_eq!(
        third.timestamp,
        Timestamp::from_epoch_second(1738554016).unwrap()
    );
    assert_eq!(third.virtual_base_reserves, 1070660588003693);
    assert_eq!(third.virtual_quote_reserves, 30065550522);
}
