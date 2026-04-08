// @lc app=leetcode id=362 lang=rust
// #[Medium] - Design Hit Counter
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.056us
// ai_solution  | avg: 0.026us
// Performance: AI 2.17x faster

/// # 362. Design Hit Counter
///
/// Design a hit counter that counts the number of hits received in the past 5 minutes
/// (300 seconds). Calls are made in chronological order (timestamp monotonically increasing).
///
/// Constraints:
/// - 1 <= timestamp <= 2 * 10^9
/// - At most 300 calls to hit and getHits

/// ## My Solution — Circular buffer O(1) hit, O(1) getHits
///
/// Use two arrays of size 300: one stores the timestamp, the other the hit count.
/// On hit, if the timestamp slot is current, increment; otherwise reset.
/// On getHits, sum only slots where the stored timestamp is within the window.
///
/// - Time: O(1) hit, O(300) = O(1) getHits
/// - Space: O(300) = O(1)
mod my_solution {
    pub struct HitCounter {
        times: [i32; 300],
        hits: [i32; 300],
    }

    impl HitCounter {
        pub fn new() -> Self {
            Self {
                times: [0; 300],
                hits: [0; 300],
            }
        }

        /// Record a hit at the given timestamp.
        pub fn hit(&mut self, timestamp: i32) {
            let idx = timestamp as usize % 300;
            if self.times[idx] != timestamp {
                // New time window for this slot — reset
                self.times[idx] = timestamp;
                self.hits[idx] = 1;
            } else {
                self.hits[idx] += 1;
            }
        }

        /// Return total hits in the past 300 seconds.
        pub fn get_hits(&self, timestamp: i32) -> i32 {
            let mut total = 0;
            for i in 0..300 {
                // Only count if the stored timestamp is within the window
                if timestamp - self.times[i] < 300 {
                    total += self.hits[i];
                }
            }
            total
        }
    }
}

/// ## AI Solution — Sorted vec + binary search
///
/// Store every timestamp in a vec. On getHits, binary search for the boundary.
///
/// - Time: O(1) hit, O(log n) getHits
/// - Space: O(n)
mod ai_solution {
    pub struct HitCounter {
        ts: Vec<i32>,
    }

    impl HitCounter {
        pub fn new() -> Self {
            Self { ts: Vec::new() }
        }

        pub fn hit(&mut self, timestamp: i32) {
            self.ts.push(timestamp);
        }

        pub fn get_hits(&self, timestamp: i32) -> i32 {
            let target = timestamp - 300 + 1;
            // Find first index >= target via binary search
            let lo = self.ts.partition_point(|&t| t < target);
            (self.ts.len() - lo) as i32
        }
    }
}

#[cfg(test)]
fn run_tests<T: HitCounterApi>(mut counter: T) {
    counter.hit(1);
    counter.hit(2);
    counter.hit(3);
    assert_eq!(counter.get_hits(4), 3);
    counter.hit(300);
    assert_eq!(counter.get_hits(300), 4);
    assert_eq!(counter.get_hits(301), 3);
}

/// Trait to unify both solutions for testing.
#[cfg(test)]
trait HitCounterApi {
    fn hit(&mut self, timestamp: i32);
    fn get_hits(&self, timestamp: i32) -> i32;
}

#[cfg(test)]
impl HitCounterApi for my_solution::HitCounter {
    fn hit(&mut self, timestamp: i32) {
        self.hit(timestamp);
    }
    fn get_hits(&self, timestamp: i32) -> i32 {
        self.get_hits(timestamp)
    }
}

#[cfg(test)]
impl HitCounterApi for ai_solution::HitCounter {
    fn hit(&mut self, timestamp: i32) {
        self.hit(timestamp);
    }
    fn get_hits(&self, timestamp: i32) -> i32 {
        self.get_hits(timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::HitCounter::new());
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::HitCounter::new());
    }
}

fn main() {
    let iterations = 100_000;

    // Benchmark my_solution
    for _ in 0..1000 {
        let mut c = my_solution::HitCounter::new();
        c.hit(1);
        c.hit(2);
        c.hit(3);
        c.hit(300);
        std::hint::black_box(c.get_hits(301));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut c = my_solution::HitCounter::new();
        c.hit(1);
        c.hit(2);
        c.hit(3);
        c.hit(300);
        std::hint::black_box(c.get_hits(301));
    }
    let my_dur = start.elapsed();

    // Benchmark ai_solution
    for _ in 0..1000 {
        let mut c = ai_solution::HitCounter::new();
        c.hit(1);
        c.hit(2);
        c.hit(3);
        c.hit(300);
        std::hint::black_box(c.get_hits(301));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut c = ai_solution::HitCounter::new();
        c.hit(1);
        c.hit(2);
        c.hit(3);
        c.hit(300);
        std::hint::black_box(c.get_hits(301));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 362 - Design Hit Counter");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
