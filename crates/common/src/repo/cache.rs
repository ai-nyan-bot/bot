// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use tokio::sync::{RwLock, RwLockWriteGuard};

#[derive(Debug)]
struct Entry<I, K, V>
where
    V: Send,
{
    id: I,
    key: K,
    value: V,
}

#[derive(Debug)]
struct Store<I, K, V>
where
    V: Send,
{
    ids: HashMap<I, Entry<I, K, V>>,
    keys: HashMap<K, I>,
}

#[derive(Debug, Clone)]
pub struct Cache<I, K, V>
where
    I: Eq + Hash + Send,
    K: Eq + Hash + Send,
    V: Send,
{
    store: Arc<RwLock<Store<I, K, V>>>,
}

impl<I, K, V> Default for Cache<I, K, V>
where
    I: Eq + Hash + Send,
    K: Eq + Hash + Send,
    V: Send,
{
    fn default() -> Self {
        Self {
            store: Arc::new(RwLock::new(Store {
                ids: HashMap::new(),
                keys: HashMap::new(),
            })),
        }
    }
}

#[allow(dead_code)]
impl<I, K, V> Cache<I, K, V>
where
    I: Eq + Hash + Clone + Send,
    K: Eq + Hash + Clone + Send,
    V: Clone + Send,
{
    pub async fn get_by_id(&self, id: impl Into<I>) -> Option<V> {
        let store = self.store.read().await;
        store.ids.get(&id.into()).map(|item| item.value.clone())
    }

    pub async fn get_by_key(&self, key: impl Into<K>) -> Option<V> {
        let store = self.store.read().await;
        store.keys.get(&key.into()).and_then(|id| store.ids.get(id).map(|item| item.value.clone()))
    }

    pub async fn put(&self, id: impl Into<I>, key: impl Into<K>, value: impl Into<V>) {
        let entry = Entry {
            id: id.into(),
            key: key.into(),
            value: value.into(),
        };

        let mut store = self.store.write().await;
        Self::insert_entry(&mut store, entry);
    }

    pub async fn put_all(&self, entries: impl IntoIterator<Item = (impl Into<I>, impl Into<K>, impl Into<V>)>) {
        let mut store = self.store.write().await;
        for (id, key, value) in entries {
            let entry = Entry {
                id: id.into(),
                key: key.into(),
                value: value.into(),
            };
            Self::insert_entry(&mut store, entry);
        }
    }

    fn insert_entry(store: &mut RwLockWriteGuard<Store<I, K, V>>, entry: Entry<I, K, V>) {
        store.keys.insert(entry.key.clone(), entry.id.clone());
        store.ids.insert(entry.id.clone(), entry);
    }

    async fn count(&self) -> usize {
        let store = self.store.read().await;
        store.ids.len()
    }
}

#[cfg(test)]
mod tests {
	use crate::model::{Decimals, TokenId, TokenMint};
	use crate::repo::cache::Cache;

	#[tokio::test]
    async fn test_put_and_get() {
        let test_instance: Cache<TokenId, TokenMint, Decimals> = Cache::default();
        test_instance.put(23, "nyan", 42).await;
        let result = test_instance.get_by_key("nyan").await;
        assert_eq!(result.unwrap(), 42);

        assert_eq!(test_instance.count().await, 1);
    }

    #[tokio::test]
    async fn test_put_all_and_get() {
        let test_instance: Cache<TokenId, TokenMint, Decimals> = Cache::default();
        test_instance.put_all(vec![(1, "2", 3), (4, "5", 6)]).await;

        let value = test_instance.get_by_id(1).await;
        assert_eq!(value.unwrap(), 3);

        let value = test_instance.get_by_key("2").await;
        assert_eq!(value.unwrap(), 3);

        let value = test_instance.get_by_id(4).await;
        assert_eq!(value.unwrap(), 6);

        let value = test_instance.get_by_key("5").await;
        assert_eq!(value.unwrap(), 6);

        assert_eq!(test_instance.count().await, 2);
    }

    #[tokio::test]
    async fn test_get_non_existent_key() {
        let test_instance: Cache<TokenId, TokenMint, Decimals> = Cache::default();
        test_instance.put(23, "nyan", 42).await;

        let result = test_instance.get_by_key("nayn").await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_get_non_existent_id() {
        let test_instance: Cache<TokenId, TokenMint, Decimals> = Cache::default();
        test_instance.put(23, "nyan", 42).await;

        let result = test_instance.get_by_id(1336).await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_overwrite_existing_value() {
        let test_instance: Cache<TokenId, TokenMint, Decimals> = Cache::default();

        test_instance.put(23, "nyan", 23).await;
        test_instance.put(23, "nyan", 32).await;

        let result = test_instance.get_by_key("nyan").await;
        assert_eq!(result.unwrap(), 32);

        assert_eq!(test_instance.count().await, 1);
    }

    #[tokio::test]
    async fn multiple_entries() {
        let test_instance: Cache<TokenId, TokenMint, Decimals> = Cache::default();

        test_instance.put(1, "key_one", 42).await;
        test_instance.put(2, "key_two", 100).await;

        let value = test_instance.get_by_key("key_one").await;
        assert_eq!(value.unwrap(), 42);

        let value = test_instance.get_by_id(1).await;
        assert_eq!(value.unwrap(), 42);

        let value = test_instance.get_by_key("key_two").await;
        assert_eq!(value.unwrap(), 100);
    }
}
