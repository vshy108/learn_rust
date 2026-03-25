// LeetCode 1: Two Sum
// https://leetcode.com/problems/two-sum/
//
// Given an array of integers `nums` and an integer `target`, return indices
// of the two numbers such that they add up to `target`.
//
// Run:  cargo run --example lc_0001_two_sum
// Test: cargo test --example lc_0001_two_sum
// Bench: cargo test --example lc_0001_two_sum bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-03-25) ---
//   My solution: 2.929us avg (293ms total)
//   AI solution: 1.904us avg (190ms total)
//   AI is 1.54x faster
//

/// My solution module — write your own implementation here.
///
/// Time: O(n) — two passes over the data:
///   1. O(n) to build the entire HashMap from the Vec.
///   2. O(n) to iterate with the while loop; each HashMap lookup is O(1) amortized.
///   Total: O(n) + O(n) = O(n).
///
/// Space: O(n) — the HashMap stores up to n entries.
///
/// Note: same big-O as the AI solution, but the constant factor is higher
/// because two separate passes means more cache misses and the full HashMap
/// is allocated upfront before any lookups begin.
///
/// --- Rust concepts used ---
//
// Vec/slice indexing:
//   nums[0]       — direct access, panics if out of bounds
//   nums.get(0)   — returns Option<&T>, no panic (None if out of bounds)
//   index must be usize (not i32, u32, etc.)
//
// Vec length:
//   nums.len()    — returns usize
//
// HashMap:
//   HashMap<K, V> — K is what you look up by, V is what you get back.
//   Here HashMap<i32, usize> means: look up a number (i32) to find its index (usize).
//   .get(&key) takes a reference (&K) and returns Option<&V>.
//
// Pattern destructuring with &:
//   Some(&val) destructures &usize into owned usize (works because usize is Copy).
//   Without &: val would be &usize, requiring *val to dereference.
mod my_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            // the key is the number (i32) and the value is its position (usize)
            // convert Vec to HashMap: value -> index
            let nums_hash_map: HashMap<i32, usize> =
                nums.iter().enumerate().map(|(i, &v)| (v, i)).collect();
            // store length for loop bound
            let len = nums.len();
            let mut index = 0;
            // guarantees exactly one solution then can skip last one
            // because must have pair before it
            while index <= len - 2 {
                let difference = target - nums[index];
                // `.get(&difference)` takes a reference because HashMap::get
                // expects &K.
                // `.get()` returns Option<&usize> (a reference to the stored
                // value).
                // `Some(&diff_index)` destructures the &usize into an owned
                // usize (works because usize is Copy). Without the & in the
                // pattern, diff_index would be &usize and you'd need
                // *diff_index to dereference it.
                //
                // `&& diff_index != index` ensures we don't pair an element
                // with itself. e.g. [3, 2, 4] target=6: without this, index 0
                // (value 3) would match itself since 6-3=3 exists at index 0.
                //
                // `if let ... && condition` is an if-let chain (stabilized in
                // Rust 2024).
                if let Some(&diff_index) = nums_hash_map.get(&difference)
                    && diff_index != index
                {
                    // Cast usize indices to i32 to match LeetCode's return
                    // type.
                    return vec![index as i32, diff_index as i32];
                } else {
                    index += 1;
                }
            }
            panic!("Must have one solution")
        }
    }
}

/// AI-optimized solution — single-pass approach.
///
/// Key difference from my_solution:
///   my_solution builds the entire HashMap first, then loops to find a match
///   (two passes over the data). This solution does it in ONE pass: for each
///   element, check if its complement already exists in the map, then insert
///   the current element. This naturally avoids self-matching because the
///   current element hasn't been inserted yet when we check for its complement.
///
/// Time: O(n) — single iteration, each HashMap lookup is O(1) amortized.
/// Space: O(n) — HashMap stores up to n entries.
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            // Build the map as we go — only elements before the current index
            // are in the map, so we never match an element with itself.
            let mut seen: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

            for (i, &num) in nums.iter().enumerate() {
                let complement = target - num;

                // If the complement is already in the map, we found the pair.
                if let Some(&j) = seen.get(&complement) {
                    return vec![j, i as i32];
                }

                // Otherwise, record this number's index for future lookups.
                // Using i32 for the index so no `as` cast is needed on return.
                seen.insert(num, i as i32);
            }

            unreachable!("problem guarantees exactly one solution")
        }
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    println!("--- My Solution ---");
    let result = my_solution::Solution::two_sum(nums.clone(), target);
    println!("two_sum([2, 7, 11, 15], {target}) = {result:?}");

    println!("--- AI Solution ---");
    let result = ai_solution::Solution::two_sum(nums, target);
    println!("two_sum([2, 7, 11, 15], {target}) = {result:?}");
}

/// Macro that generates the same set of tests for any Solution type.
/// This avoids duplicating test cases — both modules run identical tests.
///
/// Usage: `test_two_sum!(test_module_name, path::to::Solution);`
macro_rules! test_two_sum {
    ($mod_name:ident, $solution:path) => {
        mod $mod_name {
            use $solution as Solution;

            #[test]
            fn test_basic() {
                assert_eq!(
                    Solution::two_sum(vec![2, 7, 11, 15], 9),
                    vec![0, 1]
                );
            }

            #[test]
            fn test_middle_pair() {
                assert_eq!(
                    Solution::two_sum(vec![3, 2, 4], 6),
                    vec![1, 2]
                );
            }

            #[test]
            fn test_same_element() {
                assert_eq!(
                    Solution::two_sum(vec![3, 3], 6),
                    vec![0, 1]
                );
            }

            #[test]
            fn test_negative_numbers() {
                assert_eq!(
                    Solution::two_sum(vec![-1, -2, -3, -4, -5], -8),
                    vec![2, 4]
                );
            }

            #[test]
            fn test_large_input() {
                // Answer is the last two elements — worst case for single-pass.
                let mut nums: Vec<i32> = (0..1000).collect();
                nums.push(5000);
                nums.push(5001);
                assert_eq!(
                    Solution::two_sum(nums, 10001),
                    vec![1000, 1001]
                );
            }
        }
    };
}

#[cfg(test)]
mod tests {
    // Run the SAME tests against both solutions.
    test_two_sum!(my_tests, crate::my_solution::Solution);
    test_two_sum!(ai_tests, crate::ai_solution::Solution);

    /// Benchmark both solutions side by side.
    ///
    /// Runs each solution many times on the same input and prints average
    /// duration. Use `cargo test --example lc_0001_two_sum bench -- --nocapture`
    /// to see the output.
    ///
    /// `std::hint::black_box` prevents the compiler from optimizing away
    /// the function call, ensuring we measure real execution time.
    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        // Larger input (1002 elements) — answer at the end to exercise full
        // iteration for both solutions, making the comparison fairer.
        let mut nums: Vec<i32> = (0..1000).collect();
        nums.push(5000);
        nums.push(5001);
        let target = 10001; // 5000 + 5001, indices [1000, 1001]

        // --- Benchmark my_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::two_sum(
                    std::hint::black_box(nums.clone()),
                    target,
                ),
            );
        }
        let my_duration = start.elapsed();

        // --- Benchmark ai_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::two_sum(
                    std::hint::black_box(nums.clone()),
                    target,
                ),
            );
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

        // Show which is faster and by how much.
        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64
                / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64
                / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
