// LeetCode 384: Shuffle an Array [Medium]
// https://leetcode.com/problems/shuffle-an-array/
//
// Given an integer array nums, design an algorithm to randomly shuffle the
// array. All permutations of the array should be equally likely as a result
// of the shuffling.
//
// Implement the Solution class:
//   - Solution::new(nums)     -- Initializes the object with nums.
//   - reset() -> Vec<i32>     -- Resets the array to its original
//                                configuration and returns it.
//   - shuffle() -> Vec<i32>   -- Returns a random shuffling of the array.
//
// Example:
//   Input: nums = [1, 2, 3]
//   solution.shuffle() -> [3, 1, 2]  (any permutation equally likely)
//   solution.reset()   -> [1, 2, 3]
//   solution.shuffle() -> [1, 3, 2]
//
// Constraints:
//   - 1 <= nums.length <= 50
//   - -10^6 <= nums[i] <= 10^6
//   - All elements of nums are unique
//   - At most 10^4 calls in total will be made to reset and shuffle
//
// Run:  cargo run --example lc_0384_shuffle_an_array
// Test: cargo test --example lc_0384_shuffle_an_array
// Bench: cargo test --example lc_0384_shuffle_an_array bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-04-08) ---
// My solution:  348.58ms (avg 3.48us)
// AI solution:  258.70ms (avg 2.59us)
// -> AI solution is 1.35x faster
// Reason: AI seeds xorshift64 once at construction and advances it with
//   cheap bit ops per swap, while my solution calls SystemTime::now() on
//   every swap — each call is a syscall with ~100ns overhead.

/// My solution module — Fisher-Yates shuffle with SystemTime-based RNG.
///
/// Keeps a copy of the original array for reset. Shuffle mutates a working
/// copy using the Fisher-Yates (Knuth) algorithm, which guarantees each
/// permutation is equally likely.
///
/// new:     O(n)
/// reset:   O(n)
/// shuffle: O(n)
/// Space:   O(n)
mod my_solution {
    /// Stores the original array and a mutable working copy.
    pub struct Solution {
        original: Vec<i32>,
        current: Vec<i32>,
    }

    impl Solution {
        /// Initialize with the given array.
        ///
        /// # Arguments
        ///
        /// * `nums` - The integer array to shuffle
        pub fn new(nums: Vec<i32>) -> Self {
            Self {
                original: nums.clone(),
                current: nums,
            }
        }

        /// Reset the array to its original configuration.
        ///
        /// # Returns
        ///
        /// The original array
        pub fn reset(&mut self) -> Vec<i32> {
            self.current.clone_from(&self.original);
            self.original.clone()
        }

        /// Return a random shuffling of the array using Fisher-Yates.
        ///
        /// Fisher-Yates works by iterating from index 0 to n-1, swapping
        /// each element with a randomly chosen element from [i, n-1].
        /// This produces a uniformly random permutation in O(n).
        ///
        /// # Returns
        ///
        /// The shuffled array
        pub fn shuffle(&mut self) -> Vec<i32> {
            use std::time::SystemTime;
            let n = self.current.len();
            for i in 0..n {
                // Generate a random index in [i, n-1]
                let seed = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("time went backwards")
                    .subsec_nanos() as usize;
                let j = i + seed % (n - i);
                self.current.swap(i, j);
            }
            self.current.clone()
        }
    }
}

/// AI solution module — Fisher-Yates with xorshift64 PRNG.
///
/// Uses an internal xorshift64 state instead of calling SystemTime on
/// every swap. This avoids the syscall overhead of reading the clock
/// n times per shuffle, making it significantly faster.
///
/// new:     O(n)
/// reset:   O(n)
/// shuffle: O(n)
/// Space:   O(n)
mod ai_solution {
    /// Stores the original array and uses xorshift64 for fast random swaps.
    pub struct Solution {
        original: Vec<i32>,
        current: Vec<i32>,
        /// Internal xorshift64 PRNG state — seeded once at construction.
        rng_state: u64,
    }

    impl Solution {
        /// Initialize with the given array.
        ///
        /// # Arguments
        ///
        /// * `nums` - The integer array to shuffle
        pub fn new(nums: Vec<i32>) -> Self {
            use std::time::SystemTime;
            let seed = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("time went backwards")
                .subsec_nanos() as u64
                | 1; // non-zero for xorshift
            Self {
                original: nums.clone(),
                current: nums,
                rng_state: seed,
            }
        }

        /// Advance xorshift64 and return a pseudo-random u64.
        fn next_rand(&mut self) -> u64 {
            let mut s = self.rng_state;
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            self.rng_state = s;
            s
        }

        /// Reset the array to its original configuration.
        ///
        /// # Returns
        ///
        /// The original array
        pub fn reset(&mut self) -> Vec<i32> {
            self.current.clone_from(&self.original);
            self.original.clone()
        }

        /// Return a random shuffling using Fisher-Yates with xorshift64.
        ///
        /// # Returns
        ///
        /// The shuffled array
        pub fn shuffle(&mut self) -> Vec<i32> {
            let n = self.current.len();
            for i in 0..n {
                let j = i + (self.next_rand() as usize) % (n - i);
                self.current.swap(i, j);
            }
            self.current.clone()
        }
    }
}

fn main() {
    println!("--- My Solution ---");
    {
        let mut s = my_solution::Solution::new(vec![1, 2, 3]);
        println!("  shuffle() -> {:?}", s.shuffle());
        println!("  reset()   -> {:?}", s.reset());
        println!("  shuffle() -> {:?}", s.shuffle());
    }

    println!("--- AI Solution ---");
    {
        let mut s = ai_solution::Solution::new(vec![1, 2, 3]);
        println!("  shuffle() -> {:?}", s.shuffle());
        println!("  reset()   -> {:?}", s.reset());
        println!("  shuffle() -> {:?}", s.shuffle());
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_shuffle {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_reset_returns_original() {
                    let mut s = Solution::new(vec![1, 2, 3]);
                    s.shuffle();
                    let reset = s.reset();
                    assert_eq!(reset, vec![1, 2, 3]);
                }

                #[test]
                fn test_shuffle_same_elements() {
                    let mut s = Solution::new(vec![1, 2, 3]);
                    let mut shuffled = s.shuffle();
                    shuffled.sort();
                    assert_eq!(shuffled, vec![1, 2, 3]);
                }

                #[test]
                fn test_single_element() {
                    let mut s = Solution::new(vec![42]);
                    assert_eq!(s.shuffle(), vec![42]);
                    assert_eq!(s.reset(), vec![42]);
                }

                #[test]
                fn test_two_elements() {
                    let mut s = Solution::new(vec![1, 2]);
                    let mut shuffled = s.shuffle();
                    shuffled.sort();
                    assert_eq!(shuffled, vec![1, 2]);
                }

                #[test]
                fn test_reset_after_multiple_shuffles() {
                    let mut s = Solution::new(vec![10, 20, 30, 40, 50]);
                    s.shuffle();
                    s.shuffle();
                    s.shuffle();
                    assert_eq!(s.reset(), vec![10, 20, 30, 40, 50]);
                }

                #[test]
                fn test_negative_values() {
                    let mut s = Solution::new(vec![-3, -1, 0, 2, 5]);
                    let mut shuffled = s.shuffle();
                    shuffled.sort();
                    assert_eq!(shuffled, vec![-3, -1, 0, 2, 5]);
                }
            }
        };
    }

    test_shuffle!(my_tests, crate::my_solution::Solution);
    test_shuffle!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let nums: Vec<i32> = (0..50).collect();

        let start = Instant::now();
        for _ in 0..iterations {
            let mut s = crate::my_solution::Solution::new(
                std::hint::black_box(nums.clone()),
            );
            std::hint::black_box(s.shuffle());
            std::hint::black_box(s.reset());
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let mut s = crate::ai_solution::Solution::new(
                std::hint::black_box(nums.clone()),
            );
            std::hint::black_box(s.shuffle());
            std::hint::black_box(s.reset());
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
