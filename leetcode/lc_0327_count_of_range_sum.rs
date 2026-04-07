// @lc app=leetcode id=327 lang=rust
// #[Hard] - Count of Range Sum
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.296us
// ai_solution  | avg: 0.194us
// Performance: AI 1.53x faster

/// # 327. Count of Range Sum
///
/// Given an integer array `nums` and two integers `lower` and `upper`,
/// return the number of range sums that lie in `[lower, upper]` inclusive.
///
/// Range sum S(i,j) = sum of nums[i..=j].
///
/// Constraints:
/// - 1 <= nums.length <= 10^5
/// - -2^31 <= nums[i] <= 2^31 - 1
/// - -10^5 <= lower <= upper <= 10^5

/// ## My Solution — Merge Sort on Prefix Sums
///
/// Build prefix sums, then use merge-sort to count pairs (i < j) where
/// lower <= prefix[j] - prefix[i] <= upper. During merge, for each
/// element in the right half, binary-search the left half for valid range.
///
/// - Time: O(n log n)
/// - Space: O(n)
mod my_solution {
    pub fn count_range_sum(nums: &[i32], lower: i32, upper: i32) -> i32 {
        let n = nums.len();
        // Build prefix sums with i64 to avoid overflow
        let mut prefix = vec![0i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }
        let lo = lower as i64;
        let hi = upper as i64;
        merge_count(&mut prefix, lo, hi) as i32
    }

    fn merge_count(arr: &mut [i64], lo: i64, hi: i64) -> usize {
        if arr.len() <= 1 {
            return 0;
        }
        let mid = arr.len() / 2;
        let (left, right) = arr.split_at_mut(mid);
        let mut count = merge_count(left, lo, hi) + merge_count(right, lo, hi);

        // For each j in right, count how many i in left satisfy:
        // lo <= right[j] - left[i] <= hi
        // i.e. right[j] - hi <= left[i] <= right[j] - lo
        // left is sorted, so use two pointers
        let mut l_ptr = 0;
        let mut r_ptr = 0;
        for &rval in right.iter() {
            // Find first i where left[i] >= rval - hi
            while l_ptr < left.len() && left[l_ptr] < rval - hi {
                l_ptr += 1;
            }
            // Find first i where left[i] > rval - lo
            while r_ptr < left.len() && left[r_ptr] <= rval - lo {
                r_ptr += 1;
            }
            count += r_ptr - l_ptr;
        }

        // Standard merge step
        let left_copy: Vec<i64> = left.to_vec();
        let right_copy: Vec<i64> = right.to_vec();
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while i < left_copy.len() && j < right_copy.len() {
            if left_copy[i] <= right_copy[j] {
                arr[k] = left_copy[i];
                i += 1;
            } else {
                arr[k] = right_copy[j];
                j += 1;
            }
            k += 1;
        }
        while i < left_copy.len() {
            arr[k] = left_copy[i];
            i += 1;
            k += 1;
        }
        while j < right_copy.len() {
            arr[k] = right_copy[j];
            j += 1;
            k += 1;
        }
        count
    }
}

/// ## AI Solution — BIT + Coordinate Compression
///
/// Build prefix sums. Collect all values `prefix[i]`, `prefix[i]-lower`,
/// `prefix[i]-upper` into a sorted deduped array for coordinate
/// compression. Sweep left-to-right: for each prefix, query BIT for
/// count of previous prefixes in [prefix-upper, prefix-lower].
///
/// - Time: O(n log n)
/// - Space: O(n)
mod ai_solution {
    pub fn count_range_sum(nums: &[i32], lower: i32, upper: i32) -> i32 {
        let n = nums.len();
        let lo = lower as i64;
        let hi = upper as i64;

        // Build prefix sums
        let mut prefix = vec![0i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        // Coordinate compression: collect all relevant values
        let mut vals: Vec<i64> = Vec::with_capacity((n + 1) * 3);
        for &p in &prefix {
            vals.push(p);
            vals.push(p - lo);
            vals.push(p - hi);
        }
        vals.sort_unstable();
        vals.dedup();

        let m = vals.len();
        let mut bit = vec![0i32; m + 1];

        // BIT operations using wrapping_neg for lowbit
        let update = |bit: &mut Vec<i32>, mut x: usize| {
            while x <= m {
                bit[x] += 1;
                x += x & x.wrapping_neg();
            }
        };
        let query = |bit: &[i32], mut x: usize| -> i32 {
            let mut s = 0;
            while x > 0 {
                s += bit[x];
                x -= x & x.wrapping_neg();
            }
            s
        };

        // Binary search helper to map value -> 1-indexed rank
        let rank = |v: i64| -> usize {
            vals.partition_point(|&x| x < v) + 1
        };

        let mut ans = 0i32;
        for &p in &prefix {
            // Query: how many previous prefix sums s satisfy lo <= p - s <= hi
            // i.e. p - hi <= s <= p - lo
            let l = rank(p - hi);
            let r = rank(p - lo);
            ans += query(&bit, r) - query(&bit, l - 1);
            update(&mut bit, rank(p));
        }
        ans
    }
}

fn run_tests(f: fn(&[i32], i32, i32) -> i32) {
    // Example 1: ranges [0,0]=-2, [2,2]=-1, [0,2]=2 => 3
    assert_eq!(f(&[-2, 5, -1], -2, 2), 3);
    // Example 2: single element
    assert_eq!(f(&[0], 0, 0), 1);
    // All negative
    assert_eq!(f(&[-1, -2, -3], -6, -1), 6);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::count_range_sum);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::count_range_sum);
    }
}

fn main() {
    let nums = vec![-2, 5, -1];
    let lower = -2;
    let upper = 2;
    let iterations = 100_000;

    // Warm up
    for _ in 0..1000 {
        std::hint::black_box(my_solution::count_range_sum(
            std::hint::black_box(&nums),
            std::hint::black_box(lower),
            std::hint::black_box(upper),
        ));
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::count_range_sum(
            std::hint::black_box(&nums),
            std::hint::black_box(lower),
            std::hint::black_box(upper),
        ));
    }
    let my_dur = start.elapsed();

    // Warm up
    for _ in 0..1000 {
        std::hint::black_box(ai_solution::count_range_sum(
            std::hint::black_box(&nums),
            std::hint::black_box(lower),
            std::hint::black_box(upper),
        ));
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::count_range_sum(
            std::hint::black_box(&nums),
            std::hint::black_box(lower),
            std::hint::black_box(upper),
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

    println!("LC 327 - Count of Range Sum");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
