// LeetCode 398: Random Pick Index [Medium]
// https://leetcode.com/problems/random-pick-index/
//
// Benchmark: 100,000 iterations (2026-04-08)
//   My solution (HashMap pre-index): avg 15ns
//   AI solution (reservoir sampling): avg 9.37us
//   -> My solution is 602.63x faster
//
// Given an integer array `nums` with possible duplicates, randomly output
// the index of a given `target` number. You can assume the target exists
// in the array. Each valid index should have equal probability.
//
// Implement the Solution class:
//   Solution(int[] nums)   — init with nums
//   int pick(int target)   — random index where nums[i] == target
//
// Examples:
//   nums = [1,2,3,3,3]
//   pick(3) -> 2 or 3 or 4 (equal probability)
//   pick(1) -> 0
//
// Constraints:
//   - 1 <= nums.length <= 2 * 10^4
//   - -2^31 <= nums[i] <= 2^31 - 1
//   - target exists in nums
//   - At most 10^4 calls to pick
//
// Run:  cargo run --example lc_0398_random_pick_index
// Test: cargo test --example lc_0398_random_pick_index
// Bench: cargo test --example lc_0398_random_pick_index bench -- --nocapture

/// My solution module — HashMap pre-index approach.
///
/// Build a HashMap<i32, Vec<usize>> mapping each value to its indices
/// during construction. On `pick`, randomly select from the stored
/// indices. O(1) per pick (after O(n) preprocessing).
///
/// Time:  O(n) constructor, O(1) pick
/// Space: O(n) for the index map
mod my_solution {
    use std::collections::HashMap;
    use rand::Rng;

    /// Pre-indexes all values to their positions for O(1) random pick.
    pub struct Solution {
        index_map: HashMap<i32, Vec<usize>>,
    }

    impl Solution {
        /// Create a new Solution with the given nums array.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array with possible duplicates
        pub fn new(nums: Vec<i32>) -> Self {
            let mut index_map: HashMap<i32, Vec<usize>> = HashMap::new();
            for (i, &v) in nums.iter().enumerate() {
                index_map.entry(v).or_default().push(i);
            }
            Self { index_map }
        }

        /// Pick a random index where nums[index] == target.
        ///
        /// # Arguments
        ///
        /// * `target` - The value to search for
        ///
        /// # Returns
        ///
        /// A random index i where nums[i] == target
        pub fn pick(&self, target: i32) -> i32 {
            let indices = self
                .index_map
                .get(&target)
                .expect("target is guaranteed to exist");
            // Simple random using a fast deterministic source
            // In production, use rand crate. Here we use a simple approach.
            let idx = rand::rng().random_range(0..indices.len());
            indices[idx] as i32
        }
    }
}

/// AI solution module — reservoir sampling (no extra storage).
///
/// Store only the original array. On each `pick`, scan the array and
/// use reservoir sampling: for the k-th occurrence of `target`, replace
/// the answer with probability 1/k. This gives uniform distribution.
///
/// Time:  O(1) constructor, O(n) per pick
/// Space: O(n) for storing nums only (no index map)
mod ai_solution {
    use rand::Rng;

    /// Uses reservoir sampling for O(n) pick with no pre-processing.
    pub struct Solution {
        nums: Vec<i32>,
    }

    impl Solution {
        /// Create a new Solution with the given nums array.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array with possible duplicates
        pub fn new(nums: Vec<i32>) -> Self {
            Self { nums }
        }

        /// Pick a random index where nums[index] == target.
        ///
        /// Uses reservoir sampling: for the k-th match, keep it with
        /// probability 1/k. This ensures uniform distribution.
        ///
        /// # Arguments
        ///
        /// * `target` - The value to search for
        ///
        /// # Returns
        ///
        /// A random index i where nums[i] == target
        pub fn pick(&self, target: i32) -> i32 {
            let mut count = 0u32;
            let mut ans = 0i32;

            for (i, &v) in self.nums.iter().enumerate() {
                if v == target {
                    count += 1;
                    // With probability 1/count, replace the answer
                    if rand::rng().random_range(1..=count) == count {
                        ans = i as i32;
                    }
                }
            }
            ans
        }
    }
}

fn main() {
    let nums = vec![1, 2, 3, 3, 3];

    println!("--- My Solution ---");
    let sol = my_solution::Solution::new(nums.clone());
    for _ in 0..5 {
        println!("  pick(3) -> {}", sol.pick(3));
    }
    println!("  pick(1) -> {}", sol.pick(1));

    println!("--- AI Solution ---");
    let sol = ai_solution::Solution::new(nums);
    for _ in 0..5 {
        println!("  pick(3) -> {}", sol.pick(3));
    }
    println!("  pick(1) -> {}", sol.pick(1));
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn test_my_pick_valid_indices() {
        let sol = crate::my_solution::Solution::new(vec![1, 2, 3, 3, 3]);
        let valid: HashSet<i32> = [2, 3, 4].iter().copied().collect();
        // Run multiple times to check we always get valid indices
        for _ in 0..100 {
            let idx = sol.pick(3);
            assert!(valid.contains(&idx), "got invalid index {idx}");
        }
    }

    #[test]
    fn test_my_pick_single() {
        let sol = crate::my_solution::Solution::new(vec![1, 2, 3, 3, 3]);
        assert_eq!(sol.pick(1), 0);
    }

    #[test]
    fn test_ai_pick_valid_indices() {
        let sol = crate::ai_solution::Solution::new(vec![1, 2, 3, 3, 3]);
        let valid: HashSet<i32> = [2, 3, 4].iter().copied().collect();
        for _ in 0..100 {
            let idx = sol.pick(3);
            assert!(valid.contains(&idx), "got invalid index {idx}");
        }
    }

    #[test]
    fn test_ai_pick_single() {
        let sol = crate::ai_solution::Solution::new(vec![1, 2, 3, 3, 3]);
        assert_eq!(sol.pick(1), 0);
    }

    #[test]
    fn test_my_distribution() {
        // Statistical test: each index should appear ~33%
        let sol = crate::my_solution::Solution::new(vec![1, 2, 3, 3, 3]);
        let mut counts = [0u32; 5];
        let trials = 30_000;
        for _ in 0..trials {
            counts[sol.pick(3) as usize] += 1;
        }
        // Each of indices 2,3,4 should get ~10000 hits
        for &i in &[2usize, 3, 4] {
            let pct = counts[i] as f64 / trials as f64;
            assert!(
                (0.28..=0.39).contains(&pct),
                "index {i} got {:.1}%, expected ~33%",
                pct * 100.0
            );
        }
    }

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let nums: Vec<i32> = (0..20_000).map(|i| i % 100).collect();

        let my_sol = crate::my_solution::Solution::new(nums.clone());
        let ai_sol = crate::ai_solution::Solution::new(nums);

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(my_sol.pick(std::hint::black_box(50)));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(ai_sol.pick(std::hint::black_box(50)));
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
