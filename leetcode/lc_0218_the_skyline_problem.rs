// LeetCode 218: The Skyline Problem [Hard]
// https://leetcode.com/problems/the-skyline-problem/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 6.529us
//   AI solution avg: 27.806us
//   Ratio: My 4.26x faster
//
// A city's skyline is the outer contour formed by all the buildings when
// viewed from a distance. Given buildings as [left, right, height],
// return the skyline as key points [[x, y], ...] sorted by x.
//
// Constraints: 1 <= buildings.length <= 10^4,
//              0 <= left < right <= 2^31 - 1,
//              1 <= height <= 2^31 - 1
//
// Run:  cargo run --example lc_0218_the_skyline_problem
// Test: cargo test --example lc_0218_the_skyline_problem
// Bench: cargo test --example lc_0218_the_skyline_problem bench -- --nocapture

/// My solution — line sweep with sorted events + BTreeMap as a multiset
/// to track active building heights.
///
/// Create two events per building: an "open" event at x=left with +height,
/// and a "close" event at x=right with -height. Sort events by x (ties
/// broken: opens before closes, taller opens first, shorter closes first).
/// Sweep through events, maintaining a BTreeMap<height, count>.
///
/// Time: O(n log n).
/// Space: O(n).
mod my_solution {
    use std::collections::BTreeMap;

    pub struct Solution;

    impl Solution {
        /// Return the skyline as a list of [x, height] key points.
        pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            // Build events: (x, height)
            // Negative height = building starts (open), Positive = building ends (close)
            // Using negative for opens so they sort before closes at the same x,
            // and taller opens sort first (more negative).
            let mut events: Vec<(i32, i32)> = Vec::with_capacity(buildings.len() * 2);
            for b in &buildings {
                let (left, right, height) = (b[0], b[1], b[2]);
                events.push((left, -height)); // open event
                events.push((right, height)); // close event
            }
            // Sort by x first, then by height value (negative opens come before
            // positive closes at the same x)
            events.sort_unstable();

            // BTreeMap acts as a multiset of active heights.
            // Key = height, Value = count of buildings at that height.
            let mut active: BTreeMap<i32, usize> = BTreeMap::new();
            active.insert(0, 1); // ground level always present

            let mut prev_max = 0;
            let mut result: Vec<Vec<i32>> = Vec::new();

            for (x, h) in events {
                if h < 0 {
                    // Open event: add height to active set
                    *active.entry(-h).or_insert(0) += 1;
                } else {
                    // Close event: remove one instance of this height
                    let count = active.get_mut(&h).expect("height must exist in active set");
                    *count -= 1;
                    if *count == 0 {
                        active.remove(&h);
                    }
                }

                // Current max height across all active buildings
                // BTreeMap's last key is the maximum
                let &cur_max = active
                    .keys()
                    .next_back()
                    .expect("active set always has ground level 0");

                if cur_max != prev_max {
                    result.push(vec![x, cur_max]);
                    prev_max = cur_max;
                }
            }

            result
        }
    }
}

/// AI solution — divide and conquer: recursively split buildings into
/// left and right halves, compute each half's skyline, then merge.
///
/// Merging two skylines is analogous to merging two sorted lists while
/// tracking the running maximum height from each side.
///
/// Time: O(n log n).
/// Space: O(n) for merge buffers and recursion stack O(log n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the skyline as a list of [x, height] key points.
        pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            if buildings.is_empty() {
                return Vec::new();
            }
            Self::divide(&buildings)
        }

        /// Recursively divide the buildings and merge the resulting skylines.
        fn divide(buildings: &[Vec<i32>]) -> Vec<Vec<i32>> {
            let n = buildings.len();
            if n == 1 {
                // Single building produces two key points:
                // [left, height] and [right, 0]
                let (left, right, height) = (buildings[0][0], buildings[0][1], buildings[0][2]);
                return vec![vec![left, height], vec![right, 0]];
            }
            let mid = n / 2;
            let left_sky = Self::divide(&buildings[..mid]);
            let right_sky = Self::divide(&buildings[mid..]);
            Self::merge(&left_sky, &right_sky)
        }

        /// Merge two skylines by walking through both simultaneously,
        /// always emitting the max height at each x position.
        fn merge(left: &[Vec<i32>], right: &[Vec<i32>]) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = Vec::with_capacity(left.len() + right.len());
            let (mut i, mut j) = (0, 0);
            // Track the current height contributed by each side
            let (mut left_h, mut right_h) = (0i32, 0i32);

            while i < left.len() && j < right.len() {
                let (lx, ly) = (left[i][0], left[i][1]);
                let (rx, ry) = (right[j][0], right[j][1]);

                let (x, max_h);
                if lx < rx {
                    left_h = ly;
                    max_h = left_h.max(right_h);
                    x = lx;
                    i += 1;
                } else if lx > rx {
                    right_h = ry;
                    max_h = left_h.max(right_h);
                    x = rx;
                    j += 1;
                } else {
                    // Same x — advance both
                    left_h = ly;
                    right_h = ry;
                    max_h = left_h.max(right_h);
                    x = lx;
                    i += 1;
                    j += 1;
                }

                // Only add point if it differs from the last emitted height
                // clippy::map_or_simplification: .map_or(true, |x| pred(x)) is
                // the same as .is_none_or(|x| pred(x)) — clearer intent that
                // we want true when None, or check a predicate when Some.
                if result.last().is_none_or(|last| last[1] != max_h) {
                    result.push(vec![x, max_h]);
                }
            }

            // Append remaining points from whichever side isn't exhausted
            Self::append_remaining(&mut result, left, i);
            Self::append_remaining(&mut result, right, j);
            result
        }

        /// Append remaining skyline points, deduplicating consecutively
        /// equal heights.
        fn append_remaining(result: &mut Vec<Vec<i32>>, sky: &[Vec<i32>], start: usize) {
            for point in &sky[start..] {
                // clippy::map_or_simplification (same as above)
                if result.last().is_none_or(|last| last[1] != point[1]) {
                    result.push(point.clone());
                }
            }
        }
    }
}

fn main() {
    let buildings = vec![
        vec![2, 9, 10],
        vec![3, 7, 15],
        vec![5, 12, 12],
        vec![15, 20, 10],
        vec![19, 24, 8],
    ];
    let my = my_solution::Solution::get_skyline(buildings.clone());
    let ai = ai_solution::Solution::get_skyline(buildings);
    println!("My solution: {my:?}");
    println!("AI solution: {ai:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_skyline {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    let buildings = vec![
                        vec![2, 9, 10],
                        vec![3, 7, 15],
                        vec![5, 12, 12],
                        vec![15, 20, 10],
                        vec![19, 24, 8],
                    ];
                    let expected = vec![
                        vec![2, 10],
                        vec![3, 15],
                        vec![7, 12],
                        vec![12, 0],
                        vec![15, 10],
                        vec![20, 8],
                        vec![24, 0],
                    ];
                    assert_eq!(<$solution>::get_skyline(buildings), expected);
                }

                #[test]
                fn test_example2() {
                    let buildings = vec![vec![0, 2, 3], vec![2, 5, 3]];
                    // Two adjacent buildings with same height merge into one stretch
                    let expected = vec![vec![0, 3], vec![5, 0]];
                    assert_eq!(<$solution>::get_skyline(buildings), expected);
                }

                #[test]
                fn test_single_building() {
                    let buildings = vec![vec![1, 5, 10]];
                    let expected = vec![vec![1, 10], vec![5, 0]];
                    assert_eq!(<$solution>::get_skyline(buildings), expected);
                }

                #[test]
                fn test_nested_buildings() {
                    // Taller building completely contains a shorter one
                    let buildings = vec![vec![1, 10, 20], vec![3, 7, 15]];
                    let expected = vec![vec![1, 20], vec![10, 0]];
                    assert_eq!(<$solution>::get_skyline(buildings), expected);
                }

                #[test]
                fn test_overlapping_same_height() {
                    let buildings = vec![vec![1, 5, 10], vec![3, 8, 10]];
                    let expected = vec![vec![1, 10], vec![8, 0]];
                    assert_eq!(<$solution>::get_skyline(buildings), expected);
                }

                #[test]
                fn test_non_overlapping() {
                    let buildings = vec![vec![1, 3, 5], vec![5, 8, 7]];
                    let expected = vec![vec![1, 5], vec![3, 0], vec![5, 7], vec![8, 0]];
                    assert_eq!(<$solution>::get_skyline(buildings), expected);
                }
            }
        };
    }

    test_skyline!(my, crate::my_solution::Solution);
    test_skyline!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Deterministic set of overlapping buildings
        let buildings: Vec<Vec<i32>> = (0..50)
            .map(|i| {
                let left = i * 3;
                let right = left + 10 + (i % 5);
                let height = 5 + (i * 7) % 30;
                vec![left, right, height]
            })
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::get_skyline(
                std::hint::black_box(buildings.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::get_skyline(
                std::hint::black_box(buildings.clone()),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
