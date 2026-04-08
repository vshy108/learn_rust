// @lc app=leetcode id=373 lang=rust
// #[Medium] - Find K Pairs with Smallest Sums
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.230us
// ai_solution  | avg: 0.355us
// Performance: My 1.54x faster

/// # 373. Find K Pairs with Smallest Sums
///
/// Given two sorted arrays nums1 and nums2, return the k pairs (u, v) with
/// the smallest sums, where u is from nums1 and v is from nums2.
///
/// Constraints:
/// - 1 <= nums1.length, nums2.length <= 10^5
/// - -10^9 <= nums1[i], nums2[i] <= 10^9
/// - Both arrays sorted in non-decreasing order
/// - 1 <= k <= 10^4, k <= nums1.len * nums2.len

/// ## My Solution — Min-heap with (sum, i, j) O(k log k)
///
/// Push (nums1[i] + nums2[0], i, 0) for i in 0..min(k, len1).
/// Pop smallest, push (nums1[i] + nums2[j+1]) if j+1 < len2.
///
/// - Time: O(k * log(min(k, n)))
/// - Space: O(min(k, n))
mod my_solution {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    pub fn k_smallest_pairs(nums1: &[i32], nums2: &[i32], k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap: BinaryHeap<Reverse<(i64, usize, usize)>> = BinaryHeap::new();

        // Seed with first element of nums2 for each index in nums1
        for i in 0..k.min(nums1.len()) {
            heap.push(Reverse((nums1[i] as i64 + nums2[0] as i64, i, 0)));
        }

        let mut result = Vec::with_capacity(k);
        while let Some(Reverse((_, i, j))) = heap.pop() {
            result.push(vec![nums1[i], nums2[j]]);
            if result.len() == k {
                break;
            }
            if j + 1 < nums2.len() {
                heap.push(Reverse((nums1[i] as i64 + nums2[j + 1] as i64, i, j + 1)));
            }
        }
        result
    }
}

/// ## AI Solution — Brute force: generate all, partial sort O(n*m)
///
/// Generate all pairs, sort by sum, take first k.
/// Only practical for small arrays.
///
/// - Time: O(n * m * log(n * m))
/// - Space: O(n * m)
mod ai_solution {
    pub fn k_smallest_pairs(nums1: &[i32], nums2: &[i32], k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut pairs: Vec<(i64, i32, i32)> = Vec::new();
        for &a in nums1 {
            for &b in nums2 {
                pairs.push((a as i64 + b as i64, a, b));
            }
        }
        pairs.sort_unstable();
        pairs
            .into_iter()
            .take(k)
            .map(|(_, a, b)| vec![a, b])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[i32], &[i32], i32) -> Vec<Vec<i32>>) {
        assert_eq!(
            f(&[1, 7, 11], &[2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );
        let res = f(&[1, 1, 2], &[1, 2, 3], 2);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], vec![1, 1]);
        assert_eq!(res[1], vec![1, 1]);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::k_smallest_pairs);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::k_smallest_pairs);
    }
}

fn main() {
    let nums1 = vec![1, 7, 11];
    let nums2 = vec![2, 4, 6];
    let k = 3;
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::k_smallest_pairs(
            std::hint::black_box(&nums1),
            std::hint::black_box(&nums2),
            k,
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::k_smallest_pairs(
            std::hint::black_box(&nums1),
            std::hint::black_box(&nums2),
            k,
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::k_smallest_pairs(
            std::hint::black_box(&nums1),
            std::hint::black_box(&nums2),
            k,
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::k_smallest_pairs(
            std::hint::black_box(&nums1),
            std::hint::black_box(&nums2),
            k,
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

    println!("LC 373 - Find K Pairs with Smallest Sums");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
