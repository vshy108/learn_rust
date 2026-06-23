# Rust Cheatsheet

## Table of Contents

- [Variables & Mutability](#variables--mutability)
- [Scalar Types](#scalar-types)
- [Numeric Methods](#numeric-methods)
- [Compound Types](#compound-types)
- [Strings](#strings)
- [Ownership Rules](#ownership-rules)
- [References & Borrowing](#references--borrowing)
- [Slices](#slices)
- [Functions](#functions)
- [Control Flow](#control-flow)
- [Structs](#structs)
- [Enums](#enums)
- [Error Handling](#error-handling)
- [Traits](#traits)
- [Type Conversions: From / Into / TryInto / as](#type-conversions-from--into--tryinto--as)
- [Generics](#generics)
- [Lifetimes](#lifetimes)
- [Advanced Type System](#advanced-type-system)
- [Collections](#collections)
  - [Vec\<T\>](#vect)
  - [HashMap\<K, V\>](#hashmapk-v)
  - [VecDeque\<T\>](#vecdequet)
  - [HashSet\<T\>](#hashsett)
  - [BinaryHeap\<T\>](#binaryheapt)
- [Iterators](#iterators)
- [Smart Pointers](#smart-pointers)
- [Closures & Higher-Order Functions](#closures--higher-order-functions)
- [Concurrency](#concurrency)
- [Async / Await](#async--await)
- [Modules & Visibility](#modules--visibility)
- [Common Patterns](#common-patterns)
- [std Utility Functions](#std-utility-functions)
- [Useful Macros](#useful-macros)
- [Testing](#testing)
- [Cargo Commands](#cargo-commands)
- [Common Gotchas](#common-gotchas)
- [Byte-Level String Processing](#byte-level-string-processing)
- [HashMap Advanced Patterns](#hashmap-advanced-patterns)
- [Useful Vec Methods](#useful-vec-methods)
- [Useful str Methods](#useful-str-methods)
- [Checked & Saturating Arithmetic](#checked--saturating-arithmetic)
- [Benchmarking Pattern](#benchmarking-pattern)
- [Unsafe & FFI](#unsafe--ffi)
- [Performance & Memory Layout](#performance--memory-layout)
- [LeetCode Algorithm Patterns](#leetcode-algorithm-patterns)
- [BTreeMap & BTreeSet](#btreemap--btreeset)
- [Trait Objects (dyn)](#trait-objects-dyn)
- [Display Trait](#display-trait)
- [Pattern Matching Advanced](#pattern-matching-advanced)
- [Bit Manipulation](#bit-manipulation)
- [Iterator: scan (Stateful Transform)](#iterator-scan-stateful-transform)
- [Array Initialization: from_fn](#array-initialization-from_fn)
- [LeetCode Data Structures](#leetcode-data-structures)
- [LeetCode Algorithm Patterns (Extended)](#leetcode-algorithm-patterns-extended)

---

## Learning Sequence

Follow this order if the goal is to become productive quickly and then move into advanced Rust systems work.

### Foundation

Start with the language core and the ownership model:

1. [Variables & Mutability](#variables--mutability)
1. [Scalar Types](#scalar-types)
1. [Numeric Methods](#numeric-methods)
1. [Compound Types](#compound-types)
1. [Strings](#strings)
1. [Ownership Rules](#ownership-rules)
1. [References & Borrowing](#references--borrowing)
1. [Slices](#slices)
1. [Functions](#functions)
1. [Control Flow](#control-flow)
1. [Structs](#structs)
1. [Enums](#enums)
1. [Type Conversions: From / Into / TryInto / as](#type-conversions-from--into--tryinto--as)
1. [Generics](#generics)
1. [Lifetimes](#lifetimes)

### Production

Move here once the ownership model feels automatic:

1. [Error Handling](#error-handling)
1. [Traits](#traits)
1. [Collections](#collections)
1. [Iterators](#iterators)
1. [Smart Pointers](#smart-pointers)
1. [Closures & Higher-Order Functions](#closures--higher-order-functions)
1. [Modules & Visibility](#modules--visibility)
1. [Common Patterns](#common-patterns)
1. [std Utility Functions](#std-utility-functions)
1. [Testing](#testing)
1. [Cargo Commands](#cargo-commands)
1. [Concurrency](#concurrency)
1. [Async / Await](#async--await)
1. [Useful Macros](#useful-macros)

### Advanced Systems

Use this phase to understand how advanced Rust codebases are built and optimized:

1. [Advanced Type System](#advanced-type-system)
1. [Unsafe & FFI](#unsafe--ffi)
1. [Performance & Memory Layout](#performance--memory-layout)

### Practice

Use these sections to build problem-solving speed and pattern recognition:

1. [Common Gotchas](#common-gotchas)
1. [Byte-Level String Processing](#byte-level-string-processing)
1. [HashMap Advanced Patterns](#hashmap-advanced-patterns)
1. [Useful Vec Methods](#useful-vec-methods)
1. [Useful str Methods](#useful-str-methods)
1. [Checked & Saturating Arithmetic](#checked--saturating-arithmetic)
1. [Benchmarking Pattern](#benchmarking-pattern)
1. [LeetCode Algorithm Patterns](#leetcode-algorithm-patterns)
1. [BTreeMap & BTreeSet](#btreemap--btreeset)
1. [Trait Objects (dyn)](#trait-objects-dyn)
1. [Display Trait](#display-trait)
1. [Pattern Matching Advanced](#pattern-matching-advanced)
1. [Bit Manipulation](#bit-manipulation)
1. [Iterator: scan (Stateful Transform)](#iterator-scan-stateful-transform)
1. [Array Initialization: from_fn](#array-initialization-from_fn)
1. [LeetCode Data Structures](#leetcode-data-structures)
1. [LeetCode Algorithm Patterns (Extended)](#leetcode-algorithm-patterns-extended)

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

// Vec<u8> -> String conversion (LC 0345)
// from_utf8: validates UTF-8, returns Result — O(n) scan
let s = String::from_utf8(bytes).unwrap();   // panics on invalid UTF-8
// from_utf8_unchecked: skips validation — O(1), just wraps Vec
let s = unsafe { String::from_utf8_unchecked(bytes) };  // UB if invalid!
// Rule: use from_utf8() by default. Only use unchecked when you can PROVE
// the bytes are valid UTF-8 (e.g., started from a valid String, only
// swapped/modified ASCII bytes).
```

## Ownership Rules

```text
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
// Array vs Vec vs Slice — three distinct but related types
// Array  [i32; 5]   — fixed size, stack, owned
// Vec    Vec<i32>   — growable, heap, owned
// Slice  [i32]      — unsized DST (Dynamically Sized Type), can't exist alone
//                      DST = size unknown at compile time (how many elements?)
//                      Must always live behind a pointer: &[i32], Box<[i32]>
//                      Same applies to str and dyn Trait — all are DSTs
// &[i32]            — slice ref = fat pointer (addr + len, 16 bytes on 64-bit)
//                      fat pointer stores runtime size info that the DST lacks

let arr: [i32; 5] = [1, 2, 3, 4, 5];
let vec: Vec<i32> = vec![1, 2, 3, 4, 5];

// Both produce &[i32] — a borrowed view into contiguous memory
let s1: &[i32] = &arr;        // borrow entire array as slice
let s2: &[i32] = &vec;        // deref coercion: &Vec<i32> → &[i32]
let s3: &[i32] = &arr[1..3];  // sub-slice of array
let s4: &[i32] = &vec[1..3];  // sub-slice of vec

// for loops require IntoIterator — &[i32] implements it, [i32] does not
for &x in &vec[1..] { }       // &vec[1..] is &[i32], &x destructures &i32 → i32
// for x in vec[1..] { }      // ERROR: [i32] is unsized, no IntoIterator

let s = String::from("hello world");
let hello: &str = &s[0..5];    // &str slice
let world: &str = &s[6..11];

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

## Advanced Type System

```rust
// Higher-ranked trait bounds (HRTBs) let a function work for any input lifetime.
fn call_with_str<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{
    f("hello");
}

// Associated types make traits easier to use than extra generic parameters.
trait IteratorLike {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Generic associated types (GATs) allow an associated type to depend on a lifetime.
trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

// Phantom types encode extra compile-time meaning without runtime cost.
use std::marker::PhantomData;

struct Meter;
struct Second;

struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

// Type-state pattern: make invalid states unrepresentable.
struct Closed;
struct Open;

struct Door<State> {
    _state: PhantomData<State>,
}

impl Door<Closed> {
    fn open(self) -> Door<Open> {
        Door { _state: PhantomData }
    }
}
```

## Collections

### Vec\<T\>

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
v.sort_unstable_by(|a, b| {     // multi-key sort with .then() (LC 0587)
    a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))  // sort by x, then by y
});
v.sort_by_key(|x| x.abs());     // sort by derived key
v.reverse();
v.binary_search(&val);      // Result<usize, usize> — must be sorted
v.iter()                    // &T iterator (borrows; vec still usable after)
v.iter_mut()                // &mut T iterator (mutates in place; vec still usable)
v.into_iter()               // T iterator (consumes vec; vec gone after)

// iter() vs into_iter() vs iter_mut()
//   iter()      → yields &T.  Vec lives on.  Need turbofish for sum: .sum::<i32>()
//   into_iter() → yields T.   Vec is consumed (moved). Sum type inferred automatically.
//   iter_mut()  → yields &mut T. Modify elements in place, vec still usable.
//
// Rule of thumb: use into_iter() when done with the vec, iter() when you need it after.
//
// Example:
//   let v = vec![1, 2, 3];
//   let total: i32 = v.iter().sum();     // v still usable
//   let total = v.into_iter().sum();     // v consumed — can't use v after this
```

### HashMap\<K, V\>

```rust
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
```

### VecDeque\<T\>

```rust
// VecDeque<T> — double-ended queue, O(1) push/pop at both ends
use std::collections::VecDeque;
let mut q: VecDeque<i32> = VecDeque::new();
q.push_back(1);             // [1]
q.push_back(2);             // [1, 2]
q.pop_front();              // Some(1), q = [2]
q.push_front(0);            // [0, 2]
// Ruby: Array#shift is O(n); VecDeque::pop_front is O(1)
// Use VecDeque for BFS queues, sliding windows, or any FIFO pattern
```

### HashSet\<T\>

```rust
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

### BinaryHeap\<T\>

```rust
// BinaryHeap<T> — max-heap (priority queue)
use std::collections::BinaryHeap;
use std::cmp::Reverse;

let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(5);
heap.peek();                 // Some(&5) — largest element, O(1)
heap.pop();                  // Some(5)  — removes largest, O(log n)
heap.push(val);              // O(1) amortized, O(log n) worst
heap.len();
heap.is_empty();

// Min-heap — wrap values in Reverse to flip Ord comparison
let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
min_heap.push(Reverse(3));
min_heap.push(Reverse(1));
min_heap.peek();             // Some(&Reverse(1)) — smallest first
min_heap.peek().unwrap().0;  // 1 — extract inner value with .0
min_heap.pop().unwrap().0;   // 1 — pop + extract inner

// Build from vec — O(n) heapify (faster than n individual pushes)
let heap = BinaryHeap::from(vec![3, 1, 5]);

// Internals: Vec-backed binary tree
//        [10]              index 0 = root (max element)
//       /    \             parent of i  = (i-1) / 2
//     [7]    [9]           children of i = 2i+1, 2i+2
//    /   \
//  [3]   [5]
// Stored as: [10, 7, 9, 3, 5]
// push: append to end, sift up (swap with parent while larger)
// pop:  swap root with last, remove last, sift down (swap with larger child)

// Ruby comparison:
//   Ruby has no built-in heap — needs a gem or manual implementation.
//   Rust BinaryHeap is always a max-heap; use Reverse for min-heap.
//   Ruby .max/.min  →  Rust .peek() returns Option<&T>
//   Ruby .pop       →  Rust .pop() returns Option<T>
//   Ruby .size      →  Rust .len()
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

// Cell<T> — interior mutability for Copy types, no runtime borrow check
// Simpler than RefCell: just get/set, no borrow guards (LC 0380)
use std::cell::Cell;
let x = Cell::new(5);
x.set(x.get() + 1);                 // no borrow_mut() needed
// Use Cell when: field is Copy type + you need mutation through &self
// Use RefCell when: field is non-Copy (Vec, HashMap) + mutation through &self
// Cell<u64> is ideal for PRNG state in structs with &self methods

// LeetCode &self pattern — when LeetCode gives `fn insert(&self, ...)` but
// you need to mutate fields, wrap them in RefCell/Cell:
struct MyStruct {
    data: RefCell<Vec<i32>>,    // non-Copy → RefCell
    counter: Cell<u64>,          // Copy → Cell
}
impl MyStruct {
    fn mutate(&self) {           // &self, not &mut self
        self.data.borrow_mut().push(1);
        self.counter.set(self.counter.get() + 1);
    }
}

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
use std::sync::{mpsc, RwLock};

// Spawn a thread
let handle = thread::spawn(|| {
    println!("Hello from thread");
});
handle.join().unwrap();

// `move` is required when the spawned thread may outlive the current scope.
let name = String::from("rust");
thread::spawn(move || println!("{name}"));

// Share data with Arc + Mutex
let counter = Arc::new(Mutex::new(0));
let c = Arc::clone(&counter);
let h = thread::spawn(move || {
    *c.lock().unwrap() += 1;
});
h.join().unwrap();

// Use RwLock when reads dominate writes.
let config = Arc::new(RwLock::new(String::from("default")));
{
    let guard = config.read().unwrap();
    println!("{guard}");
}

// Message passing with channels
let (tx, rx) = mpsc::channel();
thread::spawn(move || tx.send(42).unwrap());
let val = rx.recv().unwrap();   // blocks until message arrives

// `Send` means a value can move to another thread; `Sync` means `&T` can be shared.
// Most thread-safe shared state combines `Arc` with `Mutex` or `RwLock`.

// Scoped threads are useful when you want to borrow from the current stack frame.
thread::scope(|scope| {
    scope.spawn(|| println!("borrowed data stays valid in scope"));
});
```

## Async / Await

```rust
// Requires tokio (or another async runtime)
use tokio;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let result = fetch_data().await;
    println!("{result}");
}

async fn fetch_data() -> String {
    // async fn returns impl Future<Output = String>
    "data".to_string()
}

// Run two futures concurrently
let (a, b) = tokio::join!(task_a(), task_b());

// `tokio::spawn` runs a future on the runtime and returns a JoinHandle.
let handle = tokio::spawn(async {
    sleep(Duration::from_millis(10)).await;
    7
});
let value = handle.await.unwrap();

// Spawn background task that can be cancelled later.
let task = tokio::spawn(async { long_work().await });
task.abort();

// CPU-bound work — don't block the async reactor
tokio::task::spawn_blocking(|| heavy_computation()).await.unwrap();

// `select!` waits for the first future that completes.
tokio::select! {
    _ = task_a() => println!("task_a finished first"),
    _ = task_b() => println!("task_b finished first"),
}

// Async code should avoid long blocking calls; move blocking work to `spawn_blocking`.
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
// Built-in macros
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

// `macro_rules!` is declarative macro syntax: it rewrites tokens before type checking.
macro_rules! say_hello {
    () => {
        println!("hello");
    };
}

say_hello!();

// Repetition: capture a variable number of expressions.
macro_rules! sum {
    ($($value:expr),* $(,)?) => {
        0 $(+ $value)*
    };
}

let total = sum!(1, 2, 3, 4);

// Hygiene: use `$crate` when a macro needs to refer to items from its own crate.
// This keeps the macro working even when re-exported from another module.
macro_rules! newtype {
    ($name:ident) => {
        pub struct $name(pub i32);
    };
}

// Procedural macros are separate crates that operate on Rust syntax trees.
// - derive macros: `#[derive(Debug, Clone, Serialize)]`
// - attribute macros: `#[tokio::main]`, `#[test]`
// - function-like macros: `sql!(...)`, `serde_json::json!(...)`
// They are used when declarative macros are not expressive enough.
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

## Unsafe & FFI

```rust
// `unsafe` allows operations the compiler cannot fully verify.
// Keep the unsafe block small and document the safety invariant around it.

// Raw pointers can be null and do not obey Rust's borrow checker.
let mut value = 5;
let ptr: *mut i32 = &mut value;
unsafe {
    *ptr += 1;
}

// `Box::into_raw` hands ownership to raw pointer land; `from_raw` takes it back.
let boxed = Box::new(10);
let raw = Box::into_raw(boxed);
unsafe {
    *raw += 1;
    let boxed_again = Box::from_raw(raw);
    println!("{boxed_again}");
}

// `repr(C)` gives a stable layout for FFI with C.
#[repr(C)]
struct Point {
    x: f32,
    y: f32,
}

// `extern "C"` declares foreign functions.
extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe {
    let positive = abs(-5);
    println!("{positive}");
}

// Slice creation from raw parts is unsafe because the pointer and length must be valid.
let data = [1u8, 2, 3, 4];
let ptr = data.as_ptr();
unsafe {
    let slice = std::slice::from_raw_parts(ptr, data.len());
    println!("{}", slice[0]);
}

// Use `MaybeUninit` for partial initialization when building arrays or buffers manually.
// It avoids reading uninitialized memory, which would be undefined behavior.
```

## Performance & Memory Layout

```rust
use std::mem::{align_of, size_of};

// `size_of` and `align_of` help explain layout and padding costs.
let size = size_of::<(u8, u32)>();
let align = align_of::<(u8, u32)>();

// `repr(C)` makes layout predictable for interop and layout-sensitive code.
#[repr(C)]
struct Packet {
    tag: u8,
    id: u32,
}

// `repr(transparent)` keeps wrapper overhead at zero for newtypes.
#[repr(transparent)]
struct UserId(u64);

// Prefer contiguous storage and pre-allocation when you know the size.
let mut values = Vec::with_capacity(1024);
values.extend(0..1024);

// Copying small `Copy` values is cheap; cloning heap types can allocate.
let bytes = [0u8; 1024];
let cloned = bytes.clone();

// Allocation-heavy work should be measured with a profiler rather than guessed.
// Common tools: `cargo bench`, `cargo flamegraph`, and `perf`/Instruments.

// `#[inline]` is a hint, not a guarantee; use it sparingly and only when measured.
#[inline]
fn hot_path(x: u64) -> u64 {
    x.wrapping_mul(2)
}

// For shared-state performance, choose the lightest synchronization primitive that fits.
// - `Atomic*` for simple counters/flags
// - `Mutex` for exclusive mutation
// - `RwLock` when reads dominate writes
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

// --- Union-Find (Disjoint Set Union) (LC 0947) ---
let mut parent: Vec<usize> = (0..n).collect(); // each node is its own root
fn find(x: usize, parent: &mut [usize]) -> usize {
    if parent[x] != x {
        parent[x] = find(parent[x], parent);   // path compression
    }
    parent[x]
}
fn union(x: usize, y: usize, parent: &mut [usize]) {
    let rx = find(x, parent);
    let ry = find(y, parent);
    if rx != ry { parent[rx] = ry; }
}
// Variant: HashMap<i32, i32> parent for sparse keys (e.g., row/col ids)
// Trick: !y (bitwise NOT) maps column y to negative space, avoiding
// row/column key collisions in a single parent map

// --- Prefix sum modular arithmetic (LC 0523) ---
// Detect subarray with sum divisible by k in O(n)
let mut mod_map: HashMap<i32, i32> = HashMap::new();
mod_map.insert(0, -1);              // sentinel: sum=0 at virtual index -1
let mut sum = 0;
for (i, &num) in nums.iter().enumerate() {
    sum = (sum + num) % k;
    if let Some(&prev) = mod_map.get(&sum) {
        if i as i32 - prev >= 2 { return true; }
    } else {
        mod_map.insert(sum, i as i32);
    }
}
// Key insight: if prefix[i] % k == prefix[j] % k, then sum(i..j) % k == 0

// --- Vec<bool> as dense lookup (alternative to HashSet) (LC 0523) ---
// When keys are bounded integers [0, k), Vec<bool> is ~10x faster than HashSet
let mut seen = vec![false; k as usize];
seen[val as usize] = true;           // O(1) direct index, no hashing
// Only works when key range is small enough to allocate
```

## BTreeMap & BTreeSet

```rust
// BTreeMap<K, V> — sorted map (red-black tree), O(log n) operations
use std::collections::BTreeMap;
let mut map: BTreeMap<i32, usize> = BTreeMap::new();
map.insert(5, 1);
map.insert(2, 1);
map.insert(8, 1);

// Always iterated in sorted key order (unlike HashMap)
for (k, v) in &map { }              // 2, 5, 8

// Range queries — O(log n + k) where k = results
map.range(3..7);                     // yields (5, 1)
map.range(3..);                      // yields (5,1), (8,1) — from 3 upward
map.range(..5);                      // yields (2,1) — below 5

// First/last key — O(log n)
map.keys().next();                   // Some(&2) — minimum key
map.keys().next_back();              // Some(&8) — maximum key

// BTreeMap as multiset (count-based) (LC 0218)
*map.entry(height).or_insert(0) += 1;    // increment count
let count = map.get_mut(&height).expect("exists");
*count -= 1;
if *count == 0 { map.remove(&height); }  // remove when count hits 0

// BTreeSet<T> — sorted set, O(log n) operations
use std::collections::BTreeSet;
let mut set: BTreeSet<i32> = BTreeSet::new();
set.insert(3);
set.insert(1);
set.insert(5);

// Iterate in sorted order
set.iter().rev().nth(1)              // second-largest element (LC 0176)

// Collect for sorted dedup (LC 0176, 0177, 0178)
let distinct: BTreeSet<i32> = values.iter().copied().collect();
// Replaces: sort + dedup in one step

// Range search — find first element >= target (LC 0220, 0363)
set.range(target..).next()           // Option<&T> — like C++ lower_bound
set.range(..target).next_back()      // largest element < target

// When to use BTree vs Hash:
//   BTreeMap — need sorted order, range queries, min/max key
//   HashMap  — only need key lookup, ~2x faster for random access
```

## Trait Objects (dyn)

```rust
// dyn Trait — dynamic dispatch via vtable (runtime polymorphism)
// Use when you need a collection of DIFFERENT types sharing a trait

// &dyn Trait — borrowed trait object (fat pointer: data ptr + vtable ptr)
fn log(logger: &dyn Logger) { logger.log("msg"); }

// Box<dyn Trait> — owned trait object on the heap
fn make_logger(verbose: bool) -> Box<dyn Logger> {
    if verbose { Box::new(VerboseLogger) }
    else { Box::new(QuietLogger) }
}

// Vec of mixed types sharing a trait
let items: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle { r: 5.0 }),
    Box::new(Square { side: 3.0 }),
];
for item in &items { item.draw(); }      // dynamic dispatch

// dyn vs generics (impl Trait):
//   impl Trait → static dispatch, monomorphized (faster, larger binary)
//   dyn Trait  → dynamic dispatch, single code path (smaller binary, vtable cost)
//   Use impl Trait by default; use dyn when you need heterogeneous collections
//   or the concrete type isn't known until runtime

// Common: Box<dyn Error> for mixed error types in main()
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _n: i32 = "42".parse()?;     // ParseIntError
    let _s = std::fs::read_to_string("f")?;  // io::Error
    Ok(())
}
```

## Display Trait

```rust
use std::fmt;

// impl Display — custom {} formatting (user-facing output)
struct Temperature { celsius: f64 }

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}C", self.celsius)
    }
}
println!("{}", Temperature { celsius: 36.6 });  // "36.6C"

// Display for enums — match each variant
impl fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TrafficLight::Red => "RED",
            TrafficLight::Yellow => "YELLOW",
            TrafficLight::Green => "GREEN",
        };
        write!(f, "{name}")
    }
}

// Display vs Debug:
//   Debug  ({:?}) — #[derive(Debug)], for developers, auto-generated
//   Display ({})  — must impl manually, for end users
//   Display auto-provides .to_string() via ToString trait
```

## Pattern Matching Advanced

```rust
// @ binding — bind a value while also testing a pattern
match status_code {
    c @ 400..=499 => println!("Client error: {c}"),
    c @ 500..=599 => println!("Server error: {c}"),
    _ => {}
}

// Match guard — add a condition with `if`
match n {
    n if n > 0 => "positive",
    0 => "zero",
    _ => "negative",
}

// Slice patterns — destructure arrays/slices
match slice {
    [] => println!("empty"),
    [one] => println!("single: {one}"),
    [first, rest @ ..] => {
        // `first` is the first element, `rest` is the remaining slice
        println!("first: {first}, rest has {} items", rest.len());
    }
}

// Nested destructuring
match point {
    Point { x: 0, y } => println!("on y-axis at {y}"),
    Point { x, y: 0 } => println!("on x-axis at {x}"),
    Point { x, y } => println!("({x}, {y})"),
}

// Tuple + Option destructuring (common in linked list merge) (LC 0021)
match (list1, list2) {
    (None, None) => None,
    (Some(n), None) | (None, Some(n)) => Some(n),
    (Some(l), Some(r)) => { /* merge logic */ }
}
```

## Bit Manipulation

```rust
// Bitwise operators
a & b          // AND
a | b          // OR
a ^ b          // XOR — same bits cancel: a ^ a == 0, a ^ 0 == a
!a             // NOT (bitwise complement)
a << n         // left shift
a >> n         // right shift

// XOR to find single unique element (LC 0136)
let single = nums.into_iter().fold(0, |acc, x| acc ^ x);
// Every duplicate pair XORs to 0, leaving only the unique element

// count_ones — popcount (number of set bits) (LC 0231, 0338, 0401)
let n: i32 = 0b1011;
n.count_ones()                       // 3 (u32 result)
// Power of two check: n > 0 && n.count_ones() == 1

// Gray code formula (LC 0089)
let gray = i ^ (i >> 1);            // each consecutive pair differs by 1 bit

// Bitmask as set — up to 64 elements (LC 0079, 0216)
let mut mask: u64 = 0;
mask |= 1u64 << idx;                // add element idx
mask &= !(1u64 << idx);             // remove element idx
mask ^= 1u64 << idx;                // toggle (add/remove)
mask & (1u64 << idx) != 0           // contains element idx
mask.count_ones()                    // set size

// Enumerate all subsets of a bitmask (LC 0216)
for mask in 0..(1u32 << n) {
    if mask.count_ones() == k {
        // process subset represented by `mask`
    }
}
```

## Iterator: scan (Stateful Transform)

```rust
// .scan(init, |state, item| Option<B>) — stateful map
// Like .map() but carries mutable state between iterations.
// Returns None to stop iteration early.

// Prefix products without extra allocation (LC 0238)
let prefix: Vec<i32> = nums
    .iter()
    .scan(1, |acc, &x| {
        let prev = *acc;         // emit PREVIOUS accumulated value
        *acc *= x;               // update accumulator
        Some(prev)               // yield the prefix product before this element
    })
    .collect();
// nums = [1, 2, 3, 4]  →  prefix = [1, 1, 2, 6]
//   acc starts at 1, emits 1, then acc=1; emits 1, acc=2; emits 2, acc=6; emits 6

// When to use scan vs fold:
//   fold   — reduces to a single value (consumer)
//   scan   — yields intermediate values (adaptor, lazy)
//   map    — stateless transform
//   scan   — stateful transform with accumulator
```

## Array Initialization: from_fn

```rust
// std::array::from_fn — initialize fixed-size arrays with a closure
// Useful for arrays of non-Copy types (where [val; N] won't work)

// Trie children: array of 26 Option<Box<TrieNode>> (LC 0208, 0211, 0212)
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}
impl TrieNode {
    fn new() -> Self {
        Self {
            children: std::array::from_fn(|_| None),  // each slot is None
            is_end: false,
        }
    }
}

// Why not [None; 26]?
// [None; 26] works only if the element type implements Copy.
// Option<Box<T>> does NOT implement Copy (Box is non-Copy).
// std::array::from_fn calls the closure 26 times, creating each None independently.
```

## LeetCode Data Structures

```rust
// --- Trie (Prefix Tree) (LC 0208, 0211, 0212, 0421) ---
// Two variants: HashMap children (flexible) or [Option<Box>; 26] (faster)

// HashMap variant — supports any character set
struct TrieNode {
    children: HashMap<u8, Box<TrieNode>>,
    is_end: bool,
}

// Array variant — fixed 26 lowercase letters, faster cache performance
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

// Insert: walk/create nodes for each byte
fn insert(&mut self, word: &str) {
    let mut node = &mut self.root;
    for &b in word.as_bytes() {
        let idx = (b - b'a') as usize;
        node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
    }
    node.is_end = true;
}

// Search: walk nodes, check is_end
fn search(&self, word: &str) -> bool {
    let mut node = &self.root;
    for &b in word.as_bytes() {
        let idx = (b - b'a') as usize;
        match &node.children[idx] {
            Some(child) => node = child,
            None => return false,
        }
    }
    node.is_end
}

// --- Linked List (LC 0002, 0019, 0021, 0206) ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    pub fn new(val: i32) -> Self { ListNode { val, next: None } }
}

// Iterative traversal with dummy head (LC 0021)
let mut dummy = ListNode::new(0);
let mut tail = &mut dummy;
// ... build list by setting tail.next, then advance tail
// return dummy.next

// Reverse a linked list (LC 0206)
let mut prev: Option<Box<ListNode>> = None;
let mut curr = head;
while let Some(mut node) = curr {
    curr = node.next.take();         // detach next
    node.next = prev;                // point backward
    prev = Some(node);               // advance prev
}
// prev is now the new head

// Fast/slow pointer for cycle detection (LC 0141, 0142)
// With Rc<RefCell<ListNode>> for shared ownership:
let mut slow = head.clone();
let mut fast = head.clone();
loop {
    slow = slow.and_then(|n| n.borrow().next.clone());
    fast = fast
        .and_then(|n| n.borrow().next.clone())
        .and_then(|n| n.borrow().next.clone());
    // if slow == fast → cycle found
}

// --- Binary Tree (LC 0094, 0100, 0102, 0104) ---
use std::rc::Rc;
use std::cell::RefCell;

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeLink,
    pub right: TreeLink,
}
impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}
// Access: node.borrow().val, node.borrow().left.clone()
// Modify: node.borrow_mut().val = 5
```

## LeetCode Algorithm Patterns (Extended)

```rust
// --- Kadane's Algorithm (Maximum Subarray) (LC 0053) ---
let mut max_sum = nums[0];
let mut current = 0;
for &num in &nums {
    current += num;
    max_sum = max_sum.max(current);
    if current < 0 { current = 0; }     // reset — negative prefix hurts
}

// --- Monotonic Stack (LC 0084, 0085, 0239, 0316) ---
// Maintain a stack where elements are in increasing (or decreasing) order.
// When a new element violates the monotonic property, pop and process.
let mut stack: Vec<usize> = Vec::new();    // stores indices
for i in 0..=n {
    let curr_h = if i < n { heights[i] } else { 0 };  // sentinel to flush
    while let Some(&top) = stack.last() {
        if heights[top] <= curr_h { break; }
        stack.pop();
        let h = heights[top];
        let w = if stack.is_empty() {
            i as i32
        } else {
            (i - stack.last().unwrap() - 1) as i32
        };
        max_area = max_area.max(h * w);
    }
    stack.push(i);
}

// --- Topological Sort (Kahn's BFS) (LC 0207, 0210, 0269) ---
let n = num_courses as usize;
let mut adj = vec![vec![]; n];
let mut in_degree = vec![0usize; n];
for e in &prerequisites {
    adj[e[1] as usize].push(e[0] as usize);
    in_degree[e[0] as usize] += 1;
}
// Seed queue with all nodes that have in_degree == 0
let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
let mut order = Vec::with_capacity(n);
while let Some(node) = queue.pop_front() {
    order.push(node);
    for &next in &adj[node] {
        in_degree[next] -= 1;
        if in_degree[next] == 0 { queue.push_back(next); }
    }
}
// If order.len() == n → valid ordering; otherwise cycle exists

// --- Backtracking Template (LC 0039, 0046, 0078) ---
fn backtrack(
    candidates: &[i32],
    remaining: i32,
    start: usize,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if remaining == 0 {
        result.push(current.clone());    // found valid combination
        return;
    }
    for i in start..candidates.len() {
        if candidates[i] > remaining { break; }  // pruning (requires sorted input)
        current.push(candidates[i]);
        backtrack(candidates, remaining - candidates[i], i, current, result);
        current.pop();                   // undo choice (backtrack)
    }
}

// --- DP Space Optimization: 1D Rolling Array (LC 0062, 0072, 0221) ---
// Replace 2D dp[m][n] with 1D dp[n], reusing the same row.
// `prev` variable saves dp[i-1][j-1] before it gets overwritten.
let mut dp = vec![0i32; n];
for i in 0..m {
    let mut prev = 0;                    // dp[i-1][j-1] (diagonal)
    for j in 0..n {
        let old = dp[j];                 // save before overwrite
        dp[j] = /* recurrence using dp[j-1], dp[j] (above), prev (diagonal) */;
        prev = old;                      // old dp[j] becomes next diagonal
    }
}

// --- Graph Adjacency List (LC 0207, 0210, 0269, 0310) ---
let n = num_nodes;
let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
for edge in &edges {
    adj[edge[0]].push(edge[1]);          // directed
    adj[edge[1]].push(edge[0]);          // add this for undirected
}

// --- Interval Merge (LC 0056) ---
intervals.sort_unstable_by_key(|i| i[0]);   // sort by start
let mut merged: Vec<Vec<i32>> = Vec::new();
for interval in intervals {
    if let Some(last) = merged.last_mut() {
        if interval[0] <= last[1] {
            last[1] = last[1].max(interval[1]);  // extend overlap
            continue;
        }
    }
    merged.push(interval);
}

// --- Custom Ord for BinaryHeap (LC 0272) ---
// BinaryHeap uses Ord trait. Implement Ord to customize ordering.
#[derive(Eq, PartialEq)]
struct HeapEntry { dist: i64, val: i32 }
impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)       // reverse for min-heap behavior
    }
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// --- Vec::swap for in-place rearrangement (LC 0031, 0041) ---
nums.swap(i, j);                         // O(1), no temp variable needed
// Common in: next permutation, cycle sort, first missing positive
```
