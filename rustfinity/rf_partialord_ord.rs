//! # Rustfinity: Partialord Ord
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/partialord-ord
//!
//! # PartialOrd and Ord
//!
//! In Rust, comparing values for ordering (less than, greater than) is done through two traits: `PartialOrd` and `Ord`. These traits are the ordering counterparts to `PartialEq` and `Eq` that you learned about for equality comparison.
//!
//! `PartialOrd` defines a **partial ordering** where comparisons may not always produce a result. The classic example is floating-point numbers: comparing `NaN` to any value (including itself) returns `None` because `NaN` has no defined ordering. The trait requires implementing the `partial_cmp` method which returns `Option<Ordering>`.
//!
//! `Ord` defines a **total ordering** where every pair of values can be compared, and the result is always `Less`, `Equal`, or `Greater`. Types implementing `Ord` must also implement `PartialOrd`, `Eq`, and `PartialEq`. This is the trait used for sorting, binary search, and ordered collections like `BTreeMap`.
//!
//! ## Your Task
//!
//! Implement the following types and functions demonstrating ordering traits:
//!
//! 1. **`Score`** - A struct wrapping a `u32` that derives `PartialOrd` and `Ord` for natural ordering.
//!
//! 2. **`Version`** - A struct with `major`, `minor`, and `patch` fields (all `u32`). Implement `Ord` manually to compare versions semantically (major first, then minor, then patch).
//!
//! 3. **`Temperature`** - An enum with `Celsius(f64)` and `Fahrenheit(f64)` variants. Implement `PartialOrd` to compare temperatures after normalizing to Celsius. This should NOT implement `Ord` because floating-point values have no total ordering (NaN).
//!
//! 4. **`Priority`** - An enum with `Low`, `Medium`, `High`, and `Critical` variants. Derive `PartialOrd` and `Ord` where the order follows the declaration order (Low < Medium < High < Critical).
//!
//! 5. **`Player`** - A struct with `name: String` and `score: u32`. Implement `Ord` to sort by score descending (higher scores first), then by name ascending as a tiebreaker.
//!
//! 6. **`find_min<T: Ord>`** - Returns a reference to the minimum element in a non-empty slice, or `None` if empty.
//!
//! 7. **`find_max<T: Ord>`** - Returns a reference to the maximum element in a non-empty slice, or `None` if empty.
//!
//! 8. **`is_sorted<T: Ord>`** - Returns `true` if the slice is sorted in ascending order (each element `<=` the next).
//!
//! 9. **`clamp<T: Ord>`** - Returns a value clamped between min and max bounds (inclusive). If value `<` min, returns min. If value `>` max, returns max. Otherwise returns the value itself.
//!
//! ## Examples
//!
//! ```rust
//! use partialord_ord::*;
//!
//! // Score - derived ordering
//! let s1 = Score(100);
//! let s2 = Score(200);
//! assert!(s1 < s2);
//!
//! // Version - semantic versioning order
//! let v1 = Version::new(1, 9, 0);
//! let v2 = Version::new(2, 0, 0);
//! assert!(v1 < v2);
//!
//! // Temperature - partial ordering across units
//! let t1 = Temperature::Celsius(0.0);
//! let t2 = Temperature::Fahrenheit(32.0);  // 0°C
//! assert!(t1 == t2);  // Equal when normalized
//!
//! // Priority - enum ordering
//! assert!(Priority::Low < Priority::Critical);
//!
//! // Player - custom ordering (score desc, name asc)
//! let alice = Player::new("Alice", 100);
//! let bob = Player::new("Bob", 100);
//! assert!(alice < bob);  // Same score, alphabetical order
//!
//! // Utility functions
//! assert_eq!(find_min(&[3, 1, 4, 1, 5]), Some(&1));
//! assert_eq!(find_max(&[3, 1, 4, 1, 5]), Some(&5));
//! assert!(is_sorted(&[1, 2, 3, 4, 5]));
//! assert_eq!(clamp(&5, &1, &10), &5);
//! assert_eq!(clamp(&0, &1, &10), &1);
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here to reveal hints</summary>
//!
//! - Use `#[derive(PartialOrd, Ord)]` for simple types where field-by-field comparison is correct.
//! - When implementing `Ord` manually, you must implement the `cmp` method returning `std::cmp::Ordering`.
//! - The `Ordering` enum has variants: `Less`, `Equal`, `Greater`.
//! - For `Version`, compare fields in order of importance using `.cmp()` and `.then()` or `.then_with()`.
//! - For `Temperature`, convert to a common unit before comparing. Remember that `partial_cmp` returns `Option<Ordering>`.
//! - For `Player` with descending score, reverse the comparison order for scores.
//! - Use `.cmp()` for `Ord` types and `.partial_cmp()` for `PartialOrd` types.
//! - Derive macros for enums order variants by their declaration order (first variant is smallest).
//!
//! </details>
//!
//! Run: `cargo run --example rf_partialord_ord`
//! Test: `cargo test --example rf_partialord_ord`
//! Filter: `cargo test --example rf_partialord_ord <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::cmp::Ordering;

/// A simple score wrapper with derived ordering.
///
/// TODO: Add the appropriate derive macros for PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(pub u32);

/// A semantic version with major, minor, and patch components.
///
/// TODO: Implement PartialOrd and Ord manually to compare versions semantically
/// (major first, then minor, then patch).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    /// Creates a new Version with the given components.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version {
            major,
            minor,
            patch,
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!("compare versions partially")
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!("order versions")
    }
}

/// A temperature that can be in Celsius or Fahrenheit.
///
/// TODO: Implement PartialEq and PartialOrd to compare temperatures after
/// converting to a common unit. Do NOT implement Eq or Ord (floating-point
/// has no total ordering).
#[derive(Debug, Clone, Copy)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    /// Converts the temperature to Celsius.
    pub fn to_celsius(&self) -> f64 {
        // TODO
        unimplemented!()
    }
}

impl PartialEq for Temperature {
    fn eq(&self, other: &Self) -> bool {
        todo!("compare temperatures for equality")
    }
}

impl PartialOrd for Temperature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!("compare temperatures partially")
    }
}

/// A priority level for tasks or events.
///
/// TODO: Add the appropriate derive macros so that ordering follows the
/// declaration order: Low < Medium < High < Critical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// A player with a name and score.
///
/// TODO: Implement Ord to sort by score descending (higher scores first),
/// then by name ascending as a tiebreaker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
}

impl Player {
    /// Creates a new Player with the given name and score.
    pub fn new(name: &str, score: u32) -> Self {
        Player {
            name: name.to_string(),
            score,
        }
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!("compare players partially")
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!("order players")
    }
}

/// Returns a reference to the minimum element in a slice.
///
/// Returns `None` if the slice is empty.
pub fn find_min<T: Ord>(items: &[T]) -> Option<&T> {
    // TODO
    unimplemented!()
}

/// Returns a reference to the maximum element in a slice.
///
/// Returns `None` if the slice is empty.
pub fn find_max<T: Ord>(items: &[T]) -> Option<&T> {
    // TODO
    unimplemented!()
}

/// Checks if a slice is sorted in ascending order.
///
/// Returns `true` if the slice is sorted (each element <= the next).
/// Empty slices and single-element slices are considered sorted.
pub fn is_sorted<T: Ord>(items: &[T]) -> bool {
    // TODO
    unimplemented!()
}

/// Clamps a value to be within the given bounds (inclusive).
///
/// If the value is less than min, returns min.
/// If the value is greater than max, returns max.
/// Otherwise, returns the value itself.
pub fn clamp<'a, T: Ord>(value: &'a T, min: &'a T, max: &'a T) -> &'a T {
    // TODO
    unimplemented!()
}

// Example usage
pub fn main() {
    // Score - derived ordering
    let s1 = Score(100);
    let s2 = Score(200);
    println!("{:?} < {:?}: {}", s1, s2, s1 < s2);

    // Version - semantic versioning order
    let v1 = Version::new(1, 9, 0);
    let v2 = Version::new(2, 0, 0);
    println!("{:?} < {:?}: {}", v1, v2, v1 < v2);

    // Temperature - partial ordering across units
    let t1 = Temperature::Celsius(100.0);
    let t2 = Temperature::Fahrenheit(212.0);
    println!("{:?} == {:?}: {}", t1, t2, t1 == t2);

    // Priority - enum ordering
    println!(
        "Priority::Low < Priority::Critical: {}",
        Priority::Low < Priority::Critical
    );

    // Player - custom ordering
    let alice = Player::new("Alice", 100);
    let bob = Player::new("Bob", 200);
    println!("{:?} vs {:?}", alice, bob);

    // Utility functions
    let numbers = [3, 1, 4, 1, 5];
    println!("find_min: {:?}", find_min(&numbers));
    println!("find_max: {:?}", find_max(&numbers));
    println!("is_sorted: {}", is_sorted(&[1, 2, 3, 4, 5]));
    println!("clamp(5, 1, 10): {:?}", clamp(&5, &1, &10));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::cmp::Ordering;

    // ==================== Score Tests ====================

    #[test]
    fn score_less_than() {
        let s1 = Score(100);
        let s2 = Score(200);
        assert!(s1 < s2);
    }

    #[test]
    fn score_greater_than() {
        let s1 = Score(200);
        let s2 = Score(100);
        assert!(s1 > s2);
    }

    #[test]
    fn score_equal() {
        let s1 = Score(100);
        let s2 = Score(100);
        assert_eq!(s1, s2);
        assert!(!(s1 < s2));
        assert!(!(s1 > s2));
    }

    #[test]
    fn score_cmp() {
        assert_eq!(Score(100).cmp(&Score(200)), Ordering::Less);
        assert_eq!(Score(200).cmp(&Score(100)), Ordering::Greater);
        assert_eq!(Score(100).cmp(&Score(100)), Ordering::Equal);
    }

    #[test]
    fn score_zero() {
        assert_eq!(Score(0).cmp(&Score(0)), Ordering::Equal);
        assert!(Score(0) < Score(1));
    }

    #[test]
    fn score_max() {
        let max = Score(u32::MAX);
        let almost_max = Score(u32::MAX - 1);
        assert!(almost_max < max);
    }

    // ==================== Version Tests ====================

    #[test]
    fn version_major_comparison() {
        let v1 = Version::new(1, 9, 9);
        let v2 = Version::new(2, 0, 0);
        assert!(v1 < v2);
    }

    #[test]
    fn version_minor_comparison() {
        let v1 = Version::new(1, 0, 9);
        let v2 = Version::new(1, 1, 0);
        assert!(v1 < v2);
    }

    #[test]
    fn version_patch_comparison() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 0, 1);
        assert!(v1 < v2);
    }

    #[test]
    fn version_equal() {
        let v1 = Version::new(2, 3, 4);
        let v2 = Version::new(2, 3, 4);
        assert_eq!(v1, v2);
        assert_eq!(v1.cmp(&v2), Ordering::Equal);
    }

    #[test]
    fn version_cmp() {
        assert_eq!(
            Version::new(1, 0, 0).cmp(&Version::new(2, 0, 0)),
            Ordering::Less
        );
        assert_eq!(
            Version::new(2, 0, 0).cmp(&Version::new(1, 0, 0)),
            Ordering::Greater
        );
    }

    #[test]
    fn version_zero() {
        let v = Version::new(0, 0, 0);
        assert_eq!(v.cmp(&v), Ordering::Equal);
        assert!(v < Version::new(0, 0, 1));
    }

    #[test]
    fn version_sorting() {
        let mut versions = vec![
            Version::new(2, 0, 0),
            Version::new(1, 0, 0),
            Version::new(1, 1, 0),
            Version::new(1, 0, 1),
        ];
        versions.sort();
        assert_eq!(
            versions,
            vec![
                Version::new(1, 0, 0),
                Version::new(1, 0, 1),
                Version::new(1, 1, 0),
                Version::new(2, 0, 0),
            ]
        );
    }

    #[test]
    fn version_partial_cmp() {
        let v1 = Version::new(1, 2, 3);
        let v2 = Version::new(1, 2, 4);
        assert_eq!(v1.partial_cmp(&v2), Some(Ordering::Less));
    }

    // ==================== Temperature Tests ====================

    #[test]
    fn temperature_celsius_comparison() {
        let t1 = Temperature::Celsius(20.0);
        let t2 = Temperature::Celsius(25.0);
        assert!(t1 < t2);
    }

    #[test]
    fn temperature_fahrenheit_comparison() {
        let t1 = Temperature::Fahrenheit(68.0); // 20°C
        let t2 = Temperature::Fahrenheit(77.0); // 25°C
        assert!(t1 < t2);
    }

    #[test]
    fn temperature_cross_unit_equal() {
        let c = Temperature::Celsius(0.0);
        let f = Temperature::Fahrenheit(32.0);
        assert_eq!(c, f);
    }

    #[test]
    fn temperature_cross_unit_boiling() {
        let c = Temperature::Celsius(100.0);
        let f = Temperature::Fahrenheit(212.0);
        assert_eq!(c, f);
    }

    #[test]
    fn temperature_cross_unit_comparison() {
        let c = Temperature::Celsius(20.0);
        let f = Temperature::Fahrenheit(77.0); // 25°C
        assert!(c < f);
    }

    #[test]
    fn temperature_negative() {
        let t1 = Temperature::Celsius(-10.0);
        let t2 = Temperature::Celsius(0.0);
        assert!(t1 < t2);
    }

    #[test]
    fn temperature_fahrenheit_to_celsius() {
        let t = Temperature::Fahrenheit(32.0);
        assert!((t.to_celsius() - 0.0).abs() < 0.0001);

        let t2 = Temperature::Fahrenheit(212.0);
        assert!((t2.to_celsius() - 100.0).abs() < 0.0001);
    }

    #[test]
    fn temperature_partial_cmp() {
        let t1 = Temperature::Celsius(20.0);
        let t2 = Temperature::Celsius(25.0);
        assert_eq!(t1.partial_cmp(&t2), Some(Ordering::Less));
        assert_eq!(t2.partial_cmp(&t1), Some(Ordering::Greater));
    }

    // ==================== Priority Tests ====================

    #[test]
    fn priority_ordering() {
        assert!(Priority::Low < Priority::Medium);
        assert!(Priority::Medium < Priority::High);
        assert!(Priority::High < Priority::Critical);
    }

    #[test]
    fn priority_equal() {
        assert_eq!(Priority::Low, Priority::Low);
        assert_eq!(Priority::Critical, Priority::Critical);
    }

    #[test]
    fn priority_cmp() {
        assert_eq!(Priority::Low.cmp(&Priority::Critical), Ordering::Less);
        assert_eq!(Priority::Critical.cmp(&Priority::Low), Ordering::Greater);
        assert_eq!(Priority::High.cmp(&Priority::High), Ordering::Equal);
    }

    #[test]
    fn priority_sorting() {
        let mut priorities = vec![
            Priority::Critical,
            Priority::Low,
            Priority::High,
            Priority::Medium,
        ];
        priorities.sort();
        assert_eq!(
            priorities,
            vec![
                Priority::Low,
                Priority::Medium,
                Priority::High,
                Priority::Critical
            ]
        );
    }

    #[test]
    fn priority_min_max() {
        let priorities = [
            Priority::Medium,
            Priority::Low,
            Priority::Critical,
            Priority::High,
        ];
        assert_eq!(priorities.iter().min(), Some(&Priority::Low));
        assert_eq!(priorities.iter().max(), Some(&Priority::Critical));
    }

    // ==================== Player Tests ====================

    #[test]
    fn player_higher_score_first() {
        let alice = Player::new("Alice", 100);
        let bob = Player::new("Bob", 200);
        assert!(bob < alice); // Bob has higher score, comes first (is "less" in sort order)
    }

    #[test]
    fn player_same_score_alphabetical() {
        let alice = Player::new("Alice", 100);
        let bob = Player::new("Bob", 100);
        assert!(alice < bob); // Same score, Alice < Bob alphabetically
    }

    #[test]
    fn player_equal() {
        let p1 = Player::new("Alice", 100);
        let p2 = Player::new("Alice", 100);
        assert_eq!(p1, p2);
    }

    #[test]
    fn player_sorting() {
        let mut players = vec![
            Player::new("Charlie", 50),
            Player::new("Alice", 100),
            Player::new("Bob", 100),
            Player::new("Diana", 200),
        ];
        players.sort();
        assert_eq!(
            players,
            vec![
                Player::new("Diana", 200),  // Highest score first
                Player::new("Alice", 100),  // Same score, alphabetical
                Player::new("Bob", 100),    // Same score, alphabetical
                Player::new("Charlie", 50), // Lowest score last
            ]
        );
    }

    #[test]
    fn player_cmp() {
        let alice = Player::new("Alice", 100);
        let bob = Player::new("Bob", 200);
        assert_eq!(bob.cmp(&alice), Ordering::Less); // Bob comes first (higher score)
    }

    #[test]
    fn player_zero_score() {
        let p1 = Player::new("Alpha", 0);
        let p2 = Player::new("Beta", 0);
        assert!(p1 < p2); // Alphabetical when scores equal
    }

    // ==================== find_min Tests ====================

    #[test]
    fn find_min_basic() {
        assert_eq!(find_min(&[3, 1, 4, 1, 5]), Some(&1));
    }

    #[test]
    fn find_min_single() {
        assert_eq!(find_min(&[42]), Some(&42));
    }

    #[test]
    fn find_min_empty() {
        assert_eq!(find_min::<i32>(&[]), None);
    }

    #[test]
    fn find_min_negative() {
        assert_eq!(find_min(&[-5, -2, 0, 3]), Some(&-5));
    }

    #[test]
    fn find_min_strings() {
        assert_eq!(find_min(&["cherry", "apple", "banana"]), Some(&"apple"));
    }

    #[test]
    fn find_min_with_scores() {
        let scores = [Score(50), Score(25), Score(75)];
        assert_eq!(find_min(&scores), Some(&Score(25)));
    }

    #[test]
    fn find_min_all_same() {
        assert_eq!(find_min(&[5, 5, 5, 5]), Some(&5));
    }

    // ==================== find_max Tests ====================

    #[test]
    fn find_max_basic() {
        assert_eq!(find_max(&[3, 1, 4, 1, 5]), Some(&5));
    }

    #[test]
    fn find_max_single() {
        assert_eq!(find_max(&[42]), Some(&42));
    }

    #[test]
    fn find_max_empty() {
        assert_eq!(find_max::<i32>(&[]), None);
    }

    #[test]
    fn find_max_negative() {
        assert_eq!(find_max(&[-5, -2, 0, 3]), Some(&3));
    }

    #[test]
    fn find_max_strings() {
        assert_eq!(find_max(&["cherry", "apple", "banana"]), Some(&"cherry"));
    }

    #[test]
    fn find_max_with_scores() {
        let scores = [Score(50), Score(25), Score(75)];
        assert_eq!(find_max(&scores), Some(&Score(75)));
    }

    #[test]
    fn find_max_all_same() {
        assert_eq!(find_max(&[5, 5, 5, 5]), Some(&5));
    }

    // ==================== is_sorted Tests ====================

    #[test]
    fn is_sorted_ascending() {
        assert!(is_sorted(&[1, 2, 3, 4, 5]));
    }

    #[test]
    fn is_sorted_with_duplicates() {
        assert!(is_sorted(&[1, 1, 2, 2, 3]));
    }

    #[test]
    fn is_sorted_not_sorted() {
        assert!(!is_sorted(&[1, 3, 2]));
    }

    #[test]
    fn is_sorted_empty() {
        assert!(is_sorted::<i32>(&[]));
    }

    #[test]
    fn is_sorted_single() {
        assert!(is_sorted(&[42]));
    }

    #[test]
    fn is_sorted_all_same() {
        assert!(is_sorted(&[5, 5, 5, 5]));
    }

    #[test]
    fn is_sorted_descending() {
        assert!(!is_sorted(&[5, 4, 3, 2, 1]));
    }

    #[test]
    fn is_sorted_strings() {
        assert!(is_sorted(&["apple", "banana", "cherry"]));
        assert!(!is_sorted(&["cherry", "apple", "banana"]));
    }

    #[test]
    fn is_sorted_versions() {
        let versions = [
            Version::new(1, 0, 0),
            Version::new(1, 1, 0),
            Version::new(2, 0, 0),
        ];
        assert!(is_sorted(&versions));
    }

    // ==================== clamp Tests ====================

    #[test]
    fn clamp_within_bounds() {
        assert_eq!(clamp(&5, &1, &10), &5);
    }

    #[test]
    fn clamp_below_min() {
        assert_eq!(clamp(&0, &1, &10), &1);
    }

    #[test]
    fn clamp_above_max() {
        assert_eq!(clamp(&15, &1, &10), &10);
    }

    #[test]
    fn clamp_at_min() {
        assert_eq!(clamp(&1, &1, &10), &1);
    }

    #[test]
    fn clamp_at_max() {
        assert_eq!(clamp(&10, &1, &10), &10);
    }

    #[test]
    fn clamp_negative_range() {
        assert_eq!(clamp(&-5, &-10, &-1), &-5);
        assert_eq!(clamp(&-15, &-10, &-1), &-10);
        assert_eq!(clamp(&0, &-10, &-1), &-1);
    }

    #[test]
    fn clamp_single_value_range() {
        assert_eq!(clamp(&0, &5, &5), &5);
        assert_eq!(clamp(&5, &5, &5), &5);
        assert_eq!(clamp(&10, &5, &5), &5);
    }

    #[test]
    fn clamp_strings() {
        assert_eq!(clamp(&"dog", &"ant", &"zebra"), &"dog");
        assert_eq!(clamp(&"aardvark", &"ant", &"zebra"), &"ant");
    }

    #[test]
    fn clamp_scores() {
        let value = Score(50);
        let min = Score(25);
        let max = Score(75);
        assert_eq!(clamp(&value, &min, &max), &Score(50));
        assert_eq!(clamp(&Score(10), &min, &max), &Score(25));
        assert_eq!(clamp(&Score(100), &min, &max), &Score(75));
    }

    // ==================== Integration Tests ====================

    #[test]
    fn integration_version_min_max() {
        let versions = [
            Version::new(1, 0, 0),
            Version::new(2, 1, 0),
            Version::new(1, 5, 3),
            Version::new(0, 9, 9),
        ];
        assert_eq!(find_min(&versions), Some(&Version::new(0, 9, 9)));
        assert_eq!(find_max(&versions), Some(&Version::new(2, 1, 0)));
    }

    #[test]
    fn integration_player_leaderboard() {
        let mut players = vec![
            Player::new("Zoe", 80),
            Player::new("Alice", 100),
            Player::new("Bob", 100),
            Player::new("Charlie", 120),
        ];
        players.sort();

        // Verify order: highest score first, then alphabetical
        assert_eq!(players[0].name, "Charlie");
        assert_eq!(players[1].name, "Alice");
        assert_eq!(players[2].name, "Bob");
        assert_eq!(players[3].name, "Zoe");
    }

    #[test]
    fn integration_priority_queue_order() {
        let mut tasks: Vec<(Priority, &str)> = vec![
            (Priority::Low, "cleanup"),
            (Priority::Critical, "security_fix"),
            (Priority::High, "feature"),
            (Priority::Medium, "refactor"),
        ];
        tasks.sort_by(|a, b| b.0.cmp(&a.0)); // Sort by priority descending

        assert_eq!(tasks[0].0, Priority::Critical);
        assert_eq!(tasks[1].0, Priority::High);
        assert_eq!(tasks[2].0, Priority::Medium);
        assert_eq!(tasks[3].0, Priority::Low);
    }

    #[test]
    fn integration_temperature_range_check() {
        let room_temp = Temperature::Celsius(22.0);
        let cold = Temperature::Celsius(15.0);
        let hot = Temperature::Celsius(30.0);

        // Check if room_temp is within comfortable range
        assert!(room_temp > cold);
        assert!(room_temp < hot);
    }

    #[test]
    fn integration_version_range() {
        let supported_min = Version::new(1, 5, 0);
        let supported_max = Version::new(2, 0, 0);
        let current = Version::new(1, 8, 3);

        assert!(current >= supported_min);
        assert!(current < supported_max);
    }

    #[test]
    fn integration_find_and_clamp() {
        let values = [5, 10, 15, 20, 25];
        let min_val = find_min(&values).unwrap();
        let max_val = find_max(&values).unwrap();

        assert_eq!(clamp(&3, min_val, max_val), min_val);
        assert_eq!(clamp(&30, min_val, max_val), max_val);
        assert_eq!(clamp(&12, min_val, max_val), &12);
    }

    #[test]
    fn integration_sorted_check_after_sort() {
        let mut values = vec![5, 2, 8, 1, 9];
        assert!(!is_sorted(&values));

        values.sort();
        assert!(is_sorted(&values));
    }

    #[test]
    fn integration_score_ranking() {
        let scores = [Score(85), Score(92), Score(78), Score(92), Score(88)];

        let max_score = find_max(&scores);
        assert_eq!(max_score, Some(&Score(92)));

        let min_score = find_min(&scores);
        assert_eq!(min_score, Some(&Score(78)));

        assert!(!is_sorted(&scores));

        let mut sorted_scores = scores.to_vec();
        sorted_scores.sort();
        assert!(is_sorted(&sorted_scores));
    }

    #[test]
    fn integration_cross_unit_temperature_sorting() {
        // This would require PartialOrd, not Ord, so we can't sort directly.
        // But we can compare individual elements.
        let temps = [
            Temperature::Celsius(0.0),
            Temperature::Fahrenheit(212.0), // 100°C
            Temperature::Celsius(50.0),
            Temperature::Fahrenheit(32.0), // 0°C
        ];

        // Find extremes manually using partial_cmp
        let mut min_idx = 0;
        let mut max_idx = 0;

        for i in 1..temps.len() {
            if temps[i].partial_cmp(&temps[min_idx]) == Some(Ordering::Less) {
                min_idx = i;
            }
            if temps[i].partial_cmp(&temps[max_idx]) == Some(Ordering::Greater) {
                max_idx = i;
            }
        }

        // Both 0 and 3 are equal (0°C)
        assert!(temps[min_idx].to_celsius().abs() < 0.001);
        // Index 1 is 100°C
        assert!((temps[max_idx].to_celsius() - 100.0).abs() < 0.001);
    }
}
