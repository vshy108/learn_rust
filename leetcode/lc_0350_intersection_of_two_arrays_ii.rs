// @lc app=leetcode id=350 lang=rust
// #[Easy] - Intersection of Two Arrays II
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.064us
// ai_solution  | avg: 0.110us
// Performance: My 1.72x faster

/// # 350. Intersection of Two Arrays II
///
/// Return intersection with duplicates preserved (each element appears
/// as many times as it shows in both arrays).
///
/// Constraints:
/// - 1 <= nums1.length, nums2.length <= 1000
/// - 0 <= nums[i] <= 1000

/// ## My Solution — Counting array
///
/// Use an array of size 1001 to count occurrences in nums1.
/// For each element in nums2, if count > 0, include it and decrement.
///
/// - Time: O(n + m)
/// - Space: O(1) — fixed 1001 array
mod my_solution {
    pub fn intersect(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
        let mut cnt = [0u16; 1001];
        for &x in nums1 {
            cnt[x as usize] += 1;
        }
        let mut result = Vec::new();
        for &x in nums2 {
            let i = x as usize;
            if cnt[i] > 0 {
                result.push(x);
                cnt[i] -= 1;
            }
        }
        result
    }
}

/// ## AI Solution — HashMap counting
///
/// Count occurrences via HashMap, then match against nums2.
///
/// - Time: O(n + m)
/// - Space: O(min(n, m))
mod ai_solution {
    use std::collections::HashMap;

    pub fn intersect(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        for &x in nums1 {
            *cnt.entry(x).or_insert(0) += 1;
        }
        let mut result = Vec::new();
        for &x in nums2 {
            if let Some(count) = cnt.get_mut(&x) {
                if *count > 0 {
                    result.push(x);
                    *count -= 1;
                }
            }
        }
        result
    }
}

fn run_tests(f: fn(&[i32], &[i32]) -> Vec<i32>) {
    let mut r = f(&[1, 2, 2, 1], &[2, 2]);
    r.sort();
    assert_eq!(r, vec![2, 2]);

    let mut r = f(&[4, 9, 5], &[9, 4, 9, 8, 4]);
    r.sort();
    assert_eq!(r, vec![4, 9]);

    // No overlap
    assert_eq!(f(&[1], &[2]), Vec::<i32>::new());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::intersect);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::intersect);
    }
}

fn main() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::intersect(
            std::hint::black_box(&nums1),
            std::hint::black_box(&nums2),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::intersect(
            std::hint::black_box(&nums1),
            std::hint::black_box(&nums2),
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::intersect(
            std::hint::black_box(&nums1),
            std::hint::black_box(&nums2),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::intersect(
            std::hint::black_box(&nums1),
            std::hint::black_box(&nums2),
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

    println!("LC 350 - Intersection of Two Arrays II");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
