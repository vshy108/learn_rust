// Demonstrates ownership with arrays and tuples.
//
// Arrays and tuples have interesting ownership behavior:
//
// **Arrays `[T; N]`:**
// - If T is Copy, the whole array is Copy (assignment copies).
// - If T is NOT Copy (e.g., String), assignment MOVES the array.
// - You can move individual elements out with patterns or indexing.
//
// **Tuples `(T1, T2, ...)`:**
// - Copy if ALL fields are Copy.
// - Partial moves: you can move one field, keeping others accessible.
//
// Run: cargo run --example s07_ownership_arrays_tuples

fn main() {
    // === ARRAYS ===

    // --- 1. Copy array (i32 is Copy) ---
    let a = [1, 2, 3];
    let b = a; // Copy — both a and b are valid
    println!("a = {a:?}, b = {b:?}");

    // --- 2. Non-Copy array (String is not Copy) ---
    let names = [String::from("Alice"), String::from("Bob")];
    let moved = names; // MOVE — names is no longer accessible
    // println!("{names:?}"); // error: value used after move
    println!("moved = {moved:?}");

    // --- 3. Borrowing array elements ---
    let fruits = [String::from("apple"), String::from("banana")];
    let first = &fruits[0]; // borrow — no move
    println!("first = {first}");
    println!("fruits still valid: {fruits:?}");

    // --- 4. Moving out of an array via pattern ---
    let arr = [String::from("x"), String::from("y")];
    let [a_val, b_val] = arr; // destructure — moves each element
    println!("a_val = {a_val}, b_val = {b_val}");

    // === TUPLES ===

    // --- 5. Copy tuple ---
    let t = (1, 2.0_f64, true);
    let t2 = t; // Copy
    println!("t = {t:?}, t2 = {t2:?}");

    // --- 6. Non-Copy tuple ---
    let t = (String::from("hello"), 42);
    let t2 = t; // MOVE
    // println!("{:?}", t); // error
    println!("t2 = {:?}", t2);

    // --- 7. Partial move from tuple ---
    let pair = (String::from("key"), String::from("value"));
    let (k, _v) = pair; // Both fields moved out
    println!("k = {k}");

    // --- 8. Borrow tuple fields ---
    let data = (String::from("name"), vec![1, 2, 3]);
    let name_ref = &data.0;
    let vec_ref = &data.1;
    println!("name = {name_ref}, vec = {vec_ref:?}");
    // `data` is still fully accessible because we only borrowed:
    println!("data = {:?}", data);

    // --- 9. Mixed: Copy and non-Copy in same tuple ---
    let mixed = (42_i32, String::from("owned"));
    let num = mixed.0; // Copy — `mixed.0` is still accessible
    // But accessing `mixed` as a whole is still OK since nothing moved:
    println!("num = {num}, mixed = {:?}", mixed);

    // Move the String out:
    let s = mixed.1; // MOVE of the String field
    println!("s = {s}");
    // mixed.0 is still accessible (it was copied), but mixed as a whole
    // and mixed.1 are not.
    println!("mixed.0 = {}", mixed.0);
    // println!("{:?}", mixed); // error: partial move
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_copy_array() {
        let a = [1, 2, 3];
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_copy_tuple() {
        let t = (1, 2.0_f64, true);
        let t2 = t;
        assert_eq!(t, t2);
    }

    #[test]
    fn test_array_borrow() {
        let arr = [String::from("a"), String::from("b")];
        let r = &arr[0];
        assert_eq!(r, "a");
        assert_eq!(arr.len(), 2); // arr still accessible
    }

    #[test]
    fn test_tuple_borrow() {
        let t = (String::from("x"), 42);
        let r = &t.0;
        assert_eq!(r, "x");
        assert_eq!(t.1, 42);
    }

    #[test]
    fn test_array_destructure() {
        let arr = [String::from("one"), String::from("two")];
        let [a, b] = arr;
        assert_eq!(a, "one");
        assert_eq!(b, "two");
    }
}
