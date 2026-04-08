// @lc app=leetcode id=370 lang=rust
// #[Medium] - Range Addition
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.025us
// ai_solution  | avg: 0.029us
// Performance: My 1.15x faster

/// # 370. Range Addition
///
/// Given length and updates array where updates[i] = [startIdx, endIdx, inc],
/// apply each update to an initially-zero array and return the result.
///
/// Constraints:
/// - 1 <= length <= 10^5
/// - 0 <= updates.length <= 10^4
/// - 0 <= startIdx <= endIdx < length
/// - -1000 <= inc <= 1000

/// ## My Solution — Difference array + prefix sum O(n + k)
///
/// Classic difference array: mark +inc at start and -inc at end+1,
/// then prefix sum to get the final array.
///
/// - Time: O(n + k) where k = number of updates
/// - Space: O(n)
mod my_solution {
    pub fn get_modified_array(length: i32, updates: &[[i32; 3]]) -> Vec<i32> {
        let n = length as usize;
        let mut diff = vec![0i32; n];

        for update in updates {
            let (l, r, c) = (update[0] as usize, update[1] as usize, update[2]);
            diff[l] += c;
            if r + 1 < n {
                diff[r + 1] -= c;
            }
        }

        // Convert difference array to result via prefix sum
        for i in 1..n {
            diff[i] += diff[i - 1];
        }
        diff
    }
}

/// ## AI Solution — Direct range update (brute force) O(n * k)
///
/// For each update, iterate over the range and add inc directly.
///
/// - Time: O(n * k)
/// - Space: O(n)
mod ai_solution {
    pub fn get_modified_array(length: i32, updates: &[[i32; 3]]) -> Vec<i32> {
        let n = length as usize;
        let mut arr = vec![0i32; n];

        for update in updates {
            let (l, r, c) = (update[0] as usize, update[1] as usize, update[2]);
            for i in l..=r {
                arr[i] += c;
            }
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(i32, &[[i32; 3]]) -> Vec<i32>) {
        assert_eq!(
            f(5, &[[1, 3, 2], [2, 4, 3], [0, 2, -2]]),
            vec![-2, 0, 3, 5, 3]
        );
        assert_eq!(
            f(10, &[[2, 4, 6], [5, 6, 8], [1, 9, -4]]),
            vec![0, -4, 2, 2, 2, 4, 4, -4, -4, -4]
        );
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::get_modified_array);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::get_modified_array);
    }
}

fn main() {
    let length = 5;
    let updates = vec![[1, 3, 2], [2, 4, 3], [0, 2, -2]];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::get_modified_array(
            length,
            std::hint::black_box(&updates),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::get_modified_array(
            length,
            std::hint::black_box(&updates),
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::get_modified_array(
            length,
            std::hint::black_box(&updates),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::get_modified_array(
            length,
            std::hint::black_box(&updates),
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

    println!("LC 370 - Range Addition");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
