// LeetCode 204: Count Primes [Medium]
// https://leetcode.com/problems/count-primes/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.374us
//   AI solution avg: 0.358us
//   Ratio: ~tied
//
// Given an integer n, return the number of prime numbers that are
// strictly less than n.
//
// Constraints: 0 <= n <= 5 * 10^6
//
// Run:  cargo run --example lc_0204_count_primes
// Test: cargo test --example lc_0204_count_primes
// Bench: cargo test --example lc_0204_count_primes bench -- --nocapture

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the count of primes strictly less than n.
        pub fn count_primes(n: i32) -> i32 {
            if n <= 2 { return 0; }
            let n = n as usize;
            let mut is_prime = vec![true; n];
            is_prime[0] = false;
            is_prime[1] = false;
            // Only mark odd composites (simple sieve)
            let mut p = 2;
            while p * p < n {
                if is_prime[p] {
                    let mut m = p * p;
                    while m < n {
                        is_prime[m] = false;
                        m += p;
                    }
                }
                p += 1;
            }
            is_prime.iter().filter(|&&x| x).count() as i32
        }
    }
}

/// AI solution — Sieve of Eratosthenes.
///
/// Create a boolean array of size n, mark composites by iterating
/// multiples of each prime starting from p*p.
///
/// Time: O(n log log n).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Sieve of Eratosthenes to count primes below n.
        pub fn count_primes(n: i32) -> i32 {
            if n <= 2 {
                return 0;
            }
            let n = n as usize;
            // is_prime[i] = true means i is prime
            let mut is_prime = vec![true; n];
            is_prime[0] = false;
            is_prime[1] = false;

            let mut p = 2;
            while p * p < n {
                if is_prime[p] {
                    // Mark all multiples of p starting from p*p
                    let mut multiple = p * p;
                    while multiple < n {
                        is_prime[multiple] = false;
                        multiple += p;
                    }
                }
                p += 1;
            }
            is_prime.iter().filter(|&&x| x).count() as i32
        }
    }
}

fn main() {
    let my = my_solution::Solution::count_primes(100);
    let ai = ai_solution::Solution::count_primes(100);
    println!("primes < 100: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_primes {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_ten() {
                    // Primes < 10: 2,3,5,7 = 4
                    assert_eq!(<$solution>::count_primes(10), 4);
                }

                #[test]
                fn test_zero() {
                    assert_eq!(<$solution>::count_primes(0), 0);
                }

                #[test]
                fn test_one() {
                    assert_eq!(<$solution>::count_primes(1), 0);
                }

                #[test]
                fn test_two() {
                    // Primes strictly less than 2: none
                    assert_eq!(<$solution>::count_primes(2), 0);
                }

                #[test]
                fn test_three() {
                    // Primes < 3: {2} = 1
                    assert_eq!(<$solution>::count_primes(3), 1);
                }

                #[test]
                fn test_hundred() {
                    assert_eq!(<$solution>::count_primes(100), 25);
                }
            }
        };
    }

    test_primes!(my, crate::my_solution::Solution);
    test_primes!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::count_primes(
                std::hint::black_box(1000),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::count_primes(
                std::hint::black_box(1000),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
