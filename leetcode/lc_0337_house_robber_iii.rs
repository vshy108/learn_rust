// @lc app=leetcode id=337 lang=rust
// #[Medium] - House Robber III
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.017us
// ai_solution  | avg: 0.034us
// Performance: My 2.05x faster

/// # 337. House Robber III
///
/// Binary tree of houses. Cannot rob two directly-linked houses.
/// Return the maximum amount of money the thief can rob.
///
/// Constraints:
/// - Number of nodes in [1, 10^4]
/// - 0 <= Node.val <= 10^4
///
/// Tree stored as level-order Option<i32> array.

/// ## My Solution — DFS returning (rob, skip) pair
///
/// For each node return (max if robbed, max if skipped).
/// rob = val + left.skip + right.skip
/// skip = max(left.rob, left.skip) + max(right.rob, right.skip)
///
/// - Time: O(n)
/// - Space: O(n)
mod my_solution {
    pub fn rob(tree: &[Option<i32>]) -> i32 {
        if tree.is_empty() || tree[0].is_none() {
            return 0;
        }
        let (robbed, skipped) = dfs(tree, 0);
        robbed.max(skipped)
    }

    /// Returns (value if this node is robbed, value if skipped)
    fn dfs(tree: &[Option<i32>], idx: usize) -> (i32, i32) {
        if idx >= tree.len() || tree[idx].is_none() {
            return (0, 0);
        }
        let val = tree[idx].expect("checked Some");
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let (l_rob, l_skip) = dfs(tree, left);
        let (r_rob, r_skip) = dfs(tree, right);

        // Rob this node: must skip children
        let robbed = val + l_skip + r_skip;
        // Skip this node: take best of rob/skip from each child
        let skipped = l_rob.max(l_skip) + r_rob.max(r_skip);
        (robbed, skipped)
    }
}

/// ## AI Solution — Iterative bottom-up with array storage
///
/// Process nodes in reverse index order (children before parents).
/// Store (rob, skip) for each node in an array.
///
/// - Time: O(n)
/// - Space: O(n)
mod ai_solution {
    pub fn rob(tree: &[Option<i32>]) -> i32 {
        if tree.is_empty() || tree[0].is_none() {
            return 0;
        }
        let n = tree.len();
        // dp[i] = (rob_value, skip_value)
        let mut dp = vec![(0i32, 0i32); n];

        // Process from last index to first (children processed before parents)
        for idx in (0..n).rev() {
            if tree[idx].is_none() {
                continue;
            }
            let val = tree[idx].expect("checked Some");
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;

            let (l_rob, l_skip) = if left < n { dp[left] } else { (0, 0) };
            let (r_rob, r_skip) = if right < n { dp[right] } else { (0, 0) };

            dp[idx] = (
                val + l_skip + r_skip,                 // rob this node
                l_rob.max(l_skip) + r_rob.max(r_skip), // skip this node
            );
        }
        let (rob, skip) = dp[0];
        rob.max(skip)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[Option<i32>]) -> i32) {
        // Example 1: [3,2,3,null,3,null,1] => 3+3+1=7
        assert_eq!(
            f(&[Some(3), Some(2), Some(3), None, Some(3), None, Some(1)]),
            7
        );
        // Example 2: [3,4,5,1,3,null,1] => 4+5=9
        assert_eq!(
            f(&[Some(3), Some(4), Some(5), Some(1), Some(3), None, Some(1)]),
            9
        );
        // Single node
        assert_eq!(f(&[Some(5)]), 5);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::rob);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::rob);
    }
}

fn main() {
    let tree = vec![Some(3), Some(2), Some(3), None, Some(3), None, Some(1)];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::rob(std::hint::black_box(&tree)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::rob(std::hint::black_box(&tree)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::rob(std::hint::black_box(&tree)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::rob(std::hint::black_box(&tree)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 337 - House Robber III");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
