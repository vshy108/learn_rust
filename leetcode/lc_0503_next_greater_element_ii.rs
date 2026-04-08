// LeetCode 503: Next Greater Element II [Medium]
// https://leetcode.com/problems/next-greater-element-ii/
//
// Given a circular integer array nums (the next element of nums[n-1] is
// nums[0]), return the next greater number for every element in nums.
// The next greater number of x is the first greater number encountered by
// traversing circularly. If it doesn't exist, return -1.
//
// Examples:
//   [1,2,1]     -> [2,-1,2]
//   [1,2,3,4,3] -> [2,3,4,-1,4]
//
// Constraints:
//   1 <= nums.length <= 10^4
//   -10^9 <= nums[i] <= 10^9
//
// Run:  cargo run --example lc_0503_next_greater_element_ii
// Test: cargo test --example lc_0503_next_greater_element_ii
// Bench: cargo test --example lc_0503_next_greater_element_ii bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 578ns
//   AI solution: avg 385ns
//   AI is 1.50x faster
//   Reason: forward index-based stack only pushes in the first pass (i < n),
//   resulting in fewer stack operations than the backward value-based approach.

/// My solution — monotonic stack traversing backwards through doubled array.
///
/// Iterate from index 2n-1 down to 0, using modular indexing for the circular
/// property. Maintain a stack of values (not indices) in decreasing order.
/// Translated from Java solution in doocs/leetcode.
///
/// Time:  O(n)
/// Space: O(n)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the next greater element for each position in the circular array.
        ///
        /// # Arguments
        /// * `nums` - circular integer array
        ///
        /// # Returns
        /// Vec where ans[i] is the next greater element for nums[i], or -1
        pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
            let n = nums.len();
            let mut ans = vec![-1i32; n];
            let mut stk: Vec<i32> = Vec::new();

            // Traverse backwards through the "doubled" array
            for i in (0..n * 2).rev() {
                let j = i % n;
                // Pop elements that are <= current (they can't be "next greater")
                while let Some(&top) = stk.last() {
                    if top <= nums[j] {
                        stk.pop();
                    } else {
                        break;
                    }
                }
                if let Some(&top) = stk.last() {
                    ans[j] = top;
                }
                stk.push(nums[j]);
            }
            ans
        }
    }
}

/// AI solution — forward monotonic stack using index-based approach.
///
/// Iterate forward through 2n elements. The stack stores indices (not values).
/// When we find a larger element, we pop and assign.
///
/// Time:  O(n)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the next greater element using a forward index-based stack.
        ///
        /// # Arguments
        /// * `nums` - circular integer array
        ///
        /// # Returns
        /// Vec where ans[i] is the next greater element for nums[i], or -1
        pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
            let n = nums.len();
            let mut ans = vec![-1i32; n];
            // Stack stores indices whose next-greater hasn't been found yet
            let mut stk: Vec<usize> = Vec::new();

            for i in 0..n * 2 {
                let j = i % n;
                // Current element is "next greater" for everything on the stack
                // that is smaller
                while let Some(&top_idx) = stk.last() {
                    if nums[top_idx] < nums[j] {
                        stk.pop();
                        ans[top_idx] = nums[j];
                    } else {
                        break;
                    }
                }
                // Only push actual indices (first pass), not the duplicate pass
                if i < n {
                    stk.push(j);
                }
            }
            ans
        }
    }
}

fn main() {
    let nums = vec![1, 2, 1];
    let my = my_solution::Solution::next_greater_elements(nums.clone());
    let ai = ai_solution::Solution::next_greater_elements(nums);
    println!("my: {my:?}");
    println!("ai: {ai:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_nge {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::next_greater_elements(vec![1, 2, 1]),
                        vec![2, -1, 2]
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::next_greater_elements(vec![1, 2, 3, 4, 3]),
                        vec![2, 3, 4, -1, 4]
                    );
                }

                #[test]
                fn test_single() {
                    assert_eq!(<$solution>::next_greater_elements(vec![1]), vec![-1]);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(
                        <$solution>::next_greater_elements(vec![3, 3, 3]),
                        vec![-1, -1, -1]
                    );
                }

                #[test]
                fn test_descending() {
                    assert_eq!(
                        <$solution>::next_greater_elements(vec![5, 4, 3, 2, 1]),
                        vec![-1, 5, 5, 5, 5]
                    );
                }
            }
        };
    }

    test_nge!(my, crate::my_solution::Solution);
    test_nge!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 500_000;
        let nums: Vec<i32> = (1..=100).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::next_greater_elements(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::next_greater_elements(
                std::hint::black_box(nums.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
