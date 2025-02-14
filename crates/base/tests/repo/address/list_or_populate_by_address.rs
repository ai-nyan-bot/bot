// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use base::repo::AddressRepo;
use testing::run_test_on_empty_db;
use testing::address::count_all;

#[test_log::test(sqlx::test)]
async fn test_already_in_db() {
    run_test_on_empty_db(|mut tx| async move {
        let address_repo = AddressRepo::new();
        let _ = address_repo
            .list_or_populate_by_keys(&mut tx, vec!["BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC"])
            .await
            .unwrap();

        let test_instance = AddressRepo::new();
        let result = test_instance
            .list_or_populate_by_keys(&mut tx, vec!["BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC"])
            .await
            .unwrap();

        assert_eq!(result.len(), 1);

        let result = result.first().unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.address, "BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_already_in_cache() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = AddressRepo::new();
        let _ = test_instance
            .list_or_populate_by_keys(&mut tx, vec!["BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC"])
            .await
            .unwrap();

        let result = test_instance
            .list_or_populate_by_keys(&mut tx, vec!["BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC"])
            .await
            .unwrap();

        assert_eq!(result.len(), 1);

        let result = result.first().unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.address, "BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_insert_one() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = AddressRepo::new();

        let result = test_instance
            .list_or_populate_by_keys(&mut tx, vec!["9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3"])
            .await
            .unwrap();

        assert_eq!(result.len(), 1);

        let result = result.first().unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.address, "9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_one_in_cache_one_in_db_one_insert() {
    run_test_on_empty_db(|mut tx| async move {
        let address_repo = AddressRepo::new();
        address_repo
            .list_or_populate_by_keys(
                &mut tx,
                ["BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC", "53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC"],
            )
            .await
            .unwrap();

        let test_instance = AddressRepo::new();
        test_instance
            .list_or_populate_by_keys(&mut tx, ["BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC"])
            .await
            .unwrap();

        let result = test_instance
            .list_or_populate_by_keys(
                &mut tx,
                [
                    "53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC",
                    "MJKqp326RZCHnAAbew9MDdui3iCKWco7fsK9sVuZTX2",
                    "BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC",
                ],
            )
            .await
            .unwrap();

        assert_eq!(result.len(), 3);

        let first = result.first().unwrap();
        assert_eq!(first.id, 1);
        assert_eq!(first.address, "BFAWVmF5aoALggQ9Y2RpTijpYKRESxcdNe6JDNZEpoxC");

        let second = result.get(1).unwrap();
        assert_eq!(second.id, 2);
        assert_eq!(second.address, "53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC");

        let third = result.last().unwrap();
        assert_eq!(third.id, 3);
        assert_eq!(third.address, "MJKqp326RZCHnAAbew9MDdui3iCKWco7fsK9sVuZTX2");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 3);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_insert_many() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = AddressRepo::new();

        let result = test_instance
            .list_or_populate_by_keys(
                &mut tx,
                [
                    "9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3",
                    "EJRJswH9LyjhAfBWwPBvat1LQtrJYK4sVUzsea889cQt",
                    "53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC",
                ],
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 3);

        let first = result.first().unwrap();
        assert_eq!(first.id, 1);
        assert_eq!(first.address, "9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3");

        let second = result.get(1).unwrap();
        assert_eq!(second.id, 2);
        assert_eq!(second.address, "EJRJswH9LyjhAfBWwPBvat1LQtrJYK4sVUzsea889cQt");

        let third = result.last().unwrap();
        assert_eq!(third.id, 3);
        assert_eq!(third.address, "53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 3);
    })
    .await
}
