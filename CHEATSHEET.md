# Rust Cheatsheet

## Variables & Mutability

```rust
let x = 5;              // immutable (default)
let mut y = 5;          // mutable
let z: i32 = 5;         // explicit type

const MAX: u32 = 100;   // constant — must have type, always immutable
static HELLO: &str = "hi"; // static lifetime

// Shadowing — re-bind same name (even change type)
let x = x + 1;
let x = x.to_string();  // i32 → String is fine
```

### const vs static

```rust
// const — inlined at every use site (like a typed #define)
const MAX_SIZE: u32 = 1024;
// - No fixed memory address; compiler copies value wherever it's used
// - Must be computable at compile time
// - Large const (e.g., [u8; 1024]) gets duplicated at each use — binary bloat

// static — single fixed memory location, 'static lifetime
static GREETING: &str = "hello";
// - One copy in read-only memory; &GREETING always the same address
// - Use when you need a stable address (FFI, global singleton)

// static mut — requires unsafe; almost never use directly
static mut COUNTER: u32 = 0;
unsafe { COUNTER += 1; }  // every access must be in unsafe block
// Prefer Mutex, RwLock, or atomics instead:

use std::sync::atomic::{AtomicU32, Ordering};
static HIT_COUNT: AtomicU32 = AtomicU32::new(0);
HIT_COUNT.fetch_add(1, Ordering::Relaxed);  // safe, no unsafe needed

// Global mutable singleton — LazyLock + RwLock (std 1.80+)
use std::sync::{LazyLock, RwLock};
static CONFIG: LazyLock<RwLock<String>> = LazyLock::new(|| {
    RwLock::new(String::from("default"))
});
// initialized on first access; safe concurrent reads + exclusive writes

// Rule of thumb:
//   const  — small scalars, lookup tables you want inlined
//   static — large data, need fixed address, or global mutable state
```

## Scalar Types

```rust
// Integers
i8 i16 i32 i64 i128 isize   // signed
u8 u16 u32 u64 u128 usize   // unsigned
// isize/usize are pointer-sized (64-bit on 64-bit OS)

// Literals
let a = 1_000_000;          // visual separator
let b = 0xff;               // hex
let c = 0o77;               // octal
let d = 0b1111_0000;        // binary
let e = b'A';               // byte (u8)

// Floats
f32 f64                     // f64 is default and recommended

// Bool
let t: bool = true;

// Char — Unicode scalar value, 4 bytes
let c: char = 'Z';
let emoji: char = '😀';
```

## Numeric Methods

```rust
// Bounds
i32::MAX                         // 2_147_483_647
i32::MIN                         // -2_147_483_648
usize::MAX                       // platform-dependent (2^64-1 on 64-bit)

// Parsing & conversion
"42".parse::<i32>()              // Result<i32, ParseIntError>
i32::from_str_radix("1A", 16)   // Result<i32, _> — hex/binary/octal
42_i32.to_string()               // "42"
42 as f64                        // type cast (as keyword)

// Integer math
5_i32.pow(3)                     // 125
(-5_i32).abs()                   // 5
5_i32.min(3)                     // 3
5_i32.max(3)                     // 5
5_i32.clamp(0, 3)               // 3 — clamp between min and max

// Float math
2.0_f64.sqrt()                   // 1.414...
2.0_f64.powi(3)                  // 8.0 (integer exponent — faster)
2.0_f64.powf(0.5)               // same as sqrt (float exponent)
3.7_f64.floor()                  // 3.0
3.2_f64.ceil()                   // 4.0
3.5_f64.round()                  // 4.0
(-2.5_f64).abs()                 // 2.5
f64::INFINITY                    // positive infinity
f64::NAN                         // not a number
f64::NAN == f64::NAN             // false! NaN != NaN
val.is_nan()                     // true check for NaN
```

## Compound Types

```rust
// Tuple — fixed length, mixed types
let tup: (i32, f64, bool) = (500, 6.4, true);
let (x, y, z) = tup;        // destructure
let first = tup.0;          // index access

// Unit type — empty tuple, represents "nothing"
let unit: () = ();

// Array — fixed length, same type, stack-allocated
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 5];         // [0, 0, 0, 0, 0]
let first = arr[0];
let slice: &[i32] = &arr[1..3];
```

## Strings

```rust
// &str — string slice, borrowed, immutable, usually stack (or static)
let s: &str = "hello";

// String — owned, heap-allocated, growable
let mut s = String::from("hello");
s.push_str(", world");
s.push('!');

// Conversions
let owned: String = "hello".to_string();
let borrowed: &str = &owned;         // deref coercion
let borrowed: &str = owned.as_str(); // explicit

// Formatting
let s = format!("{} {}", "hello", "world");

// Common methods
s.len()          // byte length
s.is_empty()
s.contains("lo")
s.starts_with("he")
s.to_uppercase()
s.trim()
s.split(',').collect::<Vec<&str>>()
s.chars()        // iterator over Unicode chars
s.bytes()        // iterator over raw bytes (u8)
s.pop()          // Option<char> — remove last char (LC 0014)
s.clear()        // empty the string
s.repeat(3)     // "abcabcabc"
s.replace("old", "new")   // replace all occurrences
s.replacen("old", "new", 1) // replace first N
s.find("sub")    // Option<usize> — first byte index
s.rfind("sub")   // Option<usize> — last byte index
s.split_whitespace()     // iterator of words
s.lines()                // iterator of lines
["a", "b"].join(", ")    // "a, b"
s.char_indices() // (byte_offset, char) pairs — use over .chars().enumerate()
                 // when byte position matters for slicing (LC 0003)

// String can't be indexed: s[i] won't compile (multi-byte UTF-8)
// Collect to Vec<char> for O(1) random access (LC 0005)
let chars: Vec<char> = s.chars().collect();
let third = chars[2];

// into_bytes() vs as_bytes()
let borrowed: &[u8] = s.as_bytes();      // borrows, s still valid
let owned: Vec<u8> = s.into_bytes();      // consumes s, zero-copy (LC 0079)

// vec![String::new(); n] vs vec![""; n] (LC 0006)
let rows: Vec<String> = vec![String::new(); n]; // n owned, growable Strings
let slices: Vec<&str> = vec![""; n];             // n borrowed — can't push!()

// String += &str (AddAssign trait) (LC 0012)
let mut result = String::new();
result += "hello";    // same as result.push_str("hello")
```

## Ownership Rules

```
1. Each value has exactly one owner.
2. When the owner goes out of scope, the value is dropped.
3. Only one mutable reference OR any number of immutable references at a time.
```

```rust
let s1 = String::from("hello");
let s2 = s1;            // s1 MOVED — no longer valid
// println!("{}", s1);  // compile error

let s1 = String::from("hello");
let s2 = s1.clone();    // deep copy — both valid

// Copy types (stack-only) are copied, not moved
let x = 5;
let y = x;              // x still valid (i32 implements Copy)
```

## References & Borrowing

```rust
fn len(s: &String) -> usize { s.len() }  // borrow, not move

let s = String::from("hello");
let r = &s;             // immutable reference
println!("{}", r);      // s still valid

let mut s = String::from("hello");
let r = &mut s;         // mutable reference
r.push_str(" world");

// Rules:
// - Multiple immutable refs OK at same time
// - Only ONE mutable ref at a time
// - Cannot mix mutable and immutable refs in same scope
```

## Slices

```rust
let s = String::from("hello world");
let hello: &str = &s[0..5];    // &str slice
let world: &str = &s[6..11];

let a = [1, 2, 3, 4, 5];
let slice: &[i32] = &a[1..3];  // [2, 3]

// Slice as function parameter — accepts both &String and &str
fn first_word(s: &str) -> &str { ... }
```

## Functions

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y       // implicit return (no semicolon)
}

fn nothing() { }        // returns ()

// Early return
fn find(v: &[i32], n: i32) -> Option<usize> {
    for (i, &x) in v.iter().enumerate() {
        if x == n { return Some(i); }
    }
    None
}

// Closures
let add = |x, y| x + y;
let add = |x: i32, y: i32| -> i32 { x + y };

// Move closure — captures by value
let s = String::from("hi");
let f = move || println!("{}", s);
```

## Control Flow

```rust
// if — is an expression
let n = if condition { 5 } else { 6 };

// loop — infinite, can return a value
let x = loop {
    counter += 1;
    if counter == 10 { break counter * 2; }
};

// while
while n < 5 { n += 1; }

// for
for i in 0..5 { }           // 0,1,2,3,4
for i in 0..=5 { }          // 0,1,2,3,4,5
for x in &arr { }           // borrow each element
for (i, x) in arr.iter().enumerate() { }

// match
match x {
    1 => println!("one"),
    2 | 3 => println!("two or three"),
    4..=9 => println!("four to nine"),
    _ => println!("other"),
}

// if let — single-pattern match
if let Some(v) = opt { println!("{}", v); }

// while let
while let Some(top) = stack.pop() { println!("{}", top); }
```

## Structs

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

// Instantiate
let p = Point { x: 1.0, y: 2.0 };

// Struct update syntax
let p2 = Point { x: 3.0, ..p };

// Tuple struct
struct Color(u8, u8, u8);
let red = Color(255, 0, 0);

// Unit struct (useful for traits)
struct Marker;

// Methods
impl Point {
    // Associated function (no self) — called with Point::new()
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // Method — called with p.distance()
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    // Mutable method
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}
```

## Enums

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r)               => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h)         => w * h,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

// Option<T> — no null in Rust
let some: Option<i32> = Some(5);
let none: Option<i32> = None;
let val = some.unwrap_or(0);
let val = some.unwrap_or_default();
let mapped = some.map(|x| x * 2);

// Chaining map + unwrap_or_default — transform Option then default if None
let name: Option<String> = Some("hello".to_string());
let upper = name.map(|n| n.to_uppercase()).unwrap_or_default();
// Some("hello") → .map() → Some("HELLO") → .unwrap_or_default() → "HELLO" (String)
// None          → .map() → None           → .unwrap_or_default() → ""      (String::default())

// Additional Option methods
opt.take()                       // move value out, leave None in place (LC 0002)
opt.as_deref()                   // &Option<Box<T>> → Option<&T> without consuming
opt.map_or(default, |v| f(v))    // transform Some or return default value
opt.is_some()                    // bool check without destructure
opt.is_none()
opt.filter(|v| *v > 5)          // Some(v) if predicate true, else None
opt.or(other_opt)                // first Some wins
opt.or_else(|| compute_opt())   // lazy fallback
opt.zip(other_opt)               // (Some(a), Some(b)) → Some((a,b))
opt.flatten()                    // Option<Option<T>> → Option<T>
opt.inspect(|v| println!("{v}")) // peek without consuming (1.76+)
opt.replace(new_val)             // puts new, returns old Option
map.get(&k).copied().unwrap_or(0) // safe lookup chain: Option<&V> → Option<V> → V

// Result<T, E> — error handling
let ok: Result<i32, &str> = Ok(5);
let err: Result<i32, &str> = Err("oops");
let val = ok.unwrap_or(0);
let val = ok?;              // propagate error (in fn returning Result)
```

## Error Handling

```rust
use std::num::ParseIntError;

// Define error type with thiserror
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("parse failed: {0}")]
    Parse(#[from] ParseIntError),
    #[error("value too large: {0}")]
    TooLarge(i32),
}

// ? operator — returns Err early if Err, unwraps if Ok
fn parse_and_double(s: &str) -> Result<i32, AppError> {
    let n: i32 = s.parse()?;    // ParseIntError auto-converts via #[from]
    if n > 1000 { return Err(AppError::TooLarge(n)); }
    Ok(n * 2)
}

// anyhow for application-level errors
use anyhow::{Context, Result};

fn read_config(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path))
}
```

## Traits

```rust
trait Greet {
    fn hello(&self) -> String;

    // Default implementation
    fn goodbye(&self) -> String {
        format!("Goodbye from {}", self.hello())
    }
}

struct Person { name: String }

impl Greet for Person {
    fn hello(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

// Trait bounds
fn print_greeting<T: Greet>(item: &T) {
    println!("{}", item.hello());
}

// impl Trait syntax (shorter)
fn print_greeting(item: &impl Greet) {
    println!("{}", item.hello());
}

// where clause (for complex bounds)
fn compare<T>(a: T, b: T) -> bool
where
    T: PartialOrd + std::fmt::Debug,
{
    a > b
}

// Common standard traits
// Clone   — .clone() deep copy
// Copy    — implicit copy for stack types
// Debug   — {:?} formatting
// Display — {} formatting
// Default — Default::default()
// PartialEq / Eq   — == and !=
// PartialOrd / Ord — < > <= >=
// From / Into      — type conversions
// Iterator         — .next()
```

## Type Conversions: From / Into / TryInto / as

```rust
// From/Into — infallible, lossless conversions only
// Implement From, you get Into for free
let s: String = String::from("hello");  // explicit target type
let s: String = "hello".into();          // infers target from context

// .into() fails if compiler can't infer target type:
// let s = "hello".into();  // ERROR: type annotations needed

// Lossless numeric conversions (smaller → larger always works)
let big: i64 = 42_i32.into();       // i32 → i64
let wide: u32 = 42_u16.into();      // u16 → u32

// These DON'T compile — potentially lossy:
// let x: i32 = 42_usize.into();    // usize could be 64-bit
// let x: i32 = 42_u32.into();      // u32::MAX > i32::MAX

// TryFrom / TryInto — fallible, returns Result
use std::convert::TryInto;
let x: i32 = some_usize.try_into().expect("value too large");
let x: i32 = some_usize.try_into().unwrap_or(i32::MAX);  // saturate

// as — raw cast, silent truncation (use with caution)
let x = some_usize as i32;          // wraps if value > i32::MAX
let idx = some_i32 as usize;        // wraps if negative

// Summary:
// From / .into()     → lossless, infallible (compiler guarantees fit)
// TryFrom / .try_into() → potentially lossy, returns Result
// as                  → silent truncation, use when you're certain it fits
```

## Generics

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self { Self { items: Vec::new() } }
    fn push(&mut self, item: T) { self.items.push(item); }
    fn pop(&mut self) -> Option<T> { self.items.pop() }
}
```

## Lifetimes

```rust
// Lifetime annotation — 'a means both refs live at least as long as output
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct holding a reference
struct Excerpt<'a> {
    part: &'a str,
}

// 'static lifetime — lives for the entire program
let s: &'static str = "I live forever";
```

## Collections

```rust
// Vec<T> — growable array
let mut v: Vec<i32> = Vec::new();
let mut v = vec![1, 2, 3];
Vec::with_capacity(100);        // pre-allocate to avoid reallocations
vec![0; 10];                    // 10 zeros
v.push(4);
v.pop();                    // Option<i32>
v.len();
v.is_empty();
v[0];                       // panics if OOB
v.get(0);                   // Option<&i32> — bounds-checked
v.first();                  // Option<&i32> — first element
v.last();                   // Option<&i32> — last element
v.contains(&3);             // bool
v.insert(1, val);           // O(n) — shifts elements right
v.remove(1);                // O(n) — shifts left, returns removed
v.swap_remove(1);           // O(1) — swaps with last, returns removed
v.extend([4, 5, 6]);       // append from iterator
v.sort();                   // stable sort
v.sort_unstable();          // faster, not stable
v.sort_by(|a, b| b.cmp(a));     // custom comparator (descending)
v.sort_by_key(|x| x.abs());     // sort by derived key
v.reverse();
v.binary_search(&val);      // Result<usize, usize> — must be sorted
v.iter()                    // &T iterator
v.iter_mut()                // &mut T iterator
v.into_iter()               // T iterator (consumes)

// HashMap<K, V>
use std::collections::HashMap;
let mut map: HashMap<String, i32> = HashMap::new();
map.insert("a".to_string(), 1);
map.get("a");               // Option<&i32>
map.contains_key("a");
map.remove("a");
map.entry("b".to_string()).or_insert(0);  // insert if missing

// HashMap::from([...]) — inline initialization (LC 0013)
let map: HashMap<char, i32> = HashMap::from([
    ('I', 1), ('V', 5), ('X', 10), ('L', 50),
]);
// Gotcha: heap-allocates on every call — prefer match or const for hot paths

// map[&key] — Index trait requires reference (LC 0013)
let value = roman_map[&ch];  // must pass &K, not K
// Prefer .get(&k) for Option-based safe lookup

// VecDeque<T> — double-ended queue, O(1) push/pop at both ends
use std::collections::VecDeque;
let mut q: VecDeque<i32> = VecDeque::new();
q.push_back(1);             // [1]
q.push_back(2);             // [1, 2]
q.pop_front();              // Some(1), q = [2]
q.push_front(0);            // [0, 2]
// Ruby: Array#shift is O(n); VecDeque::pop_front is O(1)
// Use VecDeque for BFS queues, sliding windows, or any FIFO pattern

// HashSet<T>
use std::collections::HashSet;
let mut set: HashSet<i32> = HashSet::new();
set.insert(1);              // bool — true if new, false if existed
set.contains(&1);
set.remove(&1);             // bool — true if was present
set.len();
set.is_empty();

// Set operations
a.intersection(&b)           // elements in both
a.union(&b)                  // elements in either
a.difference(&b)             // in a but not b
a.symmetric_difference(&b)  // in one but not both
a.is_subset(&b)             // true if all of a is in b
```

## Iterators

```rust
let v = vec![1, 2, 3, 4, 5];

// Adaptors (lazy — chain transforms)
v.iter().map(|x| x * 2)
v.iter().filter(|&&x| x > 2)
v.iter().filter_map(|&x| if x > 2 { Some(x * 10) } else { None })
v.iter().take(3)
v.iter().skip(2)
v.iter().zip(other.iter())
v.iter().enumerate()        // (index, &value)
v.iter().flat_map(|x| vec![x, x])
v.iter().flatten()          // nested iterators: Vec<Vec<T>> → iter of &T (LC 0079)
v.iter().chain(other.iter())
std::iter::once(val).chain(iter) // prepend a single value to an iterator (LC 0005)

// Consumers (eager — trigger evaluation)
v.iter().sum::<i32>()
v.iter().product::<i32>()
v.iter().count()
v.iter().max()              // Option<&i32>
v.iter().min()
v.iter().any(|&x| x > 3)
v.iter().all(|&x| x > 0)
v.iter().find(|&&x| x > 3) // Option<&i32>
v.iter().position(|&x| x == 3) // Option<usize>
v.iter().collect::<Vec<_>>()
v.iter().collect::<String>()         // chars/&str → String
v.iter().collect::<HashMap<_, _>>()  // pairs → HashMap
v.iter().fold(0, |acc, &x| acc + x)
v.iter().reduce(|a, b| a.max(b))    // like fold but no initial value, returns Option
v.iter().cloned()                    // &T → T (requires Clone)
v.iter().copied()                    // &T → T (requires Copy, cheaper)
v.iter().rev()                       // reverse (DoubleEndedIterator)
v.iter().step_by(2)                  // every Nth element
v.iter().peekable()                  // .peek() next without consuming
v.iter().inspect(|x| println!("{x}")) // debug tap — peek without consuming
iter.unzip::<Vec<_>, Vec<_>>()       // split pairs into two Vecs
iter.partition::<Vec<_>, _>(|x| x > &3) // split by predicate into two Vecs
```

## Smart Pointers

```rust
// Box<T> — heap allocation
let b = Box::new(5);

// Option<Box<T>> linked list — Rust's idiomatic singly-linked list
type Link = Option<Box<ListNode>>;
let next = node.next.take();     // move out, leave None behind
let borrowed = boxed_opt.as_deref(); // &Option<Box<T>> → Option<&T>
// Dummy head pattern for iterative list building (LC 0021)
let mut dummy = ListNode::new(0);
let mut current = &mut dummy;
// ... current.next = Some(Box::new(...)); current = current.next.as_mut().unwrap();
// return dummy.next
// match on tuple of Options
match (list1, list2) {
    (None, None)       => None,
    (Some(n), None)    => Some(n),
    (None, Some(n))    => Some(n),
    (Some(l), Some(r)) => { /* merge logic */ }
}

// Rc<T> — reference-counted, single thread
use std::rc::Rc;
let a = Rc::new(5);
let b = Rc::clone(&a);  // increments count, does NOT deep copy
Rc::strong_count(&a);   // 2

// Arc<T> — atomic reference-counted, multi-thread
use std::sync::Arc;
let a = Arc::new(5);

// RefCell<T> — interior mutability, runtime borrow check
use std::cell::RefCell;
let x = RefCell::new(5);
*x.borrow_mut() += 1;

// Mutex<T> — mutual exclusion for multi-thread mutation
use std::sync::Mutex;
let m = Mutex::new(5);
let mut guard = m.lock().unwrap();
*guard += 1;

// Rc<RefCell<T>> combo — shared ownership + interior mutability
// Common for tree nodes where multiple pointers share a mutable node
use std::rc::Rc;
use std::cell::RefCell;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,   // nullable child
    right: Option<Rc<RefCell<TreeNode>>>,
}

let node = Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None }));
let shared = Rc::clone(&node);            // cheap ref count bump
let val = node.borrow().val;              // shared read via .borrow()
node.borrow_mut().val = 2;                // exclusive write via .borrow_mut()
// .borrow() panics at runtime if already mutably borrowed (RefCell rule)
// Ruby objects are always GC'd + mutable; Rust needs Rc for sharing
// + RefCell for mutation through shared references.

// Arc<Mutex<T>> combo — shared ownership + mutation across threads
// Arc for shared ownership, Mutex for interior mutability (thread-safe)
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
let thread_counter = Arc::clone(&counter);
std::thread::spawn(move || {
    *thread_counter.lock().unwrap() += 1;
});
// Use Arc<RwLock<T>> instead when reads far outnumber writes

// Cow<'a, T> — clone-on-write, avoids allocation when only reading
use std::borrow::Cow;
let s: Cow<str> = Cow::Borrowed("hello");
let s: Cow<str> = Cow::Owned(String::from("hello"));
```

## Closures & Higher-Order Functions

```rust
// Three closure traits (compiler picks the least restrictive)
// FnOnce — consumes captured values, can only be called once
// FnMut  — mutably borrows captured values
// Fn     — immutably borrows captured values

fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
fn apply_once<F: FnOnce() -> String>(f: F) -> String { f() }

// Returning closures
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

// When `move` is required:
// - Closures sent to other threads (spawn, spawn_blocking, tokio::spawn)
// - The closure must OWN the data, not borrow it, because the thread
//   might outlive the scope where the data was created
let name = String::from("hello");
std::thread::spawn(move || {
    // `move` transfers ownership of `name` into the closure
    // Without `move` → compile error: name borrowed but thread may outlive it
    println!("{name}");
});
// println!("{name}");  // COMPILE ERROR: name was moved
```

## Concurrency

```rust
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

// Spawn a thread
let handle = thread::spawn(|| {
    println!("Hello from thread");
});
handle.join().unwrap();

// Share data with Arc + Mutex
let counter = Arc::new(Mutex::new(0));
let c = Arc::clone(&counter);
let h = thread::spawn(move || {
    *c.lock().unwrap() += 1;
});
h.join().unwrap();

// Message passing with channels
let (tx, rx) = mpsc::channel();
thread::spawn(move || tx.send(42).unwrap());
let val = rx.recv().unwrap();   // blocks until message arrives
```

## Async / Await

```rust
// Requires tokio (or another async runtime)
use tokio;

#[tokio::main]
async fn main() {
    let result = fetch_data().await;
}

async fn fetch_data() -> String {
    // async fn returns impl Future<Output = String>
    "data".to_string()
}

// Run two futures concurrently
let (a, b) = tokio::join!(task_a(), task_b());

// Spawn background task
tokio::spawn(async { long_work().await });

// CPU-bound work — don't block the async reactor
tokio::task::spawn_blocking(|| heavy_computation()).await.unwrap();
```

## Modules & Visibility

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }  // pub = visible outside
    fn secret() -> i32 { 42 }                     // private (default)

    pub mod advanced {
        pub fn sqrt(x: f64) -> f64 { x.sqrt() }
    }
}

use math::add;
use math::advanced::sqrt;
use math::{add, advanced::sqrt};  // brace grouping

// Re-export
pub use math::add;
```

## Common Patterns

```rust
// Builder pattern
struct Config { width: u32, height: u32 }

struct ConfigBuilder { width: u32, height: u32 }
impl ConfigBuilder {
    fn new() -> Self { Self { width: 800, height: 600 } }
    fn width(mut self, w: u32) -> Self { self.width = w; self }
    fn height(mut self, h: u32) -> Self { self.height = h; self }
    fn build(self) -> Config { Config { width: self.width, height: self.height } }
}
let cfg = ConfigBuilder::new().width(1920).height(1080).build();

// Newtype — wraps a type to give it distinct semantics
struct Meters(f64);
struct Kilograms(f64);
// Meters and Kilograms can't be accidentally mixed even though both are f64.

// Type alias — just a rename, no new type
type Result<T> = std::result::Result<T, AppError>;
```

## std Utility Functions

```rust
// std::mem — move and swap values
std::mem::swap(&mut a, &mut b);       // swap two values in place
std::mem::replace(&mut a, new_val);   // replace a, returns old value
std::mem::take(&mut a);               // replace with Default, returns old

// std::cmp
std::cmp::min(a, b);                  // works on Ord types
std::cmp::max(a, b);
std::cmp::Reverse(val)                // wrapper for reverse sorting:
//   v.sort_by_key(|x| std::cmp::Reverse(x.score));

// BinaryHeap — max-heap by default
use std::collections::BinaryHeap;
use std::cmp::Reverse;
let mut min_heap = BinaryHeap::new();
min_heap.push(Reverse(5));             // wrap in Reverse for min-heap
min_heap.push(Reverse(2));
min_heap.pop();                        // Some(Reverse(2)) — smallest first
```

## Useful Macros

```rust
println!("{}", x)           // print with newline
eprintln!("{}", x)          // print to stderr
format!("{:?}", x)          // format to String
vec![1, 2, 3]               // create Vec
assert!(condition)
assert_eq!(a, b)
assert_ne!(a, b)
panic!("message")           // unrecoverable error
todo!()                     // marks unfinished code (panics if reached)
unreachable!()              // marks code that should never run
dbg!(expr)                  // prints file/line + value, returns value
```

## Testing

```rust
// Unit tests — in the same file, inside #[cfg(test)] module
#[cfg(test)]              // only compiled when running cargo test
mod tests {
    use super::*;         // import everything from parent module

    #[test]
    fn test_basic() {
        assert_eq!(add(2, 3), 5);
    }

    // #[should_panic] — test that code panics
    // `expected` is a SUBSTRING match against the panic message (case-sensitive)
    // Always include `expected` — bare #[should_panic] passes on ANY panic
    #[test]
    #[should_panic(expected = "empty name")]
    fn test_empty_name_panics() {
        Vessel::new("");  // panic!("empty name is not allowed") → passes
    }

    // Async tests with tokio
    #[tokio::test]
    async fn test_async_fetch() {
        let result = fetch_data().await;
        assert!(result.is_ok());
    }
}

// Run tests:
// cargo test                   — all tests
// cargo test my_fn             — tests matching "my_fn"
// cargo test -- --nocapture    — show println! output
// cargo test --example foo     — tests in examples/foo.rs
```

## Cargo Commands

```sh
cargo new my_project         # new binary project
cargo new --lib my_lib       # new library project
cargo build                  # compile (debug)
cargo build --release        # compile (optimized)
cargo run                    # compile + run
cargo run --example foo      # run examples/foo.rs
cargo test                   # run all tests
cargo test my_fn             # run tests matching "my_fn"
cargo test -- --nocapture    # show println! output in tests
cargo clippy                 # lint
cargo fmt                    # format code
cargo doc --open             # build + open docs
cargo add serde              # add dependency
cargo update                 # update dependencies
```

## Common Gotchas

```
usize subtraction underflow — usize is unsigned; `0_usize - 1` panics
in debug or wraps in release. Use i32/i64 when indices can go negative.
For binary search: use exclusive upper bound pattern (hi starts at len,
mid = lo + (hi - lo) / 2, set hi = mid instead of mid - 1).

Integer overflow — wraps in release, panics in debug.
Use .checked_add(), .saturating_add(), .wrapping_add() explicitly.

f64 is not Ord — use .max() / .min() methods, not std::cmp::max/min.

&str vs String — &str is borrowed (no allocation); String is owned.
Most APIs accept &str; use it as function parameter types.

Clone vs Copy — Copy types (i32, bool, char, etc.) are silently copied.
Non-Copy types (String, Vec) must be .clone()'d explicitly.

if let drops scrutinee — the temporary in `if let` lives until end of
the if block, not the surrounding scope.

Lifetime of temporaries — &some_fn() creates a reference to a temporary
that is dropped at the end of the statement; you must bind it first.

HashMap no IndexMut — can't do map[key] += 1 like Ruby Hash.new(0).
Must use entry API: *map.entry(key).or_insert(0) += 1.

&[u8] no Display/ToString — s.as_bytes()[range] returns &[u8] which
can't call .to_string(). Use std::str::from_utf8(&bytes).unwrap().

i32::MIN.abs() panics — i32::MIN = -2_147_483_648 but i32::MAX =
2_147_483_647. Guard with `if x == i32::MIN` before calling .abs().
Ruby integers are unbounded so this never arises.

Remainder preserves sign — Rust: -123 % 10 == -3 (not 7). This means
you can process negative numbers directly without abs().

Moving out of Vec by index is forbidden (LC 0014) —
let strs: Vec<String> = vec![...]; let first = strs[0]; // ERROR
Fix: clone (.clone()) or borrow (&strs[0]).

Borrow conflict: iterating + mutating same Vec (LC 0080) —
for &num in &nums { nums[write] = num; } // ERROR: immut + mut borrow
Fix: use index-based loop: for i in 0..nums.len() { nums[write] = nums[i]; }

No ||= operator for bools (LC 0010) — use |= instead:
dp[i][j] |= some_bool;  // bitwise OR assign works as logical OR for bool

#[allow(clippy::...)] — suppress specific clippy warnings (LC 0079, 0088) —
#[allow(clippy::too_many_arguments)]  for functions with > 7 params
#[allow(clippy::type_complexity)]     for long nested type annotations
```

## Byte-Level String Processing

```rust
// as_bytes() returns &[u8] — raw bytes, O(1) indexing (unlike chars())
let s = String::from("hello");
let bytes: &[u8] = s.as_bytes();
let first: u8 = bytes[0];          // b'h' = 104

// Byte literals — u8 values for ASCII characters
let space: u8 = b' ';              // 32
let a: u8 = b'a';                  // 97
let z: u8 = b'z';                  // 122

// Iterate bytes (faster than chars for ASCII-only input)
for &b in s.as_bytes() { }         // b is u8
for (i, &b) in s.as_bytes().iter().enumerate() { }

// Convert &[u8] slice back to &str
let bytes: &[u8] = &s.as_bytes()[1..4];
let substr: &str = std::str::from_utf8(bytes).unwrap();  // "ell"
// .unwrap() is safe when bytes came from a valid String

// When to use as_bytes() vs chars():
//   as_bytes() — ASCII input, need O(1) indexing, byte == char
//   chars()    — Unicode input, multi-byte chars, need char semantics

// Fixed-size array as lookup table (replaces HashMap for ASCII)
let mut freq = [0i32; 128];        // 128 ASCII slots
freq[b'a' as usize] += 1;          // direct index, no hashing
// ~20-40x faster than HashMap for small key sets (cache-friendly)
```

## HashMap Advanced Patterns

```rust
use std::collections::HashMap;

// Entry API — insert-if-missing + mutate in one step
// Ruby: hash[key] += 1 with Hash.new(0)
// Rust: HashMap has no default value — must use entry API
let mut counts: HashMap<u8, i32> = HashMap::new();
*counts.entry(b'a').or_insert(0) += 1;
//  ^-- dereference the &mut i32 returned by or_insert

// or_insert_with for lazy default (closure called only if missing)
map.entry(key).or_insert_with(Vec::new).push(value);

// if let Some(&val) — check existence + bind value in one step
if let Some(&count) = map.get(&key) {
    // `count` is i32 (copied via &), not &i32
    // Combines contains_key + indexing into one lookup
}

// Pre-allocate to avoid rehashing
let mut map: HashMap<u8, usize> = HashMap::with_capacity(128);

// Key type affects speed: HashMap<u8> > HashMap<char> > HashMap<String>
// u8 = 1-byte hash, char = 4-byte hash, String = heap + variable-length hash
```

## Useful Vec Methods

```rust
let mut v = vec![1, 2, 2, 3, 3, 3, 4];

// retain — keep only elements matching predicate (in-place)
v.retain(|&x| x != 3);             // v = [1, 2, 2, 4]

// dedup — remove consecutive duplicates (must be sorted first)
v.sort();
v.dedup();                          // v = [1, 2, 4]

// with_capacity — pre-allocate to avoid reallocations
let mut v: Vec<i32> = Vec::with_capacity(100);

// windows / chunks
let v = vec![1, 2, 3, 4, 5];
v.windows(3)                        // [1,2,3], [2,3,4], [3,4,5]
v.chunks(2)                         // [1,2], [3,4], [5]
```

## Useful str Methods

```rust
let s = "  Hello, World!  ";

// Trimming
s.trim()                            // "Hello, World!"
s.trim_start()                      // "Hello, World!  "
s.trim_end()                        // "  Hello, World!"

// Searching
s.find("World")                     // Some(9) — first byte index
s.rfind("o")                        // Some(12) — last byte index
"hello".contains("ell")             // true

// Splitting
"a.b.c".split('.').collect::<Vec<_>>()         // ["a", "b", "c"]
"a.b.c".splitn(2, '.').collect::<Vec<_>>()     // ["a", "b.c"]
"hello world".rsplit_once(' ')                  // Some(("hello", "world"))
// rsplit_once returns Option<(&str, &str)> — None if separator not found

// Slicing (byte-based, panics on non-char boundary)
let hello: &str = &"hello world"[0..5];        // "hello"

// partition_point — binary search for insert position
let v = vec![1, 3, 5, 7, 9];
let pos = v.partition_point(|&x| x < 6);       // 3 (index where 6 would go)
// Equivalent to C++ lower_bound. Returns index of first element >= target.
```

## Checked & Saturating Arithmetic

```rust
// checked_* — returns Option<T>, None on overflow
let a: Option<i32> = 100i32.checked_mul(10);        // Some(1000)
let b: Option<i32> = i32::MAX.checked_add(1);       // None

// Chain with and_then for multi-step overflow detection
let result = x.checked_mul(10).and_then(|r| r.checked_add(digit));
match result {
    Some(val) => val,
    None => return 0,       // overflow occurred
}

// saturating_* — clamps at MIN/MAX instead of overflow
i32::MAX.saturating_add(1)          // i32::MAX (does not wrap)
0i32.saturating_sub(1)              // 0 (for unsigned-like behavior)

// wrapping_* — explicitly wraps (like C behavior)
255u8.wrapping_add(1)               // 0

// Sentinel values
let sentinel = usize::MAX;          // often used as "not found" / "infinity"
let sentinel = i32::MAX;
let sentinel = i32::MIN;
```

## Benchmarking Pattern

```rust
// std::hint::black_box prevents the compiler from optimizing away
// the function call, ensuring we measure real execution time.
use std::hint::black_box;
use std::time::Instant;

let iterations = 100_000_u32;
let start = Instant::now();
for _ in 0..iterations {
    black_box(my_function(black_box(input.clone())));
}
let duration = start.elapsed();
println!("avg {:?}", duration / iterations);
```

## LeetCode Algorithm Patterns

```rust
// --- Two-pointer convergence (Container With Most Water, Trapping Rain Water) ---
let mut left = 0;
let mut right = len - 1;
while left < right {
    // process...
    if condition { left += 1; } else { right -= 1; }
}

// --- Sliding window (Longest Substring, Minimum Window Substring) ---
let mut left = 0;
for right in 0..n {
    // expand: add s[right] to window
    while window_invalid {
        // contract: remove s[left] from window
        left += 1;
    }
    // update best
}

// --- Binary search with usize (no underflow) ---
// Problem: `mid - 1` underflows when mid == 0 (usize is unsigned)
// Solution: exclusive upper bound — hi starts at len, never subtract from mid
let mut lo = 0;
let mut hi = nums.len();               // exclusive upper bound
while lo < hi {
    let mid = lo + (hi - lo) / 2;      // avoids overflow too
    if nums[mid] < target {
        lo = mid + 1;
    } else {
        hi = mid;                       // NOT mid - 1 (no underflow risk)
    }
}
// lo == insert position

// --- Greedy forward scan (Jump Game) ---
let mut max_reach = 0;
for (i, &jump) in nums.iter().enumerate() {
    if i > max_reach { return false; }  // can't reach this index
    max_reach = max_reach.max(i + jump as usize);
}

// --- BFS level-order traversal (Binary Tree Level Order) ---
use std::collections::VecDeque;
let mut queue = VecDeque::new();
queue.push_back(root);                  // seed with root node
while !queue.is_empty() {
    let level_size = queue.len();       // snapshot current level count
    for _ in 0..level_size {
        let node = queue.pop_front().expect("non-empty");
        let node_ref = node.borrow();   // RefCell → shared ref
        // process node_ref.val
        if let Some(left) = node_ref.left.clone() {
            queue.push_back(left);      // .clone() on Rc = cheap ref bump
        }
        if let Some(right) = node_ref.right.clone() {
            queue.push_back(right);
        }
    }
}
// Alt: Vec-swap BFS — process whole level as Vec, swap current/next
// (avoids VecDeque overhead but allocates a new Vec per level)

// --- HashMap single-counter trick (Minimum Window Substring) ---
// Merge need + window into ONE array; track remaining chars with ONE integer
let mut counts = [0i32; 128];
let mut total = t.len() as i32;
// Expand: if counts[r] > 0 { total -= 1; } counts[r] -= 1;
// Contract: counts[l] += 1; if counts[l] > 0 { total += 1; }
// Valid when: total == 0

// --- 2D Vec initialization (DP tables, grids) (LC 0010, 0079) ---
let mut dp = vec![vec![false; n + 1]; m + 1]; // (m+1) x (n+1) grid of false
let mut grid = vec![vec![0i32; cols]; rows];   // rows x cols grid of 0
// Ruby: Array.new(m+1) { Array.new(n+1, false) }

// --- Nested fn for recursive DFS (LC 0079) ---
// Closures can't recurse while mutably borrowing captured state
// Solution: use a nested fn, pass all state as explicit parameters
fn dfs(board: &[Vec<char>], visited: &mut [Vec<bool>],
       r: usize, c: usize) -> bool {
    // ... recursive calls to dfs() work here
    true
}

// --- isize for grid bounds checking (LC 0079) ---
// isize allows r-1 to go negative without usize underflow panic
fn dfs(r: isize, c: isize, rows: isize, cols: isize) -> bool {
    if r < 0 || c < 0 || r >= rows || c >= cols { return false; }
    let (ru, cu) = (r as usize, c as usize); // safe after bounds check
    // ...
    true
}

// --- const array/slice as zero-allocation lookup table (LC 0012) ---
const ROMAN: &[(i32, &str)] = &[(1000, "M"), (900, "CM"), (500, "D")];
// Lives in binary's read-only segment — zero heap allocation
const ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
// Direct index: ONES[n % 10] — faster than HashMap for static maps

// --- u64 bitmask as visited set for small grids (LC 0079) ---
let mut visited: u64 = 0;
let bit = 1u64 << (r * cols + c);       // up to 64 cells (e.g., 6x6 = 36)
visited |= bit;                          // mark visited
visited ^= bit;                          // backtrack (unmark)
// Single XOR vs Vec element assignment — faster, no heap allocation

// --- Flat indexing for 2D grids (LC 0079) ---
// r * cols + c avoids Vec<Vec<>> double indirection
let flat: Vec<u8> = board.iter().flatten().copied().collect();
let cell = flat[r * cols + c];

// --- copy_from_slice — bulk memory copy between slices (LC 0088) ---
nums1[start..end].copy_from_slice(&nums2[..remaining]);
// Equivalent to memcpy — faster than element-by-element loop
```
