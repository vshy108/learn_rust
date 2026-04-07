// @lc app=leetcode id=344 lang=rust
// #[Easy] - Reverse String
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.002us
// ai_solution  | avg: 0.003us
// Performance: My 1.22x faster

/// # 344. Reverse String
///
/// Reverse a string in-place. Input is a mutable array of chars.
///
/// Constraints:
/// - 1 <= s.length <= 10^5
/// - s[i] is a printable ASCII character

/// ## My Solution — Two pointers
///
/// Swap characters from both ends, moving inward.
///
/// - Time: O(n)
/// - Space: O(1)
mod my_solution {
    pub fn reverse_string(s: &mut [u8]) {
        let n = s.len();
        // Two pointers converging from ends
        for i in 0..n / 2 {
            s.swap(i, n - 1 - i);
        }
    }
}

/// ## AI Solution — std reverse
///
/// Use the standard library `reverse()` method on slices.
///
/// - Time: O(n)
/// - Space: O(1)
mod ai_solution {
    pub fn reverse_string(s: &mut [u8]) {
        s.reverse();
    }
}

fn run_tests(f: fn(&mut [u8])) {
    let mut s1 = b"hello".to_vec();
    f(&mut s1);
    assert_eq!(&s1, b"olleh");

    let mut s2 = b"Hannah".to_vec();
    f(&mut s2);
    assert_eq!(&s2, b"hannaH");

    let mut s3 = b"a".to_vec();
    f(&mut s3);
    assert_eq!(&s3, b"a");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::reverse_string);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::reverse_string);
    }
}

fn main() {
    let original = b"hello".to_vec();
    let iterations = 100_000;

    let mut s = original.clone();
    for _ in 0..1000 {
        my_solution::reverse_string(&mut s);
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        my_solution::reverse_string(std::hint::black_box(&mut s));
    }
    let my_dur = start.elapsed();

    let mut s = original.clone();
    for _ in 0..1000 {
        ai_solution::reverse_string(&mut s);
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        ai_solution::reverse_string(std::hint::black_box(&mut s));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 344 - Reverse String");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
