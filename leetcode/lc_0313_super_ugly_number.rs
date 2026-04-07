// LeetCode 313. Super Ugly Number
// https://leetcode.com/problems/super-ugly-number/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 0.321us
//   ai_solution  avg: 0.175us
//   Ratio: ai_solution 1.84x faster

/// A super ugly number is a positive integer whose prime factors are in `primes`.
/// Given n and primes, return the nth super ugly number.
///
/// Example 1:
///   Input: n = 12, primes = [2,7,13,19]
///   Output: 32
///   Explanation: [1,2,4,7,8,13,14,16,19,26,28,32]
///
/// Example 2:
///   Input: n = 1, primes = [2,3,5]
///   Output: 1
///
/// Constraints:
///   - 1 <= n <= 10^5
///   - 1 <= primes.length <= 100
///   - 2 <= primes[i] <= 1000
///   - primes[i] is guaranteed to be a prime number
///   - All values of primes are unique and sorted ascending

/// # my_solution: Min-heap with Euler sieve optimization
/// Time: O(n * m * log(n*m)) where m = primes.len()
/// Space: O(n * m) for the heap
mod my_solution {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    pub struct Solution;

    impl Solution {
        /// Find the nth super ugly number using a min-heap.
        ///
        /// Push 1, then repeatedly pop the smallest and push its
        /// multiples with each prime. Use Euler sieve trick: if x % p == 0,
        /// stop pushing further primes to avoid duplicates.
        ///
        /// # Arguments
        /// * `n` - Which super ugly number to return
        /// * `primes` - Sorted list of prime factors
        ///
        /// # Returns
        /// The nth super ugly number
        pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
            let mut heap = BinaryHeap::new();
            heap.push(Reverse(1i64));
            let mx = i32::MAX as i64;
            let mut x = 0i64;

            for _ in 0..n {
                x = heap.pop().unwrap().0;
                // Remove duplicates from the heap
                while let Some(&Reverse(top)) = heap.peek() {
                    if top == x {
                        heap.pop();
                    } else {
                        break;
                    }
                }
                for &p in &primes {
                    let p = p as i64;
                    if x <= mx / p {
                        heap.push(Reverse(x * p));
                    }
                    // Euler sieve: if p divides x, don't push larger primes
                    // This avoids generating duplicates like 2*3 and 3*2
                    if x % p == 0 {
                        break;
                    }
                }
            }

            x as i32
        }
    }
}

/// # ai_solution: DP with pointer array
/// Time: O(n * m) where m = primes.len()
/// Space: O(n + m) for the ugly array and pointer array
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the nth super ugly number using multi-pointer DP.
        ///
        /// Maintain an array of pointers, one per prime. Each pointer tracks
        /// which ugly number that prime should next multiply. At each step,
        /// pick the minimum candidate and advance all pointers that produced it.
        ///
        /// # Arguments
        /// * `n` - Which super ugly number to return
        /// * `primes` - Sorted list of prime factors
        ///
        /// # Returns
        /// The nth super ugly number
        pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
            let n = n as usize;
            let m = primes.len();
            let mut ugly = vec![0i64; n];
            ugly[0] = 1;

            // idx[j] = index into ugly[] that primes[j] should next multiply
            let mut idx = vec![0usize; m];

            for i in 1..n {
                // Compute candidate for each prime
                let mut min_val = i64::MAX;
                for j in 0..m {
                    let candidate = ugly[idx[j]] * primes[j] as i64;
                    if candidate < min_val {
                        min_val = candidate;
                    }
                }
                ugly[i] = min_val;

                // Advance all pointers whose candidate equals the chosen minimum
                // (handles duplicates like 2*3 == 3*2)
                for j in 0..m {
                    if ugly[idx[j]] * primes[j] as i64 == min_val {
                        idx[j] += 1;
                    }
                }
            }

            ugly[n - 1] as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn run_tests(solve: fn(i32, Vec<i32>) -> i32) {
        // Example 1
        assert_eq!(solve(12, vec![2, 7, 13, 19]), 32);

        // Example 2: n=1 always returns 1
        assert_eq!(solve(1, vec![2, 3, 5]), 1);

        // Classic ugly numbers (primes = [2,3,5])
        assert_eq!(solve(10, vec![2, 3, 5]), 12);

        // Single prime
        assert_eq!(solve(5, vec![2]), 16); // 1, 2, 4, 8, 16

        // n=2 with single prime
        assert_eq!(solve(2, vec![3]), 3);

        // Larger n
        assert_eq!(solve(1, vec![2, 7, 13, 19]), 1);
    }

    #[test]
    fn test_my_solution() {
        run_tests(MySolution::nth_super_ugly_number);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(AiSolution::nth_super_ugly_number);
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let n = 12;
    let primes = vec![2, 7, 13, 19];
    println!("Input: n = {n}, primes = {primes:?}");
    println!(
        "Output (my): {}",
        my_solution::Solution::nth_super_ugly_number(n, primes.clone())
    );
    println!(
        "Output (ai): {}",
        ai_solution::Solution::nth_super_ugly_number(n, primes.clone())
    );

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(my_solution::Solution::nth_super_ugly_number(
            black_box(n),
            black_box(primes.clone()),
        ));
    }
    let my_dur = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(ai_solution::Solution::nth_super_ugly_number(
            black_box(n),
            black_box(primes.clone()),
        ));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    println!("\nBenchmark ({iterations} iterations, release mode):");
    println!("  my_solution  avg: {my_avg:.3}us");
    println!("  ai_solution  avg: {ai_avg:.3}us");
    if my_avg < ai_avg {
        println!("  my_solution is {:.2}x faster", ai_avg / my_avg);
    } else {
        println!("  ai_solution is {:.2}x faster", my_avg / ai_avg);
    }
}
