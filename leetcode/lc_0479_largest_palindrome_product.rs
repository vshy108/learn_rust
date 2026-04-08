// LeetCode 479: Largest Palindrome Product [Hard]
// https://leetcode.com/problems/largest-palindrome-product/
//
// Given an integer n, return the largest palindromic integer that can be
// represented as the product of two n-digit integers, modulo 1337.
//
// Examples:
//   n=2 -> 987  (99 x 91 = 9009, 9009 % 1337 = 987)
//   n=1 -> 9
//
// Constraints:
//   1 <= n <= 8
//
// Run:  cargo run --example lc_0479_largest_palindrome_product
// Test: cargo test --example lc_0479_largest_palindrome_product
// Bench: cargo test --example lc_0479_largest_palindrome_product bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 26ns  (construct + check)
//   AI solution: avg 26ns  (construct + check)
//   Ratio: ~1.0x

/// My solution module — construct palindromes and check divisibility.
///
/// For each possible upper half a (from largest to smallest n-digit number),
/// construct palindrome by mirroring, then check if any n-digit number divides it.
///
/// Time:  O(10^n * 10^(n/2)) worst case, but exits early
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn largest_palindrome(n: i32) -> i32 {
            if n == 1 {
                return 9;
            }

            let mx = 10_i64.pow(n as u32) - 1;
            for a in (mx / 10 + 1..=mx).rev() {
                // Build palindrome from upper half a: e.g. a=99 -> 9999
                let mut x = a;
                let mut b = a;
                while b > 0 {
                    x = x * 10 + b % 10;
                    b /= 10;
                }
                // Check if any n-digit number t divides x
                let mut t = mx;
                while t * t >= x {
                    if x % t == 0 {
                        return (x % 1337) as i32;
                    }
                    t -= 1;
                }
            }
            9
        }
    }
}

/// AI solution module — same approach, slightly different structure.
///
/// Time:  O(10^n * 10^(n/2)) worst case
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn largest_palindrome(n: i32) -> i32 {
            if n == 1 {
                return 9;
            }

            let mx = 10_i64.pow(n as u32) - 1;
            let mn = mx / 10; // smallest (n-1)-digit number

            for upper in (mn + 1..=mx).rev() {
                // Mirror upper to create a 2n-digit palindrome
                let mut palindrome = upper;
                let mut tmp = upper;
                while tmp > 0 {
                    palindrome = palindrome * 10 + tmp % 10;
                    tmp /= 10;
                }
                // Find a factor in [mn+1, mx] range
                let mut divisor = mx;
                while divisor * divisor >= palindrome {
                    if palindrome % divisor == 0 {
                        return (palindrome % 1337) as i32;
                    }
                    divisor -= 1;
                }
            }
            9
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_n1() {
            assert_eq!(Solution::largest_palindrome(1), 9);
        }
        #[test]
        fn test_n2() {
            assert_eq!(Solution::largest_palindrome(2), 987);
        }
        #[test]
        fn test_n3() {
            assert_eq!(Solution::largest_palindrome(3), 123);
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
    F: Fn(i32) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(2)));
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
    bench_solution("My solution", my_solution::Solution::largest_palindrome);
    bench_solution("AI solution", ai_solution::Solution::largest_palindrome);
}

fn main() {
    println!("{}", ai_solution::Solution::largest_palindrome(2));
}
