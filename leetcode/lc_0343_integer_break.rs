// @lc app=leetcode id=343 lang=rust
// #[Medium] - Integer Break
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.001us
// ai_solution  | avg: 0.159us
// Performance: My 120.05x faster

/// # 343. Integer Break
///
/// Break integer `n` into at least 2 positive integers, maximize product.
///
/// Constraints:
/// - 2 <= n <= 58

/// ## My Solution — Math: maximize 3s
///
/// Optimal strategy is to split into as many 3s as possible.
/// If remainder is 1, replace one 3+1 with 4 (=2*2).
/// If remainder is 2, multiply by 2.
///
/// - Time: O(1)
/// - Space: O(1)
mod my_solution {
    pub fn integer_break(n: i32) -> i32 {
        if n < 4 {
            return n - 1; // must split into at least 2 parts
        }
        match n % 3 {
            0 => 3i32.pow(n as u32 / 3),
            1 => 3i32.pow(n as u32 / 3 - 1) * 4, // replace 3+1 with 2+2
            _ => 3i32.pow(n as u32 / 3) * 2, // remainder 2
        }
    }
}

/// ## AI Solution — DP: f[i] = max(j*(i-j), j*f[i-j])
///
/// For each i, try every split point j. Either stop splitting (j*(i-j))
/// or continue splitting the remainder (j*f[i-j]).
///
/// - Time: O(n^2)
/// - Space: O(n)
mod ai_solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut f = vec![0usize; n + 1];
        f[1] = 1;
        for i in 2..=n {
            for j in 1..i {
                // Either don't split further: j * (i - j)
                // Or split remainder: j * f[i - j]
                f[i] = f[i].max(f[i - j] * j).max((i - j) * j);
            }
        }
        f[n] as i32
    }
}

fn run_tests(f: fn(i32) -> i32) {
    assert_eq!(f(2), 1);   // 1*1
    assert_eq!(f(10), 36); // 3+3+4 => 3*3*4
    assert_eq!(f(3), 2);   // 1+2 => 2
    assert_eq!(f(4), 4);   // 2+2 => 4
    assert_eq!(f(58), 1549681956);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::integer_break);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::integer_break);
    }
}

fn main() {
    let n = 10;
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::integer_break(std::hint::black_box(n)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::integer_break(std::hint::black_box(n)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::integer_break(std::hint::black_box(n)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::integer_break(std::hint::black_box(n)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 343 - Integer Break");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
