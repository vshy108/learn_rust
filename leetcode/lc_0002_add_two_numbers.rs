// LeetCode 2: Add Two Numbers
// https://leetcode.com/problems/add-two-numbers/
//
// You are given two non-empty linked lists representing two non-negative
// integers. The digits are stored in reverse order, and each node contains
// a single digit. Add the two numbers and return the sum as a linked list.
//
// Example:
//   l1 = [2, 4, 3]  (represents 342)
//   l2 = [5, 6, 4]  (represents 465)
//   output = [7, 0, 8]  (represents 807, since 342 + 465 = 807)
//
// Run:  cargo run --example lc_0002_add_two_numbers
// Test: cargo test --example lc_0002_add_two_numbers
// Bench: cargo test --example lc_0002_add_two_numbers bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-03-26) ---
//   My solution (iterative): 216.771us avg (21.7s total)
//   AI solution (recursive): 245.583us avg (24.6s total)
//   My solution is 1.13x faster
//
// --- Rust concepts used ---
//
// Linked list with Option<Box<T>>:
//   `Option<Box<ListNode>>` is Rust's idiomatic linked list node pointer.
//   `Box<T>` is a heap-allocated smart pointer (needed because ListNode is
//   recursive and the compiler needs a known size).
//   `None` represents the end of the list (like null in other languages).
//
// Pattern matching on Option:
//   `Some(node)` — the list has a node.
//   `None` — end of list.
//   `as_deref()` — converts `&Option<Box<T>>` to `Option<&T>` for borrowing
//   without consuming the Box.
//
// Ownership with linked lists:
//   Each node owns its `next` via `Box`. When you move `node.next`, the
//   current node no longer owns it. Use `.take()` to move out of an Option
//   and leave `None` in its place.

/// Singly-linked list node, matching LeetCode's definition.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    /// Create a new ListNode with the given value and no next pointer.
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Helper: convert a slice of digits into a linked list.
///
/// Digits are in the same order as the linked list (reverse of the number).
/// e.g., `&[2, 4, 3]` builds 2 -> 4 -> 3 (representing 342).
fn vec_to_list(digits: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    // Build the list from the tail end so the first element becomes the head.
    for &d in digits.iter().rev() {
        let mut node = ListNode::new(d);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

/// Helper: convert a linked list into a Vec of digits for easy assertion.
fn list_to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(n) = node {
        result.push(n.val);
        node = n.next;
    }
    result
}

/// My solution module — implement your solution here.
///
/// Time: O(max(m, n)) — single pass through both lists simultaneously,
///   where m and n are the lengths of l1 and l2. Each iteration does O(1)
///   arithmetic and pointer operations.
///
/// Space: O(max(m, n)) — the result list has at most max(m, n) + 1 nodes
///   (the +1 is for a possible carry at the end).
mod my_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        pub fn add_two_numbers(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            // Dummy head node (value 0) — the real result starts at dummy.next.
            // This avoids special-casing the first node insertion.
            let mut dummy = ListNode::new(0);
            // Mutable reference that walks along the list as we build it.
            // Always points to the LAST node so we can append via .next.
            // `&mut dummy` borrows (not moves) so we can still access
            // dummy.next at the end to return the result.
            let mut current = &mut dummy;
            // Tracks overflow from digit addition (0 or 1, since max sum
            // per digit is 9 + 9 + 1 = 19).
            let mut carry = 0;

            // Move l1/l2 into mutable bindings so we can reassign them
            // in the loop. No .take() needed — move transfers ownership.
            let mut current_l1 = l1;
            let mut current_l2 = l2;

            // Loop while either list has nodes OR there's a carry to process.
            // e.g. [5] + [5]: both lists exhausted after 1 iteration, but
            // carry=1 means we need one more iteration to emit the [1] node.
            // A: check if Option is Some
            while current_l1.is_some() || current_l2.is_some() || carry != 0 {
                // Start sum with the previous carry (0 or 1).
                // Declared fresh each iteration so it doesn't accumulate.
                let mut sum = carry;

                // if let unwraps Some(Box<ListNode>) into `node`, skips if None.
                // After unwrapping, current_l1 is consumed (moved into `node`).
                // We reassign current_l1 = node.next to advance the pointer.
                if let Some(node) = current_l1 {
                    sum += node.val;
                    current_l1 = node.next;
                }

                // Same logic for l2. Using the same variable name `node` is
                // fine because each `if let` creates its own scope.
                if let Some(node) = current_l2 {
                    sum += node.val;
                    current_l2 = node.next;
                }

                // Integer division: e.g. 17 / 10 = 1, 8 / 10 = 0.
                carry = sum / 10;
                // Modulo extracts the ones digit: e.g. 17 % 10 = 7.
                // Wrap in Some(Box::new(...)) because current.next is
                // Option<Box<ListNode>>.
                current.next = Some(Box::new(ListNode::new(sum % 10)));
                // Advance current to the newly created node:
                //   .as_mut() converts &mut Option<Box<ListNode>>
                //       to Option<&mut Box<ListNode>>
                //   .unwrap() extracts the &mut Box<ListNode>
                //       (safe — we just assigned Some(...) above)
                // Now current points to the new tail node for the next
                // iteration.
                // A: obtain &mut Box<ListNode>
                current = current.next.as_mut().unwrap();
            }

            // Skip the dummy [0] node and return the real list.
            // No `return` keyword needed — last expression is the return value.
            dummy.next
        }
    }
}

/// AI solution module — recursive approach.
///
/// Instead of an iterative loop with a dummy head, this uses recursion:
/// each call processes one digit position, then recurses for the rest.
/// The base case is when both lists are exhausted AND carry is 0.
///
/// Tradeoff vs iterative:
///   - Cleaner: no dummy head, no mutable walking reference
///   - Uses O(max(m,n)) stack frames (could overflow on huge lists)
///   - Same time/space big-O: O(max(m,n))
mod ai_solution {
    use super::ListNode;

    pub struct Solution;

    impl Solution {
        /// Entry point — delegates to the recursive helper with carry = 0.
        pub fn add_two_numbers(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            Self::add_with_carry(l1, l2, 0)
        }

        /// Recursively build the result list one digit at a time.
        ///
        /// # Arguments
        ///
        /// * `l1` - Remaining nodes from the first list (or None)
        /// * `l2` - Remaining nodes from the second list (or None)
        /// * `carry` - Carry from the previous digit (0 or 1)
        fn add_with_carry(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
            carry: i32,
        ) -> Option<Box<ListNode>> {
            // Base case: nothing left to process.
            // Matching a tuple of (Option, Option, i32) lets us check
            // all three conditions in one pattern.
            if l1.is_none() && l2.is_none() && carry == 0 {
                return None;
            }

            let mut sum = carry;

            // Destructure each list: extract val, pass .next to recursion.
            // `map_or` returns the default (None) if the Option is None,
            // or applies the closure if Some.
            let next_l1 = l1.map_or(None, |node| {
                sum += node.val;
                node.next
            });
            let next_l2 = l2.map_or(None, |node| {
                sum += node.val;
                node.next
            });

            // Build the current digit node, then recurse for the rest.
            let mut node = ListNode::new(sum % 10);
            node.next = Self::add_with_carry(next_l1, next_l2, sum / 10);
            Some(Box::new(node))
        }
    }
}

fn main() {
    let l1 = vec_to_list(&[2, 4, 3]);
    let l2 = vec_to_list(&[5, 6, 4]);

    println!("--- My Solution ---");
    let result = my_solution::Solution::add_two_numbers(l1.clone(), l2.clone());
    println!("[2,4,3] + [5,6,4] = {:?}", list_to_vec(result));

    println!("--- AI Solution ---");
    let result = ai_solution::Solution::add_two_numbers(l1, l2);
    println!("[2,4,3] + [5,6,4] = {:?}", list_to_vec(result));
}

#[cfg(test)]
mod tests {
    /// Macro that generates the same set of tests for any Solution type.
    macro_rules! test_add_two_numbers {
        ($mod_name:ident, $solution:path) => {
        mod $mod_name {
            use $solution as Solution;
            use crate::{list_to_vec, vec_to_list};

            #[test]
            fn test_basic() {
                // 342 + 465 = 807
                let l1 = vec_to_list(&[2, 4, 3]);
                let l2 = vec_to_list(&[5, 6, 4]);
                let result = Solution::add_two_numbers(l1, l2);
                assert_eq!(list_to_vec(result), vec![7, 0, 8]);
            }

            #[test]
            fn test_zeros() {
                // 0 + 0 = 0
                let l1 = vec_to_list(&[0]);
                let l2 = vec_to_list(&[0]);
                let result = Solution::add_two_numbers(l1, l2);
                assert_eq!(list_to_vec(result), vec![0]);
            }

            #[test]
            fn test_different_lengths() {
                // 99 + 1 = 100
                let l1 = vec_to_list(&[9, 9]);
                let l2 = vec_to_list(&[1]);
                let result = Solution::add_two_numbers(l1, l2);
                assert_eq!(list_to_vec(result), vec![0, 0, 1]);
            }

            #[test]
            fn test_carry_propagation() {
                // 9999999 + 9999 = 10009998
                let l1 = vec_to_list(&[9, 9, 9, 9, 9, 9, 9]);
                let l2 = vec_to_list(&[9, 9, 9, 9]);
                let result = Solution::add_two_numbers(l1, l2);
                assert_eq!(
                    list_to_vec(result),
                    vec![8, 9, 9, 9, 0, 0, 0, 1]
                );
            }

            #[test]
            fn test_single_digits() {
                // 5 + 5 = 10
                let l1 = vec_to_list(&[5]);
                let l2 = vec_to_list(&[5]);
                let result = Solution::add_two_numbers(l1, l2);
                assert_eq!(list_to_vec(result), vec![0, 1]);
            }
        }
        };
    }

    // Run the SAME tests against both solutions.
    test_add_two_numbers!(my_tests, crate::my_solution::Solution);

    test_add_two_numbers!(ai_tests, crate::ai_solution::Solution);

    /// Benchmark both solutions side by side.
    ///
    /// Builds two long lists (1000 digits of 9s each) to stress-test.
    /// Use `cargo test --example lc_0002_add_two_numbers bench -- --nocapture`
    /// to see the output.
    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        // Two 1000-digit numbers, all 9s — maximizes carry propagation.
        let digits: Vec<i32> = vec![9; 1000];

        // --- Benchmark my_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::add_two_numbers(
                    std::hint::black_box(crate::vec_to_list(&digits)),
                    std::hint::black_box(crate::vec_to_list(&digits)),
                ),
            );
        }
        let my_duration = start.elapsed();

        // --- Benchmark ai_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::add_two_numbers(
                    std::hint::black_box(crate::vec_to_list(&digits)),
                    std::hint::black_box(crate::vec_to_list(&digits)),
                ),
            );
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution (iterative): {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution (recursive): {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        // Show which is faster and by how much.
        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64
                / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64
                / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
