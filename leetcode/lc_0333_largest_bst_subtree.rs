// @lc app=leetcode id=333 lang=rust
// #[Medium] - Largest BST Subtree
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.025us
// ai_solution  | avg: 0.382us
// Performance: My 15.56x faster

/// # 333. Largest BST Subtree
///
/// Given a binary tree root, find the largest subtree which is also a BST.
/// Return the size (number of nodes) of the largest such subtree.
///
/// Constraints:
/// - Number of nodes in [0, 10^4]
/// - -10^4 <= Node.val <= 10^4
///
/// Represented as a flat array for benchmarking convenience.

/// ## My Solution — Bottom-up DFS returning (min, max, size)
///
/// For each node, return (subtree_min, subtree_max, bst_size).
/// If left.max < node.val < right.min, it's a valid BST subtree.
/// Otherwise return a sentinel indicating invalid BST.
///
/// - Time: O(n)
/// - Space: O(h) recursion depth
mod my_solution {
    /// Tree stored as flat array (Option<i32>), level-order.
    pub fn largest_bst_subtree(tree: &[Option<i32>]) -> i32 {
        if tree.is_empty() || tree[0].is_none() {
            return 0;
        }
        let mut ans = 0;
        dfs(tree, 0, &mut ans);
        ans
    }

    /// Returns (min_val, max_val, bst_size). bst_size=0 means not a BST.
    fn dfs(tree: &[Option<i32>], idx: usize, ans: &mut i32) -> (i64, i64, i32) {
        if idx >= tree.len() || tree[idx].is_none() {
            // Null node: valid BST of size 0 with inverted bounds
            return (i64::MAX, i64::MIN, 0);
        }
        let val = tree[idx].expect("checked non-none") as i64;
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let (l_min, l_max, l_sz) = dfs(tree, left, ans);
        let (r_min, r_max, r_sz) = dfs(tree, right, ans);

        // Check BST property: left subtree max < val < right subtree min
        if l_sz >= 0 && r_sz >= 0 && l_max < val && val < r_min {
            let size = l_sz + r_sz + 1;
            *ans = (*ans).max(size);
            return (l_min.min(val), r_max.max(val), size);
        }
        // Invalid BST — propagate with sentinel size = -1
        (i64::MIN, i64::MAX, -1)
    }
}

/// ## AI Solution — Iterative post-order with explicit stack
///
/// Same logic but uses an explicit stack to avoid recursion overhead.
///
/// - Time: O(n)
/// - Space: O(n)
mod ai_solution {
    use std::collections::HashMap;

    pub fn largest_bst_subtree(tree: &[Option<i32>]) -> i32 {
        if tree.is_empty() || tree[0].is_none() {
            return 0;
        }
        // Post-order: process children before parent
        // memo[idx] = (min, max, bst_size); bst_size<0 means invalid
        let mut memo: HashMap<usize, (i64, i64, i32)> = HashMap::new();
        let mut ans = 0i32;

        // Collect all valid node indices in reverse level-order (post-order)
        let mut indices: Vec<usize> = (0..tree.len())
            .filter(|&i| tree[i].is_some())
            .collect();
        // Process from largest index to smallest (children before parents)
        indices.sort_unstable_by(|a, b| b.cmp(a));

        for idx in indices {
            let val = tree[idx].expect("filtered") as i64;
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;

            let (l_min, l_max, l_sz) = if left < tree.len() && tree[left].is_some() {
                *memo.get(&left).unwrap_or(&(i64::MAX, i64::MIN, 0))
            } else {
                (i64::MAX, i64::MIN, 0)
            };
            let (r_min, r_max, r_sz) = if right < tree.len() && tree[right].is_some() {
                *memo.get(&right).unwrap_or(&(i64::MAX, i64::MIN, 0))
            } else {
                (i64::MAX, i64::MIN, 0)
            };

            if l_sz >= 0 && r_sz >= 0 && l_max < val && val < r_min {
                let size = l_sz + r_sz + 1;
                ans = ans.max(size);
                memo.insert(idx, (l_min.min(val), r_max.max(val), size));
            } else {
                memo.insert(idx, (i64::MIN, i64::MAX, -1));
            }
        }
        ans
    }
}

fn run_tests(f: fn(&[Option<i32>]) -> i32) {
    // Example 1: [10,5,15,1,8,null,7]
    // BST subtree rooted at 5: {1,5,8} size 3
    assert_eq!(
        f(&[Some(10), Some(5), Some(15), Some(1), Some(8), None, Some(7)]),
        3
    );
    // Example 2: [4,2,7,2,3,5,null,2,null,None,None,None,None,1]
    // Largest BST subtree has size 2
    assert_eq!(
        f(&[
            Some(4), Some(2), Some(7), Some(2), Some(3), Some(5), None,
            Some(2), None, None, None, None, None, Some(1),
        ]),
        2
    );
    // Empty tree
    assert_eq!(f(&[]), 0);
    // Single node
    assert_eq!(f(&[Some(1)]), 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::largest_bst_subtree);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::largest_bst_subtree);
    }
}

fn main() {
    let tree = vec![Some(10), Some(5), Some(15), Some(1), Some(8), None, Some(7)];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::largest_bst_subtree(
            std::hint::black_box(&tree),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::largest_bst_subtree(
            std::hint::black_box(&tree),
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::largest_bst_subtree(
            std::hint::black_box(&tree),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::largest_bst_subtree(
            std::hint::black_box(&tree),
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

    println!("LC 333 - Largest BST Subtree");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
