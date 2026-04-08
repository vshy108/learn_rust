// @lc app=leetcode id=338 lang=rust
// #[Easy] - Counting Bits
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.026us
// ai_solution  | avg: 0.026us
// Performance: AI 1.00x faster

/// # 338. Counting Bits
///
/// Given integer `n`, return array ans of length n+1 where ans[i] is the
/// number of 1's in the binary representation of i.
///
/// Follow-up: O(n) time, single pass, no builtin popcount.
///
/// Constraints:
/// - 0 <= n <= 10^5

/// ## My Solution — DP: ans[i] = ans[i & (i-1)] + 1
///
/// `i & (i-1)` clears the lowest set bit. So the count of 1s in i is
/// one more than the count of 1s in i with its lowest bit cleared.
///
/// - Time: O(n)
/// - Space: O(n) (output)
mod my_solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![0i32; n + 1];
        for i in 1..=n {
            // i & (i-1) removes the lowest set bit of i
            ans[i] = ans[i & (i - 1)] + 1;
        }
        ans
    }
}

/// ## AI Solution — DP: ans[i] = ans[i >> 1] + (i & 1)
///
/// The number of 1-bits in i equals the count in i/2 plus the least
/// significant bit of i.
///
/// - Time: O(n)
/// - Space: O(n) (output)
mod ai_solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![0i32; n + 1];
        for i in 1..=n {
            // Right shift removes LSB; add it back via (i & 1)
            ans[i] = ans[i >> 1] + (i & 1) as i32;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(i32) -> Vec<i32>) {
        assert_eq!(f(2), vec![0, 1, 1]);
        assert_eq!(f(5), vec![0, 1, 1, 2, 1, 2]);
        assert_eq!(f(0), vec![0]);
        assert_eq!(f(1), vec![0, 1]);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::count_bits);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::count_bits);
    }
}

fn main() {
    let n = 5;
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::count_bits(std::hint::black_box(n)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::count_bits(std::hint::black_box(n)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::count_bits(std::hint::black_box(n)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::count_bits(std::hint::black_box(n)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 338 - Counting Bits");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
