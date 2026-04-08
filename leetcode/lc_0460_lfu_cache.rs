// LeetCode 460: LFU Cache [Hard]
// https://leetcode.com/problems/lfu-cache/
//
// Design and implement a Least Frequently Used (LFU) cache.
//   LFUCache(capacity) - initialize with given capacity
//   get(key)           - return value if exists, else -1
//   put(key, value)    - insert/update; evict LFU key when full
//                        (tie-break: LRU among same frequency)
// Both get and put must run in O(1) average time.
//
// Example:
//   LFUCache(2) -> put(1,1) -> put(2,2) -> get(1)=1
//   -> put(3,3) evicts key 2 -> get(2)=-1 -> get(3)=3
//   -> put(4,4) evicts key 1 -> get(1)=-1 -> get(3)=3 -> get(4)=4
//
// Constraints:
//   1 <= capacity <= 10^4
//   0 <= key <= 10^5
//   0 <= value <= 10^9
//   At most 2 * 10^5 calls
//
// Run:  cargo run --example lc_0460_lfu_cache
// Test: cargo test --example lc_0460_lfu_cache
// Bench: cargo test --example lc_0460_lfu_cache bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 337ns
//   AI solution: avg 175ns
//   Ratio: 1.93x (AI BTreeSet simpler overhead)

/// My solution — HashMap + LinkedList per frequency bucket
/// (from doocs/leetcode Rust).
///
/// Each frequency has its own doubly linked list (via Rc<RefCell>).
/// Nodes are also stored in a HashMap for O(1) access.
///
/// Time:  O(1) amortized per get/put
/// Space: O(capacity)
#[allow(dead_code)]
mod my_solution {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    struct Node {
        key: i32,
        value: i32,
        freq: i32,
        prev: Option<Rc<RefCell<Node>>>,
        next: Option<Rc<RefCell<Node>>>,
    }

    impl Node {
        fn new(key: i32, value: i32) -> Self {
            Self {
                key,
                value,
                freq: 1,
                prev: None,
                next: None,
            }
        }
    }

    struct LinkedList {
        head: Option<Rc<RefCell<Node>>>,
        tail: Option<Rc<RefCell<Node>>>,
    }

    impl LinkedList {
        fn new() -> Self {
            Self {
                head: None,
                tail: None,
            }
        }

        fn push_front(&mut self, node: &Rc<RefCell<Node>>) {
            match self.head.take() {
                Some(head) => {
                    head.borrow_mut().prev = Some(Rc::clone(node));
                    node.borrow_mut().prev = None;
                    node.borrow_mut().next = Some(head);
                    self.head = Some(Rc::clone(node));
                }
                None => {
                    node.borrow_mut().prev = None;
                    node.borrow_mut().next = None;
                    self.head = Some(Rc::clone(node));
                    self.tail = Some(Rc::clone(node));
                }
            };
        }

        fn remove(&mut self, node: &Rc<RefCell<Node>>) {
            match (
                node.borrow().prev.as_ref().map(Rc::clone),
                node.borrow().next.as_ref().map(Rc::clone),
            ) {
                (None, None) => {
                    self.head = None;
                    self.tail = None;
                }
                (None, Some(next)) => {
                    self.head = Some(Rc::clone(&next));
                    next.borrow_mut().prev = None;
                }
                (Some(prev), None) => {
                    self.tail = Some(Rc::clone(&prev));
                    prev.borrow_mut().next = None;
                }
                (Some(prev), Some(next)) => {
                    next.borrow_mut().prev = Some(Rc::clone(&prev));
                    prev.borrow_mut().next = Some(Rc::clone(&next));
                }
            };
        }

        fn pop_back(&mut self) -> Option<Rc<RefCell<Node>>> {
            self.tail.clone().inspect(|tail| {
                self.remove(tail);
            })
        }

        fn is_empty(&self) -> bool {
            self.head.is_none()
        }
    }

    /// LFU Cache using frequency-bucketed doubly linked lists.
    pub struct LFUCache {
        cache: HashMap<i32, Rc<RefCell<Node>>>,
        freq_map: HashMap<i32, LinkedList>,
        min_freq: i32,
        capacity: usize,
    }

    impl LFUCache {
        /// Create a new LFU cache with given capacity.
        pub fn new(capacity: i32) -> Self {
            Self {
                cache: HashMap::new(),
                freq_map: HashMap::new(),
                min_freq: 0,
                capacity: capacity as usize,
            }
        }

        /// Get value for key, or -1 if not found.
        pub fn get(&mut self, key: i32) -> i32 {
            if self.capacity == 0 {
                return -1;
            }
            match self.cache.get(&key) {
                Some(node) => {
                    let node = Rc::clone(node);
                    self.incr_freq(&node);
                    node.borrow().value
                }
                None => -1,
            }
        }

        /// Insert or update key-value pair.
        pub fn put(&mut self, key: i32, value: i32) {
            if self.capacity == 0 {
                return;
            }
            match self.cache.get(&key) {
                Some(node) => {
                    let node = Rc::clone(node);
                    node.borrow_mut().value = value;
                    self.incr_freq(&node);
                }
                None => {
                    if self.cache.len() == self.capacity {
                        // Evict LFU (LRU among ties = tail of min_freq list)
                        let list = self.freq_map.get_mut(&self.min_freq).unwrap();
                        let evicted = list.pop_back().unwrap();
                        self.cache.remove(&evicted.borrow().key);
                    }
                    let node = Rc::new(RefCell::new(Node::new(key, value)));
                    self.add_node(&node);
                    self.cache.insert(key, node);
                    self.min_freq = 1;
                }
            };
        }

        fn incr_freq(&mut self, node: &Rc<RefCell<Node>>) {
            let freq = node.borrow().freq;
            let list = self.freq_map.get_mut(&freq).unwrap();
            list.remove(node);
            if list.is_empty() {
                self.freq_map.remove(&freq);
                if freq == self.min_freq {
                    self.min_freq += 1;
                }
            }
            node.borrow_mut().freq += 1;
            self.add_node(node);
        }

        fn add_node(&mut self, node: &Rc<RefCell<Node>>) {
            let freq = node.borrow().freq;
            self.freq_map
                .entry(freq)
                .or_insert_with(LinkedList::new)
                .push_front(node);
        }
    }
}

/// AI solution — HashMap + BTreeMap for simpler LFU.
///
/// Uses a counter to track insertion/access order as a tiebreaker.
/// BTreeMap<(freq, order), key> gives us O(log n) eviction.
/// Simpler to implement but not strictly O(1).
///
/// Time:  O(log n) per operation
/// Space: O(capacity)
mod ai_solution {
    use std::collections::{BTreeSet, HashMap};

    /// LFU Cache using BTreeSet for ordered eviction.
    pub struct LFUCache {
        capacity: usize,
        counter: u64,
        // key -> (value, freq, order)
        cache: HashMap<i32, (i32, i32, u64)>,
        // (freq, order, key) for fast minimum lookup
        order_set: BTreeSet<(i32, u64, i32)>,
    }

    impl LFUCache {
        pub fn new(capacity: i32) -> Self {
            Self {
                capacity: capacity as usize,
                counter: 0,
                cache: HashMap::new(),
                order_set: BTreeSet::new(),
            }
        }

        pub fn get(&mut self, key: i32) -> i32 {
            if self.capacity == 0 {
                return -1;
            }
            if let Some(&(value, freq, order)) = self.cache.get(&key) {
                // Remove old entry and insert with incremented freq
                self.order_set.remove(&(freq, order, key));
                self.counter += 1;
                let new_entry = (value, freq + 1, self.counter);
                self.cache.insert(key, new_entry);
                self.order_set.insert((freq + 1, self.counter, key));
                value
            } else {
                -1
            }
        }

        pub fn put(&mut self, key: i32, value: i32) {
            if self.capacity == 0 {
                return;
            }
            if let Some(&(_, freq, order)) = self.cache.get(&key) {
                self.order_set.remove(&(freq, order, key));
                self.counter += 1;
                let new_entry = (value, freq + 1, self.counter);
                self.cache.insert(key, new_entry);
                self.order_set.insert((freq + 1, self.counter, key));
            } else {
                if self.cache.len() == self.capacity {
                    // Evict entry with smallest (freq, order)
                    if let Some(&(freq, order, evict_key)) = self.order_set.iter().next() {
                        self.order_set.remove(&(freq, order, evict_key));
                        self.cache.remove(&evict_key);
                    }
                }
                self.counter += 1;
                self.cache.insert(key, (value, 1, self.counter));
                self.order_set.insert((1, self.counter, key));
            }
        }
    }
}

fn main() {
    let mut my = my_solution::LFUCache::new(2);
    my.put(1, 1);
    my.put(2, 2);
    println!("my get(1): {}", my.get(1));
    my.put(3, 3);
    println!("my get(2): {}", my.get(2));

    let mut ai = ai_solution::LFUCache::new(2);
    ai.put(1, 1);
    ai.put(2, 2);
    println!("ai get(1): {}", ai.get(1));
    ai.put(3, 3);
    println!("ai get(2): {}", ai.get(2));
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::LFUCache;

        #[test]
        fn test_example_1() {
            let mut lfu = LFUCache::new(2);
            lfu.put(1, 1);
            lfu.put(2, 2);
            assert_eq!(lfu.get(1), 1);
            lfu.put(3, 3); // evicts key 2
            assert_eq!(lfu.get(2), -1);
            assert_eq!(lfu.get(3), 3);
            lfu.put(4, 4); // evicts key 1
            assert_eq!(lfu.get(1), -1);
            assert_eq!(lfu.get(3), 3);
            assert_eq!(lfu.get(4), 4);
        }

        #[test]
        fn test_capacity_one() {
            let mut lfu = LFUCache::new(1);
            lfu.put(1, 10);
            assert_eq!(lfu.get(1), 10);
            lfu.put(2, 20); // evicts key 1
            assert_eq!(lfu.get(1), -1);
            assert_eq!(lfu.get(2), 20);
        }

        #[test]
        fn test_update_existing() {
            let mut lfu = LFUCache::new(2);
            lfu.put(1, 1);
            lfu.put(1, 10); // update value
            assert_eq!(lfu.get(1), 10);
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_ds<F>(name: &str, f: F)
where
    F: Fn(),
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f());
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_ds("My solution", || {
        let mut lfu = my_solution::LFUCache::new(2);
        lfu.put(1, 1);
        lfu.put(2, 2);
        std::hint::black_box(lfu.get(1));
        lfu.put(3, 3);
        std::hint::black_box(lfu.get(2));
    });
    bench_ds("AI solution", || {
        let mut lfu = ai_solution::LFUCache::new(2);
        lfu.put(1, 1);
        lfu.put(2, 2);
        std::hint::black_box(lfu.get(1));
        lfu.put(3, 3);
        std::hint::black_box(lfu.get(2));
    });
}
