// @lc app=leetcode id=360 lang=rust
// #[Medium] - Sort Transformed Array
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.026us
// ai_solution  | avg: 0.025us
// Performance: AI 1.07x faster

/// # 360. Sort Transformed Array
///
/// Apply f(x) = ax^2 + bx + c to each element of sorted array, return sorted result.
///
/// Constraints:
/// - 1 <= nums.length <= 200
/// - -100 <= nums[i], a, b, c <= 100
/// - nums is sorted ascending

/// ## My Solution — Two pointers O(n)
///
/// If a > 0, parabola opens up: extremes at ends are largest, fill result from back.
/// If a <= 0, parabola opens down (or linear): extremes at ends are smallest, fill from front.
///
/// - Time: O(n)
/// - Space: O(n)
mod my_solution {
    pub fn sort_transformed_array(nums: &[i32], a: i32, b: i32, c: i32) -> Vec<i32> {
        let f = |x: i32| -> i32 { a * x * x + b * x + c };
        let n = nums.len();
        let mut result = vec![0i32; n];
        let (mut lo, mut hi) = (0usize, n - 1);

        for k in 0..n {
            let y_lo = f(nums[lo]);
            let y_hi = f(nums[hi]);
            if a > 0 {
                // Fill from back with the larger value
                if y_lo > y_hi {
                    result[n - 1 - k] = y_lo;
                    lo += 1;
                } else {
                    result[n - 1 - k] = y_hi;
                    hi -= 1;
                }
            } else {
                // Fill from front with the smaller value
                if y_lo > y_hi {
                    result[k] = y_hi;
                    hi -= 1;
                } else {
                    result[k] = y_lo;
                    lo += 1;
                }
            }
        }
        result
    }
}

/// ## AI Solution — Apply + sort
///
/// Transform all values, then sort. Simple but O(n log n).
///
/// - Time: O(n log n)
/// - Space: O(n)
mod ai_solution {
    pub fn sort_transformed_array(nums: &[i32], a: i32, b: i32, c: i32) -> Vec<i32> {
        let f = |x: i32| -> i32 { a * x * x + b * x + c };
        let mut result: Vec<i32> = nums.iter().map(|&x| f(x)).collect();
        result.sort_unstable();
        result
    }
}

fn run_tests(f: fn(&[i32], i32, i32, i32) -> Vec<i32>) {
    assert_eq!(f(&[-4, -2, 2, 4], 1, 3, 5), vec![3, 9, 15, 33]);
    assert_eq!(f(&[-4, -2, 2, 4], -1, 3, 5), vec![-23, -5, 1, 7]);
    assert_eq!(f(&[1, 2, 3], 0, 1, 0), vec![1, 2, 3]); // linear
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::sort_transformed_array);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::sort_transformed_array);
    }
}

fn main() {
    let nums = vec![-4, -2, 2, 4];
    let (a, b, c) = (1, 3, 5);
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::sort_transformed_array(
            std::hint::black_box(&nums), a, b, c,
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::sort_transformed_array(
            std::hint::black_box(&nums), a, b, c,
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::sort_transformed_array(
            std::hint::black_box(&nums), a, b, c,
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::sort_transformed_array(
            std::hint::black_box(&nums), a, b, c,
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

    println!("LC 360 - Sort Transformed Array");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
