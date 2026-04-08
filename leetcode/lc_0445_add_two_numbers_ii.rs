// LeetCode 445: Add Two Numbers II [Medium]
// https://leetcode.com/problems/add-two-numbers-ii/
//
// Given two non-empty linked lists representing two non-negative integers
// (most significant digit first), add the two numbers and return the sum
// as a linked list.
//
// Examples:
//   l1 = [7,2,4,3], l2 = [5,6,4] -> [7,8,0,7]
//   l1 = [2,4,3], l2 = [5,6,4] -> [8,0,7]
//   l1 = [0], l2 = [0] -> [0]
//
// Constraints:
//   1 <= number of nodes <= 100
//   0 <= Node.val <= 9
//   No leading zeros (except the number 0 itself)
//
// Run:  cargo run --example lc_0445_add_two_numbers_ii
// Test: cargo test --example lc_0445_add_two_numbers_ii
// Bench: cargo test --example lc_0445_add_two_numbers_ii bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 547ns
//   AI solution: avg 601ns
//   Ratio: 0.91x (reverse vs stack, similar)

/// Shared ListNode definition.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/// Build a linked list from a slice of values.
fn list_from(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

/// Convert a linked list to a Vec for easy comparison.
fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

/// My solution — reverse both lists, add, reverse result
/// (from doocs/leetcode Rust solution 1).
///
/// Reverse both input lists so digits are in least-significant-first order,
/// add them digit by digit with carry, then reverse the result.
///
/// Time:  O(m + n)
/// Space: O(max(m, n))
#[allow(dead_code)]
mod my_solution {
    use super::*;

    pub struct Solution;
    impl Solution {
        /// Add two numbers represented as linked lists (MSB first).
        ///
        /// # Arguments
        /// * `l1` - first number as linked list
        /// * `l2` - second number as linked list
        ///
        /// # Returns
        /// Sum as a linked list (MSB first).
        pub fn add_two_numbers(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut l1 = Self::reverse(l1);
            let mut l2 = Self::reverse(l2);
            // Use a dummy head; build result in reversed order
            let mut dummy = Some(Box::new(ListNode::new(0)));
            let mut cur = &mut dummy;
            let mut sum = 0;
            while l1.is_some() || l2.is_some() || sum != 0 {
                if let Some(node) = l1 {
                    sum += node.val;
                    l1 = node.next;
                }
                if let Some(node) = l2 {
                    sum += node.val;
                    l2 = node.next;
                }
                cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
                cur = &mut cur.as_mut().unwrap().next;
                sum /= 10;
            }
            Self::reverse(dummy.unwrap().next.take())
        }

        /// Reverse a singly linked list.
        fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            while let Some(mut node) = head {
                let next = node.next.take();
                node.next = prev.take();
                prev = Some(node);
                head = next;
            }
            prev
        }
    }
}

/// AI solution — use stacks (from doocs/leetcode Rust solution 2).
///
/// Push all digits onto two stacks, then pop and add with carry.
/// Build the result list by prepending nodes (no reverse needed).
///
/// Time:  O(m + n)
/// Space: O(m + n)
mod ai_solution {
    use super::*;

    pub struct Solution;
    impl Solution {
        /// Add two numbers using stacks to avoid reversing.
        pub fn add_two_numbers(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut s1 = Self::to_stack(l1);
            let mut s2 = Self::to_stack(l2);

            let mut dummy = Box::new(ListNode::new(0));
            let mut carry = 0;
            while !s1.is_empty() || !s2.is_empty() || carry != 0 {
                if let Some(val) = s1.pop() {
                    carry += val;
                }
                if let Some(val) = s2.pop() {
                    carry += val;
                }
                // Prepend new node: new_node -> dummy.next
                dummy.next = Some(Box::new(ListNode {
                    val: carry % 10,
                    next: dummy.next.take(),
                }));
                carry /= 10;
            }
            dummy.next.take()
        }

        /// Collect linked list values into a Vec (used as stack).
        fn to_stack(mut head: Option<Box<ListNode>>) -> Vec<i32> {
            let mut res = Vec::new();
            while let Some(node) = head {
                res.push(node.val);
                head = node.next;
            }
            res
        }
    }
}

fn main() {
    let l1 = list_from(&[7, 2, 4, 3]);
    let l2 = list_from(&[5, 6, 4]);
    println!(
        "my:  {:?}",
        list_to_vec(my_solution::Solution::add_two_numbers(
            l1.clone(),
            l2.clone()
        ))
    );
    println!(
        "ai:  {:?}",
        list_to_vec(ai_solution::Solution::add_two_numbers(l1, l2))
    );
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::*;
        use $module::Solution;

        #[test]
        fn test_example_1() {
            let l1 = list_from(&[7, 2, 4, 3]);
            let l2 = list_from(&[5, 6, 4]);
            let result = list_to_vec(Solution::add_two_numbers(l1, l2));
            assert_eq!(result, vec![7, 8, 0, 7]);
        }

        #[test]
        fn test_example_2() {
            let l1 = list_from(&[2, 4, 3]);
            let l2 = list_from(&[5, 6, 4]);
            let result = list_to_vec(Solution::add_two_numbers(l1, l2));
            assert_eq!(result, vec![8, 0, 7]);
        }

        #[test]
        fn test_zeros() {
            let l1 = list_from(&[0]);
            let l2 = list_from(&[0]);
            let result = list_to_vec(Solution::add_two_numbers(l1, l2));
            assert_eq!(result, vec![0]);
        }

        #[test]
        fn test_carry_propagation() {
            // 999 + 1 = 1000
            let l1 = list_from(&[9, 9, 9]);
            let l2 = list_from(&[1]);
            let result = list_to_vec(Solution::add_two_numbers(l1, l2));
            assert_eq!(result, vec![1, 0, 0, 0]);
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Option<Box<ListNode>>, Option<Box<ListNode>>) -> Option<Box<ListNode>>,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(list_from(&[7, 2, 4, 3])),
            std::hint::black_box(list_from(&[5, 6, 4])),
        ));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::add_two_numbers);
    bench_solution("AI solution", ai_solution::Solution::add_two_numbers);
}
