#![allow(unused_variables)]

struct LruCache {}

impl LruCache {
    fn new(capacity: i32) -> Self {
        Self {}
    }

    fn put(&self, key: i32, value: i32) {}

    fn get(&self, key: i32) -> i32 {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0146() {
        let lru_cache = LruCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        lru_cache.get(1); // return 1
        lru_cache.put(3, 3);
        lru_cache.get(2); // return -1
    }
}
