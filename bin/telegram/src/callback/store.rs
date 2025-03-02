// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::callback::Callback;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{sleep, Instant};
use uuid::Uuid;

struct StoredCallback {
    callback: Callback,
    at: Instant,
}

#[derive(Clone)]
pub struct CallbackStore {
    inner: Arc<Mutex<HashMap<String, StoredCallback>>>,
}

impl CallbackStore {
    pub fn new(ttl: Duration) -> Self {
        let store = Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        };

        let inner_clone = store.inner.clone();
        tokio::spawn(async move {
            loop {
                sleep(ttl).await;
                let mut map = inner_clone.lock().await;
                let now = Instant::now();
                map.retain(|_, stored| now.duration_since(stored.at) < ttl);
            }
        });

        store
    }

    pub async fn store(&self, callback: Callback) -> String {
        let result = Uuid::new_v4().to_string();
        let stored = StoredCallback {
            callback,
            at: Instant::now(),
        };
        self.inner.lock().await.insert(result.clone(), stored);
        result
    }

    pub async fn pop(&self, id: &str) -> Option<Callback> {
        self.inner
            .lock()
            .await
            .remove(id)
            .map(|stored| stored.callback)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::callback::{Callback, CallbackActionButton, TelegramActionButtonConfig};
    use base::model::Value;
    use tokio::time::Duration;

    fn buy_callback() -> Callback {
        Callback::ActionButton(CallbackActionButton {
            config: TelegramActionButtonConfig::Buy {
                value: Value::sol(12.34),
            },
        })
    }

    fn sell_callback() -> Callback {
        Callback::ActionButton(CallbackActionButton {
            config: TelegramActionButtonConfig::Sell {
                value: Value::percent(23.45),
            },
        })
    }

    #[tokio::test]
    async fn test_store_and_retrieve() {
        let test_instance = CallbackStore::new(Duration::from_secs(60));
        let callback = buy_callback();

        let id = test_instance.store(callback.clone()).await;
        let retrieved = test_instance.pop(&id).await.unwrap();

        assert_eq!(retrieved, callback);
    }

    #[tokio::test]
    async fn test_callback_removed_after_retrieve() {
        let test_instance = CallbackStore::new(Duration::from_secs(60));
        let callback = buy_callback();

        let id = test_instance.store(callback.clone()).await;
        let _ = test_instance.pop(&id).await;
        let second_retrieval = test_instance.pop(&id).await;

        assert!(
            second_retrieval.is_none(),
            "Callback should be removed after pop"
        );
    }

    #[tokio::test]
    async fn test_callback_expiration() {
        let test_instance = CallbackStore::new(Duration::from_millis(1));
        let callback = buy_callback();

        let id = test_instance.store(callback.clone()).await;
        sleep(Duration::from_millis(4)).await;

        let retrieved = test_instance.pop(&id).await;
        assert!(retrieved.is_none(), "Callback should expire and be removed");
    }

    #[tokio::test]
    async fn test_multiple_callbacks() {
        let test_instance = CallbackStore::new(Duration::from_secs(60));
        let callback_one = buy_callback();
        let callback_two = sell_callback();

        let id_one = test_instance.store(callback_one.clone()).await;
        let id_two = test_instance.store(callback_two.clone()).await;

        let retrieved_one = test_instance.pop(&id_one).await.unwrap();
        let retrieved_two = test_instance.pop(&id_two).await.unwrap();

        assert_eq!(retrieved_one, callback_one);
        assert_eq!(retrieved_two, callback_two);
    }

    #[tokio::test]
    async fn test_partial_expiration() {
        let store = CallbackStore::new(Duration::from_millis(1));

        let id_one = store.store(buy_callback()).await;

        sleep(Duration::from_millis(5)).await;

        let callback_two = sell_callback();
        let id_two = store.store(callback_two.clone()).await;

        assert!(
            store.pop(&id_one).await.is_none(),
            "First callback should have expired"
        );
        assert!(
            store.pop(&id_two).await.is_some(),
            "Second callback should still exist"
        );
    }
}
