// @lc app=leetcode id=364 lang=rust
// #[Medium] - Nested List Weight Sum II
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.037us
// ai_solution  | avg: 0.011us
// Performance: AI 3.35x faster

/// # 364. Nested List Weight Sum II
///
/// Given a nested list of integers, return the sum of each integer multiplied by
/// its weight. The weight is maxDepth - depth + 1 (deeper integers weigh less).
///
/// Constraints:
/// - 1 <= nestedList.length <= 50
/// - Values in range [-100, 100]
/// - Max depth <= 50, no empty lists

// Shared nested-integer representation.
#[derive(Debug, Clone, PartialEq)]
enum NestedInt {
    Int(i32),
    List(Vec<NestedInt>),
}

/// ## My Solution — Two-pass DFS: find maxDepth, then weighted sum
///
/// First DFS to find max depth. Second DFS to compute
/// sum * (maxDepth - depth + 1).
///
/// - Time: O(n)
/// - Space: O(d) recursion depth
mod my_solution {
    use super::NestedInt;

    pub fn depth_sum_inverse(nested_list: &[NestedInt]) -> i32 {
        let max_depth = find_max_depth(nested_list, 1);
        weighted_sum(nested_list, 1, max_depth)
    }

    fn find_max_depth(list: &[NestedInt], depth: i32) -> i32 {
        let mut md = depth;
        for item in list {
            if let NestedInt::List(inner) = item {
                md = md.max(find_max_depth(inner, depth + 1));
            }
        }
        md
    }

    fn weighted_sum(list: &[NestedInt], depth: i32, max_depth: i32) -> i32 {
        let mut total = 0;
        for item in list {
            match item {
                NestedInt::Int(v) => total += v * (max_depth - depth + 1),
                NestedInt::List(inner) => {
                    total += weighted_sum(inner, depth + 1, max_depth);
                }
            }
        }
        total
    }
}

/// ## AI Solution — Single-pass: (maxDepth+1)*s - ws formula
///
/// Accumulate s (sum of all ints) and ws (sum of val*depth). Then answer
/// is (maxDepth+1)*s - ws.
///
/// Derivation: weight = maxDepth - d + 1, so sum of val*weight =
/// (maxDepth+1)*sum(val) - sum(val*d).
///
/// - Time: O(n)
/// - Space: O(d) recursion depth
mod ai_solution {
    use super::NestedInt;

    pub fn depth_sum_inverse(nested_list: &[NestedInt]) -> i32 {
        let (max_depth, s, ws) = dfs(nested_list, 1);
        (max_depth + 1) * s - ws
    }

    /// Returns (max_depth, sum_of_all_ints, sum_of_int*depth).
    fn dfs(list: &[NestedInt], depth: i32) -> (i32, i32, i32) {
        let mut max_depth = depth;
        let mut s = 0;
        let mut ws = 0;
        for item in list {
            match item {
                NestedInt::Int(v) => {
                    s += v;
                    ws += v * depth;
                }
                NestedInt::List(inner) => {
                    let (md, si, wsi) = dfs(inner, depth + 1);
                    max_depth = max_depth.max(md);
                    s += si;
                    ws += wsi;
                }
            }
        }
        (max_depth, s, ws)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[NestedInt]) -> i32) {
        use NestedInt::{Int, List};
        // [[1,1],2,[1,1]] => depth1: [1,1],[1,1]; depth2: 2 — wait, depth1 is outer
        // maxDepth = 2. Weights: outer=2, inner=1
        // Actually: 1*1 + 1*1 + 2*2 + 1*1 + 1*1 = 8
        let input1 = vec![
            List(vec![Int(1), Int(1)]),
            Int(2),
            List(vec![Int(1), Int(1)]),
        ];
        assert_eq!(f(&input1), 8);

        // [1,[4,[6]]] => depths: 1@1, 4@2, 6@3. maxDepth=3.
        // weights: 1*(3-1+1)=3, 4*(3-2+1)=8, 6*(3-3+1)=6 => 17
        let input2 = vec![Int(1), List(vec![Int(4), List(vec![Int(6)])])];
        assert_eq!(f(&input2), 17);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::depth_sum_inverse);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::depth_sum_inverse);
    }
}

fn main() {
    use NestedInt::{Int, List};
    let input = vec![
        List(vec![Int(1), Int(1)]),
        Int(2),
        List(vec![Int(1), Int(1)]),
    ];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::depth_sum_inverse(std::hint::black_box(&input)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::depth_sum_inverse(std::hint::black_box(&input)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::depth_sum_inverse(std::hint::black_box(&input)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::depth_sum_inverse(std::hint::black_box(&input)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 364 - Nested List Weight Sum II");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
