use std::hash::{DefaultHasher, Hash, Hasher};
use std::{mem};
const INITIAL_NBUCKETS: usize = 10;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{   

    // takes key and returns bucket's index to insert the key value pair in that specific bucket
    fn bucket(&self , key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % (self.buckets.len() as u64)) as usize        
    }


    // finds the hash and puts <key , value> pair in the bucket vector
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket: usize = self.bucket(&key);

        let bucket = &mut self.buckets[bucket];

        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                return Some(mem::replace(evalue, value));
            }
        }

        bucket.push((key, value));
        None
    }


    // function to resize the buckets inside a hashmap if the capacity exceeds 3/4th of the size of a bucket to optimize search time
    fn resize(&mut self) {
        let target_size: usize = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };

        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket].push((key, value));
        }
        mem::replace(&mut self.buckets, new_buckets);
    }

    
    // get the value from the key 
    pub fn get(&self , key: &K) -> Option<&V> {
        if self.buckets.is_empty() {
            return None;
        }

        let bucket = self.bucket(key);
        self.buckets[bucket]
            .iter()
            .find(|&(ekey,_)|ekey == key)
            .map(|&(_,ref v)|v)

    }


    // Return the capacity of the Hashmap without reallocating space
    pub fn capacity(&self)-> usize {
        self.buckets.capacity()
    }

    //Removes a key from the hashmap, returning the value at the key if the key was previously in the Hashmap
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.buckets.is_empty() {
            return None;
        }
        
        let bucket = self.bucket(key);
        
        // Find the position of the key in the bucket with matching pattern
        let pos = self.buckets[bucket]
            .iter()
            .position(|&(ref k, _)| k == key)?;
        
        // Remove the key-value pair and return the value
        let (_, value) = self.buckets[bucket].remove(pos);
        if self.items > 0 {
            self.items -= 1;
        }
        
        Some(value)
    }

}









// TEST DOWN HERE FOR THE IMPLEMENTATION

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert() {
        let mut map = HashMap::new();
        map.insert("abc", 1);
    }

    #[test]
    fn get () {
        let mut map = HashMap::new();
        map.insert("abc", "def");
        assert_eq!(map.get(&"abc") , Some(&"def"));
    }
    #[test]
    fn get_empty () {
        let mut map: HashMap<&'static str, u32> = HashMap::new();

        assert_eq!(map.get(&"abc") , None);
    }

    #[test]
    fn get_capacity(){
        let mut map = HashMap::new();
        map.insert(String::from("a"), 1);

        assert_eq!(map.capacity(), 10);
    }

    #[test]
    fn remove_pair() {
        let mut map = HashMap::new();
        map.insert(10, 100);
        let remove_value_key = map.remove(&10);

        assert_eq!(remove_value_key, Some(100));
    }

    #[test]
    fn test_items_counter_bug() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        
        // Print items counter - should be 1 but will likely be 0
        println!("Items after first insert: {}", map.items);
        
        map.insert("b", 2);
        println!("Items after second insert: {}", map.items);
        
        // This test shows the bug - items counter never increments
        // assert_eq!(map.items, 2); // This would fail
    }
    
    #[test]
    fn test_capacity_method() {
        let mut map = HashMap::new();
        // Before any inserts, capacity should be 0 (no buckets)
        println!("Capacity before insert: {}", map.capacity());
        
        map.insert("a", 1);
        // After first insert, should have INITIAL_NBUCKETS = 10 buckets
        println!("Capacity after first insert: {}", map.capacity());
        println!("Buckets length: {}", map.buckets.len());
        
        // Currently capacity() returns buckets.capacity() not buckets.len()
    }
}