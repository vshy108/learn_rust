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
v.push(4);
v.pop();                    // Option<i32>
v.len();
v[0];                       // panics if OOB
v.get(0);                   // Option<&i32>
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

// HashSet<T>
use std::collections::HashSet;
let mut set: HashSet<i32> = HashSet::new();
set.insert(1);
set.contains(&1);
set.remove(&1);
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
v.iter().chain(other.iter())

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
v.iter().fold(0, |acc, &x| acc + x)
```

## Smart Pointers

```rust
// Box<T> — heap allocation
let b = Box::new(5);

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

// --- HashMap single-counter trick (Minimum Window Substring) ---
// Merge need + window into ONE array; track remaining chars with ONE integer
let mut counts = [0i32; 128];
let mut total = t.len() as i32;
// Expand: if counts[r] > 0 { total -= 1; } counts[r] -= 1;
// Contract: counts[l] += 1; if counts[l] > 0 { total += 1; }
// Valid when: total == 0
```
