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
// LeetCode Rust signature (note: all methods take &self, not &mut self):
//   fn new() -> Self
//   fn insert(&self, val: i32) -> bool
//   fn remove(&self, val: i32) -> bool
//   fn get_random(&self) -> i32
//
// Run:  cargo run --example lc_0380_insert_delete_getrandom_o1
// Test: cargo test --example lc_0380_insert_delete_getrandom_o1
// Bench: cargo test --example lc_0380_insert_delete_getrandom_o1 bench -- --nocapture
//
// --- Rust concepts used ---
// - HashMap::entry API (Entry::Occupied / Entry::Vacant): single-lookup
//   insert-or-skip pattern, avoids double hashing from contains_key + insert
// - struct with private fields + pub methods: encapsulation pattern for
//   data structure design
// - if let Some(&index) destructuring: copies the usize out of the
//   reference returned by HashMap::get
//   (Previously covered: HashMap basics -> lc_0001, Vec::push/pop -> lc_0014,
//    .expect() for invariants -> lc_0142, SystemTime for pseudo-random -> lc_0278)
//
// --- Benchmark (10_000 iterations, 2026-04-07) ---
// My solution:  976.10ms (avg 97.61us)
// AI solution:  752.83ms (avg 75.28us)
// -> AI solution is 1.30x faster
// Reason: entry API avoids double hashing on insert, and the if index !=
//   last_index guard in remove skips the unnecessary swap when removing
//   the last element.

/// My solution module.
///
/// insert:     O(1) avg
/// remove:     O(1) avg
/// get_random: O(1)
/// Space:      O(n)
mod my_solution {
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;

    /// A set that supports insert, remove, and get_random in O(1) average time.
    ///
    /// Uses a Vec for O(1) random-index access and a HashMap mapping each
    /// value to its index in the Vec for O(1) lookup/removal.
    /// All fields wrapped in `RefCell` / `Cell` for interior mutability so
    /// every method can take `&self` — matching LeetCode's required signature.
    pub struct RandomizedSet {
        /// RefCell allows mutable borrows through &self (interior mutability).
        nums: RefCell<Vec<i32>>,
        val_to_index: RefCell<HashMap<i32, usize>>,
        /// Internal xorshift64 state for O(1) random index generation.
        rng_state: Cell<u64>,
    }

    impl RandomizedSet {
        /// Initialize an empty RandomizedSet.
        pub fn new() -> Self {
            use std::time::SystemTime;
            // Seed once at construction; `| 1` ensures non-zero (xorshift
            // requirement).
            let seed = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("time went backwards")
                .subsec_nanos() as u64
                | 1;
            Self {
                nums: RefCell::new(Vec::new()),
                val_to_index: RefCell::new(HashMap::new()),
                rng_state: Cell::new(seed),
            }
        }

        /// Insert `val` into the set if not present.
        /// Takes `&self` (not `&mut self`) — LeetCode's required signature.
        /// Interior mutability via RefCell allows mutation.
        ///
        /// # Returns
        ///
        /// `true` if the item was inserted, `false` if already present
        pub fn insert(&self, val: i32) -> bool {
            let mut map = self.val_to_index.borrow_mut();
            if map.contains_key(&val) {
                return false;
            }
            let mut nums = self.nums.borrow_mut();
            map.insert(val, nums.len());
            nums.push(val);
            true
        }

        /// Remove `val` from the set if present.
        ///
        /// Swaps the target with the last element for O(1) removal.
        ///
        /// # Returns
        ///
        /// `true` if the item was removed, `false` if not found
        pub fn remove(&self, val: i32) -> bool {
            let mut map = self.val_to_index.borrow_mut();
            if let Some(&index) = map.get(&val) {
                let mut nums = self.nums.borrow_mut();
                let last_val = *nums.last().expect("set is non-empty");
                // Swap target with last element in the array
                nums[index] = last_val;
                map.insert(last_val, index);
                // Remove the last element
                nums.pop();
                map.remove(&val);
                true
            } else {
                false
            }
        }

        /// Return a random element from the set.
        ///
        /// Each element must be equally likely to be returned.
        /// Uses xorshift64 PRNG so rapid successive calls produce
        /// distinct pseudo-random indices (unlike SystemTime which
        /// can repeat within the same nanosecond).
        ///
        /// # Returns
        ///
        /// A random element from the current set
        pub fn get_random(&self) -> i32 {
            // xorshift64: advance state on every call for uniform distribution
            let mut s = self.rng_state.get();
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            self.rng_state.set(s);
            let nums = self.nums.borrow();
            nums[(s as usize) % nums.len()]
        }
    }
}

/// AI solution module — same approach with `entry` API for insert.
///
/// Uses `HashMap::entry` to avoid double lookup on insert, and
/// `swap_remove`-style logic with explicit last-element handling.
///
/// insert:     O(1) avg
/// remove:     O(1) avg
/// get_random: O(1)
/// Space:      O(n)
mod ai_solution {
    use std::collections::HashMap;

    /// A set that supports insert, remove, and get_random in O(1) average time.
    pub struct RandomizedSet {
        nums: Vec<i32>,
        val_to_index: HashMap<i32, usize>,
    }

    impl RandomizedSet {
        /// Initialize an empty RandomizedSet.
        pub fn new() -> Self {
            Self {
                nums: Vec::new(),
                val_to_index: HashMap::new(),
            }
        }

        /// Insert `val` into the set if not present.
        ///
        /// # Returns
        ///
        /// `true` if the item was inserted, `false` if already present
        pub fn insert(&mut self, val: i32) -> bool {
            // entry API: single lookup instead of contains_key + insert
            use std::collections::hash_map::Entry;
            match self.val_to_index.entry(val) {
                Entry::Occupied(_) => false,
                Entry::Vacant(e) => {
                    e.insert(self.nums.len());
                    self.nums.push(val);
                    true
                }
            }
        }

        /// Remove `val` from the set if present.
        ///
        /// # Returns
        ///
        /// `true` if the item was removed, `false` if not found
        pub fn remove(&mut self, val: i32) -> bool {
            if let Some(index) = self.val_to_index.remove(&val) {
                let last_index = self.nums.len() - 1;
                if index != last_index {
                    // Move last element into the vacated slot
                    let last_val = self.nums[last_index];
                    self.nums[index] = last_val;
                    self.val_to_index.insert(last_val, index);
                }
                self.nums.pop();
                true
            } else {
                false
            }
        }

        /// Return a random element from the set.
        ///
        /// Each element must be equally likely to be returned.
        ///
        /// # Returns
        ///
        /// A random element from the current set
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

fn main() {
    println!("--- My Solution ---");
    {
        // No longer needs `mut` — all methods take &self via RefCell
        let rs = my_solution::RandomizedSet::new();
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
            // allow(unused_mut): my_solution takes &self so `mut` is unused,
            // but ai_solution still takes &mut self and needs it.
            #[allow(unused_mut)]
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
            let rs = crate::my_solution::RandomizedSet::new();
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
