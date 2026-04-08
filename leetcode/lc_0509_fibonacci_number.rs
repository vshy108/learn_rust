// LeetCode 509: Fibonacci Number [Easy]
// https://leetcode.com/problems/fibonacci-number/
//
// The Fibonacci numbers F(n) form a sequence where each number is the sum of
// the two preceding ones: F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2).
// Given n, calculate F(n).
//
// Examples:
//   2 -> 1   (F(2) = F(1) + F(0) = 1 + 0 = 1)
//   3 -> 2   (F(3) = F(2) + F(1) = 1 + 1 = 2)
//   4 -> 3   (F(4) = F(3) + F(2) = 2 + 1 = 3)
//
// Constraints:
//   0 <= n <= 30
//
// Run:  cargo run --example lc_0509_fibonacci_number
// Test: cargo test --example lc_0509_fibonacci_number
// Bench: cargo test --example lc_0509_fibonacci_number bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 10.7ns
//   AI solution: avg 7.5ns
//   AI is 1.42x faster
//   Reason: matrix exponentiation runs in O(log n) multiplications vs O(n)
//   iterations; for n=30, ~5 matrix muls beat 30 scalar additions.

/// My solution — iterative with two variables.
///
/// From doocs/leetcode Rust solution. Maintain a, b and shift them n times.
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Calculate the n-th Fibonacci number.
        ///
        /// # Arguments
        /// * `n` - index into the Fibonacci sequence (0-indexed)
        ///
        /// # Returns
        /// F(n)
        pub fn fib(n: i32) -> i32 {
            let mut a = 0;
            let mut b = 1;
            for _ in 0..n {
                let t = b;
                b += a;
                a = t;
            }
            a
        }
    }
}

/// AI solution — matrix exponentiation for O(log n) computation.
///
/// Uses the identity: [[F(n+1), F(n)], [F(n), F(n-1)]] = [[1,1],[1,0]]^n.
/// Compute the matrix power in O(log n) via repeated squaring.
///
/// Time:  O(log n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Calculate F(n) using matrix exponentiation.
        ///
        /// # Arguments
        /// * `n` - index into the Fibonacci sequence
        ///
        /// # Returns
        /// F(n)
        pub fn fib(n: i32) -> i32 {
            if n == 0 {
                return 0;
            }
            let mut a = [[1i64, 1], [1, 0]]; // Base matrix
            let mut res = [[1i64, 0], [0, 1]]; // Identity matrix
            let mut n = n as u32;

            while n > 0 {
                if n & 1 == 1 {
                    res = mul(&res, &a);
                }
                a = mul(&a, &a);
                n >>= 1;
            }
            res[0][1] as i32
        }
    }

    /// Multiply two 2x2 matrices.
    fn mul(a: &[[i64; 2]; 2], b: &[[i64; 2]; 2]) -> [[i64; 2]; 2] {
        let mut c = [[0i64; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    c[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        c
    }
}

fn main() {
    for n in 0..10 {
        let my = my_solution::Solution::fib(n);
        let ai = ai_solution::Solution::fib(n);
        println!("F({n}) = my:{my}, ai:{ai}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_fib {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_0() {
                    assert_eq!(<$solution>::fib(0), 0);
                }

                #[test]
                fn test_1() {
                    assert_eq!(<$solution>::fib(1), 1);
                }

                #[test]
                fn test_2() {
                    assert_eq!(<$solution>::fib(2), 1);
                }

                #[test]
                fn test_3() {
                    assert_eq!(<$solution>::fib(3), 2);
                }

                #[test]
                fn test_4() {
                    assert_eq!(<$solution>::fib(4), 3);
                }

                #[test]
                fn test_10() {
                    assert_eq!(<$solution>::fib(10), 55);
                }

                #[test]
                fn test_30() {
                    assert_eq!(<$solution>::fib(30), 832_040);
                }
            }
        };
    }

    test_fib!(my, crate::my_solution::Solution);
    test_fib!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 500_000;
        let n = 30;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::fib(std::hint::black_box(n)));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::fib(std::hint::black_box(n)));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}ns avg",
            my_duration.as_secs_f64() * 1_000_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}ns avg",
            ai_duration.as_secs_f64() * 1_000_000_000.0 / iterations as f64,
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
