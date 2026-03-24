// Demonstrates ownership rules with vectors.
//
// The borrow checker enforces these rules on Vec:
// 1. Can't have a mutable borrow while immutable borrows exist.
// 2. Moving a value out of a Vec requires special handling.
// 3. Iterating borrows the whole Vec.
//
// Run: cargo run --example s13_ownership_vectors

fn main() {
    // --- 1. Borrowing vs indexing ---
    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable borrow of v
    println!("first: {first}");
    // v is still usable because we only have an immutable ref.
    println!("v: {v:?}");

    // --- 2. Can't mutate while immutably borrowed ---
    // let mut v = vec![1, 2, 3];
    // let first = &v[0];  // immutable borrow
    // v.push(4);          // ERROR: mutable borrow while immutable exists
    // println!("{first}"); // first is still in use here

    // WHY: push might reallocate the Vec's backing memory.
    // That would invalidate the existing &v[0] reference.

    // --- 3. Fix: scope the borrow ---
    let mut v = vec![1, 2, 3];
    {
        let first = &v[0];
        println!("first: {first}");
    } // immutable borrow ends here
    v.push(4); // now OK
    println!("after push: {v:?}");

    // --- 4. Moving elements out ---
    let v = vec![String::from("hello"), String::from("world")];
    // let s = v[0]; // ERROR: cannot move out of indexed content
    // Use .clone() if you need a copy:
    let s = v[0].clone();
    println!("cloned: {s}, v still: {v:?}");

    // Or use methods that take ownership:
    let v = vec![String::from("a"), String::from("b")];
    let mut iter = v.into_iter(); // v is moved!
    let first = iter.next();
    println!("from into_iter: {first:?}");
    // println!("{v:?}"); // ERROR: v has been moved

    // --- 5. Iterating by reference vs by value ---
    let v = vec![10, 20, 30];

    // By reference (borrows v):
    for item in &v {
        println!("  ref: {item}");
    }
    println!("v still exists: {v:?}");

    // By mutable reference:
    let mut v = vec![10, 20, 30];
    for item in &mut v {
        *item *= 2;
    }
    println!("doubled: {v:?}");

    // --- 6. Passing vectors to functions ---
    fn sum(nums: &[i32]) -> i32 {
        // Takes a SLICE reference — doesn't own the Vec
        nums.iter().sum()
    }
    let v = vec![1, 2, 3, 4, 5];
    println!("sum: {}", sum(&v));
    println!("v still exists: {v:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_borrow_after_read() {
        let v = vec![1, 2, 3];
        let first = &v[0];
        assert_eq!(*first, 1);
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_into_iter_moves() {
        let v = vec![String::from("x")];
        let items: Vec<String> = v.into_iter().collect();
        assert_eq!(items, vec!["x"]);
        // v is no longer accessible
    }

    #[test]
    fn test_iter_mut() {
        let mut v = vec![1, 2, 3];
        for item in &mut v {
            *item += 10;
        }
        assert_eq!(v, [11, 12, 13]);
    }
}
