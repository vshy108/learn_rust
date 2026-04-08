// LeetCode 275: H-Index II [Medium]
// https://leetcode.com/problems/h-index-ii/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 0.048us
//   AI solution avg: 0.046us
//   Ratio: AI 1.04x faster
//
// Given an array of integers citations sorted in ascending order where
// citations[i] is the number of citations a researcher received for their
// i-th paper, return the researcher's h-index. Must run in O(log n) time.
//
// Example 1: citations = [0,1,3,5,6] -> 3
// Example 2: citations = [1,2,100]   -> 2
//
// Constraints:
//   - n == citations.length
//   - 1 <= n <= 10^5
//   - 0 <= citations[i] <= 1000
//   - citations is sorted in ascending order
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0275.H-Index%20II/README_EN.md
//
// Run:  cargo run --example lc_0275_h_index_ii
// Test: cargo test --example lc_0275_h_index_ii
// Bench: cargo test --example lc_0275_h_index_ii bench -- --nocapture

/// My solution — Binary search on h (from doocs Rust).
///
/// Binary search for the largest h such that citations[n-h] >= h.
/// Since citations is sorted ascending, citations[n-h] is the h-th
/// largest value. We search the range [0, n] for h.
///
/// Time:  O(log n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        pub fn h_index(citations: Vec<i32>) -> i32 {
            let n = citations.len();
            let (mut left, mut right) = (0, n);
            while left < right {
                let mid = (left + right + 1) >> 1;
                // citations[n - mid] is the mid-th largest citation count
                if citations[n - mid] >= mid as i32 {
                    left = mid; // mid is feasible, try larger
                } else {
                    right = mid - 1;
                }
            }
            left as i32
        }
    }
}

/// AI solution — Binary search on index (leftmost where citations[i] >= n-i).
///
/// Instead of searching h directly, search for the leftmost index i where
/// citations[i] >= n - i (meaning there are n-i papers with >= citations[i]
/// citations). The answer is n - i.
///
/// Time:  O(log n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        pub fn h_index(citations: Vec<i32>) -> i32 {
            let n = citations.len();
            let (mut lo, mut hi) = (0usize, n);
            // Find leftmost index where citations[mid] >= n - mid
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if citations[mid] >= (n - mid) as i32 {
                    hi = mid; // might be answer, try further left
                } else {
                    lo = mid + 1;
                }
            }
            // lo is the leftmost index with citations[lo] >= n - lo
            (n - lo) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_hindex {
        ($name:ident, $citations:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    my_solution::Solution::h_index($citations.to_vec()),
                    $expected,
                    "my_solution failed"
                );
                assert_eq!(
                    ai_solution::Solution::h_index($citations.to_vec()),
                    $expected,
                    "ai_solution failed"
                );
            }
        };
    }

    // Example 1
    test_hindex!(test_example_1, [0, 1, 3, 5, 6], 3);
    // Example 2
    test_hindex!(test_example_2, [1, 2, 100], 2);
    // All zeros
    test_hindex!(test_all_zeros, [0, 0, 0], 0);
    // Single paper, cited
    test_hindex!(test_single_cited, [10], 1);
    // Single paper, not cited
    test_hindex!(test_single_zero, [0], 0);
    // All same, h = n
    test_hindex!(test_all_same_high, [5, 5, 5, 5, 5], 5);
    // Ascending 0..6
    test_hindex!(test_ascending, [0, 1, 2, 3, 4, 5, 6], 3);
    // All ones (n=3, h=1)
    test_hindex!(test_all_ones, [1, 1, 1], 1);
    // Two papers both highly cited
    test_hindex!(test_two_high, [11, 15], 2);
    // Large jump at end
    test_hindex!(test_large_jump, [0, 0, 0, 0, 1000], 1);
    // Increasing sequence exactly matching
    test_hindex!(test_exact_match, [0, 1, 4, 5, 6], 3);
    // All 1000
    test_hindex!(test_max_citations, [1000, 1000, 1000], 3);
    // h = 0
    test_hindex!(test_h_zero, [0], 0);
}

fn main() {
    println!("LeetCode 275: H-Index II");
    let citations = vec![0, 1, 3, 5, 6];
    println!(
        "my_solution:  h_index({:?}) = {}",
        citations,
        my_solution::Solution::h_index(citations.clone())
    );
    let result = ai_solution::Solution::h_index(citations);
    println!("ai_solution:  h_index([0,1,3,5,6]) = {}", result);
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let citations: Vec<i32> = (0..100).collect();
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(my_solution::Solution::h_index(citations.clone()));
            std::hint::black_box(ai_solution::Solution::h_index(citations.clone()));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(my_solution::Solution::h_index(citations.clone()));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(ai_solution::Solution::h_index(citations.clone()));
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
