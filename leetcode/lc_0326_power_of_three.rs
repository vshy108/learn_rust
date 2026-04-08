// @lc app=leetcode id=326 lang=rust
// #[Easy] - Power of Three
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.001us
// ai_solution  | avg: 0.002us
// Performance: My 3.58x faster

/// # 326. Power of Three
///
/// Given an integer `n`, return `true` if it is a power of three.
///
/// Follow-up: Could you solve it without loops/recursion?
///
/// Constraints:
/// - -2^31 <= n <= 2^31 - 1

/// ## My Solution — Divisor of max power of 3
///
/// 3^19 = 1_162_261_467 is the largest power of 3 that fits in i32.
/// Any power of 3 divides 3^19 evenly, and no non-power-of-3 does
/// (since 3 is prime).
///
/// - Time: O(1)
/// - Space: O(1)
mod my_solution {
    pub fn is_power_of_three(n: i32) -> bool {
        // 3^19 = 1_162_261_467 — largest power of 3 in i32 range
        n > 0 && 1_162_261_467 % n == 0
    }
}

/// ## AI Solution — Trial division
///
/// Repeatedly divide by 3 while divisible. If the result is 1, it was
/// a power of 3.
///
/// - Time: O(log_3(n))
/// - Space: O(1)
mod ai_solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        // Continuously divide by 3 until no longer divisible
        while n > 2 {
            if n % 3 != 0 {
                return false;
            }
            n /= 3;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(i32) -> bool) {
        assert!(f(27)); // 3^3
        assert!(!f(0)); // 0 is not a power of 3
        assert!(!f(-1)); // negative
        assert!(f(1)); // 3^0 = 1
        assert!(f(9)); // 3^2
        assert!(!f(45)); // 45 = 9*5, not a power of 3
        assert!(f(1_162_261_467)); // 3^19
        assert!(!f(2));
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::is_power_of_three);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::is_power_of_three);
    }
}

fn main() {
    let n = 27;
    let iterations = 100_000;

    // Warm up
    for _ in 0..1000 {
        std::hint::black_box(my_solution::is_power_of_three(std::hint::black_box(n)));
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::is_power_of_three(std::hint::black_box(n)));
    }
    let my_dur = start.elapsed();

    // Warm up
    for _ in 0..1000 {
        std::hint::black_box(ai_solution::is_power_of_three(std::hint::black_box(n)));
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::is_power_of_three(std::hint::black_box(n)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 326 - Power of Three");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
