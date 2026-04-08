// LeetCode 138: Copy List with Random Pointer [Medium]
// https://leetcode.com/problems/copy-list-with-random-pointer/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 3.260us
//   AI solution avg: 2.076us
//   Ratio: AI 1.57x faster
//
// A linked list where each node has an additional random pointer that could
// point to any node or null. Construct a deep copy of the list.
//
// Constraints: 0 <= n <= 1000, -10^4 <= Node.val <= 10^4
//   random is null or points to some node in the list
//
// Run:  cargo run --example lc_0138_copy_list_random_pointer
// Test: cargo test --example lc_0138_copy_list_random_pointer
// Bench: cargo test --example lc_0138_copy_list_random_pointer bench -- --nocapture

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Node with next + random pointer.
#[derive(Debug, Clone)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub random: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            next: None,
            random: None,
        }
    }
}

type Link = Option<Rc<RefCell<Node>>>;

/// Build list from (val, random_index) pairs. random_index = -1 means null.
fn build_list(data: &[(i32, i32)]) -> Link {
    if data.is_empty() {
        return None;
    }
    let nodes: Vec<Rc<RefCell<Node>>> = data
        .iter()
        .map(|&(v, _)| Rc::new(RefCell::new(Node::new(v))))
        .collect();
    for i in 0..nodes.len() {
        if i + 1 < nodes.len() {
            nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
        }
        let ri = data[i].1;
        if ri >= 0 {
            nodes[i].borrow_mut().random = Some(Rc::clone(&nodes[ri as usize]));
        }
    }
    Some(Rc::clone(&nodes[0]))
}

/// Serialize list to (val, random_index) pairs for comparison.
fn to_pairs(head: &Link) -> Vec<(i32, i32)> {
    // First pass: assign index to each node by Rc pointer
    let mut indices: HashMap<usize, i32> = HashMap::new();
    let mut cur = head.clone();
    let mut idx = 0;
    while let Some(node) = cur {
        let ptr = Rc::as_ptr(&node) as usize;
        indices.insert(ptr, idx);
        idx += 1;
        cur = node.borrow().next.clone();
    }

    let mut result = Vec::new();
    let mut cur = head.clone();
    while let Some(node) = cur {
        let n = node.borrow();
        let ri = match &n.random {
            Some(r) => *indices.get(&(Rc::as_ptr(r) as usize)).unwrap_or(&-1),
            None => -1,
        };
        result.push((n.val, ri));
        cur = n.next.clone();
    }
    result
}

/// My solution — HashMap mapping old-node pointer to new-node, two passes.
///
/// Time: O(n).
/// Space: O(n) for the map.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Deep copy the list with random pointers.
        pub fn copy_random_list(head: Link) -> Link {
            let mut map: HashMap<usize, Rc<RefCell<Node>>> = HashMap::new();

            // First pass: create all new nodes, map old -> new by pointer
            let mut cur = head.clone();
            while let Some(node) = cur {
                let ptr = Rc::as_ptr(&node) as usize;
                let new_node = Rc::new(RefCell::new(Node::new(node.borrow().val)));
                map.insert(ptr, new_node);
                cur = node.borrow().next.clone();
            }

            // Second pass: set next and random pointers
            let mut cur = head;
            while let Some(node) = cur {
                let ptr = Rc::as_ptr(&node) as usize;
                let new_node = Rc::clone(map.get(&ptr).expect("mapped"));
                let n = node.borrow();
                if let Some(ref next) = n.next {
                    let next_ptr = Rc::as_ptr(next) as usize;
                    new_node.borrow_mut().next =
                        Some(Rc::clone(map.get(&next_ptr).expect("mapped")));
                }
                if let Some(ref rand) = n.random {
                    let rand_ptr = Rc::as_ptr(rand) as usize;
                    new_node.borrow_mut().random =
                        Some(Rc::clone(map.get(&rand_ptr).expect("mapped")));
                }
                cur = n.next.clone();
            }

            map.get(&map.keys().min().copied().unwrap_or(0))
                .map(Rc::clone)
                // Return the head of the new list
                .or_else(|| None)
        }
    }
}

/// AI solution — interleave cloned nodes, set random, then separate.
///
/// Time: O(n).
/// Space: O(1) extra (excluding output nodes).
mod ai_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Deep copy using node interleaving (O(1) extra space).
        pub fn copy_random_list(head: Link) -> Link {
            if head.is_none() {
                return None;
            }

            // Step 1: interleave — insert copy after each original node
            let mut cur = head.clone();
            while let Some(node) = cur {
                let n = node.borrow();
                let copy = Rc::new(RefCell::new(Node::new(n.val)));
                copy.borrow_mut().next = n.next.clone();
                drop(n);
                node.borrow_mut().next = Some(Rc::clone(&copy));
                cur = copy.borrow().next.clone();
            }

            // Step 2: set random pointers on copies
            let mut cur = head.clone();
            while let Some(node) = cur {
                let n = node.borrow();
                let copy = n.next.clone().expect("interleaved copy exists");
                if let Some(ref rand) = n.random {
                    // The copy of random is random->next
                    let rand_copy = rand.borrow().next.clone().expect("interleaved copy");
                    copy.borrow_mut().random = Some(rand_copy);
                }
                cur = copy.borrow().next.clone();
            }

            // Step 3: separate the two lists
            let new_head = head.as_ref().and_then(|h| h.borrow().next.clone());
            let mut cur = head;
            while let Some(node) = cur {
                let copy = node.borrow().next.clone().expect("copy exists");
                // Restore original next
                node.borrow_mut().next = copy.borrow().next.clone();
                // Set copy's next to the next copy
                let next_orig = copy.borrow().next.clone();
                copy.borrow_mut().next = next_orig.as_ref().and_then(|n| n.borrow().next.clone());
                cur = node.borrow().next.clone();
            }

            new_head
        }
    }
}

fn main() {
    let list = build_list(&[(7, -1), (13, 0), (11, 4), (10, 2), (1, 0)]);
    let my = my_solution::Solution::copy_random_list(list.clone());
    let ai = ai_solution::Solution::copy_random_list(build_list(&[
        (7, -1),
        (13, 0),
        (11, 4),
        (10, 2),
        (1, 0),
    ]));
    println!("my: {:?}", to_pairs(&my));
    println!("ai: {:?}", to_pairs(&ai));
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_copy {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    let data = vec![(7, -1), (13, 0), (11, 4), (10, 2), (1, 0)];
                    let list = build_list(&data);
                    let copied = <$solution>::copy_random_list(list);
                    assert_eq!(to_pairs(&copied), data);
                }

                #[test]
                fn test_two_nodes() {
                    let data = vec![(1, 1), (2, 1)];
                    let list = build_list(&data);
                    let copied = <$solution>::copy_random_list(list);
                    assert_eq!(to_pairs(&copied), data);
                }

                #[test]
                fn test_empty() {
                    let copied = <$solution>::copy_random_list(None);
                    assert!(copied.is_none());
                }

                #[test]
                fn test_self_random() {
                    let data = vec![(3, 0)];
                    let list = build_list(&data);
                    let copied = <$solution>::copy_random_list(list);
                    assert_eq!(to_pairs(&copied), data);
                }
            }
        };
    }

    test_copy!(my, crate::my_solution::Solution);
    test_copy!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let data: Vec<(i32, i32)> = (0..20)
            .map(|i| (i, if i % 3 == 0 { -1 } else { (i * 7) % 20 }))
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let list = build_list(&data);
            std::hint::black_box(crate::my_solution::Solution::copy_random_list(
                std::hint::black_box(list),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let list = build_list(&data);
            std::hint::black_box(crate::ai_solution::Solution::copy_random_list(
                std::hint::black_box(list),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("Both solutions are roughly equal");
        }
    }
}
