# HashMap Implementation in Rust

This project is a from-scratch implementation of a HashMap data structure in pure Rust. It's intended as a learning exercise to understand the inner workings of hash maps, including hashing, collision handling, and dynamic resizing.

## Features

*   **Generic Keys and Values**: Can store any key `K` and value `V`.
*   **Collision Handling**: Uses separate chaining to handle hash collisions.
*   **Dynamic Resizing**: Automatically grows the map when the load factor exceeds a threshold (75%) to maintain performance.

## API

The API provides the essential HashMap operations.

| Function Signature | Description |
| --- | --- |
| `pub fn new() -> Self` | Creates a new, empty `HashMap`. |
| `pub fn insert(&mut self, key: K, value: V) -> Option<V>` | Inserts a key-value pair. If the key already exists, the value is updated, and the old value is returned. |
| `pub fn get(&self, key: &K) -> Option<&V>` | Returns a reference to the value corresponding to the key. |
| `pub fn remove(&mut self, key: &K) -> Option<V>` | Removes a key and its value from the map, returning the value if the key was present. |
| `pub fn capacity(&self) -> usize` | Returns the total number of buckets (the capacity) of the `HashMap`. |

*Note: The key type `K` must implement the `Hash` and `Eq` traits.*

## How It Works

The `HashMap` is built on a `Vec` of "buckets". Each bucket is another `Vec` that stores key-value pairs `(K, V)`.

1.  **`insert(key, value)`**:
    *   The `key` is hashed to determine its bucket index.
    *   If the key already exists in the bucket, its value is updated.
    *   Otherwise, the new `(key, value)` pair is added to the bucket.
    *   The map is resized if the number of items exceeds 75% of the bucket count.

2.  **`get(key)`**:
    *   The `key` is hashed to find its bucket.
    *   The bucket is searched linearly for the key.

3.  **`remove(key)`**:
    *   Finds the bucket for the `key`.
    *   Searches for the key in the bucket and removes it if found.

## Tests

The implementation is verified with a suite of unit tests to ensure correctness.

| Test Case | Description |
| --- | --- |
| `insert()` | Checks that a key-value pair can be inserted successfully. |
| `get()` | Verifies that a value can be retrieved for an existing key. |
| `get_empty()` | Ensures that `get()` returns `None` for a key that doesn't exist. |
| `get_capacity()` | Confirms that the initial capacity is set correctly after the first insertion. |
| `remove_pair()` | Tests that a key-value pair can be removed and that the correct value is returned. |

## Usage Example

```rust
// This is a conceptual example. To run, place it inside a main function or a test.
fn usage_example() {
    let mut map = HashMap::new();

    // Insert key-value pairs
    map.insert("one", 1);
    map.insert("two", 2);

    // Retrieve a value
    assert_eq!(map.get(&"one"), Some(&1));

    // Update a value
    map.insert("one", 11);
    assert_eq!(map.get(&"one"), Some(&11));

    // Remove a value
    assert_eq!(map.remove(&"two"), Some(2));
    assert_eq!(map.get(&"two"), None);
}
```
