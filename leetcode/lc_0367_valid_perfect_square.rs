// @lc app=leetcode id=367 lang=rust
// #[Easy] - Valid Perfect Square
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.108us
// ai_solution  | avg: 0.033us
// Performance: AI 3.23x faster

/// # 367. Valid Perfect Square
///
/// Given a positive integer num, return true if num is a perfect square.
/// Must not use any built-in sqrt function.
///
/// Constraints:
/// - 1 <= num <= 2^31 - 1

/// ## My Solution — Binary search O(log n)
///
/// Search for the smallest integer x such that x*x >= num.
/// Use i64 to avoid overflow when squaring.
///
/// - Time: O(log n)
/// - Space: O(1)
mod my_solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let (mut lo, mut hi) = (1i64, num);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if mid * mid >= num {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo * lo == num
    }
}

/// ## AI Solution — Newton's method O(log n)
///
/// Start from num, iteratively refine: x = (x + num/x) / 2.
/// Converges quadratically.
///
/// - Time: O(log n)
/// - Space: O(1)
mod ai_solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let mut x = num;
        while x * x > num {
            x = (x + num / x) / 2;
        }
        x * x == num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(i32) -> bool) {
        assert!(f(16));
        assert!(!f(14));
        assert!(f(1));
        assert!(f(9));
        assert!(!f(2));
        assert!(f(2_147_395_600)); // 46340^2
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::is_perfect_square);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::is_perfect_square);
    }
}

fn main() {
    let num = 2_147_395_600;
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::is_perfect_square(std::hint::black_box(num)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::is_perfect_square(std::hint::black_box(num)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::is_perfect_square(std::hint::black_box(num)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::is_perfect_square(std::hint::black_box(num)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 367 - Valid Perfect Square");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
