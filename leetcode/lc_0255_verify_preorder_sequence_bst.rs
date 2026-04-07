// LeetCode 255: Verify Preorder Sequence in Binary Search Tree [Medium] (Premium)
// https://leetcode.com/problems/verify-preorder-sequence-in-binary-search-tree/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 1.632us
//   AI solution avg: 1.423us
//   Ratio: AI 1.15x faster
//
// Given an array of unique integers `preorder`, return true if it is the
// correct preorder traversal sequence of a binary search tree.
//
// Example 1: preorder = [5,2,1,3,6] -> true
// Example 2: preorder = [5,2,6,1,3] -> false
//
// Constraints:
//   - 1 <= preorder.length <= 10^4
//   - 1 <= preorder[i] <= 10^4
//   - All elements are unique
//
// Follow up: Could you do it using only constant space complexity?
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0255.Verify%20Preorder%20Sequence%20in%20Binary%20Search%20Tree/README_EN.md
//
// Run:  cargo run --example lc_0255_verify_preorder_sequence_bst
// Test: cargo test --example lc_0255_verify_preorder_sequence_bst
// Bench: cargo test --example lc_0255_verify_preorder_sequence_bst bench -- --nocapture

/// My solution — monotonic stack (from doocs Python).
///
/// Maintain a decreasing stack simulating the left spine of the BST.
/// `last` tracks the last popped value (lower bound for future nodes).
/// When we see a value > stack top, we pop (moving from left subtree
/// to right subtree), updating `last`. If any value < `last`, the
/// preorder is invalid.
///
/// Time: O(n)
/// Space: O(n)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if `preorder` is a valid BST preorder traversal.
        pub fn verify_preorder(preorder: Vec<i32>) -> bool {
            let mut stk: Vec<i32> = Vec::new();
            // `last` = lower bound: all future values must exceed this
            let mut last = i32::MIN;
            for x in preorder {
                if x < last {
                    return false;
                }
                // Pop while stack top < x (transitioning to right subtree)
                while let Some(&top) = stk.last() {
                    if top < x {
                        last = stk.pop().unwrap();
                    } else {
                        break;
                    }
                }
                stk.push(x);
            }
            true
        }
    }
}

/// AI solution — O(1) extra space using input array as stack.
///
/// Reuses the preorder array itself as the stack (follow-up: constant
/// space if we exclude the input). A stack pointer `i` tracks the
/// current stack top within the array.
///
/// Time: O(n)
/// Space: O(1) extra (modifies input)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if `preorder` is a valid BST preorder traversal.
        pub fn verify_preorder(mut preorder: Vec<i32>) -> bool {
            let mut last = i32::MIN;
            // `i` is the "stack pointer" into preorder itself
            let mut i: isize = -1;
            for idx in 0..preorder.len() {
                let x = preorder[idx];
                if x < last {
                    return false;
                }
                // Pop from our in-place stack while top < x
                while i >= 0 && preorder[i as usize] < x {
                    last = preorder[i as usize];
                    i -= 1;
                }
                // Push x onto the in-place stack
                i += 1;
                preorder[i as usize] = x;
            }
            true
        }
    }
}

fn main() {
    println!(
        "my [5,2,1,3,6]: {}",
        my_solution::Solution::verify_preorder(vec![5, 2, 1, 3, 6])
    );
    println!(
        "ai [5,2,6,1,3]: {}",
        ai_solution::Solution::verify_preorder(vec![5, 2, 6, 1, 3])
    );
}

#[cfg(test)]
mod tests {
    macro_rules! test_verify_preorder {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                #[test]
                fn test_example_1() {
                    assert!(Sol::verify_preorder(vec![5, 2, 1, 3, 6]));
                }

                #[test]
                fn test_example_2() {
                    assert!(!Sol::verify_preorder(vec![5, 2, 6, 1, 3]));
                }

                #[test]
                fn test_single() {
                    assert!(Sol::verify_preorder(vec![1]));
                }

                #[test]
                fn test_two_left() {
                    // BST:  2
                    //      /
                    //     1
                    assert!(Sol::verify_preorder(vec![2, 1]));
                }

                #[test]
                fn test_two_right() {
                    // BST: 1
                    //       \
                    //        2
                    assert!(Sol::verify_preorder(vec![1, 2]));
                }

                #[test]
                fn test_decreasing() {
                    // Left-skewed BST: 5->4->3->2->1
                    assert!(Sol::verify_preorder(vec![5, 4, 3, 2, 1]));
                }

                #[test]
                fn test_increasing() {
                    // Right-skewed BST: 1->2->3->4->5
                    assert!(Sol::verify_preorder(vec![1, 2, 3, 4, 5]));
                }

                #[test]
                fn test_invalid_back_to_left() {
                    // After going right from 3, seeing 1 is invalid
                    assert!(!Sol::verify_preorder(vec![3, 1, 4, 2]));
                }

                #[test]
                fn test_balanced() {
                    // BST:     4
                    //        /   \
                    //       2     6
                    //      / \   / \
                    //     1   3 5   7
                    assert!(Sol::verify_preorder(vec![4, 2, 1, 3, 6, 5, 7]));
                }

                #[test]
                fn test_invalid_violates_lower_bound() {
                    // After 2->4 (popped 2, lower bound=2), 1 < 2 is invalid
                    assert!(!Sol::verify_preorder(vec![2, 4, 1]));
                }
            }
        };
    }

    test_verify_preorder!(my, crate::my_solution::Solution);
    test_verify_preorder!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Build a valid preorder: left-skewed then right-skewed
        let preorder: Vec<i32> = (1..=500).rev().chain(501..=1000).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::verify_preorder(
                preorder.clone(),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::verify_preorder(
                preorder.clone(),
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
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
