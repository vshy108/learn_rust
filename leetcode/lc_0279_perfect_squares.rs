// LeetCode 279: Perfect Squares [Medium]
// https://leetcode.com/problems/perfect-squares/
//
// Given an integer n, return the least number of perfect square numbers
// that sum to n.
//
// A perfect square is an integer that is the square of an integer; in
// other words, it is the product of some integer with itself. For
// example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.
//
// Examples:
//   n = 12 -> 3  (12 = 4 + 4 + 4)
//   n = 13 -> 2  (13 = 4 + 9)
//
// Constraints:
//   - 1 <= n <= 10^4
//
// Run:  cargo run --example lc_0279_perfect_squares
// Test: cargo test --example lc_0279_perfect_squares
// Bench: cargo test --example lc_0279_perfect_squares bench -- --nocapture
//
// --- Benchmark (10_000 iterations, 2026-04-06) ---
//   My solution: 3.78s (avg 377.7us)
//   AI solution: 897us (avg 89ns)
//   -> AI solution is 4209x faster
//   Reason: DP is O(n * sqrt(n)) per call — rebuilds the entire table
//     each time. The math-based approach uses Lagrange's four-square
//     theorem + Legendre's three-square theorem to answer in O(sqrt(n))
//     with no allocation. For n=999 the DP does ~31k iterations vs ~31
//     for the math check.
//
// --- Rust concepts used ---
//   - `(1..).take_while(|&i| i * i <= n)`: infinite range (1..) with
//     take_while to generate perfect squares. Ruby has no infinite range
//     with lazy termination built-in like this.
//   - Ruby `[]` vs Rust `[]`: Ruby's `[]` is a growable Array; Rust's
//     `[]` is a fixed-size array — can't push. Use `Vec::new()` or
//     iterator `.collect()`.
//   - `for i in 1..=n as usize`: cast on range bound, not loop variable.
//     `for i as usize in ...` won't compile — Rust's for-loop variable
//     is a pattern (binding), not an expression.
//   - `for &sq in &squares`: borrow Vec (not consume) + destructure &i32.
//     Ruby `.each` borrows implicitly; Rust `for sq in vec` MOVES the Vec.
//   - `saturating_add(1)`: prevents i32::MAX + 1 overflow/panic.
//   - Closure `let is_square = |x: i32| -> bool { ... }`: local helper
//     with explicit type annotations.
//   (Previously covered: `vec![val; n]` init -> lc_0006,
//    `.take_while()` -> lc_0008, `as` casting -> lc_0004,
//    closures -> lc_0005, `i32::MAX` sentinel -> lc_0004)

/// My solution module.
///
/// DP approach: for each value 1..=n, try subtracting each perfect
/// square and take the min.
///
/// Time:  O(n * sqrt(n)) — n values, each checking up to sqrt(n) squares
/// Space: O(n) — dp array of size n+1
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the least number of perfect squares that sum to `n`.
        ///
        /// # Arguments
        ///
        /// * `n` - The target sum
        ///
        /// # Returns
        ///
        /// Minimum count of perfect squares summing to `n`
        pub fn num_squares(n: i32) -> i32 {
            let mut dp = vec![i32::MAX; (n + 1) as usize];
            dp[0] = 0;
            // Ruby: squares = []; i = 1; while i*i <= n: squares << i*i; i += 1; end
            // Rust: Ruby's [] is a growable array, but Rust's [] is a fixed-size
            // array — can't push to it. Use Vec via iterator + collect instead.
            let squares: Vec<i32> = (1..)
                .take_while(|&i| i * i <= n)
                .map(|i| i * i)
                .collect();
            

            // `1..=n as usize` makes the range usize so `i` is usize for dp[i].
            // `for i as usize in 1..=n` won't compile — Rust's for-loop
            // variable is a pattern (binding), not an expression; casts
            // aren't allowed there.
            for i in 1..=n as usize {
                // Ruby: squares.each { |sq| ... } borrows implicitly.
                // Rust: `for sq in squares` MOVES (consumes) the Vec — the
                // second outer loop iteration would fail because squares is gone.
                // `for &sq in &squares` borrows the Vec and destructures &i32 to i32.
                for &sq in &squares {
                    let sq = sq as usize;
                    if sq > i { break; };
                    // saturating_add(1): if dp[i-sq] is i32::MAX (unreachable),
                    // i32::MAX + 1 would overflow/panic. saturating_add caps at MAX.
                    dp[i] = dp[i].min(dp[i - sq].saturating_add(1));
                }
            }
            

            dp[n as usize]
        }
    }
}

/// AI solution module.
///
/// Math-based approach using number theory:
/// - Answer is 1 if n is a perfect square.
/// - Answer is 4 if n = 4^a * (8b + 7) (Legendre's three-square theorem:
///   n CANNOT be expressed as sum of 3 squares iff this form holds).
/// - Answer is 2 if n = a^2 + b^2 for some integers a, b.
/// - Otherwise answer is 3.
///
/// Time:  O(sqrt(n)) — check perfect square + two-square decomposition
/// Space: O(1) — only scalar variables
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the least number of perfect squares that sum to `n`.
        ///
        /// # Arguments
        ///
        /// * `n` - The target sum
        ///
        /// # Returns
        ///
        /// Minimum count of perfect squares summing to `n`
        pub fn num_squares(n: i32) -> i32 {
            // Helper: check if x is a perfect square
            let is_square = |x: i32| -> bool {
                let s = (x as f64).sqrt() as i32;
                s * s == x
            };

            // 1: n itself is a perfect square
            if is_square(n) {
                return 1;
            }

            // 4: Legendre's three-square theorem — n cannot be written as
            // sum of 3 squares iff n = 4^a * (8b + 7). Remove all factors
            // of 4, then check if remainder % 8 == 7.
            let mut m = n;
            while m % 4 == 0 {
                m /= 4;
            }
            if m % 8 == 7 {
                return 4;
            }

            // 2: try all decompositions n = a^2 + b^2
            let mut a = 1;
            while a * a < n {
                if is_square(n - a * a) {
                    return 2;
                }
                a += 1;
            }

            // 3: by elimination (Lagrange guarantees <= 4)
            3
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, i32)> = vec![(12, 3), (13, 2)];

    println!("--- My Solution ---");
    for &(n, expected) in &test_cases {
        let result = my_solution::Solution::num_squares(n);
        println!("  n={n} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for &(n, expected) in &test_cases {
        let result = ai_solution::Solution::num_squares(n);
        println!("  n={n} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_num_squares {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_twelve() {
                    assert_eq!(Solution::num_squares(12), 3); // 4+4+4
                }

                #[test]
                fn test_thirteen() {
                    assert_eq!(Solution::num_squares(13), 2); // 4+9
                }

                #[test]
                fn test_one() {
                    assert_eq!(Solution::num_squares(1), 1); // 1
                }

                #[test]
                fn test_perfect_square() {
                    assert_eq!(Solution::num_squares(16), 1); // 16
                }

                #[test]
                fn test_two_squares() {
                    assert_eq!(Solution::num_squares(5), 2); // 1+4
                }

                #[test]
                fn test_four_needed() {
                    // By Lagrange's four-square theorem, every natural number
                    // can be represented as the sum of at most 4 perfect squares.
                    assert_eq!(Solution::num_squares(7), 4); // 4+1+1+1
                }

                #[test]
                fn test_larger() {
                    assert_eq!(Solution::num_squares(100), 1); // 100 = 10^2
                }
            }
        };
    }

    test_num_squares!(my_tests, crate::my_solution::Solution);
    test_num_squares!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 10_000_u32;
        let inputs: Vec<i32> = vec![12, 13, 1, 16, 100, 999];

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &inputs {
                std::hint::black_box(crate::my_solution::Solution::num_squares(
                    std::hint::black_box(n),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::num_squares(
                    std::hint::black_box(n),
                ));
            }
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
