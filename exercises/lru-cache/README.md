# LRU cache

This is an untouched starter. Ask to start this exercise when you want `prepare-rust-exercise` to create a working copy at the repository root. The starter compiles; its tests and demo intentionally stop at `todo!()`.

An application wants to keep a small number of recently used values in memory. Build a reusable, generic **least recently used (LRU) cache**. A successful lookup makes that entry recent. When inserting into a full cache, evict the entry that has gone unused for the longest time.

This project practices generic types and trait bounds, borrowing a value from a mutable collection, ownership of replaced and evicted values, and keeping two collections in sync. It is a different shape of Rust program from the earlier file-processing CLIs.

## Behavior

- `new(0)` returns an error; positive capacity starts empty.
- `get` returns `None` for a missing key without changing recency. A hit returns `&V` and makes the key most recently used.
- `insert` of a new key returns `Added` when there is room. At capacity, it returns `Evicted(key, value)` for the least recently used entry.
- `insert` of an existing key replaces its value, makes it most recently used, and returns `Replaced(old_value)`. Replacement never evicts another entry.
- `len` and `is_empty` reflect the number of stored keys. The cache never exceeds its capacity.
- The `HashMap` owns the values. `VecDeque` tracks keys from least recent at the front to most recent at the back. Keep each key in the deque exactly once.

You may change the internal representation if you preserve the public behavior. `K: Eq + Hash + Clone` is supplied because tracking a key in both collections may require a clone. `V` needs no trait bound.

## Build it in stages

1. Implement construction and size queries.
2. Implement insertion while there is space, then replacement.
3. Implement lookup and move a hit to the recent end.
4. Add eviction when inserting at capacity.

From inside this directory, run:

```sh
cargo test
cargo run
```

Once complete, the demo prints:

```text
Added
Added
alpha: Some(10)
Evicted("beta", 20)
beta: None
```

`cargo test` must pass. The tests cover zero capacity, borrowed values, lookup recency, replacement, eviction, and capacity one. The starter's test failures at TODOs are expected.

## Optional stretch goals

- Add `remove(&mut self, key: &K) -> Option<V>` and test that it updates both collections.
- Explore the cost of finding a key in `VecDeque` and compare this implementation with a linked structure for larger caches.
