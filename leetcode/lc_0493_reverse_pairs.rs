// LeetCode 493: Reverse Pairs [Hard]
// https://leetcode.com/problems/reverse-pairs/
//
// Given an integer array nums, return the number of reverse pairs.
// A reverse pair is a pair (i, j) where 0 <= i < j < nums.len()
// and nums[i] > 2 * nums[j].
//
// Examples:
//   [1,3,2,3,1] -> 2  ((3,1) and (3,1))
//   [2,4,3,5,1] -> 3
//
// Constraints:
//   1 <= nums.len() <= 5 * 10^4
//   -2^31 <= nums[i] <= 2^31 - 1
//
// Run:  cargo run --example lc_0493_reverse_pairs
// Test: cargo test --example lc_0493_reverse_pairs
// Bench: cargo test --example lc_0493_reverse_pairs bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 455ns
//   AI solution: avg 450ns
//   Ratio: 1.01x (same algorithm)

/// My solution module — merge sort.
///
/// Time:  O(n log n)
/// Space: O(n)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n <= 1 {
                return 0;
            }
            fn merge_sort(nums: &mut Vec<i32>, l: usize, r: usize) -> usize {
                if l >= r {
                    return 0;
                }
                let mid = l + (r - l) / 2;
                let mut count = merge_sort(nums, l, mid) + merge_sort(nums, mid + 1, r);
                let mut j = mid + 1;
                for i in l..=mid {
                    while j <= r && (nums[i] as i64) > 2 * (nums[j] as i64) {
                        j += 1;
                    }
                    count += j - (mid + 1);
                }
                let mut temp = Vec::with_capacity(r - l + 1);
                let (mut i, mut j2) = (l, mid + 1);
                while i <= mid && j2 <= r {
                    if nums[i] <= nums[j2] {
                        temp.push(nums[i]);
                        i += 1;
                    } else {
                        temp.push(nums[j2]);
                        j2 += 1;
                    }
                }
                while i <= mid {
                    temp.push(nums[i]);
                    i += 1;
                }
                while j2 <= r {
                    temp.push(nums[j2]);
                    j2 += 1;
                }
                nums[l..=r].copy_from_slice(&temp);
                count
            }
            merge_sort(&mut nums, 0, n - 1) as i32
        }
    }
}

/// AI solution module — modified merge sort.
///
/// During the merge step, before merging, count pairs where left[i] > 2*right[j]
/// using two pointers. Then merge normally.
///
/// Time:  O(n log n)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n <= 1 {
                return 0;
            }
            Self::merge_sort(&mut nums, 0, n - 1) as i32
        }

        fn merge_sort(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
            if left >= right {
                return 0;
            }
            let mid = left + (right - left) / 2;
            let mut count =
                Self::merge_sort(nums, left, mid) + Self::merge_sort(nums, mid + 1, right);

            // Count reverse pairs: nums[i] > 2 * nums[j]
            // Use i64 to avoid overflow on 2 * nums[j]
            let mut j = mid + 1;
            for i in left..=mid {
                while j <= right && (nums[i] as i64) > 2 * (nums[j] as i64) {
                    j += 1;
                }
                count += j - (mid + 1);
            }

            // Standard merge
            let mut temp = Vec::with_capacity(right - left + 1);
            let (mut i, mut j) = (left, mid + 1);
            while i <= mid && j <= right {
                if nums[i] <= nums[j] {
                    temp.push(nums[i]);
                    i += 1;
                } else {
                    temp.push(nums[j]);
                    j += 1;
                }
            }
            while i <= mid {
                temp.push(nums[i]);
                i += 1;
            }
            while j <= right {
                temp.push(nums[j]);
                j += 1;
            }

            nums[left..=right].copy_from_slice(&temp);
            count
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_example1() {
            assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
        }
        #[test]
        fn test_example2() {
            assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
        }
        #[test]
        fn test_empty() {
            assert_eq!(Solution::reverse_pairs(vec![1]), 0);
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
    F: Fn(Vec<i32>) -> i32,
{
    let iterations = 500_000;
    let input = vec![5, 4, 3, 2, 1, 0, -1, -2, -3, -4];
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(input.clone())));
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
    bench_solution("My solution", my_solution::Solution::reverse_pairs);
    bench_solution("AI solution", ai_solution::Solution::reverse_pairs);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::reverse_pairs(vec![1, 3, 2, 3, 1])
    );
}
