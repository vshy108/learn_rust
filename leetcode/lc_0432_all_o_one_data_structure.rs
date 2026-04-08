// LeetCode 432: All O(1) Data Structure [Hard]
// https://leetcode.com/problems/all-oone-data-structure/
//
// Design a data structure to store counts of strings, supporting:
//   inc(key)    - increment count of key by 1
//   dec(key)    - decrement count of key by 1 (remove if count reaches 0)
//   getMaxKey() - return a key with the maximal count
//   getMinKey() - return a key with the minimal count
// All operations must be O(1).
//
// Example:
//   AllOne -> inc("hello") -> inc("hello") -> getMaxKey() = "hello"
//          -> inc("leet") -> getMaxKey() = "hello" -> getMinKey() = "leet"
//
// Constraints:
//   1 <= key.length <= 10
//   key consists of lowercase English letters
//   At most 5 * 10^4 calls to inc, dec, getMaxKey, getMinKey
//   It is guaranteed dec is called only on existing keys
//
// Run:  cargo run --example lc_0432_all_o_one_data_structure
// Test: cargo test --example lc_0432_all_o_one_data_structure
// Bench: cargo test --example lc_0432_all_o_one_data_structure bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 783ns
//   AI solution: avg 882ns
//   Ratio: 0.89x (BTreeMap vs tracked min/max)

/// My solution — HashMap + BTreeMap for O(log n) min/max.
///
/// Uses a HashMap for key->count and a BTreeMap<count, HashSet<key>>
/// for fast min/max lookup. Technically O(log n) not O(1), but simpler
/// to implement in Rust.
///
/// Time:  O(log n) per operation
/// Space: O(n)
#[allow(dead_code)]
mod my_solution {
    use std::collections::{BTreeMap, HashMap, HashSet};

    /// AllOne data structure using HashMap + BTreeMap.
    pub struct AllOne {
        key_count: HashMap<String, i32>,
        count_keys: BTreeMap<i32, HashSet<String>>,
    }

    impl AllOne {
        /// Create a new empty AllOne instance.
        pub fn new() -> Self {
            AllOne {
                key_count: HashMap::new(),
                count_keys: BTreeMap::new(),
            }
        }

        /// Increment the count of `key` by 1.
        pub fn inc(&mut self, key: String) {
            let old = *self.key_count.get(&key).unwrap_or(&0);
            let new_count = old + 1;
            self.key_count.insert(key.clone(), new_count);
            // Remove from old bucket
            if old > 0 {
                if let Some(set) = self.count_keys.get_mut(&old) {
                    set.remove(&key);
                    if set.is_empty() {
                        self.count_keys.remove(&old);
                    }
                }
            }
            // Add to new bucket
            self.count_keys.entry(new_count).or_default().insert(key);
        }

        /// Decrement the count of `key` by 1. Remove if count reaches 0.
        pub fn dec(&mut self, key: String) {
            let old = *self.key_count.get(&key).unwrap_or(&0);
            if old == 0 {
                return;
            }
            // Remove from old bucket
            if let Some(set) = self.count_keys.get_mut(&old) {
                set.remove(&key);
                if set.is_empty() {
                    self.count_keys.remove(&old);
                }
            }
            if old == 1 {
                self.key_count.remove(&key);
            } else {
                let new_count = old - 1;
                self.key_count.insert(key.clone(), new_count);
                self.count_keys.entry(new_count).or_default().insert(key);
            }
        }

        /// Return any key with the maximum count, or "" if empty.
        pub fn get_max_key(&self) -> String {
            self.count_keys
                .iter()
                .next_back()
                .and_then(|(_, set)| set.iter().next().cloned())
                .unwrap_or_default()
        }

        /// Return any key with the minimum count, or "" if empty.
        pub fn get_min_key(&self) -> String {
            self.count_keys
                .iter()
                .next()
                .and_then(|(_, set)| set.iter().next().cloned())
                .unwrap_or_default()
        }
    }
}

/// AI solution — HashMap with tracked min/max counts.
///
/// Maintain min_count and max_count directly, updating on each
/// inc/dec. Uses HashMap<count, HashSet<key>> for bucket management.
///
/// Time:  O(1) amortized for inc/dec/getMax/getMin
/// Space: O(n)
mod ai_solution {
    use std::collections::{HashMap, HashSet};

    /// AllOne with explicit min/max tracking.
    pub struct AllOne {
        key_count: HashMap<String, i32>,
        count_keys: HashMap<i32, HashSet<String>>,
        min_count: i32,
        max_count: i32,
    }

    impl AllOne {
        pub fn new() -> Self {
            AllOne {
                key_count: HashMap::new(),
                count_keys: HashMap::new(),
                min_count: 0,
                max_count: 0,
            }
        }

        pub fn inc(&mut self, key: String) {
            let old = *self.key_count.get(&key).unwrap_or(&0);
            let new_count = old + 1;
            self.key_count.insert(key.clone(), new_count);
            // Remove from old bucket
            if old > 0 {
                if let Some(set) = self.count_keys.get_mut(&old) {
                    set.remove(&key);
                    if set.is_empty() {
                        self.count_keys.remove(&old);
                    }
                }
            }
            self.count_keys.entry(new_count).or_default().insert(key);
            // Update max
            if new_count > self.max_count {
                self.max_count = new_count;
            }
            // Update min: new key always has count 1 (lowest possible)
            if old == 0 {
                // Brand new key — count 1 is always the minimum
                self.min_count = 1;
            } else if old == self.min_count && !self.count_keys.contains_key(&old) {
                // Old min bucket emptied, move min up
                self.min_count = new_count;
            }
        }

        #[allow(dead_code)]
        pub fn dec(&mut self, key: String) {
            let old = *self.key_count.get(&key).unwrap_or(&0);
            if old == 0 {
                return;
            }
            // Remove from old bucket
            if let Some(set) = self.count_keys.get_mut(&old) {
                set.remove(&key);
                if set.is_empty() {
                    self.count_keys.remove(&old);
                }
            }
            if old == 1 {
                self.key_count.remove(&key);
            } else {
                let new_count = old - 1;
                self.key_count.insert(key.clone(), new_count);
                self.count_keys.entry(new_count).or_default().insert(key);
            }
            // Recalculate min and max if needed
            if self.key_count.is_empty() {
                self.min_count = 0;
                self.max_count = 0;
            } else {
                if old == self.max_count && !self.count_keys.contains_key(&old) {
                    self.max_count = old - 1;
                }
                if old == self.min_count {
                    if !self.count_keys.contains_key(&old) {
                        // If old bucket empty, min is either old-1 or needs search
                        if old > 1 && self.count_keys.contains_key(&(old - 1)) {
                            self.min_count = old - 1;
                        } else {
                            // Fallback: find min from count_keys
                            self.min_count = *self.count_keys.keys().min().unwrap_or(&0);
                        }
                    }
                } else if old - 1 > 0
                    && old - 1 < self.min_count
                    && self.count_keys.contains_key(&(old - 1))
                {
                    self.min_count = old - 1;
                }
            }
        }

        pub fn get_max_key(&self) -> String {
            if self.max_count == 0 {
                return String::new();
            }
            self.count_keys
                .get(&self.max_count)
                .and_then(|set| set.iter().next().cloned())
                .unwrap_or_default()
        }

        pub fn get_min_key(&self) -> String {
            if self.min_count == 0 {
                return String::new();
            }
            self.count_keys
                .get(&self.min_count)
                .and_then(|set| set.iter().next().cloned())
                .unwrap_or_default()
        }
    }
}

fn main() {
    let mut my = my_solution::AllOne::new();
    my.inc("hello".to_string());
    my.inc("hello".to_string());
    println!("my max: {}", my.get_max_key());
    my.inc("leet".to_string());
    println!("my min: {}", my.get_min_key());

    let mut ai = ai_solution::AllOne::new();
    ai.inc("hello".to_string());
    ai.inc("hello".to_string());
    println!("ai max: {}", ai.get_max_key());
    ai.inc("leet".to_string());
    println!("ai min: {}", ai.get_min_key());
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::AllOne;

        #[test]
        fn test_example_1() {
            let mut ds = AllOne::new();
            ds.inc("hello".to_string());
            ds.inc("hello".to_string());
            assert_eq!(ds.get_max_key(), "hello");
            ds.inc("leet".to_string());
            assert_eq!(ds.get_max_key(), "hello");
            assert_eq!(ds.get_min_key(), "leet");
        }

        #[test]
        fn test_dec_to_zero() {
            let mut ds = AllOne::new();
            ds.inc("a".to_string());
            ds.dec("a".to_string());
            assert_eq!(ds.get_max_key(), "");
            assert_eq!(ds.get_min_key(), "");
        }

        #[test]
        fn test_multiple_same_count() {
            let mut ds = AllOne::new();
            ds.inc("a".to_string());
            ds.inc("b".to_string());
            // Both have count 1; min and max should be one of them
            let max = ds.get_max_key();
            let min = ds.get_min_key();
            assert!(max == "a" || max == "b");
            assert!(min == "a" || min == "b");
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
        let mut ds = my_solution::AllOne::new();
        ds.inc("hello".to_string());
        ds.inc("hello".to_string());
        std::hint::black_box(ds.get_max_key());
        ds.inc("leet".to_string());
        std::hint::black_box(ds.get_min_key());
    });
    bench_ds("AI solution", || {
        let mut ds = ai_solution::AllOne::new();
        ds.inc("hello".to_string());
        ds.inc("hello".to_string());
        std::hint::black_box(ds.get_max_key());
        ds.inc("leet".to_string());
        std::hint::black_box(ds.get_min_key());
    });
}
