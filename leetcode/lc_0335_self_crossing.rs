// @lc app=leetcode id=335 lang=rust
// #[Hard] - Self Crossing
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.001us
// ai_solution  | avg: 0.001us
// Performance: My 1.00x faster

/// # 335. Self Crossing
///
/// Move from (0,0): north, west, south, east, north, ... (counter-clockwise).
/// distance[i] is the length of the i-th move.
/// Return true if the path crosses itself.
///
/// Constraints:
/// - 1 <= distance.length <= 10^5
/// - 1 <= distance[i] <= 10^5

/// ## My Solution — Three condition checks
///
/// A self-crossing can only occur in three patterns:
/// 1. Step i crosses step i-3: d[i] >= d[i-2] && d[i-1] <= d[i-3]
/// 2. Step i crosses step i-4 at a point: d[i-1] == d[i-3] && d[i]+d[i-4] >= d[i-2]
/// 3. Step i crosses step i-5: d[i-2] >= d[i-4] && d[i-1] <= d[i-3] &&
///    d[i] >= d[i-2]-d[i-4] && d[i-1]+d[i-5] >= d[i-3]
///
/// - Time: O(n)
/// - Space: O(1)
mod my_solution {
    pub fn is_self_crossing(distance: &[i32]) -> bool {
        let d = distance;
        let n = d.len();
        for i in 3..n {
            // Case 1: current line crosses the line 3 steps back
            if d[i] >= d[i - 2] && d[i - 1] <= d[i - 3] {
                return true;
            }
            // Case 2: current line meets the line 4 steps back at a point
            if i >= 4
                && d[i - 1] == d[i - 3]
                && d[i] + d[i - 4] >= d[i - 2]
            {
                return true;
            }
            // Case 3: current line crosses the line 5 steps back
            if i >= 5
                && d[i - 2] >= d[i - 4]
                && d[i - 1] <= d[i - 3]
                && d[i] >= d[i - 2] - d[i - 4]
                && d[i - 1] + d[i - 5] >= d[i - 3]
            {
                return true;
            }
        }
        false
    }
}

/// ## AI Solution — Same three conditions, early return style
///
/// Identical logic with a slightly different code organization.
///
/// - Time: O(n)
/// - Space: O(1)
mod ai_solution {
    pub fn is_self_crossing(distance: &[i32]) -> bool {
        let n = distance.len();
        if n < 4 {
            return false;
        }
        let d = distance;
        for i in 3..n {
            // Fourth line or later crosses first line
            if d[i] >= d[i - 2] && d[i - 1] <= d[i - 3] {
                return true;
            }
            // Fifth line meets first line
            if i >= 4
                && d[i - 1] == d[i - 3]
                && d[i] + d[i - 4] >= d[i - 2]
            {
                return true;
            }
            // Sixth line crosses first line
            if i >= 5
                && d[i - 2] >= d[i - 4]
                && d[i - 1] <= d[i - 3]
                && d[i] + d[i - 4] >= d[i - 2]
                && d[i - 1] + d[i - 5] >= d[i - 3]
            {
                return true;
            }
        }
        false
    }
}

fn run_tests(f: fn(&[i32]) -> bool) {
    assert!(f(&[2, 1, 1, 2]));
    assert!(!f(&[1, 2, 3, 4]));
    assert!(f(&[1, 1, 1, 2, 1]));
    assert!(!f(&[1, 1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::is_self_crossing);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::is_self_crossing);
    }
}

fn main() {
    let distance = vec![2, 1, 1, 2];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::is_self_crossing(
            std::hint::black_box(&distance),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::is_self_crossing(
            std::hint::black_box(&distance),
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::is_self_crossing(
            std::hint::black_box(&distance),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::is_self_crossing(
            std::hint::black_box(&distance),
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

    println!("LC 335 - Self Crossing");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
