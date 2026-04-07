// LeetCode 276: Paint Fence [Medium] (Premium)
// https://leetcode.com/problems/paint-fence/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 0.000us
//   AI solution avg: 0.038us
//   Ratio: My 122.96x faster
//
// You are painting a fence of n posts with k different colors. You must paint
// the posts following these rules:
//   - Every post must be painted exactly one color.
//   - There cannot be three or more consecutive posts with the same color.
// Return the number of ways you can paint the fence.
//
// Example 1: n = 3, k = 2 -> 6
// Example 2: n = 1, k = 1 -> 1
// Example 3: n = 7, k = 2 -> 42
//
// Constraints:
//   - 1 <= n <= 50
//   - 1 <= k <= 10^5
//   - Answer fits in i32
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0276.Paint%20Fence/README_EN.md
//
// Run:  cargo run --example lc_0276_paint_fence
// Test: cargo test --example lc_0276_paint_fence
// Bench: cargo test --example lc_0276_paint_fence bench -- --nocapture

/// My solution — O(1) space DP (from doocs Solution 2).
///
/// Track two variables:
///   f = ways where last two posts are different colors
///   g = ways where last two posts are the same color
/// Transition: f' = (f + g) * (k - 1), g' = f
/// Answer: f + g
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        pub fn num_ways(n: i32, k: i32) -> i32 {
            let k = k as i64;
            let mut f = k;       // ways with last two different
            let mut g = 0i64;    // ways with last two same
            for _ in 1..n {
                let ff = (f + g) * (k - 1);
                g = f;
                f = ff;
            }
            (f + g) as i32
        }
    }
}

/// AI solution — Single recurrence f(n) = (k-1) * (f(n-1) + f(n-2)).
///
/// Combine both states into one: total[i] = total ways for i posts.
///   total[1] = k
///   total[2] = k * k  (k choices for each of 2 posts)
///   total[i] = (k-1) * (total[i-1] + total[i-2])
///
/// This merges the "same" and "different" transitions into one formula:
///   - total[i-1] * (k-1): paint post i a different color from post i-1
///   - total[i-2] * (k-1): paint post i same as i-1 but different from i-2
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        pub fn num_ways(n: i32, k: i32) -> i32 {
            if n == 1 {
                return k;
            }
            let k = k as i64;
            let mut prev2 = k;          // total[1] = k
            let mut prev1 = k * k;      // total[2] = k^2
            for _ in 3..=n {
                let cur = (k - 1) * (prev1 + prev2);
                prev2 = prev1;
                prev1 = cur;
            }
            prev1 as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_paint_fence {
        ($name:ident, $n:expr, $k:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    my_solution::Solution::num_ways($n, $k),
                    $expected,
                    "my_solution failed"
                );
                assert_eq!(
                    ai_solution::Solution::num_ways($n, $k),
                    $expected,
                    "ai_solution failed"
                );
            }
        };
    }

    // Examples
    test_paint_fence!(test_example_1, 3, 2, 6);
    test_paint_fence!(test_example_2, 1, 1, 1);
    test_paint_fence!(test_example_3, 7, 2, 42);
    // Edge: 1 post, many colors
    test_paint_fence!(test_one_post_many_colors, 1, 100, 100);
    // 2 posts, 2 colors = 4 ways (all pairs allowed for 2 posts)
    test_paint_fence!(test_two_posts_two_colors, 2, 2, 4);
    // 2 posts, 3 colors = 9
    test_paint_fence!(test_two_posts_three_colors, 2, 3, 9);
    // 3 posts, 1 color = 0 (can't avoid 3 consecutive same)
    test_paint_fence!(test_three_posts_one_color, 3, 1, 0);
    // 4 posts, 2 colors
    // f(1)=2, f(2)=4, f(3) = 1*(4+2)=6, f(4) = 1*(6+4) = 10
    test_paint_fence!(test_four_posts_two_colors, 4, 2, 10);
    // 3 posts, 3 colors
    // f(1)=3, f(2)=9, f(3)=2*(9+3)=24
    test_paint_fence!(test_three_posts_three_colors, 3, 3, 24);
    // 5 posts, 2 colors
    // f(5) = 1*(10+6) = 16
    test_paint_fence!(test_five_posts_two_colors, 5, 2, 16);
    // 1 post, k=1
    test_paint_fence!(test_one_one, 1, 1, 1);
    // 2 posts, k=1 → 1 way (same color)
    test_paint_fence!(test_two_one, 2, 1, 1);
    // Large k, small n
    test_paint_fence!(test_two_posts_large_k, 2, 100000, 1_410_065_408);
}

fn main() {
    println!("LeetCode 276: Paint Fence");
    for (n, k) in [(3, 2), (1, 1), (7, 2)] {
        println!(
            "  num_ways({}, {}) = {}",
            n,
            k,
            my_solution::Solution::num_ways(n, k)
        );
    }
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let n = 50;
        let k = 100;
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(my_solution::Solution::num_ways(n, k));
            std::hint::black_box(ai_solution::Solution::num_ways(n, k));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(my_solution::Solution::num_ways(n, k));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(ai_solution::Solution::num_ways(n, k));
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
