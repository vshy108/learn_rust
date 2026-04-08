// @lc app=leetcode id=372 lang=rust
// #[Medium] - Super Pow
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.041us
// ai_solution  | avg: 0.044us
// Performance: My 1.08x faster

/// # 372. Super Pow
///
/// Calculate a^b mod 1337 where b is given as an array of digits.
///
/// Constraints:
/// - 1 <= a <= 2^31 - 1
/// - 1 <= b.length <= 2000
/// - 0 <= b[i] <= 9, no leading zeros

/// ## My Solution — Iterative modular exponent by digit
///
/// Process digits from right to left: ans = ans * pow(a, digit) mod 1337,
/// then a = pow(a, 10) for the next more-significant digit.
///
/// - Time: O(len(b) * log(10)) = O(len(b))
/// - Space: O(1)
mod my_solution {
    const MOD: i64 = 1337;

    pub fn super_pow(a: i32, b: &[i32]) -> i32 {
        let mut ans: i64 = 1;
        let mut base = a as i64 % MOD;
        for &digit in b.iter().rev() {
            ans = ans * mod_pow(base, digit as u32) % MOD;
            base = mod_pow(base, 10);
        }
        ans as i32
    }

    /// Fast modular exponentiation: base^exp mod MOD.
    fn mod_pow(mut base: i64, mut exp: u32) -> i64 {
        let mut result = 1i64;
        base %= MOD;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % MOD;
            }
            base = base * base % MOD;
            exp >>= 1;
        }
        result
    }
}

/// ## AI Solution — Recursive decomposition: a^(b) = (a^(b/10))^10 * a^(b%10)
///
/// Process digits from left to right. For each digit d, ans = ans^10 * a^d mod 1337.
///
/// - Time: O(len(b))
/// - Space: O(1)
mod ai_solution {
    const MOD: i64 = 1337;

    pub fn super_pow(a: i32, b: &[i32]) -> i32 {
        let mut ans: i64 = 1;
        let a = a as i64 % MOD;
        for &digit in b {
            // ans = ans^10 * a^digit
            ans = mod_pow(ans, 10) % MOD * mod_pow(a, digit as u32) % MOD;
        }
        ans as i32
    }

    fn mod_pow(mut base: i64, mut exp: u32) -> i64 {
        let mut result = 1i64;
        base %= MOD;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % MOD;
            }
            base = base * base % MOD;
            exp >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(i32, &[i32]) -> i32) {
        assert_eq!(f(2, &[3]), 8);
        assert_eq!(f(2, &[1, 0]), 1024);
        assert_eq!(f(1, &[4, 3, 3, 8, 5, 2]), 1);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::super_pow);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::super_pow);
    }
}

fn main() {
    let a = 2;
    let b = vec![1, 0, 0, 0, 0];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::super_pow(a, std::hint::black_box(&b)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::super_pow(a, std::hint::black_box(&b)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::super_pow(a, std::hint::black_box(&b)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::super_pow(a, std::hint::black_box(&b)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 372 - Super Pow");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
