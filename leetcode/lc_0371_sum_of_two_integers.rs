// @lc app=leetcode id=371 lang=rust
// #[Medium] - Sum of Two Integers
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.002us
// ai_solution  | avg: 0.002us
// Performance: AI 1.00x faster

/// # 371. Sum of Two Integers
///
/// Given two integers a and b, return the sum without using + or -.
///
/// Constraints:
/// - -1000 <= a, b <= 1000

/// ## My Solution — Iterative bit manipulation
///
/// XOR gives sum without carry, AND + shift gives carry.
/// Repeat until carry is zero. Use u32 to handle negative numbers correctly
/// (Rust panics on signed overflow in debug, and shifting negative values is UB-adjacent).
///
/// - Time: O(32) = O(1)
/// - Space: O(1)
mod my_solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = (a as u32, b as u32);
        while b != 0 {
            let carry = (a & b) << 1;
            a ^= b;
            b = carry;
        }
        a as i32
    }
}

/// ## AI Solution — Recursive bit manipulation
///
/// Same logic but expressed recursively.
///
/// - Time: O(32) = O(1)
/// - Space: O(1) (tail-recursive in practice)
mod ai_solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        // Compute in u32 to avoid signed overflow issues
        let (ua, ub) = (a as u32, b as u32);
        let sum = ua ^ ub;
        let carry = (ua & ub) << 1;
        get_sum(sum as i32, carry as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(i32, i32) -> i32) {
        assert_eq!(f(1, 2), 3);
        assert_eq!(f(2, 3), 5);
        assert_eq!(f(-1, 1), 0);
        assert_eq!(f(-1000, 1000), 0);
        assert_eq!(f(-5, -3), -8);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::get_sum);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::get_sum);
    }
}

fn main() {
    let (a, b) = (123, 456);
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::get_sum(a, b));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::get_sum(a, b));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::get_sum(a, b));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::get_sum(a, b));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 371 - Sum of Two Integers");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
