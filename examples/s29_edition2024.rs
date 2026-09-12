// Section 29: Rust 2024 Edition — key language changes
//
// The 2024 edition (stabilised in Rust 1.85, Feb 2025) brings:
//   1. `if let` / `while let` chains (`let_chains`, stable in 1.88)
//   2. Unsafe `extern "C"` blocks — `unsafe` is now required
//   3. `use<'a, T>` — precise lifetime capture in RPIT (impl Trait)
//   4. Ergonomic closure capture: `move` + `use` semantics tightened
//   5. `gen {}` blocks (reserved; not yet stable as of 1.95)
//
// This crate already targets edition = "2024" (see Cargo.toml).
//
// Run: cargo run --example s29_edition2024

// ---------------------------------------------------------------------------
// 1. let-chains — if let + && in a single condition (stable: Rust 1.88)
//
//    Before 1.88 you had to nest `if let` blocks:
//      if let Some(user) = get_user() {
//          if user.age >= 18 { ... }
//      }
//
//    With let-chains, the compiler desugars one flat expression:
//      if let Some(user) = get_user() && user.age >= 18 { ... }
//
//    Cross-language: Swift has `guard let x = opt, x > 0 else { ... }`.
//    Kotlin 2.1 added `if guard` conditions inside `when`. Rust's version
//    is a flat boolean-and of pattern bindings and predicates.
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    email: Option<String>,
}

impl User {
    fn new(name: &str, age: u8, email: Option<&str>) -> Self {
        User {
            name: name.to_owned(),
            age,
            email: email.map(str::to_owned),
        }
    }
}

fn describe_user(user: Option<&User>) -> String {
    // let-chain: binds `u` from the Option AND checks age in one `if`.
    // Both sides of `&&` must hold; if either fails we fall to `else`.
    if let Some(u) = user
        && u.age >= 18
    {
        format!("{} is an adult ({})", u.name, u.age)
    } else {
        "no adult user".to_owned()
    }
}

fn verified_email(user: Option<&User>) -> Option<String> {
    // Three-part chain: Option present, age >= 18, email is Some.
    if let Some(u) = user
        && u.age >= 18
        && let Some(email) = &u.email
    {
        Some(email.clone())
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// 2. Unsafe extern blocks — `unsafe extern` required in edition 2024
//
//    Before 2024, `extern "C" { fn strlen(...) -> usize; }` compiled with
//    just an implicit warning.  The 2024 edition makes the `unsafe` keyword
//    mandatory, making it obvious that each declaration is unverified by the
//    compiler — you are asserting the ABI and signature are correct.
//
//    Cross-language: C#'s `extern` with DllImport, Kotlin's `external` fun,
//    and Go's `//go:linkname` serve a similar "I know what I'm doing" role.
// ---------------------------------------------------------------------------

// SAFETY: `strlen` is a standard C function with a well-known ABI.
// The caller must ensure `ptr` points to a valid null-terminated C string.
unsafe extern "C" {
    // FIX: `u8` did not match the C `char` ABI expected by the runtime symbol,
    // which Rust 1.98 rejects. `c_char` preserves the platform C ABI type.
    fn strlen(ptr: *const std::ffi::c_char) -> usize;
}

fn c_string_len(s: &str) -> usize {
    // SAFETY: `s.as_ptr()` is a valid UTF-8 byte slice followed by at least
    // one additional byte (Rust string slices guarantee the data lives long
    // enough), but we must manually null-terminate for C ABI.  We use a
    // CString to guarantee the null terminator.
    let cstring = std::ffi::CString::new(s).expect("no interior nuls");
    // SAFETY: `cstring.as_ptr()` is a valid null-terminated C string.
    unsafe { strlen(cstring.as_ptr()) }
}

// ---------------------------------------------------------------------------
// 3. `use<'a, T>` — precise lifetime capture in RPIT  (stable: Rust 1.82)
//
//    Before 1.82 (and formalised in edition 2024), `impl Trait` in return
//    position implicitly captured *all* in-scope lifetimes and type params.
//    That was surprising and sometimes forced callers to add unneeded bounds.
//
//    `use<'a>` lets you explicitly declare which lifetimes the opaque type
//    captures, making the intent visible at the call site.
//
//    Cross-language: OCaml's explicit polymorphic annotations `'a.` serve a
//    similar "I am declaring my variance" purpose.
// ---------------------------------------------------------------------------

struct Cache<'a> {
    data: &'a [u8],
}

/// Without `use<'a>`, the compiler might complain that the returned `impl`
/// type captures `'a` even if it doesn't need to for the caller's purpose.
/// With `use<'a>`, we explicitly state "this return value is tied to `'a`".
fn make_cache(data: &[u8]) -> impl ExactSizeIterator<Item = &u8> + use<'_> {
    data.iter()
}

fn first_nonzero(data: &[u8]) -> Option<u8> {
    // `make_cache` returns an iterator whose lifetime is explicit.
    make_cache(data).copied().find(|&b| b != 0)
}

// A struct method that returns an opaque type capturing the struct's lifetime.
impl<'a> Cache<'a> {
    fn iter_bytes(&self) -> impl Iterator<Item = u8> + use<'a> {
        // `use<'a>` captures the `'a` lifetime from the struct, not `'self`.
        // This means the caller can use the iterator after `self` is dropped.
        self.data.iter().copied()
    }
}

// ---------------------------------------------------------------------------
// 4. Combining edition features in a realistic example
// ---------------------------------------------------------------------------

fn summarize_data(raw: Option<&[u8]>) -> String {
    // let-chain: Option is Some AND slice is non-empty.
    if let Some(bytes) = raw
        && !bytes.is_empty()
    {
        let nonzero_count = bytes.iter().filter(|&&b| b != 0).count();
        format!("{} bytes, {} non-zero", bytes.len(), nonzero_count)
    } else {
        "empty or absent".to_owned()
    }
}

// ---------------------------------------------------------------------------
// main — exercise all examples
// ---------------------------------------------------------------------------

fn main() {
    println!("=== Rust 2024 Edition Features ===\n");

    // 1. let-chains
    println!("--- 1. let-chains ---");
    let alice = User::new("Alice", 30, Some("alice@example.com"));
    let bob = User::new("Bob", 16, None);
    println!("{}", describe_user(Some(&alice))); // Alice is an adult (30)
    println!("{}", describe_user(Some(&bob))); // no adult user
    println!("{}", describe_user(None)); // no adult user
    println!("Alice email: {:?}", verified_email(Some(&alice))); // Some(...)
    println!("Bob email:   {:?}", verified_email(Some(&bob))); // None

    // 2. unsafe extern
    println!("\n--- 2. unsafe extern ---");
    println!("strlen(\"hello\") = {}", c_string_len("hello")); // 5
    println!("strlen(\"\")      = {}", c_string_len("")); // 0

    // 3. use<'a> RPIT capture
    println!("\n--- 3. use<'_> RPIT capture ---");
    let data = vec![0u8, 1, 2, 3];
    println!("first nonzero: {:?}", first_nonzero(&data)); // Some(1)
    let cache = Cache { data: &data };
    let sum: u8 = cache.iter_bytes().sum();
    println!("sum of bytes: {}", sum); // 6

    // 4. combined
    println!("\n--- 4. combined ---");
    println!("{}", summarize_data(Some(&[0, 1, 2, 3]))); // 4 bytes, 3 non-zero
    println!("{}", summarize_data(Some(&[]))); // empty or absent
    println!("{}", summarize_data(None)); // empty or absent
}

// ---------------------------------------------------------------------------
// inline tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn let_chain_adult() {
        let u = User::new("Alice", 30, Some("a@b.com"));
        assert_eq!(describe_user(Some(&u)), "Alice is an adult (30)");
    }

    #[test]
    fn let_chain_minor() {
        let u = User::new("Bob", 16, None);
        assert_eq!(describe_user(Some(&u)), "no adult user");
    }

    #[test]
    fn let_chain_none() {
        assert_eq!(describe_user(None), "no adult user");
    }

    #[test]
    fn let_chain_email() {
        let u = User::new("Carol", 25, Some("c@d.com"));
        assert_eq!(verified_email(Some(&u)), Some("c@d.com".to_owned()));
    }

    #[test]
    fn let_chain_no_email() {
        let u = User::new("Dave", 22, None);
        assert_eq!(verified_email(Some(&u)), None);
    }

    #[test]
    fn unsafe_extern_strlen() {
        assert_eq!(c_string_len("hello"), 5);
        assert_eq!(c_string_len(""), 0);
        assert_eq!(c_string_len("rust"), 4);
    }

    #[test]
    fn rpit_first_nonzero() {
        assert_eq!(first_nonzero(&[0, 0, 3, 1]), Some(3));
        assert_eq!(first_nonzero(&[0, 0, 0]), None);
        assert_eq!(first_nonzero(&[]), None);
    }

    #[test]
    fn summarize_combined() {
        assert_eq!(summarize_data(Some(&[0, 1, 2, 3])), "4 bytes, 3 non-zero");
        assert_eq!(summarize_data(Some(&[])), "empty or absent");
        assert_eq!(summarize_data(None), "empty or absent");
    }
}
