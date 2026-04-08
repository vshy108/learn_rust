# Rust Concepts Index — LeetCode Problems

A centralized reference of Rust concepts encountered across LeetCode problems.
Each concept is listed once with the LC problem numbers where it appears.
Study any listed problem to see the concept in action.

---

## Table of Contents

- [Ownership and Borrowing](#ownership-and-borrowing)
- [Type System and Casting](#type-system-and-casting)
- [Strings and Bytes](#strings-and-bytes)
- [Collections — Vec](#collections--vec)
- [Collections — HashMap and HashSet](#collections--hashmap-and-hashset)
- [Collections — BTreeMap and BTreeSet](#collections--btreemap-and-btreeset)
- [Collections — BinaryHeap](#collections--binaryheap)
- [Collections — VecDeque](#collections--vecdeque)
- [Option and Result](#option-and-result)
- [Pattern Matching](#pattern-matching)
- [Iterators and Closures](#iterators-and-closures)
- [Numeric Operations and Overflow](#numeric-operations-and-overflow)
- [Bitwise Operations](#bitwise-operations)
- [Linked Lists](#linked-lists)
- [Trees — Rc RefCell TreeNode](#trees--rc-refcell-treenode)
- [Sorting and Searching](#sorting-and-searching)
- [Dynamic Programming Patterns](#dynamic-programming-patterns)
- [Struct and Encapsulation](#struct-and-encapsulation)
- [Concurrency and Thread-Local](#concurrency-and-thread-local)
- [Unsafe and Performance](#unsafe-and-performance)
- [Clippy and Compiler Directives](#clippy-and-compiler-directives)
- [Algorithmic Patterns](#algorithmic-patterns)
- [Ruby vs Rust Gotchas](#ruby-vs-rust-gotchas)

---

## Ownership and Borrowing

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `Box<T>` heap allocation | Smart pointer for known-size heap data; required for recursive types like `ListNode` | 0002, 0206 |
| `.take()` on Option | Moves value out of `Option` leaving `None` in its place; avoids double-move | 0002, 0019, 0024, 0025, 0083, 0086, 0114, 0143, 0147, 0203, 0226, 0237, 0430, 0445, 0450 |
| `as_deref()` | Converts `&Option<Box<T>>` to `Option<&T>` for borrowing without consuming | 0002 |
| Consuming `Vec` with `into_iter()` | Consumes the Vec, avoiding borrows; yields owned values | 0134 |
| `&mut Vec<i32>` borrowing | One mutable borrow allows multiple index accesses | 0027, 0080, 0088 |
| `for &sq in &squares` | Borrow Vec (not consume) + destructure `&i32` into owned `i32`; `for sq in vec` MOVES the Vec | 0279 |
| Borrow traversal `let mut curr = &head` | Borrow chain to count list length without consuming | 0876 |
| `.clone()` on `Option<Rc<...>>` | Cheap ref-count bump, not a deep copy; needed to move fields out of borrowed RefCell | 0222 |
| `Rc::clone(&root)` vs `root.clone()` | Idiomatic explicit Rc clone (makes intent clear) | 0222 |
| DST vs fat pointer | `[i32]` is unsized; `&[i32]` is a fat pointer that implements `IntoIterator` | 0121 |
| `into_bytes()` | Owned `Vec<u8>` from String (vs `as_bytes()` which borrows); useful when mutation needed | 0079, 0127, 0345, 0484, 1323 |
| Copy trait and index-based assignment | `i32` is Copy so index-based assignment works without explicit `.clone()` | 0027 |

## Type System and Casting

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `as usize` / `as i32` casts | Rust won't auto-cast between integer types; explicit `as` needed for Vec indexing | 0004, 0012, 0027, 0080, 0088, 0219, 0704 |
| `as f64` for float division | Integer division truncates; cast to `f64` first for decimal results | 0004 |
| `as i64` / `as i32` widening/narrowing | Widen for overflow-safe arithmetic, narrow for return types | 0374, 0446, 2256 |
| `i64` key for difference | `nums[i] as i64 - nums[j] as i64` avoids i32 overflow for full-range values | 0446 |
| `i64` widening for sum | `nums.iter().map(|&x| x as i64).sum()` prevents overflow for large accumulations | 2256 |
| `bool as i32` cast | `true` -> 1, `false` -> 0; replaces if-else with branchless arithmetic | 0409 |
| `usize` arithmetic and underflow | Unsigned type; subtracting past 0 panics in debug. Guard with `> 0` checks | 0005, 0035, 0058, 0135, 0189, 0279 |
| Integer division truncation | Rust `/` truncates toward zero; matches "round down" for non-negative values | 2256 |
| `for i in 1..=n as usize` | Cast on range bound, not loop variable; `for i as usize in ...` won't compile | 0279 |

## Strings and Bytes

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `s.as_bytes()` | Returns `&[u8]` with O(1) indexing; zero-cost for ASCII problems | 0005, 0008, 0058, 0076, 0079, 0121, 0205, 0392, 0409, 1544 |
| `String` can't be indexed by `usize` | UTF-8 encoding means byte index might split multi-byte char; use `as_bytes()` or `chars()` | 0008, 0345 |
| `s.chars()` and `s.char_indices()` | Yields Unicode scalar values; `.char_indices()` gives `(byte_index, char)` | 0003 |
| `.bytes()` iterator | Yields owned `u8` values (no lifetime tie to String), unlike `as_bytes()` which borrows | 0017, 0020, 0038, 0049, 0125, 0208, 0242, 0266, 0291, 0383, 0387, 0389, 0392, 0408, 0415, 0468, 0472, 0500 |
| `s.into_bytes()` | Converts owned String into `Vec<u8>` — mutable, indexable | 0079, 0345, 1323 |
| `b'x'` byte literals | `b' '`, `b'0'` are `u8` values (ASCII byte); use with `as_bytes()` indexing | 0008, 0058 |
| `b"aeiouAEIOU"` byte string literal | `&[u8; 10]`; `.contains(&byte)` does linear scan | 0345 |
| `Vec<char>` for O(1) indexing | `String` doesn't support `s[i]`; `Vec<char>` gives O(1) random access | 0005 |
| `String::push(char)` vs `push_str(&str)` | `.push()` appends single char; `.push_str()` appends slice | 0006 |
| `String::with_capacity(n)` | Pre-allocates n bytes to avoid reallocations | 0006, 1047 |
| `String::from_utf8(bytes)` | Safe conversion from `Vec<u8>` back to String with UTF-8 validation | 0022, 0038, 0051, 0060, 0067, 0093, 0126, 0151, 0267, 0282, 0306, 0316, 0345, 0405, 0415, 0482, 1047 |
| `unsafe { String::from_utf8_unchecked() }` | Skips UTF-8 validation; caller must guarantee valid UTF-8 | 0345 |
| `std::str::from_utf8()` | Converts `&[u8]` back to `&str` (borrowed, not owned) | 0076 |
| `str::find()` | Returns `Option<usize>`; use `map_or(-1, \|i\| i as i32)` for LC return | 0028 |
| `str::rsplit_once(' ')` | Split at last occurrence; returns `Option<(&str, &str)>` | 0028, 0058 |
| `str::starts_with(&str)` | Auto-derefs `&String` to `&str`; like Ruby `start_with?` | 0014 |
| `String::pop()` | Removes and returns last Unicode char; safer than `truncate(len-1)` for non-ASCII | 0014 |
| `split_whitespace()` | Splits on any Unicode whitespace, trims edges, collapses runs of spaces | 0151, 0192, 0194, 0290, 0434 |
| `trim_start()` | Returns `&str` with leading whitespace removed; zero-cost slice adjustment | 0008 |
| `is_ascii_digit()` | Method on `u8` and `char`; equivalent to `>= b'0' && <= b'9'` | 0008 |
| `eq_ignore_ascii_case(&ch)` | Stdlib case-insensitive char comparison | 1544 |
| `String::ends_with(char)` | O(1) check on last byte for ASCII | 1047 |
| `PartialEq<String>` for str | `&str == String` comparison works automatically via auto-deref | 0028 |
| `Vec<String>::concat()` | Joins all Strings with no separator | 0006 |
| `.collect::<Vec<_>>().join(" ")` | Materialise iterator then concatenate with separator | 0151 |
| `into_iter().collect::<String>()` | Consumes `Vec<String>` and concatenates into single owned String | 1662 |
| XOR 32 case-flip trick | `top ^ b == 32` exploits ASCII layout where upper/lower differ only in bit 5 | 1544 |

## Collections — Vec

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `vec![val; n]` init | Creates n copies of val; for `String`, each is independently owned | 0006, 0135, 0238, 0274, 0279, 0446 |
| `vec![vec![val; n]; m]` 2D init | Creates 2D Vec; each inner Vec is independent (no shared refs) | 0010 |
| `Vec::with_capacity(n)` | Pre-allocates space when size is known; avoids reallocations | 0006, 0042 |
| `Vec::dedup()` / `dedup_by()` | Removes consecutive duplicates in-place; only works on sorted data | 0022, 0026, 0083, 0128, 0315, 0327, 0356 |
| `Vec::retain()` | Predicate-based in-place removal; like Ruby `.select!` | 0027 |
| `Vec::push` / `Vec::pop` | Append/remove last element; O(1) amortized | 0014, 1047 |
| `Vec::insert(pos, val)` | Shifts elements right — O(n) but cache-friendly due to contiguous memory | 0295 |
| `result[i] = val` | Direct index assignment (overwrite); NOT `insert` which shifts | 0238 |
| `result[i] *= suffix` | Augmented assignment works on Vec elements | 0238 |
| `copy_from_slice` | Bulk memcpy for remaining elements; faster than element-by-element copy | 0088 |
| `.swap(i, j)` on slice/Vec | Swaps two elements in place; no parallel assignment in Rust | 0031, 0041, 0046, 0075, 0186, 0189, 0215, 0280, 0283, 0344, 0345, 0384, 0442, 0448 |
| `.rotate_left(k)` / `.rotate_right(k)` | Stdlib O(n) rotation; block-swap algorithm internally | 0061, 0081, 0153, 0154, 0189 |
| `.reverse()` on slice | In-place O(n) reversal with no allocation | 0025, 0031, 0048, 0067, 0092, 0103, 0107, 0126, 0145, 0151, 0168, 0186, 0189, 0210, 0269, 0282, 0306, 0332, 0344, 0368, 0405, 0415, 0426, 0482 |
| `slice::partition_point()` | Stdlib lower-bound binary search on sorted slice | 0034, 0035, 0259, 0300, 0315, 0327, 0362, 0436, 0475, 0480, 0497 |
| `.windows(n)` on slice | Overlapping windows of length n; iterator adaptor, no index math | 0031, 0098, 0118, 0122, 0163, 0164, 0180, 0188, 0217, 0249, 0252, 0444 |
| `.chunks(n)` on slice | Non-overlapping chunks of length n; last chunk may be shorter | 0459 |
| `.split_at(mid)` | Splits slice into two at index; returns `(&[T], &[T])` | 0327 |
| `VecDeque` as double-ended queue | O(1) push/pop at both ends; used for BFS and sliding window | 0094, 0102, 0103, 0107, 0116, 0126, 0127, 0130, 0146, 0199, 0207, 0210, 0225, 0226, 0232, 0239, 0269, 0281, 0286, 0297, 0301, 0310, 0317, 0322, 0346, 0353, 0365, 0399, 0417, 0427, 0433, 0444, 0488, 0499, 0505, 0513 |
| Moving out of Vec by index is forbidden | `strs[0]` tries to MOVE String; use `.clone()` or work with refs | 0014 |
| `&strs[1..]` to borrow a slice | Yields references without transferring ownership | 0014 |
| `Vec<u8>` as byte stack | push/pop O(1) for character matching; `.last() == Some(&b)` peeks | 1047, 1544 |
| Flat indexing `r*cols+c` | Avoids `Vec<Vec<>>` double indirection | 0079 |
| `for range (0..len)` vs `for-in` iterator | Index-based loop avoids borrow-after-move issues | 0027, 0080 |

## Collections — HashMap and HashSet

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `HashMap<K, V>` basics | `.get(&key)` takes `&K` returns `Option<&V>`; key must be `Hash + Eq` | 0001, 0003 |
| `HashMap::entry` API | Single-lookup insert-or-mutate: `.entry(k).or_insert(0) += 1` | 0076, 0380, 0446, 0835 |
| `Entry::Occupied` / `Entry::Vacant` | Avoids double hashing from `contains_key` + `insert` | 0380 |
| `.entry(b).or_default()` | Uses `Default` trait (e.g., empty Vec) for missing keys | 0514 |
| `.and_modify(\|d\| ...).or_insert(v)` | Mutate existing or insert default; entry API pattern | 0076 |
| `HashMap::from([...])` inline init | Convenient but heap-allocates on every call; prefer `match` for hot paths | 0013 |
| `map[&key]` Index trait | Requires `&K` reference, not `K` directly; panics if key missing | 0013 |
| `HashMap::with_capacity(n)` | Pre-allocate to reduce rehashing | 0142, 0446 |
| `HashMap<(i32, i32), i32>` tuple key | Compound key for multi-dimensional counting | 0835 |
| `HashMap<(usize, usize), i32>` memo | Tuple key for DP memoization state | 0010, 0514 |
| `Vec<HashMap<i64, i32>>` per-index DP | Each element has its own map of diff -> count | 0446 |
| `if let Some(&val) = map.get(&key)` | Combined existence check + binding with destructure | 0076, 0380 |
| `.get().copied().unwrap_or(0)` | Safe HashMap lookup returning owned value | 0079 |
| `.values().copied().max().unwrap_or(0)` | Chain to extract max from HashMap values | 0835 |
| `HashSet<char>` for tracking | Track which characters are in current window | 0003 |
| `HashSet::insert` returns `bool` | Returns `false` if already present; dual-purpose check+insert | 0142 |
| `HashSet::with_capacity(n)` | Pre-allocate to avoid rehashing | 0142 |
| `.remove(&val)` on HashSet | Takes `&T`, not `T` | 0219 |
| `*num` dereference for HashSet | `.iter()` yields `&i32` but `HashSet<i32>::insert` takes `i32` | 0219 |
| Fixed-size array `[i32; 128]` as lookup table | Stack-allocated, O(1) indexed by ASCII; replaces HashMap | 0076, 0205, 0345, 0409 |

## Collections — BTreeMap and BTreeSet

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `BTreeMap<K, V>` | Sorted map with O(log n) operations; useful when you need ordered keys, range queries, or `first_key_value()`/`last_key_value()` | 0218, 0220, 0262, 0314, 0352, 0432, 0460, 0480 |
| `BTreeSet<T>` | Sorted set with O(log n) insert/remove/contains; `.range()` for range queries | 0176, 0177, 0178, 0220, 0363, 0381, 0460 |

## Collections — BinaryHeap

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `BinaryHeap<T>` | Rust's only built-in heap — always a max-heap | 0264, 0272, 0295, 0313, 0347, 0355, 0358, 0373, 0378, 0407, 0499, 0502, 0505 |
| `BinaryHeap<Reverse<i32>>` for min-heap | `std::cmp::Reverse` wraps value and flips `Ord`; push with `Reverse(val)`, extract with `.0` | 0264, 0295, 0313, 0347, 0373, 0378, 0407, 0499, 0502, 0505 |
| `.peek()` returns `Option<&T>` | Need `*` to dereference; unlike Ruby's `.max` which returns value directly | 0295 |
| `.pop()` returns `Option<T>` | Must `.unwrap()` or pattern match | 0295 |
| `binary_search(&val)` returns `Result<usize, usize>` | `Ok(idx)` if found, `Err(idx)` if not; `.unwrap_or_else(\|i\| i)` collapses both | 0074, 0167, 0170, 0240, 0295, 0354, 0480, 0501 |

## Collections — VecDeque

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `VecDeque<T>` | Double-ended queue with O(1) amortized push/pop at both ends | 0094, 0102, 0103, 0107, 0116, 0126, 0127, 0130, 0146, 0199, 0207, 0210, 0225, 0232, 0239, 0269, 0281, 0286, 0297, 0301, 0310, 0322, 0346, 0353, 0365, 0399, 0417, 0427, 0433, 0444, 0488, 0499, 0505, 0513 |
| `VecDeque` for BFS | Standard BFS queue: `push_back()` to enqueue, `pop_front()` to dequeue | 0094, 0102, 0107, 0116, 0126, 0127, 0130, 0199, 0207, 0210, 0226, 0269, 0286, 0297, 0301, 0310, 0417, 0433, 0444, 0488, 0505, 0513 |
| `VecDeque` as sliding window deque | Maintain monotonic invariant for O(1) min/max queries | 0239, 0346, 0353 |

## Option and Result

| Concept | Description | LeetCode |
|---------|-------------|----------|
| Pattern matching `Some(node)` / `None` | Rust's null-safety: no implicit nil checks like Ruby | 0002, 0058 |
| `Option::map_or` | Transforms `Some`/`None` into a single return value | 0028 |
| `map_or(true, \|&last\| ...)` | Provides default for empty case | 3437 |
| `.and_then(\|f\| next(f))` | Option chaining for two-hop traversal (like Ruby's `&.`) | 0019, 0116, 0138, 0141, 0142, 0143, 0160, 0332, 0432, 0876 |
| Option state machine | `let mut current = iter.next()` + `if let Some(target)` to drive manual iterator | 0392 |
| `match` with early return | `match expr { Some(v) => v, None => return 0 }` — idiomatic unwrap-or-bail | 0007 |
| `.expect()` for invariant assertions | Panics with message if None; use only when invariant is guaranteed | 0142 |
| Some(&val) destructuring | Copies the inner value out of `&T` if `T: Copy` (e.g., `usize`) | 0001, 0380 |

## Pattern Matching

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `match` on `(Option, Option)` tuple | Destructures both lists simultaneously; arms checked top-to-bottom | 0021 |
| `match` on tuple `(a % 3, a % 5)` | Pattern-matches two values simultaneously; cleaner than cascading if-else | 0412 |
| `match` with guard | `Some(&v) if v != ct =>` replaces Ruby's postfix `return false if` | 0205 |
| `match` as zero-cost lookup | Compiles to a jump table; no heap allocation (vs HashMap) | 0013 |
| `match` on `Ordering` | `std::cmp::Ordering::{Less, Equal, Greater}` for three-way comparison | 0154, 0165, 0167, 0272, 0374, 0450 |
| `unreachable!()` macro | Marks impossible branches; panics with message in debug | 0013, 0075, 0150, 0241, 0270, 0353 |
| `while let Some(mut node) = current` | Destructure Option + move ownership in loop | 0206 |
| `if let Some(target)` | Single-pattern matching; cleaner than full `match` for one case | 0392 |
| Tuple destructuring `let (mut i, mut j, mut k) = (...)` | Compact multi-variable init | 0088, 0121 |
| `(center_index, &max_len)` pattern destructuring | Destructures `&usize` into owned `usize` (Copy types) | 0005, 0012 |
| `.1` tuple field access | Extract nth element from fold/tuple result | 0121 |

## Iterators and Closures

| Concept | Description | LeetCode |
|---------|-------------|----------|
| Iterator chaining `once`, `chain`, `flat_map` | Building transformed data by composing iterator adapters | 0005 |
| `.map().sum()` / `.filter().map().sum()` | Functional chain for aggregate computations | 0005, 0042, 0122 |
| `.iter().sum::<i32>()` turbofish | Disambiguates accumulator type on `sum()` | 0724 |
| `.iter().map(\|r\| r.iter().sum::<i32>()).max()` | Nested iterator chain: sum each row, find global max | 1672 |
| `.enumerate()` | Yields `(index, element)` — opposite of Ruby's `each_with_index` | 0045, 0134, 0219, 0724, 2256 |
| `.zip()` | Pairs elements from two iterators | 0134, 0238 |
| `.take(n)` / `.take_while()` | Limit iterator length or yield while predicate holds | 0008, 0045, 0279 |
| `(1..).take_while(\|&i\| i * i <= n)` | Infinite range with lazy termination | 0279 |
| `(0..n).rev()` | Reverse range iterator for right-to-left traversal | 0042, 0135 |
| `(1..=n).map().collect()` | Inclusive range + map + collect for Vec building | 0412 |
| `.scan(init, \|acc, x\| ...)` | Stateful iterator transform; yields value each step while mutating accumulator | 0238, 1480 |
| `fold(init, \|acc, x\| ...)` | Accumulates like Ruby's `inject`; `take_while` + `fold` is common | 0008, 0121 |
| `.collect::<Vec<_>>()` turbofish | Specifies collection target type | 0151, 0238, 0412 |
| `peekable()` iterator | Adds `.peek()` — look at next element without consuming | 0008 |
| `.clamp(min, max)` | Returns value clamped to [min, max]; built-in on all numeric types | 0008 |
| Closures capturing by reference | `let expand = \|left, right\| { ... }` captures outer variables by reference | 0005, 0587 |
| Nested `fn` vs closure | Recursive DFS needs `&mut` state but closures can't recurse; use nested functions | 0079, 0345 |
| `.unwrap_or(0)` on Option | Handles empty iterator case safely | 0124, 0139, 0155, 0164, 0269, 0272, 0295, 0333, 0395, 0402, 0414, 0437, 0468, 0496, 0508, 1672 |
| `.unwrap_or_default()` | Returns `Default::default()` for `None`; zero for numbers, empty for collections | 0432 |
| `.any(\|x\| pred)` | Short-circuit iterator test; returns `true` if any element matches | 0087, 0211, 0217, 0302, 0421, 0497 |
| `.all(\|x\| pred)` | Short-circuit iterator test; returns `true` if all elements match | 0098, 0210, 0242, 0252, 0346, 0399, 0411, 0438, 0459, 0468, 0480, 0500 |
| `.position(\|x\| pred)` | Returns `Option<usize>` index of first matching element | 0105, 0106, 0389, 0402, 0496 |
| `.skip(n)` | Skips first n elements of an iterator | 0298, 0407, 0427, 0431, 0452 |
| `.skip_while(\|x\| pred)` | Skips elements while predicate holds; yields remaining | 0402 |
| `move \|...\|` closure | Captures variables by value (ownership) instead of by reference | 0419 |
| `iter().flatten().filter()` | Counting elements across 2D grid without nested loops | 0079 |

## Numeric Operations and Overflow

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `i32::MIN` / `i32::MAX` sentinels | Used for empty partitions and unreachable min/max values | 0004, 0029, 0041, 0053, 0069, 0120, 0155, 0174, 0188, 0250, 0258, 0265, 0279, 0286, 0313, 0356, 0375, 0391, 0405, 0449, 0499, 0505 |
| `i32::MIN.abs()` panics | `-2_147_483_648` has no positive i32 representation; guard first | 0007 |
| Remainder preserves sign | `-123 % 10 == -3` (not 7); can process negatives directly | 0007 |
| `checked_mul` / `checked_add` | Return `Option<i32>`; `None` on overflow. Chain via `.and_then()` | 0007 |
| `saturating_sub()` / `saturating_add()` | Prevents overflow/underflow; clamps to min/max instead of panicking | 0015, 0016, 0131, 0135, 0139, 0279, 0289, 0293, 0294, 0321, 0345, 0355, 0403, 0444 |
| `wrapping_add()` / `wrapping_mul()` | Wraps around on overflow (modular arithmetic); useful for hash computation | 0249, 0260, 0266, 0307, 0308, 0315, 0325, 0327, 0353, 0375, 0411, 0465 |
| `usize::MAX` sentinel | Used as "infinity" or "not found" marker for usize values | 0209, 0353, 0366, 0368, 0387 |
| Floating-point division | `5 / 2 == 2` (truncates); cast to `f64` for `2.5` | 0004 |
| `i64` intermediate for overflow | Accumulate in i64, clamp to i32 range at end | 0008, 0374, 0446, 2256 |
| `i64::abs()` | Absolute value on difference of averages | 2256 |
| `let mut x = x` parameter rebinding | Re-binds immutable param as mutable; avoids `mut` in signature | 0009, 0263, 1342, 1480 |
| Half-reversal vs full comparison | Only reverse half the number to avoid overflow | 0009 |

## Bitwise Operations

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `>> 1` for midpoint | Bit shift equivalent to `/ 2` for unsigned | 0278, 1342 |
| `1 << height` | Bit shift for `2^height` (Ruby: `2**height`) | 0222 |
| `& 1` and `>>= 1` | Test odd/even and divide-by-2 in a single pass | 1342 |
| Bit masking `idx & bit` | Navigate binary tree levels by treating node index as binary path | 0222 |
| `\|=` (bitwise OR assign) for bool | Rust has no `\|\|=`; `dp[i][j] \|= val` is equivalent to `dp[i][j] = dp[i][j] \|\| val` | 0010 |
| `u64` bitmask for visited grid | Encodes 6x6 visited grid in a single integer | 0079 |
| `!y` (bitwise NOT) for namespace separation | Maps column index to distinct negative value for union-find | 0947 |
| `.count_ones()` popcount | Counts set bits in an integer; hardware-accelerated on most CPUs | 0191, 0216, 0231, 0401, 0461, 0465 |
| `.leading_zeros()` / `.trailing_zeros()` | Count leading/trailing zero bits; useful for power-of-2 and bit manipulation | 0465, 0476 |
| XOR 32 case-flip | `top ^ b == 32` exploits ASCII upper/lower differing only in bit 5 | 1544 |

## Linked Lists

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `Option<Box<ListNode>>` | Idiomatic linked list node pointer; `None` = end of list | 0002, 0019, 0021, 0023, 0024, 0025, 0061, 0082, 0083, 0086, 0092, 0109, 0142, 0143, 0147, 0148, 0203, 0206, 0234, 0237, 0382, 0445, 0876 |
| Ownership transfer in loops | `while let Some(mut node) = current` moves each node | 0206 |
| Recursive ownership | `Some(mut l1)` moves Box out of Option; return `Some(l1)` flows ownership back up | 0021 |
| Dummy head / mutable borrow chain | `current = &mut dummy` walks list via mutable refs without moving head ownership | 0021 |
| `.take()` to move out of Option | Leave `None` in place; essential for linked list manipulation | 0002 |
| `.and_then(\|node\| node.next)` | Option chaining to advance by consuming current node | 0876 |
| Rebuild from Vec slice | `vals[mid..].iter().rev()` constructs list from slice by iterating in reverse | 0876 |
| Simulating cycles with indices | Rust ownership prevents Box cycles; use index-based simulation instead | 0142 |
| Floyd's tortoise and hare | Two-phase cycle detection using closure to simulate traversal | 0142 |

## Trees — Rc RefCell TreeNode

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `Option<Rc<RefCell<TreeNode>>>` | Layered smart pointers: Option=nil, Rc=shared ownership, RefCell=runtime borrow checking | 0094, 0095, 0098, 0099, 0100, 0101, 0102, 0103, 0104, 0105, 0106, 0107, 0108, 0109, 0110, 0111, 0112, 0113, 0114, 0116, 0117, 0124, 0129, 0133, 0144, 0145, 0156, 0173, 0199, 0222, 0226, 0230, 0235, 0236, 0250, 0257, 0270, 0272, 0285, 0297, 0298, 0314, 0404, 0426, 0431, 0437, 0449, 0450, 0501, 0508, 0513 |
| `.borrow()` / `.borrow_mut()` | RefCell runtime borrow check; panics on simultaneous read+write | 0222 |
| `.clone()` on `Option<Rc<...>>` | Cheap ref-count bump; needed to move fields out of borrowed RefCell | 0222 |
| `Rc::clone(&root)` | Idiomatic explicit clone (vs `root.clone()` which is ambiguous) | 0222 |
| Recursive DFS with `&Option<Box<Node>>` | Borrows tree without consuming; allows tree reuse | 0589 |
| `Vec<Option<Box<Node>>>` for n-ary children | Each child is `Option<Box<Node>>` for nullable entries | 0589 |
| Iterative stack `into_iter().rev()` | Consumes children in reverse for left-to-right preorder via LIFO | 0589 |

## Sorting and Searching

| Concept | Description | LeetCode |
|---------|-------------|----------|
| Binary search | O(log n) search on sorted data; multiple boundary variations | 0004, 0035, 0278, 0374, 0704 |
| Exclusive upper bound: `right = len` | `left < right` and `right = mid` avoids usize underflow when `right = 0` | 0035, 0704 |
| Inclusive bounds with `as usize` indexing | Simpler loop but requires casting mid to usize | 0704 |
| Overflow-safe midpoint | `left + (right - left) / 2` instead of `(left + right) / 2` | 0035, 0278, 0374 |
| `sort_unstable()` | No stability guarantee but faster for primitives; no extra allocation | 0015, 0016, 0018, 0034, 0039, 0040, 0047, 0049, 0128, 0147, 0164, 0197, 0216, 0217, 0218, 0253, 0254, 0269, 0274, 0280, 0296, 0315, 0324, 0327, 0336, 0356, 0360, 0368, 0373, 0414, 0436, 0455, 0462, 0475, 0480, 0488, 0501, 0508 |
| `sort_unstable()` + `reverse()` | Can't chain because `sort` returns `()` | 0274 |
| `sort_unstable_by` with `cmp().then()` | Multi-key sort without allocating stable temp buffer | 0179, 0185, 0215, 0332, 0333, 0354, 0355, 0356, 0358, 0406, 0451, 0473, 0506, 0587 |
| `.sort_by()` | Stable sort with custom comparator; allocates temp buffer | 0178, 0179, 0184, 0185, 0192, 0249, 0252 |
| `select_nth_unstable` | O(n) avg quickselect; partial sort to find median | 0169 |
| Counting sort | Bucket into `vec![0; n+1]`, scan in reverse; O(n) space for O(n) time | 0274 |
| `slice::partition_point()` | Stdlib lower-bound binary search; returns insertion point | 0035 |

## Dynamic Programming Patterns

| Concept | Description | LeetCode |
|---------|-------------|----------|
| Top-down DP with HashMap memo | `HashMap<(state), value>` for recursive memoization | 0010, 0514 |
| 1D DP space optimization | When row i only depends on row i-1; compress 2D table to 1D | 0010, 0514 |
| Rolling 1D DP with `i16` | Smaller memory footprint; last-position tracking prunes inner loop | 0514 |
| `Vec<HashMap<i64, i32>>` per-index DP | Each element has its own map of diff -> count | 0446 |
| Prefix sum | Running sum for range queries; can be combined with modular arithmetic | 0523, 0724, 2256 |
| Sentinel seeding | `mod_map.insert(0, -1)` handles subarrays starting at index 0 | 0523 |
| Delayed insertion | Inserts previous iteration's value; guarantees length constraint | 0523 |

## Struct and Encapsulation

| Concept | Description | LeetCode |
|---------|-------------|----------|
| Struct with private fields + pub methods | Encapsulation pattern for data structure design | 0380 |
| `#[inline]` | Compiler hint to inline small functions at call sites | 0222 |
| `const` slice of tuples | `const ROMAN: &[(i32, &str)]` lives in read-only memory; zero heap allocation | 0012 |
| `const` arrays of `&str` | `const ONES: [&str; 10]` for direct index lookup | 0012 |
| `const { ... }` block | Compile-time evaluation; e.g., building lookup tables at zero runtime cost | 0278, 0345 |
| `String += &str` | `AddAssign` trait appends without reallocating each time | 0012 |
| `format!()` macro | Concatenates values into one owned String | 0012 |

## Concurrency and Thread-Local

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `thread_local!` + `Cell<i32>` | Per-thread mutable state without locks; simulates external API | 0278, 0374 |
| `const { Cell::new(1) }` in `thread_local!` | Const initializer avoids runtime initialization check | 0278 |

## Unsafe and Performance

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `unsafe { String::from_utf8_unchecked() }` | Skips UTF-8 validation for known-ASCII data | 0345 |
| Register-cached sliding window | Cache array values into local vars to avoid repeated reads | 0011, 0080 |
| `const VOWEL_LUT: [bool; 128]` | Compile-time lookup table; zero runtime cost | 0345 |

## Clippy and Compiler Directives

| Concept | Description | LeetCode |
|---------|-------------|----------|
| `#[allow(clippy::too_many_arguments)]` | Suppresses warning when LC signature requires many params | 0223 |
| `#[allow(clippy::only_used_in_recursion)]` | Clippy false-positive when params are read via indexing in recursive fns | 0514 |

## Algorithmic Patterns

| Concept | Description | LeetCode |
|---------|-------------|----------|
| Two-pointer convergence | Left/right pointers moving inward | 0003, 0011, 0042, 0345 |
| Sliding window | Two-pointer window expanding/contracting | 0003, 0076, 0080 |
| Greedy algorithm | Single-pass with running totals / window tracking | 0045, 0122, 0134, 0135, 0169 |
| Boyer-Moore Voting | Single-pass O(1) space majority-finding | 0169 |
| Backtracking DFS | Push/pop + set/unset for O(n) space reuse | 0017, 0022, 0037, 0039, 0040, 0046, 0047, 0051, 0077, 0078, 0079, 0090, 0093, 0131, 0216, 0254, 0267, 0301, 0491, 3437 |
| Union-Find with path compression | HashMap-based parent map with `find`/`union` as nested fns | 0128, 0130, 0200, 0261, 0305, 0323, 0399, 0421, 0947 |
| Floyd's cycle detection | Tortoise and hare two-phase algorithm | 0142 |
| Monotonic stack | Maintain stack invariant for next-greater-element patterns | 0042, 0084, 0085, 0255, 0316, 0321, 0402, 0456, 0496, 0503 |
| BFS with level tracking | Use `VecDeque` with `len()` loop for level-order traversal | 0094, 0102, 0103, 0107, 0116, 0127, 0199, 0207, 0210, 0226, 0269, 0286, 0297, 0301, 0310, 0417, 0433, 0513 |
| Topological sort (Kahn's algorithm) | BFS with in-degree tracking for DAG ordering | 0207, 0210, 0269, 0310, 0444 |
| Interval merging / sweepline | Sort intervals, then merge overlapping or track active set | 0056, 0057, 0218, 0252, 0253, 0435, 0436 |
| Prefix sum modular arithmetic | Running sum % k with remainder -> first-index map | 0523 |

## Ruby vs Rust Gotchas

| Concept | Description | LeetCode |
|---------|-------------|----------|
| No ternary `? :` | Use `if x { a } else { b }` (it's an expression, returns a value) | 0008 |
| Ruby `[]` vs Rust `[]` | Ruby's is growable Array; Rust's is fixed-size array. Use `Vec` for growable | 0279 |
| No `||=` operator | Use `\|=` (bitwise OR assign) for booleans | 0010 |
| `.swap()` instead of parallel assignment | Ruby `a, b = b, a` becomes `.swap(i, j)` in Rust | 0189, 0345 |
| `.len()` not `.size` | Rust convention for collection size | 0295 |
| `for sq in vec` moves Vec | Use `for &sq in &vec` to borrow; Ruby `.each` borrows implicitly | 0279 |
| `sort()` returns `()` | Can't chain `sort().reverse()` — these are separate statements | 0274 |
| `return` is statement, not expression | `expr.unwrap_or(return 0)` won't compile; use `match` | 0007 |
| usize can't go negative | `0_usize - 1` panics; use `> 0` guard or `saturating_sub` | 0005, 0058, 0135, 0189 |
| `.abs()` panics on `i32::MIN` | `-2_147_483_648` overflows; guard with `if x == i32::MIN` | 0007 |
