// @lc app=leetcode id=354 lang=rust
// #[Hard] - Russian Doll Envelopes
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.086us
// ai_solution  | avg: 0.099us
// Performance: My 1.14x faster

/// # 354. Russian Doll Envelopes
///
/// Given envelopes[i] = [w, h], find maximum nesting depth.
/// Envelope A fits in B iff A.w < B.w AND A.h < B.h. Cannot rotate.
///
/// Constraints:
/// - 1 <= envelopes.length <= 10^5
/// - envelopes[i].length == 2
/// - 1 <= w, h <= 10^5

/// ## My Solution — Sort + LIS with binary search
///
/// Sort by width ascending, then height descending (to prevent same-width nesting).
/// Find LIS on heights using patience sorting (binary search).
///
/// - Time: O(n log n)
/// - Space: O(n)
mod my_solution {
    pub fn max_envelopes(envelopes: &mut [[i32; 2]]) -> i32 {
        // Sort by width asc, then height desc (same width can't nest)
        envelopes.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));

        // LIS on heights via patience sorting
        let mut tails: Vec<i32> = Vec::new();
        for env in envelopes.iter() {
            let h = env[1];
            // Binary search for leftmost tail >= h
            match tails.binary_search(&h) {
                Ok(pos) => tails[pos] = h, // duplicate, replace in place
                Err(pos) => {
                    if pos == tails.len() {
                        tails.push(h);
                    } else {
                        tails[pos] = h;
                    }
                }
            }
        }
        tails.len() as i32
    }
}

/// ## AI Solution — DP O(n^2)
///
/// Sort by width asc, then height asc. DP: dp[i] = max chain ending at i.
/// For each i, check all j < i where both dims are strictly smaller.
///
/// - Time: O(n^2)
/// - Space: O(n)
mod ai_solution {
    pub fn max_envelopes(envelopes: &mut [[i32; 2]]) -> i32 {
        envelopes.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
        let n = envelopes.len();
        let mut dp = vec![1i32; n];
        let mut ans = 1;
        for i in 1..n {
            for j in 0..i {
                if envelopes[j][0] < envelopes[i][0]
                    && envelopes[j][1] < envelopes[i][1]
                {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            ans = ans.max(dp[i]);
        }
        ans
    }
}

fn run_tests(f: fn(&mut [[i32; 2]]) -> i32) {
    let mut e1 = vec![[5, 4], [6, 4], [6, 7], [2, 3]];
    assert_eq!(f(&mut e1), 3); // [2,3] => [5,4] => [6,7]

    let mut e2 = vec![[1, 1], [1, 1], [1, 1]];
    assert_eq!(f(&mut e2), 1);

    let mut e3 = vec![[1, 2], [2, 3], [3, 4], [4, 5]];
    assert_eq!(f(&mut e3), 4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::max_envelopes);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::max_envelopes);
    }
}

fn main() {
    let base = vec![[5, 4], [6, 4], [6, 7], [2, 3]];
    let iterations = 100_000;

    let mut data = base.clone();
    for _ in 0..1000 {
        let mut d = data.clone();
        std::hint::black_box(my_solution::max_envelopes(&mut d));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut d = data.clone();
        std::hint::black_box(my_solution::max_envelopes(&mut d));
    }
    let my_dur = start.elapsed();

    data = base.clone();
    for _ in 0..1000 {
        let mut d = data.clone();
        std::hint::black_box(ai_solution::max_envelopes(&mut d));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut d = data.clone();
        std::hint::black_box(ai_solution::max_envelopes(&mut d));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 354 - Russian Doll Envelopes");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
