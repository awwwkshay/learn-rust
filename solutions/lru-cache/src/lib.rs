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
        if capacity == 0 {
            return Err("Capacity must be greater than zero".to_string());
        }
        Ok(Self {
            capacity,
            entries: HashMap::new(),
            order: VecDeque::new(),
        })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns a reference to the value and marks the key as most recently used.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let val = self.entries.get(key);
        if val.is_some() {
            // Move the key to the back of the order queue to mark it as most recently used
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
        }
        val
    }

    /// Adds or replaces a value, marking its key as most recently used.
    /// Only a new key can evict the least recently used entry.
    pub fn insert(&mut self, key: K, value: V) -> InsertOutcome<K, V> {
        if self.entries.contains_key(&key) {
            let old_value = self.entries.insert(key.clone(), value).unwrap();
            // Move the key to the back of the order queue to mark it as most recently used
            self.order.retain(|k| k != &key);
            self.order.push_back(key);
            return InsertOutcome::Replaced(old_value);
        } else {
            if self.entries.len() == self.capacity {
                // Evict the least recently used entry
                let lru_key = self
                    .order
                    .pop_front()
                    .expect("cache order must contain every entry");
                let lru_value = self.entries.remove(&lru_key).unwrap();
                self.entries.insert(key.clone(), value);
                self.order.push_back(key);
                return InsertOutcome::Evicted(lru_key, lru_value);
            } else {
                self.entries.insert(key.clone(), value);
                self.order.push_back(key);
                return InsertOutcome::Added;
            }
        }
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
