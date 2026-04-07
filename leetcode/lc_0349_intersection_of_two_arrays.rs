// @lc app=leetcode id=349 lang=rust
// #[Easy] - Intersection of Two Arrays
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.041us
// ai_solution  | avg: 0.252us
// Performance: My 6.17x faster

/// # 349. Intersection of Two Arrays
///
/// Return unique intersection of two arrays.
///
/// Constraints:
/// - 1 <= nums1.length, nums2.length <= 1000
/// - 0 <= nums[i] <= 1000

/// ## My Solution — Boolean array O(max_val)
///
/// Use a boolean array of size 1001. Mark elements from nums1,
/// then collect from nums2 (clearing after first match to avoid dupes).
///
/// - Time: O(n + m)
/// - Space: O(1) — fixed 1001 booleans
mod my_solution {
    pub fn intersection(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
        let mut seen = [false; 1001];
        for &x in nums1 {
            seen[x as usize] = true;
        }
        let mut result = Vec::new();
        for &x in nums2 {
            let i = x as usize;
            if seen[i] {
                result.push(x);
                // Mark false to avoid duplicates
                seen[i] = false;
            }
        }
        result
    }
}

/// ## AI Solution — HashSet intersection
///
/// Collect both arrays into HashSets, compute intersection.
///
/// - Time: O(n + m)
/// - Space: O(n + m)
mod ai_solution {
    use std::collections::HashSet;

    pub fn intersection(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
        let s1: HashSet<i32> = nums1.iter().copied().collect();
        let s2: HashSet<i32> = nums2.iter().copied().collect();
        s1.intersection(&s2).copied().collect()
    }
}

fn run_tests(f: fn(&[i32], &[i32]) -> Vec<i32>) {
    let mut r = f(&[1, 2, 2, 1], &[2, 2]);
    r.sort();
    assert_eq!(r, vec![2]);

    let mut r = f(&[4, 9, 5], &[9, 4, 9, 8, 4]);
    r.sort();
    assert_eq!(r, vec![4, 9]);

    // No intersection
    assert_eq!(f(&[1, 2], &[3, 4]), Vec::<i32>::new());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::intersection);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::intersection);
    }
}

fn main() {
    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::intersection(
            std::hint::black_box(&nums1),
            std::hint::black_box(&nums2),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::intersection(
            std::hint::black_box(&nums1),
            std::hint::black_box(&nums2),
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::intersection(
            std::hint::black_box(&nums1),
            std::hint::black_box(&nums2),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::intersection(
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

    println!("LC 349 - Intersection of Two Arrays");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
