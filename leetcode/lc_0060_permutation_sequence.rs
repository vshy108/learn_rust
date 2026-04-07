// LeetCode 60: Permutation Sequence [Hard]
// https://leetcode.com/problems/permutation-sequence/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 0.860us
//   AI solution avg: 0.839us
//   Ratio: ~tied
//
// Given n and k, return the k-th permutation sequence of [1..n].
//
// Example 1: n=3, k=3 -> "213"
// Example 2: n=4, k=9 -> "2314"
// Example 3: n=3, k=1 -> "123"
//
// Constraints: 1 <= n <= 9, 1 <= k <= n!
//
// Run:  cargo run --example lc_0060_permutation_sequence
// Test: cargo test --example lc_0060_permutation_sequence
// Bench: cargo test --example lc_0060_permutation_sequence bench -- --nocapture

/// My solution — factorial number system (direct computation).
///
/// Determine each digit by dividing k by (n-1)!, (n-2)!, etc.
/// Each quotient picks the index into remaining available digits.
///
/// Time: O(n^2) — n iterations, each removing from a Vec (O(n) shift).
/// Space: O(n).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the k-th permutation of [1..n].
        ///
        /// # Arguments
        /// * `n` - number of digits (1 to 9)
        /// * `k` - 1-based permutation index
        ///
        /// # Returns
        /// The k-th permutation as a string
        pub fn get_permutation(n: i32, k: i32) -> String {
            let n = n as usize;
            let mut k = k as usize - 1; // 0-based index
            // Precompute factorials
            let mut fact = vec![1usize; n + 1];
            for i in 1..=n {
                fact[i] = fact[i - 1] * i;
            }
            let mut digits: Vec<u8> = (1..=n as u8).collect();
            let mut result = Vec::with_capacity(n);

            for i in (1..=n).rev() {
                let idx = k / fact[i - 1];
                result.push(b'0' + digits.remove(idx));
                k %= fact[i - 1];
            }
            // Safety: only ASCII digit bytes
            String::from_utf8(result).expect("valid ASCII digits")
        }
    }
}

/// AI solution — same factorial number system (canonical optimal).
///
/// Both solutions use the same approach because it's already O(n^2) optimal
/// for n <= 9 (no benefit from more complex data structures).
///
/// Time: O(n^2).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the k-th permutation of [1..n].
        ///
        /// # Arguments
        /// * `n` - number of digits (1 to 9)
        /// * `k` - 1-based permutation index
        ///
        /// # Returns
        /// The k-th permutation as a string
        pub fn get_permutation(n: i32, k: i32) -> String {
            let n = n as usize;
            let mut k = (k - 1) as usize;
            let mut fact = vec![1usize; n];
            for i in 1..n {
                fact[i] = fact[i - 1] * i;
            }
            let mut digits: Vec<u8> = (b'1'..=b'0' + n as u8).collect();
            let mut result = Vec::with_capacity(n);

            for i in (0..n).rev() {
                let idx = k / fact[i];
                result.push(digits.remove(idx));
                k %= fact[i];
            }
            String::from_utf8(result).expect("valid ASCII digits")
        }
    }
}

fn main() {
    let cases = [(3, 3), (4, 9), (3, 1)];
    for (n, k) in &cases {
        println!(
            "n={n}, k={k} -> my:{}, ai:{}",
            my_solution::Solution::get_permutation(*n, *k),
            ai_solution::Solution::get_permutation(*n, *k),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_get_permutation {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_n3_k3() {
                    assert_eq!(<$solution>::get_permutation(3, 3), "213");
                }

                #[test]
                fn test_n4_k9() {
                    assert_eq!(<$solution>::get_permutation(4, 9), "2314");
                }

                #[test]
                fn test_n3_k1() {
                    assert_eq!(<$solution>::get_permutation(3, 1), "123");
                }

                #[test]
                fn test_n1_k1() {
                    assert_eq!(<$solution>::get_permutation(1, 1), "1");
                }

                #[test]
                fn test_n3_k6() {
                    assert_eq!(<$solution>::get_permutation(3, 6), "321");
                }

                #[test]
                fn test_n4_k1() {
                    assert_eq!(<$solution>::get_permutation(4, 1), "1234");
                }
            }
        };
    }

    test_get_permutation!(my, crate::my_solution::Solution);
    test_get_permutation!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::get_permutation(
                std::hint::black_box(9),
                std::hint::black_box(100_000),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::get_permutation(
                std::hint::black_box(9),
                std::hint::black_box(100_000),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
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
