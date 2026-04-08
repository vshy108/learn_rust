// @lc app=leetcode id=369 lang=rust
// #[Medium] - Plus One Linked List
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.024us
// ai_solution  | avg: 0.024us
// Performance: My 1.02x faster

/// # 369. Plus One Linked List
///
/// Given a non-negative integer represented as a linked list of digits (most significant
/// at head), add one to the integer and return the result as a linked list.
///
/// Constraints:
/// - Number of nodes in [1, 100]
/// - 0 <= Node.val <= 9
/// - No leading zeros except 0 itself

/// ## My Solution — Reverse, add, reverse O(n)
///
/// Reverse the list, add 1 with carry propagation, then reverse back.
/// Uses a Vec<u8> to represent the linked list for simplicity.
///
/// - Time: O(n)
/// - Space: O(1) extra (in-place modification)
mod my_solution {
    pub fn plus_one(digits: &[u8]) -> Vec<u8> {
        let n = digits.len();
        let mut result = digits.to_vec();
        let mut carry = 1u8;

        // Process from least significant (rightmost) to most significant
        for i in (0..n).rev() {
            let sum = result[i] + carry;
            result[i] = sum % 10;
            carry = sum / 10;
            if carry == 0 {
                break; // No further propagation needed
            }
        }

        if carry > 0 {
            result.insert(0, carry);
        }
        result
    }
}

/// ## AI Solution — Find rightmost non-9, increment, zero the rest
///
/// Walk forward to find the rightmost digit that isn't 9. Increment it,
/// then set all subsequent digits to 0. If all digits are 9, prepend 1.
///
/// - Time: O(n)
/// - Space: O(1) extra
mod ai_solution {
    pub fn plus_one(digits: &[u8]) -> Vec<u8> {
        let mut result = digits.to_vec();
        let n = result.len();

        // Find the rightmost non-9 digit
        let mut target = -1i32; // index of rightmost non-9
        for i in (0..n).rev() {
            if result[i] != 9 {
                target = i as i32;
                break;
            }
        }

        if target < 0 {
            // All 9s — result is 1 followed by n zeros
            let mut ans = vec![0u8; n + 1];
            ans[0] = 1;
            return ans;
        }

        let t = target as usize;
        result[t] += 1;
        // Set everything after target to 0
        for i in (t + 1)..n {
            result[i] = 0;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[u8]) -> Vec<u8>) {
        assert_eq!(f(&[1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(f(&[0]), vec![1]);
        assert_eq!(f(&[9, 9, 9]), vec![1, 0, 0, 0]);
        assert_eq!(f(&[1, 9, 9]), vec![2, 0, 0]);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::plus_one);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::plus_one);
    }
}

fn main() {
    let digits: Vec<u8> = vec![1, 2, 3];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::plus_one(std::hint::black_box(&digits)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::plus_one(std::hint::black_box(&digits)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::plus_one(std::hint::black_box(&digits)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::plus_one(std::hint::black_box(&digits)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 369 - Plus One Linked List");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
