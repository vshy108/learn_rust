// Demonstrates adding and removing elements from a Vec.
//
// Vectors are dynamically sized: push, pop, insert, remove, etc.
//
// Run: cargo run --example s13_add_remove

fn main() {
    // --- 1. push: add to end ---
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("after push: {v:?}");

    // --- 2. pop: remove from end (returns Option<T>) ---
    let last = v.pop();
    println!("popped: {last:?}, vec: {v:?}");
    // Pop from empty vec returns None.

    // --- 3. insert: add at index ---
    v.insert(0, 99); // insert 99 at index 0
    println!("after insert(0, 99): {v:?}");

    // --- 4. remove: remove at index (shifts elements left) ---
    let removed = v.remove(0);
    println!("removed index 0: {removed}, vec: {v:?}");

    // --- 5. extend: add multiple elements ---
    v.extend([10, 20, 30]);
    println!("after extend: {v:?}");

    // --- 6. extend from another vec ---
    let more = vec![40, 50];
    v.extend(more);
    println!("after extend vec: {v:?}");

    // --- 7. append: move all from another vec ---
    let mut other = vec![60, 70];
    v.append(&mut other); // other is now empty
    println!("after append: {v:?}");
    println!("other is empty: {other:?}");

    // --- 8. retain: keep only matching elements ---
    v.retain(|&x| x >= 10);
    println!("after retain (>= 10): {v:?}");

    // --- 9. truncate: shorten to a given length ---
    v.truncate(3);
    println!("after truncate(3): {v:?}");

    // --- 10. clear: remove all elements ---
    v.clear();
    println!("after clear: {v:?}, len: {}", v.len());

    // --- 11. swap_remove: O(1) removal (swaps with last) ---
    let mut nums = vec![10, 20, 30, 40, 50];
    nums.swap_remove(1); // removes 20, puts 50 in its place
    println!("after swap_remove(1): {nums:?}"); // [10, 50, 30, 40]
    // Does NOT preserve order — but O(1) instead of O(n).

    // --- 12. dedup: remove consecutive duplicates ---
    let mut dups = vec![1, 1, 2, 3, 3, 3, 2];
    dups.dedup();
    println!("after dedup: {dups:?}"); // [1, 2, 3, 2] — only consecutive!
    // For all duplicates, sort first or use dedup_by/dedup_by_key.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_push_pop() {
        let mut v = vec![1, 2];
        v.push(3);
        assert_eq!(v, [1, 2, 3]);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v, [1, 2]);
    }

    #[test]
    fn test_insert_remove() {
        let mut v = vec![1, 3];
        v.insert(1, 2);
        assert_eq!(v, [1, 2, 3]);
        v.remove(0);
        assert_eq!(v, [2, 3]);
    }

    #[test]
    fn test_retain() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.retain(|&x| x % 2 == 0);
        assert_eq!(v, [2, 4]);
    }

    #[test]
    fn test_swap_remove() {
        let mut v = vec![10, 20, 30, 40];
        let removed = v.swap_remove(1);
        assert_eq!(removed, 20);
        assert_eq!(v.len(), 3);
    }
}
