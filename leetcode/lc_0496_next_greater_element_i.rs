// LeetCode 496: Next Greater Element I [Easy]
// https://leetcode.com/problems/next-greater-element-i/
//
// The next greater element of some element x in an array is the first greater
// element that is to the right of x in the same array.
// Given two distinct integer arrays nums1 (a subset of nums2), for each
// element in nums1, find its next greater element in nums2. Return -1 if
// it doesn't exist.
//
// Examples:
//   nums1 = [4,1,2], nums2 = [1,3,4,2] -> [-1,3,-1]
//   nums1 = [2,4], nums2 = [1,2,3,4]   -> [3,-1]
//
// Constraints:
//   1 <= nums1.len() <= nums2.len() <= 1000
//   0 <= nums1[i], nums2[i] <= 10^4
//   All values in nums1 and nums2 are unique
//   All values of nums1 also appear in nums2
//
// Run:  cargo run --example lc_0496_next_greater_element_i
// Test: cargo test --example lc_0496_next_greater_element_i
// Bench: cargo test --example lc_0496_next_greater_element_i bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 118ns (brute force O(n*m))
//   AI solution: avg 538ns (monotone stack + HashMap)
//   Ratio: 0.22x (brute force faster on small input)

/// My solution module — monotone stack + HashMap.
///
/// Time:  O(m + n)
/// Space: O(n)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
            // Brute force: for each num in nums1, scan right in nums2
            nums1
                .iter()
                .map(|&target| {
                    let pos = nums2.iter().position(|&x| x == target).unwrap();
                    nums2[pos + 1..]
                        .iter()
                        .find(|&&x| x > target)
                        .copied()
                        .unwrap_or(-1)
                })
                .collect()
        }
    }
}

/// AI solution module — use a monotone decreasing stack to find the next
/// greater element for each value in nums2, store in a HashMap, then look
/// up for each element in nums1.
///
/// Time:  O(m + n)
/// Space: O(n)
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;
    impl Solution {
        pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
            let mut next_greater: HashMap<i32, i32> = HashMap::new();
            let mut stack: Vec<i32> = Vec::new();

            for &num in &nums2 {
                // Pop all elements smaller than current — their next greater is num
                while let Some(&top) = stack.last() {
                    if top >= num {
                        break;
                    }
                    stack.pop();
                    next_greater.insert(top, num);
                }
                stack.push(num);
            }

            nums1
                .iter()
                .map(|x| *next_greater.get(x).unwrap_or(&-1))
                .collect()
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_example1() {
            assert_eq!(
                Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
                vec![-1, 3, -1]
            );
        }
        #[test]
        fn test_example2() {
            assert_eq!(
                Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
                vec![3, -1]
            );
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
    F: Fn(Vec<i32>, Vec<i32>) -> Vec<i32>,
{
    let iterations = 500_000;
    let nums1 = vec![4, 1, 2];
    let nums2 = vec![1, 3, 4, 2, 5, 6, 7, 8, 9, 10];
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(nums1.clone()),
            std::hint::black_box(nums2.clone()),
        ));
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
    bench_solution("My solution", my_solution::Solution::next_greater_element);
    bench_solution("AI solution", ai_solution::Solution::next_greater_element);
}

fn main() {
    println!(
        "{:?}",
        ai_solution::Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
    );
}
