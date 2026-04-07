# Rust CRUD Service — Interview Q&A

> See also: [API Design](RUST_API_DESIGN.md) | [Error Handling](RUST_ERROR_HANDLING.md) | [Full-Stack General](RUST_FULLSTACK_GENERAL.md)

## Table of Contents

- [Q1: Implement a full CRUD service with axum and sqlx](#q1-implement-a-full-crud-service-in-rust-with-axum-and-sqlx)
- [Q2: How does sqlx differ from an ORM like Diesel?](#q2-how-does-sqlx-differ-from-an-orm-like-diesel)
- [Q3: How do you handle database migrations?](#q3-how-do-you-handle-database-migrations-in-rust)
- [Q4: How do you test a CRUD service?](#q4-how-do-you-test-a-crud-service-in-rust)
- [Q5: fetch_one vs fetch_optional vs fetch_all](#q5-what-is-the-difference-between-fetch_one-fetch_optional-and-fetch_all-in-sqlx)
- [Q6: How do you handle partial updates (PATCH)?](#q6-how-do-you-handle-partial-updates-patch-semantics)
- [Q7: How does ownership help in a CRUD context?](#q7-how-does-rusts-ownership-system-help-in-a-crud-context)

> **Note:** All SQL queries use parameterized bindings (`$1`, `$2`, ...) which prevent SQL injection by design — user input is never interpolated into SQL strings.

---

## Q1: Implement a full CRUD service in Rust with axum and sqlx.

**Answer:**

```rust
use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

// --- Models ---

/// Database row representation.
#[derive(Debug, Serialize, sqlx::FromRow)]
struct Vessel {
    id: i32,
    name: String,
    imo_number: String,
    status: String,
    created_at: chrono::NaiveDateTime,
}

/// Request body for creating a vessel.
#[derive(Debug, Deserialize)]
struct CreateVessel {
    name: String,
    imo_number: String,
}

/// Request body for updating a vessel.
/// All fields are Option — only provided fields are updated.
#[derive(Debug, Deserialize)]
struct UpdateVessel {
    name: Option<String>,
    status: Option<String>,
}

// --- Shared state ---

/// Application state shared across all handlers via Arc (axum clones this).
#[derive(Clone)]
struct AppState {
    db: PgPool,
}

// --- Handlers ---

/// CREATE — POST /vessels
/// Returns 201 Created with the new vessel.
async fn create_vessel(
    State(state): State<AppState>,
    Json(payload): Json<CreateVessel>,
) -> Result<(StatusCode, Json<Vessel>), AppError> {
    // RETURNING avoids a second SELECT after INSERT
    let vessel = sqlx::query_as!(Vessel,
        "INSERT INTO vessels (name, imo_number, status)
         VALUES ($1, $2, 'active')
         RETURNING id, name, imo_number, status, created_at",
        payload.name, payload.imo_number
    )
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(vessel)))
}

/// READ (list) — GET /vessels
async fn list_vessels(
    State(state): State<AppState>,
) -> Result<Json<Vec<Vessel>>, AppError> {
    let vessels = sqlx::query_as!(Vessel,
        "SELECT id, name, imo_number, status, created_at
         FROM vessels ORDER BY id"
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(vessels))
}

/// READ (single) — GET /vessels/:id
/// Returns 404 if vessel not found.
async fn get_vessel(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Vessel>, AppError> {
    // fetch_optional returns Option<Vessel> — None if no row matched
    let vessel = sqlx::query_as!(Vessel,
        "SELECT id, name, imo_number, status, created_at
         FROM vessels WHERE id = $1", id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;   // Convert None → 404 error

    Ok(Json(vessel))
}

/// UPDATE — PUT /vessels/:id
/// Uses COALESCE to only update provided fields.
async fn update_vessel(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateVessel>,
) -> Result<Json<Vessel>, AppError> {
    // COALESCE($2, name) — if $2 is NULL (field not provided),
    // keep the existing value. This enables partial updates.
    let vessel = sqlx::query_as!(Vessel,
        "UPDATE vessels SET
            name = COALESCE($2, name),
            status = COALESCE($3, status)
         WHERE id = $1
         RETURNING id, name, imo_number, status, created_at",
        id, payload.name, payload.status
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(vessel))
}

/// DELETE — DELETE /vessels/:id
/// Returns 204 No Content on success, 404 if not found.
async fn delete_vessel(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query!("DELETE FROM vessels WHERE id = $1", id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)   // 204 — success, no response body
}

// --- Router ---

fn vessel_routes() -> Router<AppState> {
    Router::new()
        .route("/vessels", get(list_vessels).post(create_vessel))
        .route("/vessels/:id", get(get_vessel).put(update_vessel).delete(delete_vessel))
}
```

**Key patterns to highlight in interview:**
- `RETURNING` clause avoids a second SELECT — Postgres returns the new row (with auto-generated `id`, `created_at`) in the same query. Without it, you'd need `INSERT` then `SELECT WHERE id = last_insert_id` — two DB round trips instead of one.
- `fetch_optional` + `.ok_or(NotFound)` for clean 404s — `fetch_optional` returns `Option<Vessel>` (`Some` if found, `None` if no row). `.ok_or(AppError::NotFound)` converts `None` to a 404 in one line. The alternative (`fetch_one`) throws an ugly sqlx error if the row is missing — bad for the client.
- `COALESCE` for partial updates without overwriting unset fields — `COALESCE($2, name)` means "use the new value if provided, otherwise keep the existing column value." When the client omits a field, Rust deserializes it as `None`, sqlx sends `NULL` to Postgres, and `COALESCE(NULL, name)` preserves the old value. Without it, omitted fields would be overwritten with `NULL`.
- `StatusCode::CREATED` (201) for POST, `NO_CONTENT` (204) for DELETE — REST convention: 201 = "I made something new, here it is in the body"; 204 = "Done, nothing to send back." Using 200 for everything works but is less informative to the client.
- All handlers return `Result<T, AppError>` — centralized error handling. The `IntoResponse` impl on `AppError` maps each variant to an HTTP status (NotFound → 404, Unauthorized → 401, etc.) in one place. Adding a new error type = add one enum variant + one match arm, not touching every handler.

---

## Q2: How does sqlx differ from an ORM like Diesel?

**Answer:**

| Aspect | sqlx | Diesel |
|---|---|---|
| Approach | Raw SQL with compile-time verification | Query builder DSL |
| SQL knowledge | You write real SQL | DSL abstracts SQL away |
| Compile-time checks | `query_as!` macro checks SQL against live DB | Schema checked via generated Rust code |
| Async | Native async support | Sync only (async via `tokio::task::spawn_blocking`) |
| Learning curve | Know SQL → know sqlx | Must learn Diesel DSL |
| Migrations | `sqlx migrate` | `diesel migration` |

```rust
// sqlx — write SQL directly, compiler checks it
let vessel = sqlx::query_as!(Vessel,
    "SELECT id, name FROM vessels WHERE id = $1", id
)
.fetch_one(&pool).await?;

// Diesel — query builder DSL
let vessel = vessels::table
    .filter(vessels::id.eq(id))
    .select((vessels::id, vessels::name))
    .first::<Vessel>(&mut conn)?;
```

**When to choose sqlx:** When you want full SQL control, async support, and the team is comfortable writing SQL. Most backend interview scenarios favor sqlx for its simplicity.

---

## Q3: How do you handle database migrations in Rust?

**Answer:**

```bash
# Create a migration
sqlx migrate add create_vessels_table

# This creates: migrations/20260328120000_create_vessels_table.sql
```

```sql
-- migrations/20260328120000_create_vessels_table.sql
CREATE TABLE vessels (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    imo_number  VARCHAR(10) UNIQUE NOT NULL,
    status      VARCHAR(20) NOT NULL DEFAULT 'active'
                CHECK (status IN ('active', 'inactive', 'maintenance')),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_vessels_status ON vessels(status);
```

```rust
// Run migrations on app startup
sqlx::migrate!("./migrations")
    .run(&pool)
    .await?;
```

**Best practices:**
- Each migration is a single SQL file — forward only (no down migrations in production)
- Migration files are ordered by timestamp — run sequentially
- Never modify a migration that's already been applied — create a new one
- Use `TIMESTAMPTZ` (not `TIMESTAMP`) — always store timezone

---

## Q4: How do you test a CRUD service in Rust?

**Answer:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;  // axum-test crate for HTTP-level tests

    // Integration test — tests the full HTTP roundtrip
    #[tokio::test]
    async fn test_create_and_get_vessel() {
        // Setup: create test DB pool (use a test database or transaction)
        let pool = setup_test_db().await;
        let state = AppState { db: pool };
        let app = vessel_routes().with_state(state);
        let server = TestServer::new(app).unwrap();

        // CREATE
        let response = server.post("/vessels")
            .json(&serde_json::json!({
                "name": "Test Ship",
                "imo_number": "IMO1234567"
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        let vessel: Vessel = response.json();
        assert_eq!(vessel.name, "Test Ship");
        assert_eq!(vessel.status, "active");

        // READ — verify it was persisted
        let response = server.get(&format!("/vessels/{}", vessel.id)).await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let fetched: Vessel = response.json();
        assert_eq!(fetched.id, vessel.id);
    }

    // Test 404 for missing resource
    #[tokio::test]
    async fn test_get_nonexistent_vessel_returns_404() {
        let pool = setup_test_db().await;
        let state = AppState { db: pool };
        let app = vessel_routes().with_state(state);
        let server = TestServer::new(app).unwrap();

        let response = server.get("/vessels/99999").await;
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    // Test DELETE
    #[tokio::test]
    async fn test_delete_vessel() {
        let pool = setup_test_db().await;
        let state = AppState { db: pool };
        let app = vessel_routes().with_state(state);
        let server = TestServer::new(app).unwrap();

        // Create then delete
        let response = server.post("/vessels")
            .json(&serde_json::json!({"name": "Doomed Ship", "imo_number": "IMO0000001"}))
            .await;
        let vessel: Vessel = response.json();

        let response = server.delete(&format!("/vessels/{}", vessel.id)).await;
        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

        // Verify it's gone
        let response = server.get(&format!("/vessels/{}", vessel.id)).await;
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    /// Use a transaction that rolls back after each test — no test pollution.
    async fn setup_test_db() -> PgPool {
        let pool = PgPool::connect("postgres://localhost/test_db").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }
}
```

**Testing strategy:**
- **Unit tests** — test service functions with a mock/test DB pool
- **Integration tests** — test full HTTP roundtrip via `axum-test` crate
- **Test isolation** — each test uses a transaction that rolls back, so tests don't affect each other
- **Arrange-Act-Assert** pattern — setup data, perform action, verify result

---

## Q5: What is the difference between `fetch_one`, `fetch_optional`, and `fetch_all` in sqlx?

**Answer:**

```rust
// fetch_one — expects exactly 1 row. Panics/errors if 0 or 2+ rows.
// Use for: queries that MUST return a result (e.g., INSERT ... RETURNING)
let vessel = sqlx::query_as!(Vessel, "... RETURNING *")
    .fetch_one(&pool).await?;

// fetch_optional — returns Option<T>. None if no rows, Some(row) if 1.
// Use for: lookups by ID where the row might not exist.
let vessel = sqlx::query_as!(Vessel, "SELECT * FROM vessels WHERE id = $1", id)
    .fetch_optional(&pool).await?    // Result<Option<Vessel>>
    .ok_or(AppError::NotFound)?;     // Convert None to 404

// fetch_all — returns Vec<T>. Empty vec if no rows.
// Use for: list queries.
let vessels = sqlx::query_as!(Vessel, "SELECT * FROM vessels")
    .fetch_all(&pool).await?;        // Result<Vec<Vessel>>
```

**Common mistake:** Using `fetch_one` for a SELECT by ID — it returns an ugly sqlx error if not found. Use `fetch_optional` + `.ok_or(AppError::NotFound)` for clean 404s.

---

## Q6: How do you handle partial updates (PATCH semantics)?

**Answer:**

```rust
/// All fields are Option — None means "don't change this field".
#[derive(Deserialize)]
struct UpdateVessel {
    name: Option<String>,
    status: Option<String>,
}

async fn update_vessel(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateVessel>,
) -> Result<Json<Vessel>, AppError> {
    // SQL COALESCE: if the new value is NULL, keep the old value.
    // This lets the client send only the fields they want to change.
    //
    // Example: { "status": "inactive" }
    //   → name stays the same, status changes to "inactive"
    //
    // Without COALESCE, omitted fields would be set to NULL.
    let vessel = sqlx::query_as!(Vessel,
        "UPDATE vessels SET
            name = COALESCE($2, name),
            status = COALESCE($3, status)
         WHERE id = $1
         RETURNING id, name, imo_number, status, created_at",
        id, payload.name, payload.status
    )
    .fetch_optional(&state.db).await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(vessel))
}
```

**Alternative approach — build dynamic SQL:**

```rust
// For many optional fields, dynamic queries are cleaner
use sqlx::QueryBuilder;

let mut builder = QueryBuilder::new("UPDATE vessels SET ");
let mut separated = builder.separated(", ");

if let Some(name) = &payload.name {
    separated.push("name = ").push_bind(name);
}
if let Some(status) = &payload.status {
    separated.push("status = ").push_bind(status);
}
builder.push(" WHERE id = ").push_bind(id);
builder.push(" RETURNING *");

let vessel = builder.build_query_as::<Vessel>()
    .fetch_optional(&state.db).await?
    .ok_or(AppError::NotFound)?;
```

---

## Q7: How does Rust's ownership system help in a CRUD context?

**Answer:**

```rust
// 1. Request body is MOVED into the handler — no shared mutable state
async fn create_vessel(Json(payload): Json<CreateVessel>) -> ... {
    // `payload` is owned by this handler — impossible to accidentally
    // share it with another request. In Node.js, a bug could leak
    // request data between concurrent requests via shared closure state.
}

// 2. Database pool is safely shared — PgPool uses Arc internally
// The compiler enforces that PgPool is Clone + Send + Sync, meaning
// it's safe to share across async tasks without locks.
//
// How the pool works:
//   App startup: PgPool::connect() creates N pre-opened DB connections (e.g., 10)
//   Each query:  sqlx borrows an idle connection from the pool (not a new TCP conn)
//   Query done:  connection auto-returned to pool (Rust's Drop trait)
//   Pool full:   new queries wait until a connection frees up
//   Without a pool, every query opens a new TCP connection (~5-50ms handshake each)
//   Ruby equivalent: ActiveRecord::Base.connection_pool
#[derive(Clone)]       // Clone is cheap — just increments Arc refcount
struct AppState {
    db: PgPool,        // internally Arc<Pool> — shared, not copied
}

// 3. Borrowed vs owned — prevents wasteful cloning
async fn get_vessel(Path(id): Path<i32>) -> ... {
    // `id` is i32 (Copy) — no allocation
    // If it were Path(name): Path<String>, the String is MOVED in, not cloned
}

// 4. Drop guarantees cleanup — DB connections returned to pool
async fn list_vessels(State(state): State<AppState>) -> ... {
    let conn = state.db.acquire().await?;
    // ... use conn ...
    // conn is Dropped here → automatically returned to the pool
    // Even if the function returns early via ?, Drop still runs
}
```

**Bottom line:** Rust's ownership model eliminates entire classes of bugs that are common in CRUD services: connection leaks, shared mutable state between requests, and use-after-free of request data.
