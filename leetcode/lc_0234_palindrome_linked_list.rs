// LeetCode 234: Palindrome Linked List [Easy]
// https://leetcode.com/problems/palindrome-linked-list/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 6.120us
//   AI solution avg: 5.723us
//   Ratio: AI 1.07x faster
//
// Given the head of a singly linked list, return true if it is a
// palindrome or false otherwise.
//
// Constraints: [1, 10^5] nodes, 0 <= Node.val <= 9
// Follow up: Could you do it in O(n) time and O(1) space?
//
// Run:  cargo run --example lc_0234_palindrome_linked_list
// Test: cargo test --example lc_0234_palindrome_linked_list
// Bench: cargo test --example lc_0234_palindrome_linked_list bench -- --nocapture

/// Singly-linked list node.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    /// Create a node with value `val` and no next pointer.
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

/// Build a linked list from a slice of values.
fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in vals.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

/// My solution — collect into Vec and check palindrome.
///
/// Simple approach: gather all values into a vector, then compare
/// first half with reversed second half.
///
/// Time: O(n).
/// Space: O(n) for the vector.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return true if the linked list is a palindrome.
        pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
            let mut vals = Vec::new();
            let mut current = &head;
            while let Some(node) = current {
                vals.push(node.val);
                current = &node.next;
            }
            // Compare the first half with the reversed second half
            let n = vals.len();
            for i in 0..n / 2 {
                if vals[i] != vals[n - 1 - i] {
                    return false;
                }
            }
            true
        }
    }
}

/// AI solution — fast/slow pointer to find middle, reverse second half.
///
/// 1. Use slow/fast pointers to find the middle of the list.
/// 2. Reverse the second half in-place.
/// 3. Compare the first half with the reversed second half.
///
/// This achieves O(1) extra space (ignoring the input mutation).
///
/// Time: O(n).
/// Space: O(1) extra (modifies the list in-place).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return true if the linked list is a palindrome.
        pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
            // Count the total length
            let mut len = 0;
            let mut cur = &head;
            while let Some(node) = cur {
                len += 1;
                cur = &node.next;
            }
            if len <= 1 {
                return true;
            }

            // Split the list at the midpoint: advance (len/2) nodes,
            // detach the second half
            let mut first_half = head;
            let mut ptr = &mut first_half;
            for _ in 0..len / 2 {
                ptr = &mut ptr.as_mut().expect("non-null during split").next;
            }
            // `ptr` now points to the link at position len/2
            let second_half = ptr.take(); // detach second half

            // Reverse the second half
            let mut reversed: Option<Box<ListNode>> = None;
            let mut cur = second_half;
            while let Some(mut node) = cur {
                cur = node.next.take();
                node.next = reversed;
                reversed = Some(node);
            }

            // Compare first half and reversed second half
            let mut a = &first_half;
            let mut b = &reversed;
            while let (Some(na), Some(nb)) = (a, b) {
                if na.val != nb.val {
                    return false;
                }
                a = &na.next;
                b = &nb.next;
            }

            true
        }
    }
}

fn main() {
    let cases: Vec<(Vec<i32>, bool)> = vec![
        (vec![1, 2, 2, 1], true),
        (vec![1, 2], false),
    ];

    for (vals, expected) in &cases {
        let my = my_solution::Solution::is_palindrome(build_list(vals));
        let ai = ai_solution::Solution::is_palindrome(build_list(vals));
        println!("{vals:?}: my={my}, ai={ai} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_palindrome {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    assert!(<$solution>::is_palindrome(build_list(&[1, 2, 2, 1])));
                }

                #[test]
                fn test_example2() {
                    assert!(!<$solution>::is_palindrome(build_list(&[1, 2])));
                }

                #[test]
                fn test_single() {
                    assert!(<$solution>::is_palindrome(build_list(&[1])));
                }

                #[test]
                fn test_odd_palindrome() {
                    assert!(<$solution>::is_palindrome(build_list(&[1, 2, 1])));
                }

                #[test]
                fn test_odd_not_palindrome() {
                    assert!(!<$solution>::is_palindrome(build_list(&[1, 2, 3])));
                }

                #[test]
                fn test_all_same() {
                    assert!(<$solution>::is_palindrome(build_list(&[5, 5, 5, 5, 5])));
                }
            }
        };
    }

    test_palindrome!(my, crate::my_solution::Solution);
    test_palindrome!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Build a 100-element palindrome list
        let mut vals: Vec<i32> = (0..50).collect();
        let rev: Vec<i32> = (0..50).rev().collect();
        vals.extend(rev);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let list = build_list(&vals);
            std::hint::black_box(crate::my_solution::Solution::is_palindrome(list));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let list = build_list(&vals);
            std::hint::black_box(crate::ai_solution::Solution::is_palindrome(list));
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
