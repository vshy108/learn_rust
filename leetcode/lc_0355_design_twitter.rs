// @lc app=leetcode id=355 lang=rust
// #[Medium] - Design Twitter
// Iterations: 100000 | Date: 2025-07-09
// my_solution  | avg: 0.616us
// ai_solution  | avg: 0.722us
// Performance: My 1.17x faster

/// # 355. Design Twitter
///
/// Design a simplified Twitter: postTweet, getNewsFeed (10 most recent from
/// self + followed users), follow, unfollow.
///
/// Constraints:
/// - 1 <= userId, followerId, followeeId <= 500
/// - 0 <= tweetId <= 10^4
/// - All tweets have unique IDs
/// - At most 3*10^4 total calls

/// Operations for the simulation
#[derive(Debug, Clone)]
enum Op {
    Post(i32, i32),       // userId, tweetId
    Feed(i32),            // userId -> Vec<i32>
    Follow(i32, i32),     // followerId, followeeId
    Unfollow(i32, i32),   // followerId, followeeId
}

/// ## My Solution — HashMap + global timestamp, sorted merge
///
/// Store per-user tweet list with timestamps. On getNewsFeed, collect tweets
/// from self + followees, sort by timestamp desc, take first 10.
///
/// - Time: O(T log T) per getNewsFeed where T = total tweets from followed users
/// - Space: O(users + tweets)
mod my_solution {
    use super::Op;
    use std::collections::{HashMap, HashSet};

    /// Process operations, return feed results.
    pub fn process(ops: &[Op]) -> Vec<Vec<i32>> {
        let mut tweets: HashMap<i32, Vec<(usize, i32)>> = HashMap::new(); // user -> [(time, id)]
        let mut following: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut time = 0usize;
        let mut results = Vec::new();

        for op in ops {
            match op {
                Op::Post(user, tweet) => {
                    tweets.entry(*user).or_default().push((time, *tweet));
                    time += 1;
                }
                Op::Feed(user) => {
                    let followees = following.get(user);
                    // Collect last 10 tweets from each relevant user
                    let mut all: Vec<(usize, i32)> = Vec::new();

                    // Self tweets
                    if let Some(ts) = tweets.get(user) {
                        let start = ts.len().saturating_sub(10);
                        all.extend_from_slice(&ts[start..]);
                    }

                    // Followee tweets
                    if let Some(set) = followees {
                        for &f in set {
                            if let Some(ts) = tweets.get(&f) {
                                let start = ts.len().saturating_sub(10);
                                all.extend_from_slice(&ts[start..]);
                            }
                        }
                    }

                    // Sort by time descending, take first 10
                    all.sort_unstable_by(|a, b| b.0.cmp(&a.0));
                    all.truncate(10);
                    results.push(all.into_iter().map(|(_, id)| id).collect());
                }
                Op::Follow(follower, followee) => {
                    following.entry(*follower).or_default().insert(*followee);
                }
                Op::Unfollow(follower, followee) => {
                    if let Some(set) = following.get_mut(follower) {
                        set.remove(followee);
                    }
                }
            }
        }
        results
    }
}

/// ## AI Solution — HashMap + BinaryHeap merge (k-way merge)
///
/// On getNewsFeed, use a max-heap to merge k sorted tweet streams.
/// Each user's tweets are in chronological order; iterate from end.
///
/// - Time: O(10 * log k) per getNewsFeed where k = number of followed users
/// - Space: O(users + tweets)
mod ai_solution {
    use super::Op;
    use std::collections::{BinaryHeap, HashMap, HashSet};

    /// Process operations, return feed results.
    pub fn process(ops: &[Op]) -> Vec<Vec<i32>> {
        let mut tweets: HashMap<i32, Vec<(usize, i32)>> = HashMap::new();
        let mut following: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut time = 0usize;
        let mut results = Vec::new();

        for op in ops {
            match op {
                Op::Post(user, tweet) => {
                    tweets.entry(*user).or_default().push((time, *tweet));
                    time += 1;
                }
                Op::Feed(user) => {
                    // Collect all relevant user IDs
                    let mut users: Vec<i32> = vec![*user];
                    if let Some(set) = following.get(user) {
                        users.extend(set.iter());
                    }

                    // Push the latest tweet index from each user into a max-heap
                    // Heap entry: (timestamp, tweet_id, user_id, index_in_list)
                    let mut heap: BinaryHeap<(usize, i32, i32, usize)> = BinaryHeap::new();
                    for &uid in &users {
                        if let Some(ts) = tweets.get(&uid) {
                            if !ts.is_empty() {
                                let idx = ts.len() - 1;
                                heap.push((ts[idx].0, ts[idx].1, uid, idx));
                            }
                        }
                    }

                    let mut feed = Vec::with_capacity(10);
                    while feed.len() < 10 {
                        if let Some((_, tweet_id, uid, idx)) = heap.pop() {
                            feed.push(tweet_id);
                            // Push previous tweet from same user
                            if idx > 0 {
                                let prev = idx - 1;
                                let ts = &tweets[&uid];
                                heap.push((ts[prev].0, ts[prev].1, uid, prev));
                            }
                        } else {
                            break;
                        }
                    }
                    results.push(feed);
                }
                Op::Follow(follower, followee) => {
                    following.entry(*follower).or_default().insert(*followee);
                }
                Op::Unfollow(follower, followee) => {
                    if let Some(set) = following.get_mut(follower) {
                        set.remove(followee);
                    }
                }
            }
        }
        results
    }
}

fn run_tests(f: fn(&[Op]) -> Vec<Vec<i32>>) {
    let ops = vec![
        Op::Post(1, 5),
        Op::Feed(1),          // [5]
        Op::Follow(1, 2),
        Op::Post(2, 6),
        Op::Feed(1),          // [6, 5]
        Op::Unfollow(1, 2),
        Op::Feed(1),          // [5]
    ];
    let result = f(&ops);
    assert_eq!(result[0], vec![5]);
    assert_eq!(result[1], vec![6, 5]);
    assert_eq!(result[2], vec![5]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::process);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::process);
    }
}

fn main() {
    let ops = vec![
        Op::Post(1, 5),
        Op::Follow(1, 2),
        Op::Post(2, 6),
        Op::Feed(1),
        Op::Unfollow(1, 2),
        Op::Feed(1),
    ];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::process(std::hint::black_box(&ops)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::process(std::hint::black_box(&ops)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::process(std::hint::black_box(&ops)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::process(std::hint::black_box(&ops)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 355 - Design Twitter");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
