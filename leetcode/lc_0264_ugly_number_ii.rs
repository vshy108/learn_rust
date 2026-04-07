// LeetCode 264: Ugly Number II [Medium]
// https://leetcode.com/problems/ugly-number-ii/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 6.902us
//   AI solution avg: 113.697us
//   Ratio: My 16.47x faster
//
// An ugly number is a positive integer whose prime factors are limited to
// 2, 3, and 5. Given an integer n, return the nth ugly number.
//
// Example 1: n = 10 -> 12
//   [1, 2, 3, 4, 5, 6, 8, 9, 10, 12]
// Example 2: n = 1  -> 1
//
// Constraints:
//   - 1 <= n <= 1690
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0264.Ugly%20Number%20II/README_EN.md
//
// Run:  cargo run --example lc_0264_ugly_number_ii
// Test: cargo test --example lc_0264_ugly_number_ii
// Bench: cargo test --example lc_0264_ugly_number_ii bench -- --nocapture

/// My solution — three-pointer DP (from doocs Solution 2).
///
/// Maintain three pointers p2, p3, p5 into the dp array. The next ugly
/// number is min(dp[p2]*2, dp[p3]*3, dp[p5]*5). Advance whichever
/// pointer(s) produced the minimum (handles duplicates).
///
/// Time: O(n)
/// Space: O(n)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the nth ugly number.
        pub fn nth_ugly_number(n: i32) -> i32 {
            let n = n as usize;
            let mut dp = vec![0i32; n];
            dp[0] = 1;
            let (mut p2, mut p3, mut p5) = (0usize, 0usize, 0usize);
            for i in 1..n {
                let next2 = dp[p2] * 2;
                let next3 = dp[p3] * 3;
                let next5 = dp[p5] * 5;
                dp[i] = next2.min(next3).min(next5);
                // Advance all pointers that matched (avoids duplicates)
                if dp[i] == next2 { p2 += 1; }
                if dp[i] == next3 { p3 += 1; }
                if dp[i] == next5 { p5 += 1; }
            }
            dp[n - 1]
        }
    }
}

/// AI solution — min-heap with HashSet dedup (from doocs Solution 1).
///
/// Use a BinaryHeap (flipped to min-heap) with a HashSet to track seen
/// values. Pop smallest, multiply by 2/3/5, push unseen products.
///
/// Time: O(n log n)
/// Space: O(n)
mod ai_solution {
    use std::collections::{BinaryHeap, HashSet};
    use std::cmp::Reverse;

    pub struct Solution;

    impl Solution {
        /// Return the nth ugly number.
        pub fn nth_ugly_number(n: i32) -> i32 {
            let n = n as usize;
            let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
            let mut vis: HashSet<i64> = HashSet::new();
            heap.push(Reverse(1));
            vis.insert(1);
            let mut ans = 1i64;
            for _ in 0..n {
                ans = heap.pop().unwrap().0;
                for &factor in &[2i64, 3, 5] {
                    let nxt = ans * factor;
                    if vis.insert(nxt) {
                        heap.push(Reverse(nxt));
                    }
                }
            }
            ans as i32
        }
    }
}

fn main() {
    println!("my n=10: {}", my_solution::Solution::nth_ugly_number(10));
    println!("ai n=10: {}", ai_solution::Solution::nth_ugly_number(10));
}

#[cfg(test)]
mod tests {
    macro_rules! test_ugly_number_ii {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                #[test]
                fn test_example_1() {
                    assert_eq!(Sol::nth_ugly_number(10), 12);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Sol::nth_ugly_number(1), 1);
                }

                #[test]
                fn test_first_10() {
                    let expected = [1, 2, 3, 4, 5, 6, 8, 9, 10, 12];
                    for (i, &e) in expected.iter().enumerate() {
                        assert_eq!(
                            Sol::nth_ugly_number((i + 1) as i32),
                            e,
                            "nth_ugly_number({}) should be {}",
                            i + 1,
                            e
                        );
                    }
                }

                #[test]
                fn test_n_11_to_15() {
                    // 11th=15, 12th=16, 13th=18, 14th=20, 15th=24
                    assert_eq!(Sol::nth_ugly_number(11), 15);
                    assert_eq!(Sol::nth_ugly_number(15), 24);
                }

                #[test]
                fn test_large() {
                    // Known: 1690th ugly number = 2123366400
                    assert_eq!(Sol::nth_ugly_number(1690), 2_123_366_400);
                }
            }
        };
    }

    test_ugly_number_ii!(my, crate::my_solution::Solution);
    test_ugly_number_ii!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::nth_ugly_number(1690));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::nth_ugly_number(1690));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
