// LeetCode 456: 132 Pattern [Medium]
// https://leetcode.com/problems/132-pattern/
//
// Given an array of n integers nums, a 132 pattern is a subsequence
// nums[i] < nums[k] < nums[j] where i < j < k.
// Return true if such a pattern exists.
//
// Examples:
//   [1,2,3,4] -> false
//   [3,1,4,2] -> true  (1,4,2)
//   [-1,3,2,0] -> true  (-1,3,2)
//
// Constraints:
//   n == nums.length, 1 <= n <= 2 * 10^5
//   -10^9 <= nums[i] <= 10^9
//
// Run:  cargo run --example lc_0456_132_pattern
// Test: cargo test --example lc_0456_132_pattern
// Bench: cargo test --example lc_0456_132_pattern bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 63ns
//   AI solution: avg 64ns
//   Ratio: 0.98x (same algorithm)

/// My solution module — monotone stack from right.
///
/// Scan right to left, maintaining a decreasing stack. Track the largest
/// popped value as the "3" candidate (vk). If any element is less than vk,
/// we found a "1" and the pattern exists.
///
/// Time:  O(n)
/// Space: O(n)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn find132pattern(nums: Vec<i32>) -> bool {
            // Monotone stack from right: track max "two" value
            let mut stack: Vec<i32> = Vec::new();
            let mut two = i32::MIN; // the "2" in 1-3-2
            for &num in nums.iter().rev() {
                if num < two {
                    return true;
                } // found "1" < "2"
                while let Some(&top) = stack.last() {
                    if top >= num {
                        break;
                    }
                    two = stack.pop().expect("non-empty stack");
                }
                stack.push(num);
            }
            false
        }
    }
}

/// AI solution module — monotone stack (doocs Rust approach).
///
/// Time:  O(n)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn find132pattern(nums: Vec<i32>) -> bool {
            let n = nums.len();
            // vk tracks the largest value popped from the stack
            // (the "2" in the 1-3-2 pattern, which is the "k" element)
            let mut vk = i32::MIN;
            let mut stk: Vec<i32> = Vec::new();
            for i in (0..n).rev() {
                // If current element is less than vk, we found the "1"
                if nums[i] < vk {
                    return true;
                }
                // Pop smaller elements: they become candidates for "k" (the "2")
                while let Some(&top) = stk.last() {
                    if top < nums[i] {
                        vk = stk.pop().expect("stack is non-empty");
                    } else {
                        break;
                    }
                }
                stk.push(nums[i]);
            }
            false
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_no_pattern() {
            assert!(!Solution::find132pattern(vec![1, 2, 3, 4]));
        }
        #[test]
        fn test_has_pattern() {
            assert!(Solution::find132pattern(vec![3, 1, 4, 2]));
        }
        #[test]
        fn test_negative() {
            assert!(Solution::find132pattern(vec![-1, 3, 2, 0]));
        }
        #[test]
        fn test_single() {
            assert!(!Solution::find132pattern(vec![1]));
        }
        #[test]
        fn test_two_elements() {
            assert!(!Solution::find132pattern(vec![1, 2]));
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
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<i32>) -> bool,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(vec![3, 1, 4, 2])));
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
    bench_solution("My solution", my_solution::Solution::find132pattern);
    bench_solution("AI solution", ai_solution::Solution::find132pattern);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::find132pattern(vec![3, 1, 4, 2])
    );
}
