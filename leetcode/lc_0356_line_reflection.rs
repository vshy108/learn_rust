// @lc app=leetcode id=356 lang=rust
// #[Medium] - Line Reflection
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.093us
// ai_solution  | avg: 0.040us
// Performance: AI 2.34x faster

/// # 356. Line Reflection
///
/// Given n points on a 2D plane, check if there exists a vertical line that
/// symmetrically reflects all points.
///
/// Constraints:
/// - 1 <= n <= 10^4
/// - -10^8 <= points[i][j] <= 10^8

/// ## My Solution — HashSet + sum check O(n)
///
/// The reflection line x = (minX + maxX) / 2. For each point (x,y),
/// check that (sum - x, y) exists in the set, where sum = minX + maxX.
///
/// - Time: O(n)
/// - Space: O(n)
mod my_solution {
    use std::collections::HashSet;

    pub fn is_reflected(points: &[[i32; 2]]) -> bool {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        for p in points {
            min_x = min_x.min(p[0]);
            max_x = max_x.max(p[0]);
            set.insert((p[0], p[1]));
        }
        let sum = min_x as i64 + max_x as i64;
        points
            .iter()
            .all(|p| set.contains(&((sum - p[0] as i64) as i32, p[1])))
    }
}

/// ## AI Solution — Sort + two pointers
///
/// Sort points by (y, x). Group by y-coordinate. For each group,
/// check mirror symmetry with two pointers.
///
/// - Time: O(n log n)
/// - Space: O(n)
mod ai_solution {
    pub fn is_reflected(points: &[[i32; 2]]) -> bool {
        let mut pts: Vec<[i32; 2]> = points.to_vec();
        // Deduplicate first
        pts.sort_unstable();
        pts.dedup();

        let min_x = pts.iter().map(|p| p[0]).min().unwrap_or(0);
        let max_x = pts.iter().map(|p| p[0]).max().unwrap_or(0);
        let sum = min_x as i64 + max_x as i64;

        // Sort by (y, x) to group by y
        pts.sort_unstable_by(|a, b| a[1].cmp(&b[1]).then(a[0].cmp(&b[0])));

        let mut i = 0;
        while i < pts.len() {
            // Find range of same y
            let y = pts[i][1];
            let mut j = i;
            while j < pts.len() && pts[j][1] == y {
                j += 1;
            }
            // Two-pointer check for this y-group
            let mut lo = i;
            let mut hi = j - 1;
            while lo < hi {
                if pts[lo][0] as i64 + pts[hi][0] as i64 != sum {
                    return false;
                }
                lo += 1;
                hi -= 1;
            }
            // If odd count, middle point must be on the reflection line
            if lo == hi && pts[lo][0] as i64 * 2 != sum {
                return false;
            }
            i = j;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[[i32; 2]]) -> bool) {
        assert!(f(&[[1, 1], [-1, 1]]));
        assert!(!f(&[[1, 1], [-1, -1]]));
        // Duplicate points
        assert!(f(&[[0, 0], [0, 0]]));
        // Single point
        assert!(f(&[[5, 5]]));
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::is_reflected);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::is_reflected);
    }
}

fn main() {
    let points = vec![[1, 1], [-1, 1], [0, 0]];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::is_reflected(std::hint::black_box(&points)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::is_reflected(std::hint::black_box(&points)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::is_reflected(std::hint::black_box(&points)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::is_reflected(std::hint::black_box(&points)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 356 - Line Reflection");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
