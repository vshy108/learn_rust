# Rust API Design — Interview Q&A

## Q1: How would you design a REST API in Rust? What framework would you choose?

**Answer:**
Use **axum** — it's built on top of `tokio` and `tower`, async-first, and has strong type safety via extractors.

```rust
use axum::{
    extract::{Path, Query, State, Json},
    routing::{get, post, put, delete},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    let state = AppState { db: db_pool };

    let app = Router::new()
        .route("/api/v1/vessels", get(list_vessels).post(create_vessel))
        .route("/api/v1/vessels/:id", get(get_vessel).put(update_vessel).delete(delete_vessel))
        .route("/api/v1/vessels/:id/alerts", get(list_alerts))
        .with_state(state)
        .layer(TraceLayer::new_for_http())             // observability
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(CompressionLayer::new());               // gzip responses

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```

**Why axum over alternatives:**

| Framework | Pros | Cons |
|---|---|---|
| **axum** | Type-safe extractors, tower ecosystem, maintained by tokio team | Newer, smaller community than actix |
| actix-web | Mature, fast benchmarks | Actor model complexity, macros-heavy |
| rocket | Ergonomic, Rails-like | Slower adoption of async, less flexible |

**What is the tower ecosystem?**
Tower is a Rust library that defines a `Service` trait — a function that takes a request and returns a future of a response. The "tower ecosystem" is the collection of crates built on this shared trait: **tower** (core combinators: retry, rate-limit, load-balance), **tower-http** (HTTP middleware: tracing, compression, CORS, timeouts), **hyper** (HTTP implementation), **axum** (web framework), and **tonic** (gRPC). Because they all share the same `Service` trait, middleware is interchangeable across frameworks — like Ruby's Rack middleware, but with compile-time type safety.

---

## Q2: How do axum extractors work and why are they useful?

**Answer:**
Extractors pull data from incoming requests into typed Rust values. The compiler verifies types at build time — no runtime parsing surprises.

```rust
// Path — extracts URL segments into typed values
async fn get_vessel(Path(id): Path<i32>) -> Result<Json<Vessel>, AppError> { ... }
//                        ^^^^ compiler ensures id is i32

// Query — deserializes ?key=value params into a struct
#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    per_page: Option<u32>,
}
async fn list_vessels(Query(params): Query<Pagination>) -> ... { ... }

// Json — deserializes request body into a struct
#[derive(Deserialize)]
struct CreateVessel {
    name: String,
    imo_number: String,
}
async fn create_vessel(Json(payload): Json<CreateVessel>) -> ... { ... }

// State — shares application state (DB pool, config) across handlers
// Clone is required by axum — it clones state for each concurrent handler.
// PgPool is internally Arc<Pool>, so clone is cheap (ref count bump, not deep copy).
#[derive(Clone)]
struct AppState { db: PgPool }
async fn list_vessels(State(state): State<AppState>) -> ... {
    let vessels = sqlx::query_as!(Vessel, "SELECT * FROM vessels")
        .fetch_all(&state.db).await?;
    Ok(Json(vessels))
}
```

**Key benefit:** If the request body doesn't match `CreateVessel`, axum automatically returns 422 with an error message — you don't need manual parsing or validation boilerplate.

---

## Q3: How do you version an API and handle breaking changes?

**Answer:**

```rust
// URL path versioning — most common, explicit
let app = Router::new()
    .nest("/api/v1", v1_routes())
    .nest("/api/v2", v2_routes());

fn v1_routes() -> Router<AppState> {
    Router::new()
        .route("/vessels", get(v1::list_vessels))
}

fn v2_routes() -> Router<AppState> {
    Router::new()
        .route("/vessels", get(v2::list_vessels))  // different response shape
}
```

**Versioning strategies:**
- **Path versioning** (`/v1/`, `/v2/`) — most visible, easy to route, recommended for most APIs
- **Header versioning** (`Accept: application/vnd.api+json; version=2`) — cleaner URLs but hidden
- **Query param** (`?version=2`) — simple but pollutes caching

**Handling transitions:**
- Keep v1 running while clients migrate to v2
- Deprecation header: `Deprecation: true` + `Sunset: 2026-06-01`
- Internally, v1 and v2 handlers can share the same service layer — only the request/response shapes differ

---

## Q4: How do you handle pagination in a Rust API?

**Answer:**

```rust
#[derive(Deserialize)]
struct PaginationParams {
    page: Option<u32>,
    per_page: Option<u32>,
}

impl PaginationParams {
    /// Returns clamped per_page (1..=100, default 20).
    fn limit(&self) -> i64 {
        self.per_page.unwrap_or(20).clamp(1, 100) as i64
    }

    /// Returns byte offset for SQL OFFSET clause.
    fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        (page as i64 - 1) * self.limit()
    }
}

#[derive(Serialize)]
struct PaginatedResponse<T: Serialize> {
    data: Vec<T>,
    meta: PaginationMeta,
}

#[derive(Serialize)]
struct PaginationMeta {
    page: u32,
    per_page: u32,
    total: i64,
}

async fn list_vessels(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Vessel>>, AppError> {
    let total = sqlx::query_scalar!("SELECT COUNT(*) FROM vessels")
        .fetch_one(&state.db).await?;

    let vessels = sqlx::query_as!(Vessel,
        "SELECT * FROM vessels ORDER BY id LIMIT $1 OFFSET $2",
        params.limit(), params.offset()
    )
    .fetch_all(&state.db).await?;

    Ok(Json(PaginatedResponse {
        data: vessels,
        meta: PaginationMeta {
            page: params.page.unwrap_or(1),
            per_page: params.limit() as u32,
            total: total.unwrap_or(0),
        },
    }))
}
```

**Offset vs cursor pagination:**
- **Offset** (`LIMIT/OFFSET`) — simple, supports jumping to page N, but slow on large tables (DB scans skipped rows)
- **Cursor** (`WHERE id > $cursor LIMIT $n`) — fast on large tables, but no page jumping. Best for infinite scroll / real-time feeds.

---

## Q5: How do you add middleware in axum?

**Answer:**
axum uses tower middleware layers — composable, reusable, and async-native.

```rust
use axum::middleware;
use tower_http::{
    trace::TraceLayer,
    timeout::TimeoutLayer,
    compression::CompressionLayer,
    cors::CorsLayer,
};

let app = Router::new()
    .route("/api/v1/vessels", get(list_vessels))
    .layer(TraceLayer::new_for_http())       // logging/tracing every request
    .layer(TimeoutLayer::new(Duration::from_secs(30)))  // kill slow requests
    .layer(CompressionLayer::new())          // gzip responses
    .layer(CorsLayer::permissive());         // CORS headers

// Custom middleware — e.g., auth check
async fn auth_middleware(
    headers: HeaderMap,
    request: axum::extract::Request,
    next: middleware::Next,
) -> Result<axum::response::Response, AppError> {
    let token = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    validate_token(token)?;
    Ok(next.run(request).await)
}

// Apply to specific routes
let protected = Router::new()
    .route("/vessels", get(list_vessels))
    .layer(middleware::from_fn(auth_middleware));
```

**Layer order matters:** Layers wrap from bottom to top. The last `.layer()` is the outermost (runs first). Tracing should be outermost to capture timing of everything including auth.

---

## Q6: How does Rust's type system benefit API design?

**Answer:**

```rust
// 1. Newtypes prevent mixing semantically different IDs
struct VesselId(i32);
struct AlertId(i32);
// Can't accidentally pass AlertId where VesselId is expected

// 2. Enums for finite states — compiler ensures exhaustive handling
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum VesselStatus {
    Active,
    Inactive,
    Maintenance,
}
// Serde rejects any status not in this list — no "actve" typos reaching DB

// 3. Option<T> eliminates null checks
#[derive(Deserialize)]
struct UpdateVessel {
    name: Option<String>,     // explicitly optional — caller may omit
    status: Option<VesselStatus>,
}
// Compiler forces you to handle the None case — no null pointer at runtime

// 4. Result<T, E> for every fallible operation
async fn get_vessel(Path(id): Path<VesselId>) -> Result<Json<Vessel>, AppError> {
    // Must handle the error — code won't compile otherwise
    let vessel = find_vessel(id).await?;
    Ok(Json(vessel))
}

// How ? works — the error propagation flow:
//   find_vessel(id).await returns Result<Vessel, AppError>
//     → Ok(vessel):  ? unwraps the value, execution continues
//     → Err(e):      ? returns early with Err(AppError), which axum
//                     converts to an HTTP response via IntoResponse (see Q9)
//
// Inside find_vessel, errors originate from two places:
async fn find_vessel(id: VesselId, db: &PgPool) -> Result<Vessel, AppError> {
    sqlx::query_as!(Vessel, "SELECT * FROM vessels WHERE id = $1", id.0)
        .fetch_optional(db).await?   // DB error → AppError via From<sqlx::Error>
        .ok_or(AppError::NotFound)   // None (no row) → AppError::NotFound (404)
}
// Without ?, you'd write: match find_vessel(id).await { Ok(v) => v, Err(e) => return Err(e) }
// Ruby/Node silently returns nil/undefined on missing data — Rust forces you to handle it.
```

**Compared to dynamically-typed languages:** In Node.js/Python, a misspelled field name, wrong ID type, or missing null check only crashes at runtime. In Rust, the compiler catches all of these before the code ever runs.

---

## Q7: How do you handle request validation at the API boundary?

**Answer:**

```rust
use validator::Validate;

#[derive(Deserialize, Validate)]
struct CreateVessel {
    #[validate(length(min = 1, max = 100))]
    name: String,

    #[validate(regex(path = "IMO_REGEX"))]
    imo_number: String,
}

lazy_static::lazy_static! {
    static ref IMO_REGEX: regex::Regex = regex::Regex::new(r"^IMO\d{7}$").unwrap();
}

// Custom extractor that validates after deserializing
async fn create_vessel(Json(payload): Json<CreateVessel>) -> Result<..., AppError> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    // If we reach here, payload is guaranteed valid
    ...
}
```

**Validation layers (defense in depth):**

| Layer | What it catches | Example |
|---|---|---|
| **Serde deserialization** | Wrong types, missing required fields | `name: 123` when String expected |
| **validator crate** | Business rules | Name too long, invalid IMO format |
| **Application logic** | Semantic rules | Duplicate IMO number |
| **Database constraints** | Final safety net | UNIQUE violation, NOT NULL |

**Principle:** Validate at the boundary (handler), not deep inside business logic. Fail fast with clear error messages.

---

## Q8: How do you structure a larger Rust API project?

**Answer:**

```
src/
├── main.rs              # entry point, router setup
├── config.rs            # environment variables, settings
├── routes/
│   ├── mod.rs           # re-exports, route registration
│   ├── vessels.rs       # vessel handlers
│   └── alerts.rs        # alert handlers
├── models/
│   ├── mod.rs
│   ├── vessel.rs        # Vessel struct, DB mappings
│   └── alert.rs
├── services/
│   ├── mod.rs
│   ├── vessel_service.rs  # business logic (testable without HTTP)
│   └── alert_service.rs
├── errors.rs            # AppError enum, IntoResponse impl
└── db.rs                # connection pool setup, migrations
```

**Key design principles:**
- **Handlers** are thin — extract request data, call service, format response
- **Services** contain business logic — testable without HTTP
- **Models** define data shapes — shared between handlers and services
- **Errors** centralized — one `AppError` enum with `IntoResponse`

```rust
// Handler — thin layer
async fn create_vessel(
    State(state): State<AppState>,
    Json(payload): Json<CreateVessel>,
) -> Result<(StatusCode, Json<Vessel>), AppError> {
    payload.validate()?;
    let vessel = vessel_service::create(&state.db, payload).await?;
    Ok((StatusCode::CREATED, Json(vessel)))
}

// Service — business logic, independently testable
pub async fn create(db: &PgPool, input: CreateVessel) -> Result<Vessel, AppError> {
    // Business rule: check for duplicate IMO
    if exists_by_imo(db, &input.imo_number).await? {
        return Err(AppError::Conflict("IMO number already registered".into()));
    }
    insert_vessel(db, input).await
    // ^ No ? here — this is the last expression, and its type (Result<Vessel, AppError>)
    // already matches the function's return type. Rust returns it directly.
    // With ?: Ok(insert_vessel(db, input).await?) — unwraps then re-wraps, redundant.
    // Skipping ? on the tail expression is idiomatic Rust.
}
```

---

## Q9: How do you manually control HTTP status codes in axum?

**Answer:**
Anything that implements the `IntoResponse` trait can be a handler return type. axum's automatic 422 is just the default — you can override it completely.

```rust
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};

// 1. Tuple — (StatusCode, body)
async fn create_vessel(...) -> (StatusCode, Json<Vessel>) {
    (StatusCode::CREATED, Json(vessel))       // 201
}

// 2. Just a StatusCode (no body)
async fn delete_vessel(...) -> StatusCode {
    StatusCode::NO_CONTENT                    // 204
}

// 3. Result with inline error status
async fn get_vessel(...) -> Result<Json<Vessel>, (StatusCode, String)> {
    Err((StatusCode::NOT_FOUND, "vessel not found".into()))  // 404
}

// 4. Full control via Response builder
async fn custom(...) -> Response {
    Response::builder()
        .status(StatusCode::ACCEPTED)         // 202
        .header("X-Custom", "value")
        .body(Body::from("accepted"))
        .unwrap()
}

// 5. Centralized — AppError implementing IntoResponse (recommended)
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::NotFound       => StatusCode::NOT_FOUND,             // 404
            AppError::Unauthorized   => StatusCode::UNAUTHORIZED,          // 401
            AppError::Validation(_)  => StatusCode::UNPROCESSABLE_ENTITY,  // 422
            AppError::Conflict(_)    => StatusCode::CONFLICT,              // 409
            _                        => StatusCode::INTERNAL_SERVER_ERROR, // 500
        };
        (status, self.to_string()).into_response()
    }
}
```

**Common HTTP status codes in CRUD APIs:**

| Code | Constant | When to use |
|---|---|---|
| 200 | `OK` | Successful GET, PUT |
| 201 | `CREATED` | Successful POST (resource created) |
| 204 | `NO_CONTENT` | Successful DELETE (no response body) |
| 400 | `BAD_REQUEST` | Malformed syntax (invalid JSON) |
| 401 | `UNAUTHORIZED` | Missing or invalid auth token |
| 403 | `FORBIDDEN` | Authenticated but not permitted |
| 404 | `NOT_FOUND` | Resource doesn't exist |
| 409 | `CONFLICT` | Duplicate resource (e.g., unique constraint) |
| 422 | `UNPROCESSABLE_ENTITY` | Valid JSON but semantically wrong values |
| 500 | `INTERNAL_SERVER_ERROR` | Unexpected server failure |

**Best practice:** Use approach #5 — define one `AppError` enum with `IntoResponse`. All handlers return `Result<T, AppError>`, and status codes are centralized in one place.
