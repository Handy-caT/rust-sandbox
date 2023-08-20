use std::collections::{HashMap};
use std::hash::Hash;
use crate::base::Storage;

pub struct HashTableStorage<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> HashTableStorage<K, V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::<K, V>::new(),
        }
    }
}

impl<K, V> Storage<K, V> for HashTableStorage<K, V>
where
    K: Eq + Hash,
{
    fn set(&mut self, key: K, val: V) {
        self.data.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
}

