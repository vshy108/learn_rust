// LeetCode 319. Bulb Switcher
// https://leetcode.com/problems/bulb-switcher/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 0.001us
//   ai_solution  avg: 0.033us
//   Ratio: my_solution 42.33x faster

/// There are n bulbs, initially all off. In round i, toggle every i-th bulb.
/// Return how many bulbs are on after n rounds.
///
/// Key insight: bulb i is toggled once per divisor of i.
/// Only perfect squares have an odd number of divisors, so answer = floor(sqrt(n)).
///
/// Example 1: n=3, Output: 1
/// Example 2: n=0, Output: 0
/// Example 3: n=1, Output: 1
///
/// Constraints: 0 <= n <= 10^9

/// # my_solution: Integer square root via f64 sqrt
/// Time: O(1)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count bulbs that remain on after n rounds.
        ///
        /// Only bulbs at perfect-square positions stay on, because only
        /// perfect squares have an odd number of divisors.
        ///
        /// # Arguments
        /// * `n` - Number of bulbs
        ///
        /// # Returns
        /// Number of bulbs that are on = floor(sqrt(n))
        pub fn bulb_switch(n: i32) -> i32 {
            (n as f64).sqrt() as i32
        }
    }
}

/// # ai_solution: Integer square root via binary search
/// Time: O(log n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count bulbs on using integer binary search for sqrt.
        ///
        /// Avoids floating-point imprecision by using binary search
        /// to find the largest k where k*k <= n.
        ///
        /// # Arguments
        /// * `n` - Number of bulbs
        ///
        /// # Returns
        /// floor(sqrt(n))
        pub fn bulb_switch(n: i32) -> i32 {
            if n <= 0 {
                return 0;
            }
            let n = n as i64;
            let (mut lo, mut hi) = (1i64, n.min(46341)); // sqrt(2^31) < 46341
            while lo < hi {
                let mid = lo + (hi - lo + 1) / 2;
                if mid * mid <= n {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
            lo as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn run_tests(solve: fn(i32) -> i32) {
        assert_eq!(solve(3), 1);
        assert_eq!(solve(0), 0);
        assert_eq!(solve(1), 1);
        assert_eq!(solve(4), 2);   // sqrt(4)=2
        assert_eq!(solve(9), 3);   // sqrt(9)=3
        assert_eq!(solve(10), 3);  // sqrt(10)≈3.16 -> 3
        assert_eq!(solve(100), 10);
        assert_eq!(solve(2), 1);
        assert_eq!(solve(1_000_000_000), 31622);
    }

    #[test]
    fn test_my_solution() {
        run_tests(MySolution::bulb_switch);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(AiSolution::bulb_switch);
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let n = 3;
    println!("Input: n = {n}");
    println!("Output (my): {}", my_solution::Solution::bulb_switch(n));
    println!("Output (ai): {}", ai_solution::Solution::bulb_switch(n));

    let iterations = 100_000;
    let test_n = 1_000_000_000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(my_solution::Solution::bulb_switch(black_box(test_n)));
    }
    let my_dur = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(ai_solution::Solution::bulb_switch(black_box(test_n)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    println!("\nBenchmark ({iterations} iterations, n={test_n}, release mode):");
    println!("  my_solution  avg: {my_avg:.3}us");
    println!("  ai_solution  avg: {ai_avg:.3}us");
    if my_avg < ai_avg {
        println!("  my_solution is {:.2}x faster", ai_avg / my_avg);
    } else {
        println!("  ai_solution is {:.2}x faster", my_avg / ai_avg);
    }
}
