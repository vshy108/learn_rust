// @lc app=leetcode id=376 lang=rust
// #[Medium] - Wiggle Subsequence
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.006us
// ai_solution  | avg: 0.124us
// Performance: My 19.77x faster

/// # 376. Wiggle Subsequence
///
/// Return the length of the longest wiggle subsequence where differences between
/// successive numbers strictly alternate between positive and negative.
///
/// Constraints:
/// - 1 <= nums.length <= 1000
/// - 0 <= nums[i] <= 1000

/// ## My Solution — Greedy O(n)
///
/// Track whether the last difference was up or down.
/// Count a new element in the subsequence whenever the direction changes.
///
/// - Time: O(n)
/// - Space: O(1)
mod my_solution {
    pub fn wiggle_max_length(nums: &[i32]) -> i32 {
        let n = nums.len();
        if n < 2 {
            return n as i32;
        }
        // up[i] = length of longest wiggle ending with an up
        // down[i] = length ending with a down
        let mut up = 1;
        let mut down = 1;
        for i in 1..n {
            if nums[i] > nums[i - 1] {
                up = down + 1;
            } else if nums[i] < nums[i - 1] {
                down = up + 1;
            }
            // If equal, both stay the same
        }
        up.max(down)
    }
}

/// ## AI Solution — DP O(n^2)
///
/// f[i] = longest wiggle ending at i with last move up.
/// g[i] = longest wiggle ending at i with last move down.
///
/// - Time: O(n^2)
/// - Space: O(n)
mod ai_solution {
    pub fn wiggle_max_length(nums: &[i32]) -> i32 {
        let n = nums.len();
        let mut f = vec![1i32; n]; // ends with up move
        let mut g = vec![1i32; n]; // ends with down move
        let mut ans = 1;
        for i in 1..n {
            for j in 0..i {
                if nums[j] < nums[i] {
                    f[i] = f[i].max(g[j] + 1);
                } else if nums[j] > nums[i] {
                    g[i] = g[i].max(f[j] + 1);
                }
            }
            ans = ans.max(f[i]).max(g[i]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[i32]) -> i32) {
        assert_eq!(f(&[1, 7, 4, 9, 2, 5]), 6);
        assert_eq!(f(&[1, 17, 5, 10, 13, 15, 10, 5, 16, 8]), 7);
        assert_eq!(f(&[1, 2, 3, 4, 5, 6, 7, 8, 9]), 2);
        assert_eq!(f(&[0]), 1);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::wiggle_max_length);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::wiggle_max_length);
    }
}

fn main() {
    let nums = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::wiggle_max_length(std::hint::black_box(&nums)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::wiggle_max_length(std::hint::black_box(&nums)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::wiggle_max_length(std::hint::black_box(&nums)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::wiggle_max_length(std::hint::black_box(&nums)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 376 - Wiggle Subsequence");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
