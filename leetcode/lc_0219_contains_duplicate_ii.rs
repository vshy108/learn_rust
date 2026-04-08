// LeetCode 219: Contains Duplicate II [Easy]
// https://leetcode.com/problems/contains-duplicate-ii/
//
// Given an integer array nums and an integer k, return true if there are
// two distinct indices i and j in the array such that nums[i] == nums[j]
// and abs(i - j) <= k.
//
// Examples:
//   nums = [1,2,3,1],       k = 3 -> true
//   nums = [1,0,1,1],       k = 1 -> true
//   nums = [1,2,3,1,2,3],   k = 2 -> false
//
// Constraints:
//   - 1 <= nums.length <= 10^5
//   - -10^9 <= nums[i] <= 10^9
//   - 0 <= k <= 10^5
//
// --- Benchmark (100_000 iterations, 2026-04-05) ---
//   My solution: 6.262s (avg 62.627us)
//   AI solution: 5.023s (avg 50.237us)
//   -> AI solution is 1.25x faster
//   Reason: with_capacity(k+1) pre-allocates the HashSet, avoiding rehashing
//     during window growth; early return on k==0 skips all work for that case.
/// My solution module.
///
/// Sliding-window HashSet: maintains a window of at most k elements.
/// When a duplicate insert fails, we found a nearby duplicate.
/// Evicts the oldest element when the window exceeds size k.
///
/// Time:  O(n) — single pass; HashSet insert/remove is O(1) amortized
/// Space: O(min(n, k)) — HashSet holds at most k+1 elements
mod my_solution {
    pub struct Solution;
    use std::collections::HashSet;

    impl Solution {
        /// Return `true` if there exist two distinct indices `i` and `j`
        /// such that `nums[i] == nums[j]` and `abs(i - j) <= k`.
        ///
        /// # Arguments
        ///
        /// * `nums` - The input integer array
        /// * `k` - Maximum allowed index distance
        ///
        /// # Returns
        ///
        /// `true` if a nearby duplicate exists, `false` otherwise
        pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
            // Ruby: seen = Set.new -> Rust: HashSet::new()
            let mut seen: HashSet<i32> = HashSet::new();

            // Ruby: nums.each_with_index { |num, i| ... }
            // Rust: .iter() borrows elements, .enumerate() adds index
            // Without .iter(), `for num in nums` would consume (move) the Vec
            for (i, num) in nums.iter().enumerate() {
                // Ruby: return true unless seen.add?(num)
                // Rust: HashSet::insert returns false if value already present
                // *num dereferences &i32 -> i32 because HashSet<i32> stores owned values
                if !seen.insert(*num) {
                    return true;
                }
                // Ruby: seen.delete(nums[i - k])
                // Rust: .remove() takes &T (a reference), not T (owned value),
                // so we must pass &nums[...] instead of nums[...]
                if i >= (k as usize) {
                    seen.remove(&nums[i - (k as usize)]);
                }
            }

            false
        }
    }
}

/// AI solution module.
///
/// Same sliding-window HashSet approach as my_solution, but optimized:
/// - Early return when k == 0 (no two distinct indices can have distance 0)
/// - Pre-allocates HashSet with `with_capacity(k + 1)` to avoid rehashing
///
/// Time:  O(n) — single pass; HashSet insert/remove is O(1) amortized
/// Space: O(min(n, k)) — HashSet holds at most k+1 elements
mod ai_solution {
    pub struct Solution;
    use std::collections::HashSet;

    impl Solution {
        /// Return `true` if there exist two distinct indices `i` and `j`
        /// such that `nums[i] == nums[j]` and `abs(i - j) <= k`.
        ///
        /// # Arguments
        ///
        /// * `nums` - The input integer array
        /// * `k` - Maximum allowed index distance
        ///
        /// # Returns
        ///
        /// `true` if a nearby duplicate exists, `false` otherwise
        pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
            // k == 0 means no two *distinct* indices can satisfy abs(i-j) <= 0
            if k == 0 {
                return false;
            }

            let k = k as usize;
            // Pre-allocate exact capacity to avoid rehashing as the window grows
            // HashSet::new() starts at capacity 0 and reallocates on growth
            let mut window = HashSet::with_capacity(k + 1);

            for i in 0..nums.len() {
                // Direct index access nums[i] avoids the *num dereference needed
                // with .iter().enumerate() — both compile to the same code
                if !window.insert(nums[i]) {
                    return true;
                }
                // Evict oldest element when window exceeds size k
                // Uses .len() > k instead of i >= k — equivalent check but
                // reads more clearly as "window is full"
                if window.len() > k {
                    window.remove(&nums[i - k]);
                }
            }

            false
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, bool)> = vec![
        (vec![1, 2, 3, 1], 3, true),
        (vec![1, 0, 1, 1], 1, true),
        (vec![1, 2, 3, 1, 2, 3], 2, false),
    ];

    println!("--- My Solution ---");
    for (nums, k, expected) in &test_cases {
        let result = my_solution::Solution::contains_nearby_duplicate(nums.clone(), *k);
        println!("  nums={nums:?}, k={k} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, k, expected) in &test_cases {
        let result = ai_solution::Solution::contains_nearby_duplicate(nums.clone(), *k);
        println!("  nums={nums:?}, k={k} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_contains_nearby_dup {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_duplicate_within_k() {
                    assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
                }

                #[test]
                fn test_duplicate_at_distance_1() {
                    assert!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
                }

                #[test]
                fn test_duplicate_outside_k() {
                    assert!(!Solution::contains_nearby_duplicate(
                        vec![1, 2, 3, 1, 2, 3],
                        2
                    ));
                }

                #[test]
                fn test_no_duplicates() {
                    assert!(!Solution::contains_nearby_duplicate(vec![1, 2, 3, 4], 2));
                }

                #[test]
                fn test_single_element() {
                    assert!(!Solution::contains_nearby_duplicate(vec![1], 1));
                }

                #[test]
                fn test_all_same_k_zero() {
                    assert!(!Solution::contains_nearby_duplicate(vec![1, 1, 1], 0));
                }

                #[test]
                fn test_duplicate_at_exact_k() {
                    // indices 0 and 2, distance = 2, k = 2 -> true
                    assert!(Solution::contains_nearby_duplicate(vec![1, 2, 1], 2));
                }
            }
        };
    }

    test_contains_nearby_dup!(my_tests, crate::my_solution::Solution);
    test_contains_nearby_dup!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(Vec<i32>, i32)> = vec![
            (vec![1, 2, 3, 1], 3),
            (vec![1, 0, 1, 1], 1),
            (vec![1, 2, 3, 1, 2, 3], 2),
            ((0..100).collect(), 50),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums, k) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::contains_nearby_duplicate(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(*k),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums, k) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::contains_nearby_duplicate(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(*k),
                ));
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
