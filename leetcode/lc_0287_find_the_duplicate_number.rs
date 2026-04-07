// LeetCode 287: Find the Duplicate Number [Medium]
// https://leetcode.com/problems/find-the-duplicate-number/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 2.422us
//   AI solution avg: 1.471us
//   Ratio: AI 1.65x faster
//
// Given an array of integers nums containing n + 1 integers where each integer
// is in [1, n] inclusive. There is only one repeated number in nums, return it.
//
// Must solve WITHOUT modifying the array and using only O(1) extra space.
//
// Example 1: nums = [1,3,4,2,2] -> 2
// Example 2: nums = [3,1,3,4,2] -> 3
// Example 3: nums = [3,3,3,3,3] -> 3
//
// Constraints:
//   - 1 <= n <= 10^5
//   - nums.length == n + 1
//   - 1 <= nums[i] <= n
//   - All integers appear once except one which appears 2+ times
//
// Run:  cargo run --example lc_0287_find_the_duplicate_number
// Test: cargo test --example lc_0287_find_the_duplicate_number
// Bench: cargo test --example lc_0287_find_the_duplicate_number bench -- --nocapture

struct Solution;

/// My solution — Floyd's cycle detection (tortoise and hare).
///
/// Treat the array as a linked list where index i points to nums[i].
/// Since there's a duplicate, there must be a cycle. Use slow/fast
/// pointers to find the cycle entry point, which is the duplicate.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    use super::Solution;

    impl Solution {
        pub fn find_duplicate_my(nums: &[i32]) -> i32 {
            // Phase 1: find meeting point inside the cycle
            let mut slow = nums[0] as usize;
            let mut fast = nums[0] as usize;
            loop {
                slow = nums[slow] as usize;
                fast = nums[nums[fast] as usize] as usize;
                if slow == fast {
                    break;
                }
            }
            // Phase 2: find cycle entry (the duplicate value)
            let mut finder = nums[0] as usize;
            while finder != slow {
                finder = nums[finder] as usize;
                slow = nums[slow] as usize;
            }
            finder as i32
        }
    }
}

/// AI solution — Binary search on value range (doocs Rust).
///
/// Binary search on the answer space [1, n]. For a candidate mid,
/// count how many numbers in nums are <= mid. If count > mid, the
/// duplicate must be in [1, mid]. Otherwise it's in [mid+1, n].
///
/// Time:  O(n log n)
/// Space: O(1)
mod ai_solution {
    use super::Solution;

    impl Solution {
        pub fn find_duplicate_ai(nums: &[i32]) -> i32 {
            let mut left: usize = 1;
            let mut right: usize = nums.len() - 1; // n = nums.len() - 1
            while left < right {
                let mid = (left + right) >> 1;
                // Count elements in nums that are <= mid
                let cnt = nums.iter().filter(|&&x| x <= mid as i32).count();
                if cnt > mid {
                    right = mid; // duplicate in [left, mid]
                } else {
                    left = mid + 1; // duplicate in [mid+1, right]
                }
            }
            left as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_dup {
        ($name:ident, $nums:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Solution::find_duplicate_my(&$nums),
                    $expected,
                    "my_solution failed"
                );
                assert_eq!(
                    Solution::find_duplicate_ai(&$nums),
                    $expected,
                    "ai_solution failed"
                );
            }
        };
    }

    test_dup!(test_example_1, [1, 3, 4, 2, 2], 2);
    test_dup!(test_example_2, [3, 1, 3, 4, 2], 3);
    test_dup!(test_example_3, [3, 3, 3, 3, 3], 3);

    // Duplicate at the start
    test_dup!(test_dup_at_start, [2, 2, 1, 3, 4], 2);

    // Minimum size: n=1, nums has 2 elements
    test_dup!(test_min_size, [1, 1], 1);

    // Duplicate is1, in a larger array
    test_dup!(test_dup_is_one, [1, 1, 2, 3, 4, 5], 1);

    // Duplicate is n
    test_dup!(test_dup_is_n, [1, 2, 3, 4, 5, 5], 5);

    // Duplicate appears more than twice
    test_dup!(test_triple, [2, 2, 2, 1, 3], 2);

    // Sequential with dup in middle
    test_dup!(test_middle, [1, 2, 3, 4, 3], 3);

    // Reversed with dup
    test_dup!(test_reversed, [4, 3, 2, 1, 4], 4);

    // Two elements reversed
    test_dup!(test_two_elem, [2, 1, 2], 2);
}

fn main() {
    println!("LeetCode 287: Find the Duplicate Number");
    let nums = vec![1, 3, 4, 2, 2];
    println!(
        "my_solution:  find_duplicate({:?}) = {}",
        nums,
        Solution::find_duplicate_my(&nums)
    );
    println!(
        "ai_solution:  find_duplicate({:?}) = {}",
        nums,
        Solution::find_duplicate_ai(&nums)
    );
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        // Build array [1, 2, ..., 999, 500] (n=999, duplicate=500)
        let mut nums: Vec<i32> = (1..=999).collect();
        nums.push(500);
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(Solution::find_duplicate_my(&nums));
            std::hint::black_box(Solution::find_duplicate_ai(&nums));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::find_duplicate_my(&nums));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::find_duplicate_ai(&nums));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ratio = if my_avg < ai_avg {
            format!("My {:.2}x faster", ai_avg / my_avg)
        } else {
            format!("AI {:.2}x faster", my_avg / ai_avg)
        };

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution avg: {my_avg:.3}us");
        println!("AI solution avg: {ai_avg:.3}us");
        println!("Ratio: {ratio}");
    }
}
