// @lc app=leetcode id=346 lang=rust
// #[Easy] - Moving Average from Data Stream
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.151us
// ai_solution  | avg: 0.268us
// Performance: My 1.78x faster

/// # 346. Moving Average from Data Stream
///
/// Given a stream of integers and a window size, calculate the moving average.
///
/// Constraints:
/// - 1 <= size <= 1000
/// - -10^5 <= val <= 10^5
/// - At most 10^4 calls to next

/// ## My Solution — Circular array
///
/// Maintain a fixed-size array, overwrite oldest entry using modular index.
/// Track running sum and count.
///
/// - Time: O(1) per next
/// - Space: O(size)
mod my_solution {
    /// Compute moving averages from a stream of values with given window size.
    pub fn moving_average(size: usize, vals: &[i32]) -> Vec<f64> {
        let mut data = vec![0i32; size];
        let mut s: i64 = 0;
        let mut cnt: usize = 0;
        let mut result = Vec::with_capacity(vals.len());
        for &val in vals {
            // Circular index: overwrite the oldest element
            let i = cnt % size;
            s += val as i64 - data[i] as i64;
            data[i] = val;
            cnt += 1;
            result.push(s as f64 / cnt.min(size) as f64);
        }
        result
    }
}

/// ## AI Solution — VecDeque (queue)
///
/// Use a queue to store last `size` elements. Pop front when full.
///
/// - Time: O(1) per next
/// - Space: O(size)
mod ai_solution {
    use std::collections::VecDeque;

    /// Compute moving averages from a stream of values with given window size.
    pub fn moving_average(size: usize, vals: &[i32]) -> Vec<f64> {
        let mut q: VecDeque<i32> = VecDeque::with_capacity(size);
        let mut s: i64 = 0;
        let mut result = Vec::with_capacity(vals.len());
        for &val in vals {
            if q.len() == size {
                // Remove oldest element from the front
                s -= q.pop_front().unwrap_or(0) as i64;
            }
            q.push_back(val);
            s += val as i64;
            result.push(s as f64 / q.len() as f64);
        }
        result
    }
}

fn approx_eq(a: &[f64], b: &[f64]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-4)
}

fn run_tests(f: fn(usize, &[i32]) -> Vec<f64>) {
    // Example: size=3, vals=[1,10,3,5]
    let result = f(3, &[1, 10, 3, 5]);
    let expected = vec![1.0, 5.5, 4.66667, 6.0];
    assert!(
        approx_eq(&result, &expected),
        "got {result:?}, expected {expected:?}"
    );
    // Single element window
    let result = f(1, &[5, 10, 15]);
    assert!(approx_eq(&result, &[5.0, 10.0, 15.0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::moving_average);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::moving_average);
    }
}

fn main() {
    let size = 3;
    let vals: Vec<i32> = (0..100).collect();
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::moving_average(size, std::hint::black_box(&vals)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::moving_average(size, std::hint::black_box(&vals)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::moving_average(size, std::hint::black_box(&vals)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::moving_average(size, std::hint::black_box(&vals)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 346 - Moving Average from Data Stream");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
