// LeetCode 146: LRU Cache [Medium]
// https://leetcode.com/problems/lru-cache/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.255us
//   AI solution avg: 1.173us
//   Ratio: ~tied
//
// Design a data structure for a Least Recently Used (LRU) cache.
// Implement get(key) and put(key, value) in O(1) time.
//
// Constraints: 1 <= capacity <= 3000, 0 <= key <= 10^4, 0 <= value <= 10^5,
//   At most 2*10^5 calls to get and put
//
// Run:  cargo run --example lc_0146_lru_cache
// Test: cargo test --example lc_0146_lru_cache
// Bench: cargo test --example lc_0146_lru_cache bench -- --nocapture

use std::collections::HashMap;

/// My solution — HashMap + Vec-based order tracking (O(n) remove, simple).
///
/// Time: get O(n), put O(n) due to linear removal from order vec.
/// Space: O(capacity).
mod my_solution {
    use super::*;

    pub struct LRUCache {
        capacity: usize,
        map: HashMap<i32, i32>,
        /// Most recently used at the end, least recently used at front
        order: Vec<i32>,
    }

    impl LRUCache {
        /// Create a new LRU cache with given capacity.
        pub fn new(capacity: i32) -> Self {
            LRUCache {
                capacity: capacity as usize,
                map: HashMap::new(),
                order: Vec::new(),
            }
        }

        /// Get value by key, returning -1 if not found. Marks as recently used.
        pub fn get(&mut self, key: i32) -> i32 {
            if let Some(&val) = self.map.get(&key) {
                // Move key to the end (most recently used)
                self.order.retain(|&k| k != key);
                self.order.push(key);
                val
            } else {
                -1
            }
        }

        /// Put key-value pair. Evicts LRU entry if at capacity.
        pub fn put(&mut self, key: i32, value: i32) {
            if self.map.contains_key(&key) {
                self.order.retain(|&k| k != key);
            } else if self.map.len() == self.capacity {
                // Evict least recently used (front of order vec)
                let lru = self.order.remove(0);
                self.map.remove(&lru);
            }
            self.map.insert(key, value);
            self.order.push(key);
        }
    }
}

/// AI solution — HashMap + doubly-linked list via LinkedHashMap pattern.
///
/// Uses `std::collections::linked_list`-like approach with a VecDeque for O(1)
/// amortized operations. For true O(1), uses IndexMap-style approach.
///
/// Time: get O(1) amortized, put O(1) amortized.
/// Space: O(capacity).
mod ai_solution {
    use std::collections::HashMap;

    /// Node in the doubly-linked list.
    struct DllNode {
        key: i32,
        val: i32,
        prev: usize, // index into nodes vec
        next: usize, // index into nodes vec
    }

    pub struct LRUCache {
        capacity: usize,
        map: HashMap<i32, usize>, // key -> index in nodes
        nodes: Vec<DllNode>,
        head: usize, // sentinel head index
        tail: usize, // sentinel tail index
    }

    impl LRUCache {
        /// Create a new LRU cache.
        pub fn new(capacity: i32) -> Self {
            // Use sentinel head (idx 0) and tail (idx 1) to simplify edge cases
            let head_node = DllNode {
                key: 0,
                val: 0,
                prev: 0,
                next: 1,
            };
            let tail_node = DllNode {
                key: 0,
                val: 0,
                prev: 0,
                next: 1,
            };
            let mut cache = LRUCache {
                capacity: capacity as usize,
                map: HashMap::new(),
                nodes: vec![head_node, tail_node],
                head: 0,
                tail: 1,
            };
            cache.nodes[0].next = 1;
            cache.nodes[1].prev = 0;
            cache
        }

        /// Get value by key. Returns -1 if not found.
        pub fn get(&mut self, key: i32) -> i32 {
            if let Some(&idx) = self.map.get(&key) {
                let val = self.nodes[idx].val;
                self.remove_node(idx);
                self.add_before_tail(idx);
                val
            } else {
                -1
            }
        }

        /// Put key-value pair.
        pub fn put(&mut self, key: i32, value: i32) {
            if let Some(&idx) = self.map.get(&key) {
                self.nodes[idx].val = value;
                self.remove_node(idx);
                self.add_before_tail(idx);
            } else {
                if self.map.len() == self.capacity {
                    // Evict LRU (node right after head sentinel)
                    let lru_idx = self.nodes[self.head].next;
                    let lru_key = self.nodes[lru_idx].key;
                    self.remove_node(lru_idx);
                    self.map.remove(&lru_key);
                    // Reuse the slot
                    self.nodes[lru_idx] = DllNode {
                        key,
                        val: value,
                        prev: 0,
                        next: 0,
                    };
                    self.map.insert(key, lru_idx);
                    self.add_before_tail(lru_idx);
                } else {
                    let idx = self.nodes.len();
                    self.nodes.push(DllNode {
                        key,
                        val: value,
                        prev: 0,
                        next: 0,
                    });
                    self.map.insert(key, idx);
                    self.add_before_tail(idx);
                }
            }
        }

        fn remove_node(&mut self, idx: usize) {
            let prev = self.nodes[idx].prev;
            let next = self.nodes[idx].next;
            self.nodes[prev].next = next;
            self.nodes[next].prev = prev;
        }

        fn add_before_tail(&mut self, idx: usize) {
            let prev = self.nodes[self.tail].prev;
            self.nodes[idx].prev = prev;
            self.nodes[idx].next = self.tail;
            self.nodes[prev].next = idx;
            self.nodes[self.tail].prev = idx;
        }
    }
}

fn main() {
    let mut cache = my_solution::LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    println!("get(1) = {}", cache.get(1));
    cache.put(3, 3);
    println!("get(2) = {}", cache.get(2));

    let mut cache2 = ai_solution::LRUCache::new(2);
    cache2.put(1, 1);
    cache2.put(2, 2);
    println!("get(1) = {}", cache2.get(1));
    cache2.put(3, 3);
    println!("get(2) = {}", cache2.get(2));
}

#[cfg(test)]
mod tests {
    macro_rules! test_lru {
        ($mod_name:ident, $cache_type:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    let mut cache = <$cache_type>::new(2);
                    cache.put(1, 1);
                    cache.put(2, 2);
                    assert_eq!(cache.get(1), 1);
                    cache.put(3, 3); // evicts key 2
                    assert_eq!(cache.get(2), -1);
                    cache.put(4, 4); // evicts key 1
                    assert_eq!(cache.get(1), -1);
                    assert_eq!(cache.get(3), 3);
                    assert_eq!(cache.get(4), 4);
                }

                #[test]
                fn test_update_existing() {
                    let mut cache = <$cache_type>::new(2);
                    cache.put(1, 1);
                    cache.put(2, 2);
                    cache.put(1, 10); // update key 1
                    assert_eq!(cache.get(1), 10);
                    cache.put(3, 3); // evicts key 2 (1 was recently used)
                    assert_eq!(cache.get(2), -1);
                }

                #[test]
                fn test_capacity_one() {
                    let mut cache = <$cache_type>::new(1);
                    cache.put(1, 1);
                    cache.put(2, 2); // evicts 1
                    assert_eq!(cache.get(1), -1);
                    assert_eq!(cache.get(2), 2);
                }

                #[test]
                fn test_get_promotes() {
                    let mut cache = <$cache_type>::new(2);
                    cache.put(1, 1);
                    cache.put(2, 2);
                    cache.get(1); // promote 1
                    cache.put(3, 3); // evicts 2, not 1
                    assert_eq!(cache.get(1), 1);
                    assert_eq!(cache.get(2), -1);
                }
            }
        };
    }

    test_lru!(my, crate::my_solution::LRUCache);
    test_lru!(ai, crate::ai_solution::LRUCache);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let ops: Vec<(bool, i32, i32)> = (0..50)
            .map(|i| {
                if i % 3 == 0 {
                    (false, i % 10, 0) // get
                } else {
                    (true, i % 10, i * 7) // put
                }
            })
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut cache = crate::my_solution::LRUCache::new(5);
            for &(is_put, k, v) in &ops {
                if is_put {
                    cache.put(k, v);
                } else {
                    std::hint::black_box(cache.get(k));
                }
            }
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut cache = crate::ai_solution::LRUCache::new(5);
            for &(is_put, k, v) in &ops {
                if is_put {
                    cache.put(k, v);
                } else {
                    std::hint::black_box(cache.get(k));
                }
            }
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("Both solutions are roughly equal");
        }
    }
}
