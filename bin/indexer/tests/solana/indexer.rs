// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use indexer::solana::indexer::IndexerRepo;
use testing::run_test_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_set_indexer_first_time() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = IndexerRepo::default();
        test_instance.set(&mut tx, 42).await.unwrap();

        let indexer = test_instance.get(&mut tx).await.unwrap();
        assert_eq!(indexer.slot, 42);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_set_indexer_multiple_times() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = IndexerRepo::default();

        test_instance.set(&mut tx, 3).await.unwrap();
        test_instance.set(&mut tx, 4).await.unwrap();
        test_instance.set(&mut tx, 10).await.unwrap();
        test_instance.set(&mut tx, 100).await.unwrap();

        let indexer = test_instance.get(&mut tx).await.unwrap();
        assert_eq!(indexer.slot, 100);
    })
    .await
}
