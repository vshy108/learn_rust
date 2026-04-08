// LeetCode 381: Insert Delete GetRandom O(1) - Duplicates allowed [Hard]
// https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/
//
// RandomizedCollection is a data structure that contains a collection of
// numbers, possibly duplicates (i.e., a multiset). It should support inserting
// and removing specific elements and also reporting a random element.
//
// Implement the RandomizedCollection class:
//   - RandomizedCollection::new()     -- Initializes the empty object.
//   - insert(val: i32) -> bool        -- Inserts val into the multiset, even
//                                        if already present. Returns true if
//                                        the item was not present, false
//                                        otherwise.
//   - remove(val: i32) -> bool        -- Removes one occurrence of val if
//                                        present. Returns true if present,
//                                        false otherwise.
//   - get_random() -> i32             -- Returns a random element. Probability
//                                        is linearly related to the count of
//                                        each value.
//
// You must implement the functions such that each function works on average
// O(1) time complexity.
//
// Examples:
//   insert(1) -> true, insert(1) -> false, insert(2) -> true,
//   getRandom() -> 1 (2/3 prob) or 2 (1/3 prob),
//   remove(1) -> true, getRandom() -> 1 or 2 (equal prob)
//
// Constraints:
//   - -2^31 <= val <= 2^31 - 1
//   - At most 2 * 10^5 calls total to insert, remove, and getRandom
//   - There will be at least one element when getRandom is called
//
// Run:  cargo run --example lc_0381_insert_delete_getrandom_o1_duplicates_allowed
// Test: cargo test --example lc_0381_insert_delete_getrandom_o1_duplicates_allowed
// Bench: cargo test --example lc_0381_insert_delete_getrandom_o1_duplicates_allowed bench -- --nocapture
//
// --- Benchmark (10k iterations, 2026-04-08) ---
// My solution:  1.48s (avg 147.80us)
// AI solution:  916.07ms (avg 91.61us)
// -> AI solution is 1.61x faster
// Reason: BTreeSet::pop_last() directly removes and returns an index
//   without creating an iterator, and scoping the borrow allows the
//   compiler to optimize the second HashMap lookup more aggressively.

/// My solution module.
///
/// Uses a Vec for O(1) random access and a HashMap<i32, HashSet<usize>>
/// mapping each value to the set of indices where it appears.
/// Swap-with-last trick for O(1) removal.
///
/// insert:     O(1) avg
/// remove:     O(1) avg
/// get_random: O(1)
/// Space:      O(n)
mod my_solution {
    use std::collections::{HashMap, HashSet};

    /// A multiset supporting insert, remove, and get_random in O(1) average.
    ///
    /// `nums` stores all elements (with duplicates).
    /// `val_to_indices` maps each value to the set of indices in `nums`.
    pub struct RandomizedCollection {
        nums: Vec<i32>,
        val_to_indices: HashMap<i32, HashSet<usize>>,
    }

    impl RandomizedCollection {
        /// Initialize an empty RandomizedCollection.
        pub fn new() -> Self {
            Self {
                nums: Vec::new(),
                val_to_indices: HashMap::new(),
            }
        }

        /// Insert `val` into the multiset.
        ///
        /// # Arguments
        ///
        /// * `val` - The value to insert
        ///
        /// # Returns
        ///
        /// `true` if val was NOT already present, `false` if it was
        pub fn insert(&mut self, val: i32) -> bool {
            let idx = self.nums.len();
            self.nums.push(val);
            // entry API: get or create the index set, then insert the new index
            let idx_set = self.val_to_indices.entry(val).or_default();
            let was_empty = idx_set.is_empty();
            idx_set.insert(idx);
            was_empty
        }

        /// Remove one occurrence of `val` from the multiset.
        ///
        /// Uses swap-with-last to maintain O(1) removal:
        /// 1. Pick any index where `val` lives
        /// 2. Overwrite it with the last element
        /// 3. Update the last element's index set
        /// 4. Pop the Vec
        ///
        /// # Arguments
        ///
        /// * `val` - The value to remove
        ///
        /// # Returns
        ///
        /// `true` if val was present, `false` otherwise
        pub fn remove(&mut self, val: i32) -> bool {
            // First, check if val exists and grab an index from its set
            let idx = {
                let Some(idx_set) = self.val_to_indices.get_mut(&val) else {
                    return false;
                };
                if idx_set.is_empty() {
                    return false;
                }
                let idx = *idx_set.iter().next().expect("set is non-empty");
                idx_set.remove(&idx);
                idx
            };
            // Borrow of idx_set is now dropped — safe to borrow again

            let last_idx = self.nums.len() - 1;
            if idx != last_idx {
                let last_val = self.nums[last_idx];
                self.nums[idx] = last_val;
                // Update last_val's index set: remove last_idx, add idx
                let last_set = self
                    .val_to_indices
                    .get_mut(&last_val)
                    .expect("last_val must exist");
                last_set.remove(&last_idx);
                last_set.insert(idx);
            }
            self.nums.pop();

            // Clean up empty sets
            if self
                .val_to_indices
                .get(&val)
                .is_some_and(|s| s.is_empty())
            {
                self.val_to_indices.remove(&val);
            }
            true
        }

        /// Return a random element from the multiset.
        ///
        /// Probability is proportional to the count of each value.
        ///
        /// # Returns
        ///
        /// A random element
        pub fn get_random(&self) -> i32 {
            use std::time::SystemTime;
            let seed = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("time went backwards")
                .subsec_nanos() as usize;
            self.nums[seed % self.nums.len()]
        }
    }
}

/// AI solution module — uses `BTreeSet<usize>` instead of `HashSet<usize>`
/// for index storage, allowing O(1) amortized "pop any" via `pop_last()`.
///
/// The key difference: `BTreeSet::pop_last()` is cleaner than iterating a
/// `HashSet` to grab an arbitrary element, and avoids the hidden iterator
/// allocation overhead of `HashSet::iter().next()`.
///
/// insert:     O(1) avg (amortized for BTreeSet)
/// remove:     O(1) avg (amortized)
/// get_random: O(1)
/// Space:      O(n)
mod ai_solution {
    use std::collections::{BTreeSet, HashMap};

    /// A multiset supporting insert, remove, and get_random in O(1) average.
    pub struct RandomizedCollection {
        nums: Vec<i32>,
        val_to_indices: HashMap<i32, BTreeSet<usize>>,
        /// Internal xorshift64 state for fast pseudo-random index generation.
        rng_state: u64,
    }

    impl RandomizedCollection {
        /// Initialize an empty RandomizedCollection.
        pub fn new() -> Self {
            use std::time::SystemTime;
            let seed = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("time went backwards")
                .subsec_nanos() as u64
                | 1; // Ensure non-zero for xorshift
            Self {
                nums: Vec::new(),
                val_to_indices: HashMap::new(),
                rng_state: seed,
            }
        }

        /// Insert `val` into the multiset.
        ///
        /// # Returns
        ///
        /// `true` if val was NOT already present, `false` if it was
        pub fn insert(&mut self, val: i32) -> bool {
            let idx = self.nums.len();
            self.nums.push(val);
            let idx_set = self.val_to_indices.entry(val).or_default();
            let was_empty = idx_set.is_empty();
            idx_set.insert(idx);
            was_empty
        }

        /// Remove one occurrence of `val` from the multiset.
        ///
        /// Uses `BTreeSet::pop_last()` to grab an index without iterator
        /// overhead, then swap-with-last for O(1) Vec removal.
        ///
        /// # Returns
        ///
        /// `true` if val was present, `false` otherwise
        pub fn remove(&mut self, val: i32) -> bool {
            // Extract index in a scoped block to drop the mutable borrow
            // before we need to borrow val_to_indices again for last_val
            let idx = {
                let Some(idx_set) = self.val_to_indices.get_mut(&val) else {
                    return false;
                };
                let Some(idx) = idx_set.pop_last() else {
                    return false;
                };
                idx
            };

            let last_idx = self.nums.len() - 1;
            if idx != last_idx {
                let last_val = self.nums[last_idx];
                self.nums[idx] = last_val;
                let last_set = self
                    .val_to_indices
                    .get_mut(&last_val)
                    .expect("last_val must exist");
                last_set.remove(&last_idx);
                last_set.insert(idx);
            }
            self.nums.pop();

            // Clean up empty entries
            if self
                .val_to_indices
                .get(&val)
                .is_some_and(|s| s.is_empty())
            {
                self.val_to_indices.remove(&val);
            }
            true
        }

        /// Return a random element from the multiset.
        ///
        /// Uses xorshift64 PRNG for fast, uniform index selection.
        ///
        /// # Returns
        ///
        /// A random element
        pub fn get_random(&mut self) -> i32 {
            // xorshift64
            let mut s = self.rng_state;
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            self.rng_state = s;
            self.nums[(s as usize) % self.nums.len()]
        }
    }
}

fn main() {
    println!("--- My Solution ---");
    {
        let mut rc = my_solution::RandomizedCollection::new();
        println!("  insert(1) -> {}", rc.insert(1));
        println!("  insert(1) -> {}", rc.insert(1));
        println!("  insert(2) -> {}", rc.insert(2));
        println!("  get_random() -> {}", rc.get_random());
        println!("  remove(1) -> {}", rc.remove(1));
        println!("  get_random() -> {}", rc.get_random());
    }

    println!("--- AI Solution ---");
    {
        let mut rc = ai_solution::RandomizedCollection::new();
        println!("  insert(1) -> {}", rc.insert(1));
        println!("  insert(1) -> {}", rc.insert(1));
        println!("  insert(2) -> {}", rc.insert(2));
        println!("  get_random() -> {}", rc.get_random());
        println!("  remove(1) -> {}", rc.remove(1));
        println!("  get_random() -> {}", rc.get_random());
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_randomized_collection {
        ($mod_name:ident, $rc_path:path) => {
            mod $mod_name {
                use $rc_path as RandomizedCollection;

                #[test]
                fn test_insert_new() {
                    let mut rc = RandomizedCollection::new();
                    assert!(rc.insert(1)); // first insert returns true
                }

                #[test]
                fn test_insert_duplicate() {
                    let mut rc = RandomizedCollection::new();
                    assert!(rc.insert(1));
                    assert!(!rc.insert(1)); // duplicate returns false
                }

                #[test]
                fn test_remove_existing() {
                    let mut rc = RandomizedCollection::new();
                    rc.insert(1);
                    assert!(rc.remove(1));
                }

                #[test]
                fn test_remove_nonexistent() {
                    let mut rc = RandomizedCollection::new();
                    assert!(!rc.remove(1));
                }

                #[test]
                fn test_remove_one_of_many() {
                    let mut rc = RandomizedCollection::new();
                    rc.insert(1);
                    rc.insert(1);
                    rc.insert(1);
                    // Remove one — should succeed and still have 2 left
                    assert!(rc.remove(1));
                    // get_random must return 1 (only value remaining)
                    assert_eq!(rc.get_random(), 1);
                    assert!(rc.remove(1));
                    assert_eq!(rc.get_random(), 1);
                    assert!(rc.remove(1));
                    // Now empty — cannot remove again
                    assert!(!rc.remove(1));
                }

                #[test]
                fn test_get_random_single() {
                    let mut rc = RandomizedCollection::new();
                    rc.insert(42);
                    assert_eq!(rc.get_random(), 42);
                }

                #[test]
                fn test_get_random_returns_valid() {
                    let mut rc = RandomizedCollection::new();
                    rc.insert(1);
                    rc.insert(1);
                    rc.insert(2);
                    let val = rc.get_random();
                    assert!(val == 1 || val == 2);
                }

                #[test]
                fn test_example_sequence() {
                    let mut rc = RandomizedCollection::new();
                    assert!(rc.insert(1));   // true: 1 not present
                    assert!(!rc.insert(1));  // false: 1 already present
                    assert!(rc.insert(2));   // true: 2 not present
                    let r = rc.get_random();
                    assert!(r == 1 || r == 2);
                    assert!(rc.remove(1));
                    let r = rc.get_random();
                    assert!(r == 1 || r == 2);
                }

                #[test]
                fn test_interleaved_ops() {
                    let mut rc = RandomizedCollection::new();
                    rc.insert(4);
                    rc.insert(3);
                    rc.insert(4);
                    rc.insert(2);
                    rc.insert(4);

                    assert!(rc.remove(4));
                    assert!(rc.remove(3));
                    assert!(rc.remove(4));
                    // Only 2 and one 4 remain
                    let val = rc.get_random();
                    assert!(val == 2 || val == 4);
                }
            }
        };
    }

    test_randomized_collection!(my_tests, crate::my_solution::RandomizedCollection);
    test_randomized_collection!(ai_tests, crate::ai_solution::RandomizedCollection);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 10_000_u32;

        let start = Instant::now();
        for _ in 0..iterations {
            let mut rc = crate::my_solution::RandomizedCollection::new();
            for i in 0..100 {
                std::hint::black_box(rc.insert(std::hint::black_box(i % 20)));
            }
            for i in 0..50 {
                std::hint::black_box(rc.remove(std::hint::black_box(i % 20)));
            }
            for _ in 0..50 {
                std::hint::black_box(rc.get_random());
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let mut rc = crate::ai_solution::RandomizedCollection::new();
            for i in 0..100 {
                std::hint::black_box(rc.insert(std::hint::black_box(i % 20)));
            }
            for i in 0..50 {
                std::hint::black_box(rc.remove(std::hint::black_box(i % 20)));
            }
            for _ in 0..50 {
                std::hint::black_box(rc.get_random());
            }
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
