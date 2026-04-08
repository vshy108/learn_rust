// @lc app=leetcode id=357 lang=rust
// #[Medium] - Count Numbers with Unique Digits
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.004us
// ai_solution  | avg: 121.672us
// Performance: My 28923.62x faster

/// # 357. Count Numbers with Unique Digits
///
/// Count all numbers with unique digits x, where 0 <= x < 10^n.
///
/// Constraints:
/// - 0 <= n <= 8

/// ## My Solution — Combinatorics O(1)
///
/// For k-digit numbers, first digit has 9 choices (1-9), subsequent digits
/// have 9, 8, 7, ... choices (avoiding repeats). Sum up for all lengths 1..=n.
///
/// - Time: O(n)
/// - Space: O(1)
mod my_solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut total = 10; // 0-digit (just 0) + 1-digit (1..9) = 10
        let mut product = 9; // choices for k-digit: 9 * 9 * 8 * 7 * ...
        for i in 2..=n.min(10) as usize {
            // Available digits for position i: (10 - i + 1) but first digit uses 9
            product *= (11 - i) as i32;
            total += product;
        }
        total
    }
}

/// ## AI Solution — Digit DP with bitmask
///
/// Use memoized DFS over digit positions with a bitmask tracking used digits.
///
/// - Time: O(n * 2^10 * 10)
/// - Space: O(n * 2^10)
mod ai_solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let n = n as usize;
        // f[pos][mask] = count of valid completions from position pos with used digits mask
        let mut f = vec![vec![-1i32; 1 << 10]; n];

        fn dfs(i: usize, mask: u16, lead: bool, f: &mut [Vec<i32>], n: usize) -> i32 {
            if i >= n {
                return 1; // successfully placed all digits
            }
            // Only cache when not leading (leading zeros make mask semantics ambiguous)
            if !lead && f[i][mask as usize] >= 0 {
                return f[i][mask as usize];
            }
            let mut ans = 0i32;
            for d in 0..10u16 {
                if mask >> d & 1 == 1 {
                    continue; // digit already used
                }
                if lead && d == 0 {
                    // Still leading zeros, don't mark 0 as used
                    ans += dfs(i + 1, mask, true, f, n);
                } else {
                    ans += dfs(i + 1, mask | (1 << d), false, f, n);
                }
            }
            if !lead {
                f[i][mask as usize] = ans;
            }
            ans
        }

        dfs(0, 0, true, &mut f, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(i32) -> i32) {
        assert_eq!(f(0), 1);
        assert_eq!(f(1), 10);
        assert_eq!(f(2), 91);
        assert_eq!(f(3), 739);
        assert_eq!(f(8), 2345851);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::count_numbers_with_unique_digits);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::count_numbers_with_unique_digits);
    }
}

fn main() {
    let n = 8;
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::count_numbers_with_unique_digits(
            std::hint::black_box(n),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::count_numbers_with_unique_digits(
            std::hint::black_box(n),
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::count_numbers_with_unique_digits(
            std::hint::black_box(n),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::count_numbers_with_unique_digits(
            std::hint::black_box(n),
        ));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 357 - Count Numbers with Unique Digits");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
