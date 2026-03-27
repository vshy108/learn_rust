// LeetCode 21: Merge Two Sorted Lists [Easy]
// https://leetcode.com/problems/merge-two-sorted-lists/
//
// You are given the heads of two sorted linked lists list1 and list2.
// Merge the two lists into one sorted list by splicing together the nodes.
// Return the head of the merged linked list.
//
// Examples:
//   list1 = [1,2,4], list2 = [1,3,4] -> [1,1,2,3,4,4]
//   list1 = [],      list2 = []      -> []
//   list1 = [],      list2 = [0]     -> [0]
//
// Constraints:
//   - The number of nodes in both lists is in [0, 50]
//   - -100 <= Node.val <= 100
//   - Both lists are sorted in non-decreasing order
//
// Run:  cargo run --example lc_0021_merge_two_sorted_lists
// Test: cargo test --example lc_0021_merge_two_sorted_lists
// Bench: cargo test --example lc_0021_merge_two_sorted_lists bench -- --nocapture
//
// Benchmark (100_000 iterations, 2026-03-27):
//   My solution (recursive):  85.28ms (avg 852ns)
//   AI solution (iterative):  94.42ms (avg 944ns)
//   -> My solution is 1.11x faster
//   Reason: For short lists (5 nodes each), recursion overhead is minimal and the compiler
//   inlines well; the iterative solution pays for extra .take()/.as_mut().unwrap() per node.
//
// --- Rust concepts used ---
//
// match on (Option, Option) tuple:
//   Destructures both lists simultaneously. (None, l2) catches (None, None) too,
//   because `l2` binds to any value. Arms are checked top-to-bottom.
//
// Recursive ownership transfer:
//   `Some(mut l1)` moves the Box out of the Option. After attaching the recursive
//   result to l1.next, we return Some(l1) — ownership flows back up the call stack.
//
// Dummy head / mutable borrow chain:
//   The iterative AI solution uses a dummy ListNode. `current = &mut dummy` walks
//   the list via mutable references without moving ownership of the head pointer.
//   `current.next.as_mut().unwrap()` re-borrows the next node mutably.

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    /// Create a new ListNode with the given value.
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Helper: build a linked list from a slice.
fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

/// Helper: collect linked list values into a Vec.
fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

/// My solution module.
///
/// Time:  O(m + n) — each recursive call processes one node; at most m + n calls
/// Space: O(m + n) — recursion stack depth equals total number of nodes
mod my_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Merge two sorted linked lists into one sorted list.
        ///
        /// # Arguments
        ///
        /// * `list1` - Head of the first sorted list
        /// * `list2` - Head of the second sorted list
        ///
        /// # Returns
        ///
        /// Head of the merged sorted list
        pub fn merge_two_lists(
            list1: Option<Box<ListNode>>,
            list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            // Rust matches arms top-to-bottom. (None, l2) binds l2 to *any* value,
            // so (None, None) is already handled here — l2 is None, returns None.
            // An explicit (None, None) arm would compile but is redundant.
            match (list1, list2) {
                (None, l2) => l2,
                (l1, None) => l1,
                (Some(mut l1), Some(mut l2)) => {
                    if l1.val <= l2.val {
                        l1.next = Solution::merge_two_lists(l1.next, Some(l2));
                        Some(l1)
                    } else {
                        l2.next = Solution::merge_two_lists(Some(l1), l2.next);
                        Some(l2)
                    }
                }
            }
        }
    }
}

/// AI solution module.
///
/// Time:  O(m + n) — single pass through both lists
/// Space: O(1) — iterative; only a dummy node and a mutable pointer
mod ai_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Merge two sorted linked lists into one sorted list.
        ///
        /// # Arguments
        ///
        /// * `list1` - Head of the first sorted list
        /// * `list2` - Head of the second sorted list
        ///
        /// # Returns
        ///
        /// Head of the merged sorted list
        pub fn merge_two_lists(
            mut list1: Option<Box<ListNode>>,
            mut list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            // Iterative approach using a dummy head node.
            // Avoids recursion stack overhead — O(1) extra space.
            let mut dummy = ListNode::new(0);
            let mut current = &mut dummy;

            // Walk both lists, always appending the smaller node.
            // `.as_ref()` borrows the inner Box<ListNode> as &Box<ListNode> without moving it
            // out of the Option. This lets us peek at l1.val and l2.val to compare, while
            // list1 and list2 still own their nodes for the `.take()` call below.
            while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
                if l1.val <= l2.val {
                    // `.take()` moves the value out of list1 and replaces it with None.
                    // We can't just assign `current.next = list1` because that would move
                    // the entire Option, leaving list1 unusable. `.take()` transfers
                    // ownership of the Box to current.next while resetting list1 to None.
                    current.next = list1.take();
                    // `.as_mut().unwrap()` borrows the just-appended node mutably:
                    // - `.as_mut()` converts Option<Box<ListNode>> -> Option<&mut Box<ListNode>>
                    //   (borrows without moving, since current.next must stay owned by the list)
                    // - `.unwrap()` is safe because we just assigned Some(..) to current.next
                    // This advances `current` to the tail of the merged list so far.
                    current = current.next.as_mut().unwrap();
                    // `.take()` on current.next detaches the rest of the original list1
                    // and reassigns it back to list1 for the next iteration.
                    list1 = current.next.take();
                } else {
                    current.next = list2.take();
                    current = current.next.as_mut().unwrap();
                    list2 = current.next.take();
                }
            }

            // Append whichever list has remaining nodes.
            current.next = if list1.is_some() { list1 } else { list2 };

            dummy.next
        }
    }
}

fn main() {
    let l1 = build_list(&[1, 2, 4]);
    let l2 = build_list(&[1, 3, 4]);
    let result = my_solution::Solution::merge_two_lists(l1, l2);
    println!("My:  {:?}", list_to_vec(result));

    let l1 = build_list(&[1, 2, 4]);
    let l2 = build_list(&[1, 3, 4]);
    let result = ai_solution::Solution::merge_two_lists(l1, l2);
    println!("AI:  {:?}", list_to_vec(result));
}

#[cfg(test)]
mod tests {
    use super::{build_list, list_to_vec};

    macro_rules! test_merge {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    let l1 = build_list(&[1, 2, 4]);
                    let l2 = build_list(&[1, 3, 4]);
                    assert_eq!(
                        list_to_vec(Solution::merge_two_lists(l1, l2)),
                        vec![1, 1, 2, 3, 4, 4]
                    );
                }

                #[test]
                fn test_both_empty() {
                    assert_eq!(
                        list_to_vec(Solution::merge_two_lists(None, None)),
                        Vec::<i32>::new()
                    );
                }

                #[test]
                fn test_one_empty() {
                    let l2 = build_list(&[0]);
                    assert_eq!(list_to_vec(Solution::merge_two_lists(None, l2)), vec![0]);
                }

                #[test]
                fn test_single_elements() {
                    let l1 = build_list(&[2]);
                    let l2 = build_list(&[1]);
                    assert_eq!(list_to_vec(Solution::merge_two_lists(l1, l2)), vec![1, 2]);
                }

                #[test]
                fn test_negative_values() {
                    let l1 = build_list(&[-3, -1, 4]);
                    let l2 = build_list(&[-2, 0, 5]);
                    assert_eq!(
                        list_to_vec(Solution::merge_two_lists(l1, l2)),
                        vec![-3, -2, -1, 0, 4, 5]
                    );
                }
            }
        };
    }

    test_merge!(my_tests, crate::my_solution::Solution);
    test_merge!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;

        let start = Instant::now();
        for _ in 0..iterations {
            let l1 = build_list(&[1, 2, 4, 6, 8]);
            let l2 = build_list(&[1, 3, 5, 7, 9]);
            std::hint::black_box(crate::my_solution::Solution::merge_two_lists(l1, l2));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let l1 = build_list(&[1, 2, 4, 6, 8]);
            let l2 = build_list(&[1, 3, 5, 7, 9]);
            std::hint::black_box(crate::ai_solution::Solution::merge_two_lists(l1, l2));
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
