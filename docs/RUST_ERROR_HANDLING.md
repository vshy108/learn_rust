# Rust Error Handling — Interview Q&A

## Q1: How does Rust's error handling differ from exceptions in other languages?

**Answer:**

| Aspect | Rust | TypeScript/Python/Ruby |
|---|---|---|
| Mechanism | `Result<T, E>` return type + `?` operator | `try/catch` + thrown exceptions |
| Forgettable? | **No** — must handle `Result` or code won't compile | **Yes** — can forget `try/catch` |
| Null safety | `Option<T>` — compiler forces check | `null`/`None` — runtime crash |
| Control flow | Explicit — errors are just values | Implicit — exceptions jump up the stack |
| Performance | Zero-cost — no stack unwinding | Exception throwing is expensive |

```rust
// Rust: compiler forces you to handle the error
let content = std::fs::read_to_string("config.toml")?;
// The ? operator:
// - If Ok(value), unwraps and continues
// - If Err(e), returns early with the error (converted via From trait)

// You CANNOT ignore a Result without a compiler warning:
std::fs::read_to_string("config.toml");  // warning: unused Result
```

```typescript
// TypeScript: nothing stops you from forgetting try/catch
const data = JSON.parse(userInput);  // throws at runtime if invalid
// No compiler warning — the error surfaces in production
```

**Key insight for interview:** "In Rust, errors are values, not control flow. The type system makes error handling mandatory at compile time, not a runtime afterthought."

---

## Q2: What are `Result<T, E>` and `Option<T>`?

**Answer:**

```rust
// Option<T> — represents a value that might not exist
// Equivalent to T | null in TypeScript, but compiler-enforced
enum Option<T> {
    Some(T),    // value exists
    None,       // no value
}

let name: Option<&str> = map.get("name");  // might be None
// Must handle both cases:
match name {
    Some(n) => println!("Found: {n}"),
    None => println!("Not found"),
}
// Or use combinators:
let upper = name.map(|n| n.to_uppercase()).unwrap_or_default();

// Option<T> — represents a computation that might fail
// Equivalent to throwing an exception, but as a return type
enum Result<T, E> {
    Ok(T),      // success
    Err(E),     // failure with error info
}

let parsed: Result<i32, ParseIntError> = "42".parse();
match parsed {
    Ok(n) => println!("Got: {n}"),
    Err(e) => println!("Failed: {e}"),
}
```

**Common methods:**

```rust
// Option<T>
opt.unwrap()                // panics if None — NEVER in production
opt.unwrap_or(default)      // returns default if None
opt.unwrap_or_default()     // uses Default trait
opt.ok_or(MyError)?         // converts Option → Result, then propagates
opt.map(|v| v + 1)          // transforms the inner value if Some
opt.and_then(|v| another_option(v))  // chains Option-returning functions

// Result<T, E>
res.unwrap()                // panics if Err — NEVER in production
res?                        // propagates Err, unwraps Ok
res.map(|v| v + 1)          // transforms Ok value
res.map_err(|e| convert(e)) // transforms Err value
res.ok()                    // Result → Option (discards error)
res.context("msg")?         // anyhow: adds context to error chain
```

---

## Q3: Explain the `?` operator. When can you use it?

**Answer:**

```rust
// The ? operator is syntactic sugar for early error returns.
// It can ONLY be used in functions that return Result or Option.

// Without ?:
fn read_config() -> Result<Config, io::Error> {
    let content = match std::fs::read_to_string("config.toml") {
        Ok(c) => c,
        Err(e) => return Err(e),    // early return on error
    };
    let config = match toml::from_str(&content) {
        Ok(c) => c,
        Err(e) => return Err(e.into()),
    };
    Ok(config)
}

// With ? — same logic, much cleaner:
fn read_config() -> Result<Config, anyhow::Error> {
    let content = std::fs::read_to_string("config.toml")?;
    let config = toml::from_str(&content)?;
    Ok(config)
}
```

**How `?` works internally:**
1. If the value is `Ok(v)` — unwraps and continues
2. If the value is `Err(e)` — calls `From::from(e)` to convert the error type, then returns early

**Consequence:** The function's error type must implement `From<OriginalError>`. This is why `thiserror`'s `#[from]` attribute and `anyhow::Error` (which accepts any error) are so useful.

```rust
// ? also works with Option<T> in functions returning Option:
fn first_even(nums: &[i32]) -> Option<i32> {
    let first = nums.first()?;  // returns None if slice is empty
    if first % 2 == 0 { Some(*first) } else { None }
}
```

---

## Q4: What is `thiserror` and when do you use it?

**Answer:**
`thiserror` is a derive macro for creating custom error types. Use it for **library/domain errors** where callers need to match on specific error variants.

```rust
use thiserror::Error;

/// Application-level error type.
/// Each variant maps to an HTTP status code (see IntoResponse impl).
#[derive(Debug, Error)]
enum AppError {
    #[error("Resource not found")]
    NotFound,

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Duplicate resource: {0}")]
    Conflict(String),

    #[error("Unauthorized")]
    Unauthorized,

    // #[from] auto-implements From<sqlx::Error> for AppError
    // This lets the ? operator convert sqlx errors automatically
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
```

**What `#[from]` does:**
```rust
// #[from] generates this impl automatically:
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}
// Now you can use ? on sqlx Results in functions returning Result<T, AppError>
```

---

## Q5: What is `anyhow` and when do you use it vs `thiserror`?

**Answer:**

| Crate | Purpose | Use when |
|---|---|---|
| `thiserror` | Define structured error types with variants | Library code, domain errors callers match on |
| `anyhow` | Catch-all error type, any error + context | Application code, binary/CLI, "just propagate it" |

```rust
// thiserror — callers can match on variants
match do_something() {
    Err(AppError::NotFound) => return 404,
    Err(AppError::Validation(msg)) => return 422 with msg,
    Err(e) => return 500,
    Ok(v) => ...,
}

// anyhow — just propagate with context, don't care about variant
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {   // anyhow::Result
    let path = std::env::var("CONFIG_PATH")
        .context("CONFIG_PATH not set")?;         // adds context string

    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read {path}"))?;  // lazy context

    let config: Config = toml::from_str(&content)
        .context("Invalid TOML")?;

    Ok(config)
}
// Error output: "Invalid TOML: expected `=`, found newline at line 5"
//   ↑ context         ↑ original error
```

**Practical rule:** Use `thiserror` for your `AppError` enum. Use `anyhow` inside application functions where you just want to add context and propagate.

---

## Q6: How do you implement centralized HTTP error handling in axum?

**Answer:**

```rust
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            // Client errors — safe to show to the user
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),

            // Server errors — log the real error, return generic message
            // SECURITY: never expose internal details to the client
            AppError::Database(e) => {
                tracing::error!("Database error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into())
            }
            AppError::Internal(e) => {
                tracing::error!("Internal error: {e:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into())
            }
        };

        let body = serde_json::json!({
            "error": {
                "status": status.as_u16(),
                "message": message,
            }
        });

        (status, Json(body)).into_response()
    }
}
```

**Key principles:**
- **Never expose internal errors** to clients — `Database(e)` logs `e` internally but returns generic "Internal server error"
- **Consistent response format** — every error returns `{ "error": { "status": 404, "message": "..." } }`
- **Use `tracing::error!`** not `println!` — structured logging, filterable, production-ready
- Every handler returns `Result<T, AppError>` — no ad-hoc error handling

---

## Q7: When is `.unwrap()` acceptable in Rust?

**Answer:**

```rust
// NEVER in production code paths — use .expect() or ? instead.
// .unwrap() panics with an unhelpful "called unwrap on None/Err" message.

// ACCEPTABLE in these cases:

// 1. Tests — test should panic on unexpected errors
#[test]
fn test_parse() {
    let result: i32 = "42".parse().unwrap();
    assert_eq!(result, 42);
}

// 2. Proven invariants — when you can mathematically prove it won't fail
// Use .expect() with a message explaining WHY it's safe
let regex = Regex::new(r"^IMO\d{7}$").expect("hard-coded regex is valid");

// 3. Static initialization
lazy_static! {
    static ref CONFIG: Config = load_config().expect("config must be valid at startup");
}

// 4. After just checking the condition
if map.contains_key(&key) {
    let val = map[&key];  // safe — we just checked
    // But prefer: if let Some(val) = map.get(&key) { ... }
}
```

**Interview answer:** "I never use `.unwrap()` in production code. I use `?` for propagation, `.ok_or()` to convert to my error type, or `.expect()` for invariants with a clear message explaining why the value must be present."

---

## Q8: How do you handle multiple error types in one function?

**Answer:**

```rust
// Problem: this function can fail with io::Error OR serde_json::Error
fn load_vessels(path: &str) -> Result<Vec<Vessel>, ???> {
    let content = std::fs::read_to_string(path)?;  // io::Error
    let vessels: Vec<Vessel> = serde_json::from_str(&content)?;  // serde_json::Error
    Ok(vessels)
}

// Solution 1: thiserror — define an enum with From impls
#[derive(Debug, Error)]
enum LoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
}
fn load_vessels(path: &str) -> Result<Vec<Vessel>, LoadError> { ... }

// Solution 2: anyhow — catch-all, adds context
fn load_vessels(path: &str) -> anyhow::Result<Vec<Vessel>> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read vessel file")?;
    let vessels = serde_json::from_str(&content)
        .context("Failed to parse vessel JSON")?;
    Ok(vessels)
}

// Solution 3: Box<dyn Error> — quick and dirty (avoid in real code)
fn load_vessels(path: &str) -> Result<Vec<Vessel>, Box<dyn std::error::Error>> { ... }
```

**Which to choose:**
- `thiserror` when callers need to match on specific error variants
- `anyhow` when you just want to propagate and add context
- Never `Box<dyn Error>` in production — no type safety, no context

---

## Q9: What is the `From` trait and how does it relate to error handling?

**Answer:**

```rust
// From<T> converts type T into Self
// The ? operator calls From::from(error) to convert error types automatically

// Manual implementation:
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

// Now ? works:
async fn get_vessel(pool: &PgPool, id: i32) -> Result<Vessel, AppError> {
    let vessel = sqlx::query_as!(Vessel, "SELECT * FROM vessels WHERE id = $1", id)
        .fetch_one(pool)
        .await?;  // sqlx::Error → AppError::Database via From
    Ok(vessel)
}

// thiserror's #[from] generates the From impl for you:
#[derive(Debug, Error)]
enum AppError {
    #[error("DB: {0}")]
    Database(#[from] sqlx::Error),  // auto-generates From<sqlx::Error>
}
```

**Chain of conversions:** `sqlx::Error` → `From::from()` → `AppError::Database(e)` → `?` returns it → `IntoResponse` → HTTP 500

---

## Q10: How do you log errors properly in a Rust service?

**Answer:**

```rust
// Use the tracing crate — structured, async-aware, filterable
use tracing::{info, warn, error, instrument};

// #[instrument] automatically logs function entry/exit with parameters
#[instrument(skip(state))]  // skip logging the full state struct
async fn get_vessel(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Vessel>, AppError> {
    let vessel = sqlx::query_as!(Vessel, "SELECT * FROM vessels WHERE id = $1", id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            // Log the error with structured fields
            error!(vessel_id = id, error = %e, "Database query failed");
            AppError::Database(e)
        })?
        .ok_or_else(|| {
            warn!(vessel_id = id, "Vessel not found");
            AppError::NotFound
        })?;

    info!(vessel_id = id, "Vessel retrieved successfully");
    Ok(Json(vessel))
}

// Setup in main.rs:
tracing_subscriber::fmt()
    .with_env_filter("info,sqlx=warn")  // default info, sqlx only warnings
    .json()                              // structured JSON for production
    .init();
```

**Key rules:**
- `tracing::error!` for server errors (500s) — alerts, needs investigation
- `tracing::warn!` for expected client errors (404s) — informational
- `tracing::info!` for successful operations — audit trail
- Never log sensitive data (passwords, tokens, PII)
- Use structured fields (`vessel_id = id`) not string interpolation — enables log querying
