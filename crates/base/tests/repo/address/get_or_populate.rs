// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use base::repo::AddressRepo;
use testing::run_test_on_empty_db;
use testing::address::count_all;

#[sqlx::test]
async fn test_already_in_db() {
    run_test_on_empty_db(|mut tx| async move {
        let address_repo = AddressRepo::new();
        let _ = address_repo
            .get_or_populate_by_key(&mut tx, "BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC")
            .await
            .unwrap();

        let test_instance = AddressRepo::new();
        let result = test_instance
            .get_or_populate_by_key(&mut tx, "BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC")
            .await
            .unwrap();

        assert_eq!(result.id, 1);
        assert_eq!(result.address, "BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await
}

#[sqlx::test]
async fn test_already_in_cache() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = AddressRepo::new();

        let _ = test_instance
            .get_or_populate_by_key(&mut tx, "BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC")
            .await
            .unwrap();

        let result = test_instance
            .get_or_populate_by_key(&mut tx, "BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC")
            .await
            .unwrap();

        assert_eq!(result.id, 1);
        assert_eq!(result.address, "BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await
}

#[sqlx::test]
async fn test_insert_one() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = AddressRepo::new();

        let result = test_instance
            .get_or_populate_by_key(&mut tx, "Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm")
            .await
            .unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.address, "Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await
}

#[sqlx::test]
async fn test_insert_many() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = AddressRepo::new();

        let result = test_instance
            .get_or_populate_by_key(&mut tx, "Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm")
            .await
            .unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.address, "Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 1);

        let result = test_instance
            .get_or_populate_by_key(&mut tx, "BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC")
            .await
            .unwrap();
        assert_eq!(result.id, 2);
        assert_eq!(result.address, "BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 2);

        let result = test_instance
            .get_or_populate_by_key(&mut tx, "9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3")
            .await
            .unwrap();
        assert_eq!(result.id, 3);
        assert_eq!(result.address, "9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 3);
    })
    .await
}
