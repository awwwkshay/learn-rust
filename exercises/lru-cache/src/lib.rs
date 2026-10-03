use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub enum InsertOutcome<K, V> {
    Added,
    Replaced(V),
    Evicted(K, V),
}

/// A fixed-capacity cache. The front of `order` is the least recently used key.
pub struct LruCache<K, V> {
    capacity: usize,
    entries: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    /// Rejects zero capacity.
    pub fn new(capacity: usize) -> Result<Self, String> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Returns a reference to the value and marks the key as most recently used.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        todo!()
    }

    /// Adds or replaces a value, marking its key as most recently used.
    /// Only a new key can evict the least recently used entry.
    pub fn insert(&mut self, key: K, value: V) -> InsertOutcome<K, V> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_capacity() {
        assert!(LruCache::<String, i32>::new(0).is_err());
    }

    #[test]
    fn starts_empty_and_returns_borrowed_values() {
        let mut cache = LruCache::new(2).unwrap();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.get(&"missing".to_string()), None);
        assert_eq!(
            cache.insert("one".to_string(), String::from("first")),
            InsertOutcome::Added
        );
        assert_eq!(
            cache.get(&"one".to_string()).map(String::as_str),
            Some("first")
        );
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn access_changes_which_entry_is_evicted() {
        let mut cache = LruCache::new(2).unwrap();
        cache.insert(1, "one");
        cache.insert(2, "two");
        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.insert(3, "three"), InsertOutcome::Evicted(2, "two"));
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn replacement_returns_old_value_and_does_not_evict() {
        let mut cache = LruCache::new(2).unwrap();
        cache.insert("a", 1);
        cache.insert("b", 2);
        assert_eq!(cache.insert("a", 10), InsertOutcome::Replaced(1));
        assert_eq!(cache.insert("c", 3), InsertOutcome::Evicted("b", 2));
        assert_eq!(cache.get(&"a"), Some(&10));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn capacity_one_and_missing_get() {
        let mut cache = LruCache::new(1).unwrap();
        assert_eq!(cache.insert(1, 10), InsertOutcome::Added);
        assert_eq!(cache.get(&9), None);
        assert_eq!(cache.insert(2, 20), InsertOutcome::Evicted(1, 10));
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&20));
    }
}
