// LeetCode 300: Longest Increasing Subsequence [Medium]
// https://leetcode.com/problems/longest-increasing-subsequence/
// Given an integer array nums, return the length of the longest strictly increasing subsequence.
//
// Benchmark: 100_000 iterations
// My solution (O(n^2) DP):          avg 0.112us
// AI solution (O(n log n) patience): avg 0.094us
// Winner: AI solution (1.20x faster)

// ------ My Solution: O(n^2) DP ------
// Time: O(n^2) — nested loops comparing all pairs
// Space: O(n) — dp array of length n
/// Uses classic DP where `f[i]` = length of LIS ending at index `i`.
/// For each `i`, check all `j < i` and if `nums[j] < nums[i]`, update
/// `f[i] = max(f[i], f[j] + 1)`. Return the max of `f`.
mod my_solution {
    pub struct Solution;

    impl Solution {
        pub fn length_of_lis(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            // f[i] = length of longest increasing subsequence ending at i
            let mut f = vec![1; n];
            for i in 1..n {
                for j in 0..i {
                    if nums[j] < nums[i] {
                        f[i] = f[i].max(f[j] + 1);
                    }
                }
            }
            // The answer is the maximum value in f
            *f.iter().max().unwrap()
        }
    }
}

// ------ AI Solution: Patience Sorting / Binary Search on Tails ------
// Time: O(n log n) — one pass with binary search per element
// Space: O(n) — tails array at most length n
/// Maintains a `tails` array where `tails[i]` is the smallest tail element
/// for an increasing subsequence of length `i+1`. For each number, binary
/// search for the position to replace (or extend). The length of `tails`
/// at the end is the LIS length.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        pub fn length_of_lis(nums: Vec<i32>) -> i32 {
            // tails[i] = smallest possible tail of increasing subsequence of length i+1
            let mut tails: Vec<i32> = Vec::new();
            for &x in &nums {
                // Find the first index in tails where tails[pos] >= x
                // using binary search (partition_point finds first where predicate is false)
                let pos = tails.partition_point(|&t| t < x);
                if pos == tails.len() {
                    // x is larger than all tails — extend the subsequence
                    tails.push(x);
                } else {
                    // Replace tails[pos] with x to keep tails as small as possible
                    tails[pos] = x;
                }
            }
            tails.len() as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    #[test]
    fn test_example1() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(MySolution::length_of_lis(nums.clone()), 4);
        assert_eq!(AiSolution::length_of_lis(nums), 4);
    }

    #[test]
    fn test_example2() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(MySolution::length_of_lis(nums.clone()), 4);
        assert_eq!(AiSolution::length_of_lis(nums), 4);
    }

    #[test]
    fn test_all_same() {
        let nums = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(MySolution::length_of_lis(nums.clone()), 1);
        assert_eq!(AiSolution::length_of_lis(nums), 1);
    }

    #[test]
    fn test_single() {
        assert_eq!(MySolution::length_of_lis(vec![5]), 1);
        assert_eq!(AiSolution::length_of_lis(vec![5]), 1);
    }

    #[test]
    fn test_increasing() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(MySolution::length_of_lis(nums.clone()), 5);
        assert_eq!(AiSolution::length_of_lis(nums), 5);
    }

    #[test]
    fn test_decreasing() {
        let nums = vec![5, 4, 3, 2, 1];
        assert_eq!(MySolution::length_of_lis(nums.clone()), 1);
        assert_eq!(AiSolution::length_of_lis(nums), 1);
    }

    #[test]
    fn test_two_elements_inc() {
        assert_eq!(MySolution::length_of_lis(vec![1, 2]), 2);
        assert_eq!(AiSolution::length_of_lis(vec![1, 2]), 2);
    }

    #[test]
    fn test_two_elements_dec() {
        assert_eq!(MySolution::length_of_lis(vec![2, 1]), 1);
        assert_eq!(AiSolution::length_of_lis(vec![2, 1]), 1);
    }

    #[test]
    fn test_negatives() {
        let nums = vec![-5, -3, -1, 0, 2];
        assert_eq!(MySolution::length_of_lis(nums.clone()), 5);
        assert_eq!(AiSolution::length_of_lis(nums), 5);
    }

    #[test]
    fn test_mixed() {
        let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
        // LIS: 1, 4, 5, 9 or 1, 2, 6 -> length 4
        assert_eq!(MySolution::length_of_lis(nums.clone()), 4);
        assert_eq!(AiSolution::length_of_lis(nums), 4);
    }

    #[test]
    fn test_duplicates_in_sequence() {
        // Strictly increasing, so duplicates don't extend
        let nums = vec![1, 3, 6, 7, 9, 4, 10, 5, 6];
        // LIS: 1, 3, 6, 7, 9, 10 -> length 6
        assert_eq!(MySolution::length_of_lis(nums.clone()), 6);
        assert_eq!(AiSolution::length_of_lis(nums), 6);
    }

    #[test]
    fn test_zigzag() {
        let nums = vec![1, 5, 2, 6, 3, 7];
        // LIS: 1, 2, 3, 7 or 1, 5, 6, 7 -> length 4
        assert_eq!(MySolution::length_of_lis(nums.clone()), 4);
        assert_eq!(AiSolution::length_of_lis(nums), 4);
    }
}

fn main() {
    println!("LeetCode 300: Longest Increasing Subsequence");

    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    println!(
        "Input: {:?}, My: {}, AI: {}",
        nums,
        my_solution::Solution::length_of_lis(nums.clone()),
        ai_solution::Solution::length_of_lis(nums.clone())
    );

    // Benchmark
    let iterations = 100_000;
    let test_nums = vec![10, 9, 2, 5, 3, 7, 101, 18];

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::Solution::length_of_lis(std::hint::black_box(
            test_nums.clone(),
        )));
    }
    let my_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::Solution::length_of_lis(std::hint::black_box(
            test_nums.clone(),
        )));
    }
    let ai_time = start.elapsed();

    let my_avg = my_time.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_time.as_nanos() as f64 / iterations as f64 / 1000.0;

    println!("\nBenchmark ({iterations} iterations):");
    println!("  My solution:  {my_avg:.3}us avg");
    println!("  AI solution:  {ai_avg:.3}us avg");
    if my_avg < ai_avg {
        println!("  Winner: My solution ({:.2}x faster)", ai_avg / my_avg);
    } else {
        println!("  Winner: AI solution ({:.2}x faster)", my_avg / ai_avg);
    }
}
