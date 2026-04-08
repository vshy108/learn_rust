// LeetCode 483: Smallest Good Base [Hard]
// https://leetcode.com/problems/smallest-good-base/
//
// Given an integer n (as string), return the smallest good base.
// k >= 2 is a good base of n if all digits of n in base k are 1's.
// i.e., n = 1 + k + k^2 + ... + k^(m-1) for some m >= 2.
//
// Examples:
//   "13" -> "3"   (13 in base 3 is 111)
//   "4681" -> "8" (4681 in base 8 is 11111)
//   "1000000000000000000" -> "999999999999999999" (base n-1, two 1s)
//
// Constraints:
//   n is in [3, 10^18], given as string
//
// Run:  cargo run --example lc_0483_smallest_good_base
// Test: cargo test --example lc_0483_smallest_good_base
// Bench: cargo test --example lc_0483_smallest_good_base bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 38.838us (binary search over all m)
//   AI solution: avg 686ns    (nth root approach)
//   Ratio: 56.6x (AI >>x faster -- nth root avoids full binary search)

/// My solution module — binary search for each possible digit count.
///
/// For digit count m from max down to 2, binary search for base k
/// such that 1 + k + k^2 + ... + k^(m-1) = n.
/// Max m is about 60 (since 2^60 > 10^18).
///
/// Time:  O(log^2(n))
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn smallest_good_base(n: String) -> String {
            let num: u128 = n.parse().expect("valid number");

            // Try from largest m (most 1s = smallest base) down to 2
            for m in (2..=64).rev() {
                // Binary search for k where 1+k+k^2+...+k^(m-1) = num
                let mut lo: u128 = 2;
                let mut hi: u128 = num;
                while lo < hi {
                    let mid = lo + (hi - lo) / 2;
                    let val = Self::geometric_sum(mid, m);
                    if val >= num {
                        hi = mid;
                    } else {
                        lo = mid + 1;
                    }
                }
                if Self::geometric_sum(lo, m) == num {
                    return lo.to_string();
                }
            }
            // Fallback: base = n-1 gives "11" (1 + (n-1) = n)
            (num - 1).to_string()
        }

        /// Compute 1 + k + k^2 + ... + k^(m-1), with overflow protection.
        fn geometric_sum(k: u128, m: u32) -> u128 {
            let mut sum: u128 = 0;
            let mut power: u128 = 1;
            for _ in 0..m {
                sum = match sum.checked_add(power) {
                    Some(s) => s,
                    None => return u128::MAX,
                };
                power = match power.checked_mul(k) {
                    Some(p) => p,
                    None => return u128::MAX, // overflow means too large
                };
            }
            sum
        }
    }
}

/// AI solution module — nth root approach (C++ style from doocs).
///
/// For each m from max down to 2, compute k = floor(n^(1/m))
/// and check if 1+k+...+k^(m-1) = n.
///
/// Time:  O(log(n) * log(n))
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn smallest_good_base(n: String) -> String {
            let v: u128 = n.parse().expect("valid number");
            // max m is floor(log2(v)), since base=2 gives most digits
            let mx = (v as f64).log2().floor() as u32;

            for m in (2..=mx).rev() {
                // k ~ v^(1/m)
                let k = (v as f64).powf(1.0 / m as f64) as u128;
                // Check k and k+1 (floating point might be off by one)
                for candidate in [k, k + 1] {
                    if candidate < 2 {
                        continue;
                    }
                    if Self::geometric_sum(candidate, m + 1) == v {
                        return candidate.to_string();
                    }
                }
            }
            // base = v-1: two digits "11"
            (v - 1).to_string()
        }

        fn geometric_sum(k: u128, digits: u32) -> u128 {
            let mut sum: u128 = 0;
            let mut mul: u128 = 1;
            for _ in 0..digits {
                sum = match sum.checked_add(mul) {
                    Some(s) => s,
                    None => return u128::MAX,
                };
                mul = match mul.checked_mul(k) {
                    Some(p) => p,
                    None => return u128::MAX,
                };
            }
            sum
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_13() {
            assert_eq!(Solution::smallest_good_base("13".into()), "3");
        }
        #[test]
        fn test_4681() {
            assert_eq!(Solution::smallest_good_base("4681".into()), "8");
        }
        #[test]
        fn test_large() {
            assert_eq!(
                Solution::smallest_good_base("1000000000000000000".into()),
                "999999999999999999"
            );
        }
        #[test]
        fn test_three() {
            // 3 = 1 + 2, so base 2 gives "11"
            assert_eq!(Solution::smallest_good_base("3".into()), "2");
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
    F: Fn(String) -> String,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box("4681".into())));
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
    bench_solution("My solution", my_solution::Solution::smallest_good_base);
    bench_solution("AI solution", ai_solution::Solution::smallest_good_base);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::smallest_good_base("4681".into())
    );
}
