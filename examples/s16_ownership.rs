// Demonstrates HashMaps and ownership.
//
// Ownership rules for HashMap:
// - Types implementing `Copy` (like i32) are copied into the map.
// - Owned types (like `String`) are MOVED into the map.
// - References can be stored but must outlive the map.
//
// Run: cargo run --example s16_ownership

use std::collections::HashMap;

fn main() {
    // --- 1. Copy types are copied ---
    let key = 1;
    let value = 100;
    let mut map = HashMap::new();
    map.insert(key, value);
    // key and value still usable (i32 is Copy):
    println!("key: {key}, value: {value}");
    println!("map: {map:?}");

    // --- 2. Owned types are moved ---
    let name = String::from("Alice");
    let city = String::from("NYC");
    let mut map = HashMap::new();
    map.insert(name, city);
    // name and city are MOVED — can't use them anymore:
    // println!("{name}"); // ERROR: value borrowed after move
    println!("map: {map:?}");

    // --- 3. Clone to keep ownership ---
    let name = String::from("Bob");
    let mut map = HashMap::new();
    map.insert(name.clone(), 42);
    println!("name still usable: {name}");

    // --- 4. References as values —  must outlive the map ---
    let alice = String::from("Alice");
    let bob = String::from("Bob");
    let mut ref_map = HashMap::new();
    ref_map.insert("a", &alice);
    ref_map.insert("b", &bob);
    println!("ref_map: {ref_map:?}");
    // alice and bob must live at least as long as ref_map

    // --- 5. get() borrows, remove() takes ownership ---
    let mut map = HashMap::from([("key".to_string(), "value".to_string())]);
    // get() returns Option<&V>:
    if let Some(val) = map.get("key") {
        println!("get (borrowed): {val}");
    }
    // remove() returns Option<V> (owned):
    if let Some(val) = map.remove("key") {
        println!("remove (owned): {val}");
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_copy_types() {
        let k = 1;
        let v = 2;
        let mut m = HashMap::new();
        m.insert(k, v);
        assert_eq!(k, 1); // still usable
        assert_eq!(v, 2);
    }

    #[test]
    fn test_string_moved() {
        let key = String::from("test");
        let mut m = HashMap::new();
        m.insert(key, 42);
        assert_eq!(m.get("test"), Some(&42));
    }

    #[test]
    fn test_clone_preserves() {
        let key = String::from("hello");
        let mut m = HashMap::new();
        m.insert(key.clone(), 1);
        assert_eq!(key, "hello"); // clone preserved original
    }
}
