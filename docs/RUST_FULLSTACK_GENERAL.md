# Rust Full-Stack & General — Interview Q&A

## Q1: Why choose Rust for backend development?

**Answer:**

| Advantage | Detail |
|---|---|
| **Memory safety without GC** | No null pointers, no data races — guaranteed at compile time |
| **Performance** | Comparable to C/C++ — no garbage collection pauses |
| **Type system** | `Result<T, E>` forces error handling; `Option<T>` eliminates null |
| **Concurrency safety** | `Send`/`Sync` traits prevent data races at compile time |
| **Low resource usage** | Small binaries, low memory — good for containers |
| **Reliability** | "If it compiles, it works" — fewer runtime surprises |

**When Rust is NOT the best choice:**
- Rapid prototyping (slower development velocity than Python/Node)
- Teams unfamiliar with Rust (steep learning curve)
- Simple CRUD APIs where Go/Node would suffice

**Interview phrasing:** "I'd choose Rust when reliability and performance matter — systems where a runtime crash or data race would be costly. For a quick internal tool, I'd reach for something faster to develop."

---

## Q2: Explain Rust's ownership model in simple terms.

**Answer:**

```rust
// Three rules:
// 1. Each value has exactly ONE owner
// 2. When the owner goes out of scope, the value is dropped (freed)
// 3. You can have EITHER one mutable reference OR any number of immutable references

let s1 = String::from("hello");
let s2 = s1;       // s1 is MOVED to s2 — s1 is no longer valid
// println!("{s1}"); // COMPILE ERROR: s1 was moved

// Borrowing — access without taking ownership
fn print_len(s: &str) {  // borrows s, doesn't own it
    println!("{}", s.len());
}
let s = String::from("hello");
print_len(&s);     // borrow
println!("{s}");    // still valid — ownership was never transferred

// One mutable reference at a time — prevents data races
let mut s = String::from("hello");
let r1 = &mut s;
// let r2 = &mut s;  // COMPILE ERROR: can't have two mutable refs
r1.push_str(" world");
```

**Why this matters for backends:**
- Database connections are automatically returned to the pool when dropped (no connection leaks)
- Request data can't accidentally be shared between concurrent handlers
- No use-after-free, no double-free — entire classes of security vulnerabilities eliminated at compile time

---

## Q3: What are `Send` and `Sync` traits?

**Answer:**

```rust
// Send — safe to TRANSFER ownership to another thread
// "I can give this to another thread"
//   ✅ String, Vec, i32, Arc<T> — most types
//   ❌ Rc<T> — reference count is not atomic, unsafe to share

// Sync — safe to SHARE references (&T) across threads
// "Multiple threads can read this simultaneously"
//   ✅ i32, String, Mutex<T>, Arc<T>
//   ❌ RefCell<T> — no locking, single-thread interior mutability only

// The compiler checks these AUTOMATICALLY:
tokio::spawn(async move {
    // Everything captured here must be Send
    // If you try to send Rc<T> across threads → compile error
});
```

**Practical impact:** You never need to remember which types are thread-safe — the compiler tells you. In Go or Java, thread safety bugs surface at runtime (often in production).

---

## Q4: How does async/await work in Rust?

**Answer:**

```rust
// async fn returns a Future — lazy, doesn't execute until awaited
async fn fetch_vessel(id: i32) -> Result<Vessel, AppError> {
    // .await yields control to the runtime while waiting for I/O
    let vessel = sqlx::query_as!(Vessel, "SELECT * FROM vessels WHERE id = $1", id)
        .fetch_one(&pool)
        .await?;  // pauses here, other tasks can run
    Ok(vessel)
}

// tokio is the async runtime — schedules and runs Futures
#[tokio::main]
async fn main() {
    // Spawn concurrent tasks
    let (a, b) = tokio::join!(
        fetch_vessel(1),
        fetch_vessel(2),
    );  // both run concurrently, not sequentially
}
```

**Critical rule: never block the async runtime**

```rust
// BAD — blocks the entire tokio thread, starving other tasks
async fn bad_handler() {
    std::thread::sleep(Duration::from_secs(5));  // blocks!
    heavy_computation();                          // blocks!
}

// GOOD — yield to runtime during sleep
async fn good_handler() {
    tokio::time::sleep(Duration::from_secs(5)).await;  // yields
}

// GOOD — offload CPU-bound work to a blocking thread pool
async fn compute_handler() {
    let result = tokio::task::spawn_blocking(|| {
        heavy_computation()  // runs on a separate thread
    }).await?;
}
```

**Interview answer:** "Rust's async is cooperative — tasks must yield control via `.await`. CPU-heavy work should use `spawn_blocking` to avoid starving the reactor. This is different from Go's goroutines which are preemptively scheduled."

---

## Q5: How does Rust's type system prevent common bugs?

**Answer:**

```rust
// 1. No null — Option<T> forces you to handle the missing case
let user: Option<User> = find_user(id);
// user.name  ← COMPILE ERROR: Option has no field `name`
// Must unwrap safely:
if let Some(u) = user {
    println!("{}", u.name);
}

// 2. No unchecked errors — Result<T, E> must be handled
let file = std::fs::read_to_string("data.json");
// file.len()  ← COMPILE ERROR: Result has no method `len`
// Must handle the error:
let content = file?;  // propagate error
// or
let content = file.unwrap_or_default();  // provide fallback

// 3. Exhaustive matching — compiler ensures all cases handled
enum Status { Active, Inactive, Maintenance }
match vessel.status {
    Status::Active => ...,
    Status::Inactive => ...,
    // COMPILE ERROR: non-exhaustive, forgot Maintenance
}

// 4. Newtypes prevent mixing values
struct VesselId(i32);
struct AlertId(i32);
fn get_vessel(id: VesselId) -> ... { ... }
// get_vessel(AlertId(5))  ← COMPILE ERROR: expected VesselId, got AlertId

// 5. Immutable by default
let x = 5;
// x += 1;  ← COMPILE ERROR: x is not mutable
let mut x = 5;
x += 1;    // OK
```

---

## Q6: What is `serde` and how does serialization work in Rust?

**Answer:**

```rust
use serde::{Deserialize, Serialize};

// Derive macros auto-generate serialization code at compile time
// — no reflection, no runtime overhead
#[derive(Serialize, Deserialize, Debug)]
struct Vessel {
    id: i32,
    name: String,
    #[serde(rename = "imoNumber")]    // JSON field name differs from Rust
    imo_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]  // omit if None
    description: Option<String>,
}

// Deserialize JSON → Rust struct
let json = r#"{"id": 1, "name": "Ship", "imoNumber": "IMO1234567"}"#;
let vessel: Vessel = serde_json::from_str(json)?;

// Serialize Rust struct → JSON
let json_string = serde_json::to_string(&vessel)?;

// Common serde attributes:
#[serde(rename_all = "camelCase")]    // all fields: snake_case → camelCase
#[serde(default)]                     // use Default::default() for missing fields
#[serde(deny_unknown_fields)]         // reject extra fields in input
#[serde(tag = "type")]                // tagged enum: {"type": "circle", "radius": 5}
```

**Why serde is powerful:**
- Zero-cost abstractions — code generated at compile time, no runtime reflection
- Works with JSON, TOML, YAML, MessagePack, etc. — same derive, different serializer
- Type-safe — invalid JSON structure → compile-time error (with `serde_json::from_str`)

---

## Q7: How do you structure a Rust project with multiple crates?

**Answer:**

```
my-project/
├── Cargo.toml              # workspace root
├── crates/
│   ├── api/                # HTTP layer (axum handlers)
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── domain/             # Business logic (no framework dependency)
│   │   ├── Cargo.toml
│   │   └── src/
│   └── db/                 # Database layer (sqlx queries)
│       ├── Cargo.toml
│       └── src/
└── migrations/
```

```toml
# Root Cargo.toml
[workspace]
members = ["crates/*"]

# crates/api/Cargo.toml
[dependencies]
domain = { path = "../domain" }
db = { path = "../db" }
axum = "0.7"
tokio = { version = "1", features = ["full"] }
```

**Benefits:**
- **Compile-time boundaries** — `domain` crate has no HTTP dependency, can't accidentally import axum types
- **Parallel compilation** — independent crates build in parallel
- **Reusable** — `domain` crate can be used in CLI tools, WASM, tests without HTTP
- **Clear dependency direction** — `api` depends on `domain`, never the reverse

---

## Q8: What are lifetimes and when do you need to annotate them?

**Answer:**

```rust
// Lifetimes tell the compiler how long a reference is valid.
// Usually the compiler infers them — you only annotate when it can't.

// Case 1: Compiler can't tell which input the output borrows from
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
// 'a means: the returned reference lives as long as BOTH inputs

// Case 2: Struct holding a reference
struct Excerpt<'a> {
    text: &'a str,  // this struct can't outlive the string it borrows
}

// Case 3: You DON'T need lifetimes (most of the time)
fn first_word(s: &str) -> &str {  // compiler infers: same lifetime as input
    s.split_whitespace().next().unwrap_or("")
}
```

**Interview answer:** "Lifetimes are the compiler's way of ensuring references don't outlive their data. I rarely need to write them explicitly — the compiler infers them in most cases. I only annotate when a function returns a reference that could come from multiple inputs."

---

## Q9: How do you handle configuration and secrets in a Rust service?

**Answer:**

```rust
// Use environment variables via dotenvy (loads .env in development)
// NEVER hardcode secrets in source code

use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
struct Config {
    database_url: String,
    port: u16,
    jwt_secret: String,  // sensitive — from env, never logged
}

impl Config {
    fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();  // load .env file if it exists (dev only)

        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .context("DATABASE_URL must be set")?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .context("PORT must be a valid number")?,
            jwt_secret: env::var("JWT_SECRET")
                .context("JWT_SECRET must be set")?,
        })
    }
}
```

```
# .env (git-ignored)
DATABASE_URL=postgres://user:pass@localhost/mydb
PORT=3000
JWT_SECRET=supersecret
```

```gitignore
# .gitignore — MUST include
.env
```

**Security rules:**
- `.env` in `.gitignore` — never commit secrets
- Different `.env` per environment (dev, staging, prod)
- In production, use a secrets manager (AWS Secrets Manager, Vault) — not .env files
- Never log secrets — use `secrecy` crate to wrap sensitive strings

---

## Q10: What Rust tools and practices do you use for code quality?

**Answer:**

```bash
# Formatting — consistent style across the team
cargo fmt                # auto-format
cargo fmt --check        # CI: fail if not formatted

# Linting — catches common mistakes and non-idiomatic code
cargo clippy             # lint with suggestions
cargo clippy -- -D warnings   # CI: treat warnings as errors

# Testing
cargo test               # run all tests
cargo test -- --nocapture  # show println! output

# Building
cargo build              # debug build (fast compile, slow runtime)
cargo build --release    # release build (slow compile, fast runtime)

# Documentation
cargo doc --open         # generate and view docs
```

**CI pipeline should run (in order):**
1. `cargo fmt --check` — style
2. `cargo clippy -- -D warnings` — lint
3. `cargo test` — correctness
4. `cargo build --release` — compilation

**Additional practices:**
- `#[cfg(test)]` modules for test code — not compiled in release
- `#[deny(unused)]` in CI — no dead code
- Benchmark critical paths with `criterion` or manual timing (as in this repo's LeetCode files)

---

## Q11: How does Rust compare to Go for backend services?

**Answer:**

| Aspect | Rust | Go |
|---|---|---|
| **Safety** | Memory safety at compile time | GC handles memory; race detector at runtime |
| **Performance** | Faster (no GC pauses, zero-cost abstractions) | Fast (but GC pauses under load) |
| **Concurrency** | `Send`/`Sync` — compile-time data race prevention | Goroutines — simple but races are runtime bugs |
| **Error handling** | `Result<T, E>` — must handle | `error` interface — can be ignored with `_` |
| **Learning curve** | Steep (ownership, lifetimes, borrow checker) | Gentle (simple syntax, small language) |
| **Compile time** | Slow (especially initial build) | Fast |
| **Ecosystem** | Growing (axum, sqlx, tokio) | Mature (gin, echo, gorm) |
| **Binary size** | Small, static | Small, static |

**When to choose Rust over Go:**
- Performance-critical services (data processing, real-time systems)
- When memory safety guarantees are worth the development overhead
- Systems programming (close to hardware, embedded, WASM)

**When Go is better:**
- Team velocity matters more than peak performance
- Simple microservices with straightforward CRUD
- Team is new to systems programming

---

## Q12: How do you write tests in Rust?

**Answer:**

```rust
// Unit tests — in the same file, inside #[cfg(test)] module
#[cfg(test)]  // only compiled when running tests
mod tests {
    use super::*;  // import everything from parent module

    #[test]
    fn test_vessel_status_default() {
        let vessel = Vessel::new("Ship", "IMO1234567");
        assert_eq!(vessel.status, Status::Active);
    }

    #[test]
    #[should_panic(expected = "empty name")]
    fn test_empty_name_panics() {
        Vessel::new("", "IMO1234567");
    }

    // Async tests with tokio
    #[tokio::test]
    async fn test_fetch_vessel() {
        let pool = setup_test_db().await;
        let vessel = get_vessel(&pool, 1).await.unwrap();
        assert_eq!(vessel.name, "Test Ship");
    }
}

// Integration tests — in tests/ directory, separate compilation unit
// tests/vessel_api.rs
#[tokio::test]
async fn test_create_vessel_endpoint() {
    let app = setup_test_app().await;
    let response = app.post("/vessels").json(&json!({"name": "Ship"})).await;
    assert_eq!(response.status(), 201);
}
```

**Testing patterns:**
- **Arrange-Act-Assert** — setup, do the thing, verify the result
- **Test isolation** — each test sets up own data, no dependency on test order
- **Mock external services** — use `mockall` crate or trait objects for dependency injection

```rust
// Dependency injection via traits — makes testing easy
trait VesselRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<Vessel>>;
}

// Real implementation
struct PgVesselRepo { pool: PgPool }
impl VesselRepository for PgVesselRepo { ... }

// Mock for tests
struct MockVesselRepo { vessels: Vec<Vessel> }
impl VesselRepository for MockVesselRepo {
    async fn find_by_id(&self, id: i32) -> Result<Option<Vessel>> {
        Ok(self.vessels.iter().find(|v| v.id == id).cloned())
    }
}
```

---

## Q13: What is `Cow<'a, str>` and when would you use it?

**Answer:**

```rust
use std::borrow::Cow;

// Cow = Clone on Write
// Holds either a borrowed reference OR an owned value.
// Only clones (allocates) when you need to mutate.

fn normalize_name(name: &str) -> Cow<'_, str> {
    if name.contains(' ') {
        // Need to modify — allocate a new String
        Cow::Owned(name.trim().to_lowercase())
    } else {
        // No modification needed — just borrow the original, zero-cost
        Cow::Borrowed(name)
    }
}

// Usage: works like &str in both cases
let result = normalize_name("hello");     // Cow::Borrowed — no allocation
let result = normalize_name("  Hello ");  // Cow::Owned — allocates only when needed
println!("{result}");  // works either way
```

**When to use:** When a function SOMETIMES needs to allocate a new string and sometimes can just return the input. Avoids unnecessary cloning in the common path.
