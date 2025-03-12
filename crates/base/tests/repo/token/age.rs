// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::Decimals;
use base::repo::{TokenRepo, TokenToInsert};
use base::test::SuccessfulTokenInfoLoader;
use common::model::{BlockTime, Timestamp};
use testing::run_test_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_insert_has_no_age_but_getting_the_token_does() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(Box::new(SuccessfulTokenInfoLoader::default()));

        let now = Timestamp::now().to_epoch_seconds();
        let block_time = BlockTime(Timestamp::from_epoch_second(now - 99).unwrap());
        let mut result = test_instance
            .insert_token(
                &mut tx,
                [TokenToInsert {
                    block: Some(42.into()),
                    block_time: Some(block_time.clone()),
                    mint: "44J6Um1tTiTbtL9nd4hU6MqDyPppeWtGr3rMFQ6ppump".into(),
                    name: None,
                    symbol: None,
                    decimals: Decimals::from(6),
                    supply: None,
                    metadata: None,
                    description: None,
                    image: None,
                    website: None,
                    creator: None,
                }],
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        let result = result.pop().unwrap();
        assert_eq!(result.id, 1000);
        assert_eq!(result.block_time.unwrap(), block_time);
        assert_eq!(result.age, None);

        let result = test_instance
            .get_by_mint(&mut tx, "44J6Um1tTiTbtL9nd4hU6MqDyPppeWtGr3rMFQ6ppump")
            .await
            .unwrap();

        assert_eq!(result.block_time.unwrap(), block_time);
        assert!(result.age.unwrap() >= 99, "Must be at least 99 seconds old");
    })
    .await
}