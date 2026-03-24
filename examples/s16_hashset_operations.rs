// Demonstrates HashSet operations: union, intersection, difference, etc.
//
// These set operations mirror mathematical set theory and are
// useful for comparing collections efficiently.
//
// Run: cargo run --example s16_hashset_operations

use std::collections::HashSet;

fn main() {
    let a: HashSet<i32> = HashSet::from([1, 2, 3, 4, 5]);
    let b: HashSet<i32> = HashSet::from([3, 4, 5, 6, 7]);

    // --- 1. Union: all elements from both sets ---
    let union: HashSet<_> = a.union(&b).copied().collect();
    println!("A: {a:?}");
    println!("B: {b:?}");
    println!("union: {union:?}");

    // --- 2. Intersection: elements in both sets ---
    let intersection: HashSet<_> = a.intersection(&b).copied().collect();
    println!("intersection: {intersection:?}"); // {3, 4, 5}

    // --- 3. Difference: elements in A but not B ---
    let diff: HashSet<_> = a.difference(&b).copied().collect();
    println!("A - B: {diff:?}"); // {1, 2}

    let diff_b: HashSet<_> = b.difference(&a).copied().collect();
    println!("B - A: {diff_b:?}"); // {6, 7}

    // --- 4. Symmetric difference: elements in either but not both ---
    let sym_diff: HashSet<_> = a.symmetric_difference(&b).copied().collect();
    println!("symmetric diff: {sym_diff:?}"); // {1, 2, 6, 7}

    // --- 5. Subset and superset ---
    let small = HashSet::from([1, 2]);
    let big = HashSet::from([1, 2, 3, 4]);
    println!("{{1,2}} subset of {{1,2,3,4}}: {}", small.is_subset(&big));
    println!(
        "{{1,2,3,4}} superset of {{1,2}}: {}",
        big.is_superset(&small)
    );

    // --- 6. Disjoint: no common elements ---
    let c = HashSet::from([10, 20]);
    println!("disjoint A and C: {}", a.is_disjoint(&c)); // true

    // --- 7. Operator-style (using & | - ^) ---
    // These work because HashSet implements BitAnd, BitOr, Sub, BitXor.
    let union_op: HashSet<_> = &a | &b;
    let inter_op: HashSet<_> = &a & &b;
    let diff_op: HashSet<_> = &a - &b;
    let sym_op: HashSet<_> = &a ^ &b;
    println!("A | B: {union_op:?}");
    println!("A & B: {inter_op:?}");
    println!("A - B: {diff_op:?}");
    println!("A ^ B: {sym_op:?}");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn test_union() {
        let a = HashSet::from([1, 2]);
        let b = HashSet::from([2, 3]);
        let u: HashSet<_> = a.union(&b).copied().collect();
        assert_eq!(u, HashSet::from([1, 2, 3]));
    }

    #[test]
    fn test_intersection() {
        let a = HashSet::from([1, 2, 3]);
        let b = HashSet::from([2, 3, 4]);
        let i: HashSet<_> = a.intersection(&b).copied().collect();
        assert_eq!(i, HashSet::from([2, 3]));
    }

    #[test]
    fn test_difference() {
        let a = HashSet::from([1, 2, 3]);
        let b = HashSet::from([2, 3, 4]);
        let d: HashSet<_> = a.difference(&b).copied().collect();
        assert_eq!(d, HashSet::from([1]));
    }

    #[test]
    fn test_subset() {
        let small = HashSet::from([1, 2]);
        let big = HashSet::from([1, 2, 3]);
        assert!(small.is_subset(&big));
        assert!(!big.is_subset(&small));
    }
}
