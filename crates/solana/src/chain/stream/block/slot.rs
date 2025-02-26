// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Slot;
use std::cell::RefCell;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub(crate) struct SlotsToDownload(Arc<Mutex<RefCell<Inner>>>);

#[derive(Debug)]
struct Inner {
    previous_slot: Slot,
    latest_slot: Slot,
    concurrency: usize,
}

impl SlotsToDownload {
    pub(crate) fn new(concurrency: usize, previous_slot: Option<Slot>) -> Self {
        let slot = previous_slot.unwrap_or(Slot(0));
        Self(Arc::new(Mutex::new(RefCell::new(Inner {
            previous_slot: slot,
            latest_slot: slot,
            concurrency,
        }))))
    }

    pub(crate) async fn update(&self, latest_slot: impl Into<Slot>) {
        let inner_lock = self.0.lock().await;
        let mut inner = inner_lock.borrow_mut();
        inner.latest_slot = latest_slot.into();
    }

    pub(crate) async fn next_slots(&self) -> Vec<Slot> {
        let inner_lock = self.0.lock().await;
        let mut inner = inner_lock.borrow_mut();
        let latest = inner.latest_slot.clone();

        if inner.previous_slot == latest || inner.concurrency == 0 {
            return vec![];
        }

        if inner.previous_slot == 0 {
            inner.previous_slot = latest;
            return vec![latest];
        }

        let mut slots_to_download = Vec::new();

        for slot_num in inner.previous_slot.0 + 1..=latest.0 {
            slots_to_download.push(Slot(slot_num));
            inner.previous_slot = Slot(slot_num);
            if slots_to_download.len() >= inner.concurrency {
                break;
            }
        }

        slots_to_download
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Slot;
    use crate::stream::block::slot::SlotsToDownload;

    #[test_log::test(tokio::test)]
    async fn test_nothing_to_do() {
        // Ensures that without receiving the latest slot nothing is to do
        let test_instance = SlotsToDownload::new(4, Some(Slot::from(23)));
        let result = test_instance.next_slots().await;
        assert!(result.is_empty());
    }

    #[test_log::test(tokio::test)]
    async fn test_nothing_to_do_never_executed() {
        // Ensures that if it never ran before and the latest slot was not submitted that there is nothing todo
        let test_instance = SlotsToDownload::new(4, None);
        let result = test_instance.next_slots().await;
        assert!(result.is_empty());
    }

    #[test_log::test(tokio::test)]
    async fn test_latest_slot_is_starting_point() {
        // If it never ran before and received latest slot - then use it as a starting point
        let test_instance = SlotsToDownload::new(4, None);
        test_instance.update(42).await;

        let result = test_instance.next_slots().await;
        assert_eq!(result, vec![Slot(42)]);
    }

    #[test_log::test(tokio::test)]
    async fn test_ok() {
        // simulates a flow
        
        let test_instance = SlotsToDownload::new(4, Some(Slot::from(1330)));
        test_instance.update(1337).await;

        let result = test_instance.next_slots().await;
        assert_eq!(
            result,
            vec![
                Slot::from(1331),
                Slot::from(1332),
                Slot::from(1333),
                Slot::from(1334),
            ]
        );

        test_instance.update(1338).await;

        let result = test_instance.next_slots().await;
        assert_eq!(
            result,
            vec![
                Slot::from(1335),
                Slot::from(1336),
                Slot::from(1337),
                Slot::from(1338),
            ]
        );

        let result = test_instance.next_slots().await;
        assert!(result.is_empty());

        test_instance.update(1339).await;
        let result = test_instance.next_slots().await;
        assert_eq!(result, vec![Slot::from(1339)]);

        test_instance.update(1500).await;
        let result = test_instance.next_slots().await;
        assert_eq!(
            result,
            vec![
                Slot::from(1340),
                Slot::from(1341),
                Slot::from(1342),
                Slot::from(1343)
            ]
        );
    }

    #[test_log::test(tokio::test)]
    async fn test_respects_concurrency() {
        let test_instance = SlotsToDownload::new(0, Some(Slot::from(1330)));
        test_instance.update(1337).await;

        let result = test_instance.next_slots().await;
        assert!(result.is_empty());

        let test_instance = SlotsToDownload::new(1, Some(Slot::from(1330)));
        test_instance.update(1337).await;

        let result = test_instance.next_slots().await;
        assert_eq!(result, vec![Slot::from(1331),]);

        let test_instance = SlotsToDownload::new(3, Some(Slot::from(1330)));
        test_instance.update(1337).await;
        let result = test_instance.next_slots().await;
        assert_eq!(
            result,
            vec![Slot::from(1331), Slot::from(1332), Slot::from(1333)]
        );
    }
}
