// LeetCode 380: Insert Delete GetRandom O(1) [Medium]
// https://leetcode.com/problems/insert-delete-getrandom-o1/
//
// Implement the RandomizedSet struct:
//   - RandomizedSet::new()           -- Initializes the object.
//   - insert(val: i32) -> bool       -- Inserts val if not present.
//                                       Returns true if inserted, false if already present.
//   - remove(val: i32) -> bool       -- Removes val if present.
//                                       Returns true if removed, false if not found.
//   - get_random() -> i32            -- Returns a random element from the set.
//                                       Each element must have the same probability.
//
// You must implement the functions such that each function works in
// average O(1) time complexity.
//
// Examples:
//   insert(1) -> true, remove(2) -> false, insert(2) -> true,
//   get_random() -> 1 or 2, remove(1) -> true, insert(2) -> false,
//   get_random() -> 2
//
// Constraints:
//   - -2^31 <= val <= 2^31 - 1
//   - At most 2 * 10^5 calls total to insert, remove, and get_random
//   - There will be at least one element when get_random is called
//
// Run:  cargo run --example lc_0380_insert_delete_getrandom_o1
// Test: cargo test --example lc_0380_insert_delete_getrandom_o1
// Bench: cargo test --example lc_0380_insert_delete_getrandom_o1 bench -- --nocapture

/// My solution module.
///
/// insert:     O(?) avg
/// remove:     O(?) avg
/// get_random: O(?) avg
/// Space:      O(?)
mod my_solution {
    /// A set that supports insert, remove, and get_random in O(1) average time.
    pub struct RandomizedSet {
        // @nums = []          array for O(1) random access
        // @val_to_index = {}  hash for O(1) lookup
    }

    impl RandomizedSet {
        /// Initialize an empty RandomizedSet.
        pub fn new() -> Self {
            // @nums = []
            // @val_to_index = {}
            todo!()
        }

        /// Insert `val` into the set if not present.
        ///
        /// # Returns
        ///
        /// `true` if the item was inserted, `false` if already present
        pub fn insert(&mut self, val: i32) -> bool {
            // return false if @val_to_index.key?(val)
            // @val_to_index[val] = @nums.size
            // @nums << val
            // true
            todo!()
        }

        /// Remove `val` from the set if present.
        ///
        /// # Returns
        ///
        /// `true` if the item was removed, `false` if not found
        pub fn remove(&mut self, val: i32) -> bool {
            // return false unless @val_to_index.key?(val)
            // index = @val_to_index[val]
            // last_val = @nums[-1]
            // @nums[index] = last_val
            // @val_to_index[last_val] = index
            // @nums.pop
            // @val_to_index.delete(val)
            // true
            todo!()
        }

        /// Return a random element from the set.
        ///
        /// Each element must be equally likely to be returned.
        ///
        /// # Returns
        ///
        /// A random element from the current set
        pub fn get_random(&self) -> i32 {
            // @nums.sample
            todo!()
        }
    }
}

/// AI solution module.
///
/// insert:     O(?) avg
/// remove:     O(?) avg
/// get_random: O(?) avg
/// Space:      O(?)
mod ai_solution {
    /// A set that supports insert, remove, and get_random in O(1) average time.
    pub struct RandomizedSet {
        // TODO: choose internal data structures
    }

    impl RandomizedSet {
        /// Initialize an empty RandomizedSet.
        pub fn new() -> Self {
            todo!()
        }

        /// Insert `val` into the set if not present.
        ///
        /// # Returns
        ///
        /// `true` if the item was inserted, `false` if already present
        pub fn insert(&mut self, val: i32) -> bool {
            todo!()
        }

        /// Remove `val` from the set if present.
        ///
        /// # Returns
        ///
        /// `true` if the item was removed, `false` if not found
        pub fn remove(&mut self, val: i32) -> bool {
            todo!()
        }

        /// Return a random element from the set.
        ///
        /// Each element must be equally likely to be returned.
        ///
        /// # Returns
        ///
        /// A random element from the current set
        pub fn get_random(&self) -> i32 {
            todo!()
        }
    }
}

fn main() {
    println!("--- My Solution ---");
    {
        let mut rs = my_solution::RandomizedSet::new();
        println!("  insert(1) -> {}", rs.insert(1));
        println!("  remove(2) -> {}", rs.remove(2));
        println!("  insert(2) -> {}", rs.insert(2));
        println!("  get_random() -> {}", rs.get_random());
        println!("  remove(1) -> {}", rs.remove(1));
        println!("  insert(2) -> {}", rs.insert(2));
        println!("  get_random() -> {}", rs.get_random());
    }

    println!("--- AI Solution ---");
    {
        let mut rs = ai_solution::RandomizedSet::new();
        println!("  insert(1) -> {}", rs.insert(1));
        println!("  remove(2) -> {}", rs.remove(2));
        println!("  insert(2) -> {}", rs.insert(2));
        println!("  get_random() -> {}", rs.get_random());
        println!("  remove(1) -> {}", rs.remove(1));
        println!("  insert(2) -> {}", rs.insert(2));
        println!("  get_random() -> {}", rs.get_random());
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_randomized_set {
        ($mod_name:ident, $rs_path:path) => {
            mod $mod_name {
                use $rs_path as RandomizedSet;

                #[test]
                fn test_insert_new() {
                    let mut rs = RandomizedSet::new();
                    assert!(rs.insert(1));
                }

                #[test]
                fn test_insert_duplicate() {
                    let mut rs = RandomizedSet::new();
                    assert!(rs.insert(1));
                    assert!(!rs.insert(1));
                }

                #[test]
                fn test_remove_existing() {
                    let mut rs = RandomizedSet::new();
                    rs.insert(1);
                    assert!(rs.remove(1));
                }

                #[test]
                fn test_remove_nonexistent() {
                    let mut rs = RandomizedSet::new();
                    assert!(!rs.remove(1));
                }

                #[test]
                fn test_get_random_single() {
                    let mut rs = RandomizedSet::new();
                    rs.insert(42);
                    assert_eq!(rs.get_random(), 42);
                }

                #[test]
                fn test_insert_remove_insert() {
                    let mut rs = RandomizedSet::new();
                    rs.insert(1);
                    rs.remove(1);
                    assert!(rs.insert(1)); // should succeed after removal
                }

                #[test]
                fn test_get_random_returns_valid_element() {
                    let mut rs = RandomizedSet::new();
                    rs.insert(10);
                    rs.insert(20);
                    rs.insert(30);
                    let val = rs.get_random();
                    assert!(val == 10 || val == 20 || val == 30);
                }
            }
        };
    }

    test_randomized_set!(my_tests, crate::my_solution::RandomizedSet);
    test_randomized_set!(ai_tests, crate::ai_solution::RandomizedSet);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 10_000_u32;

        let start = Instant::now();
        for _ in 0..iterations {
            let mut rs = crate::my_solution::RandomizedSet::new();
            for i in 0..100 {
                std::hint::black_box(rs.insert(std::hint::black_box(i)));
            }
            for i in 0..50 {
                std::hint::black_box(rs.remove(std::hint::black_box(i)));
            }
            for _ in 0..50 {
                std::hint::black_box(rs.get_random());
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let mut rs = crate::ai_solution::RandomizedSet::new();
            for i in 0..100 {
                std::hint::black_box(rs.insert(std::hint::black_box(i)));
            }
            for i in 0..50 {
                std::hint::black_box(rs.remove(std::hint::black_box(i)));
            }
            for _ in 0..50 {
                std::hint::black_box(rs.get_random());
            }
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
