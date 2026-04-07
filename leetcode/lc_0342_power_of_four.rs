// @lc app=leetcode id=342 lang=rust
// #[Easy] - Power of Four
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.001us
// ai_solution  | avg: 0.002us
// Performance: My 2.73x faster

/// # 342. Power of Four
///
/// Given an integer `n`, return `true` if it is a power of four.
///
/// Follow-up: Solve without loops/recursion.
///
/// Constraints:
/// - -2^31 <= n <= 2^31 - 1

/// ## My Solution — Bit manipulation (no loop)
///
/// n is a power of 4 iff:
/// 1. n > 0
/// 2. n is a power of 2: (n & (n-1)) == 0
/// 3. The single set bit is at an even position: (n & 0xAAAAAAAA) == 0
///
/// - Time: O(1)
/// - Space: O(1)
mod my_solution {
    pub fn is_power_of_four(n: i32) -> bool {
        // 0xAAAAAAAA has bits set at all odd positions (1,3,5,...)
        // Powers of 4 have their single bit at even positions (0,2,4,...)
        n > 0 && (n & (n - 1)) == 0 && (n & 0xAAAAAAAAu32 as i32) == 0
    }
}

/// ## AI Solution — Trial division
///
/// Repeatedly divide by 4 while divisible. Check if result is 1.
///
/// - Time: O(log_4(n))
/// - Space: O(1)
mod ai_solution {
    pub fn is_power_of_four(mut n: i32) -> bool {
        while n > 3 {
            if n % 4 != 0 {
                return false;
            }
            n /= 4;
        }
        n == 1
    }
}

fn run_tests(f: fn(i32) -> bool) {
    assert!(f(16));       // 4^2
    assert!(!f(5));
    assert!(f(1));        // 4^0
    assert!(f(4));
    assert!(!f(0));
    assert!(!f(-4));
    assert!(!f(8));       // 2^3, not a power of 4
    assert!(f(1073741824)); // 4^15 = 2^30
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::is_power_of_four);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::is_power_of_four);
    }
}

fn main() {
    let n = 16;
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::is_power_of_four(std::hint::black_box(n)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::is_power_of_four(std::hint::black_box(n)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::is_power_of_four(std::hint::black_box(n)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::is_power_of_four(std::hint::black_box(n)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 342 - Power of Four");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
