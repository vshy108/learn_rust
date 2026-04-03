// Full CRUD REST API with axum — in-memory Todo store.
//
// Demonstrates: extractors (Path, Query, Json, State), centralized error
// handling, CORS middleware, and all standard HTTP methods.
//
// Run: cargo run -p s28_axum_api
// Test: cargo test -p s28_axum_api

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use axum::Router;
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_swagger_ui::SwaggerUi;

// ---------------------------------------------------------------------------
// Models
// ---------------------------------------------------------------------------

/// A Todo item stored in our in-memory database.
///
/// # Fields
///
/// * `id` - Auto-incremented unique identifier
/// * `title` - Description of the todo
/// * `completed` - Whether the todo is done
///
/// # Derive rationale
///
/// * `Debug` - Enables `{:?}` formatting for logging, `dbg!()`, and test output
/// * `Clone` - Required to return copies from behind `RwLock` (the original stays in HashMap)
/// * `Serialize` - Required by `Json(todo)` to convert struct -> JSON response body
/// * `Deserialize` - Required by `Json<Todo>` extractor to parse JSON request body -> struct
/// * `PartialEq` - Required by `assert_eq!()` in tests to compare two Todo values
///
/// Not derived:
/// * `Eq` — could add (all fields support it) but nothing in code requires it
/// * `Hash` — would be needed if Todo were a HashMap key, but we use `u64` as key
/// * `Default` — no sensible default (what would the default title be?)
/// * `Copy` — impossible: `String` is heap-allocated and doesn't implement `Copy`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

/// Request body for creating a new todo (POST /todos).
///
/// `completed` defaults to `false` if omitted.
///
/// Only `Debug` + `Deserialize` — these are input-only types (parsed from
/// JSON, never sent back or compared), so no need for Clone/Serialize/PartialEq.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTodo {
    pub title: String,
    #[serde(default)]
    pub completed: bool,
}

/// Request body for full update (PUT /todos/:id).
///
/// All fields are required — replaces the entire todo.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTodo {
    pub title: String,
    pub completed: bool,
}

/// Request body for partial update (PATCH /todos/:id).
///
/// All fields are optional — only provided fields are updated.
#[derive(Debug, Deserialize, ToSchema)]
pub struct PatchTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

/// Query parameters for filtering todos (GET /todos?completed=true).
#[derive(Debug, Deserialize)]
pub struct TodoFilter {
    pub completed: Option<bool>,
}

// ---------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------

/// Shared state for all handlers.
///
/// Uses `Arc<RwLock<...>>` so multiple handlers can read concurrently,
/// and only one can write at a time (no Mutex needed — RwLock is better
/// when reads are more frequent than writes).
///
/// # Performance tips applied here
///
/// | Technique | Why | Applied? |
/// |-----------|-----|----------|
/// | `tokio::sync::RwLock` | Async-aware — yields to the runtime while waiting instead of blocking the OS thread. **Required** when holding the lock across `.await` points. `std::sync::RwLock` would block the entire tokio worker thread. | Yes — `todos` field |
/// | `AtomicU64` for counters | Lock-free increment via `fetch_add`. Avoids acquiring a write lock just to bump a number. ~10x faster than `RwLock<u64>` under contention. | Yes — `next_id` field |
/// | `Arc` wrapping | `Clone` on `AppState` cheaply increments reference counts instead of copying data. axum clones state into every handler invocation. | Yes — both fields |
/// | `Vec::with_capacity(n)` | Pre-allocates when size is known, avoiding reallocations. Use in list handlers when you hold the lock and know the count. | Yes — `list_todos` |
/// | `parking_lot::RwLock` | Faster than `std::sync::RwLock` (no poisoning overhead), but still blocking. Use only for short critical sections that never cross `.await`. | No — we cross `.await` |
/// | `DashMap` | Concurrent HashMap with shard-level locking. Better than `RwLock<HashMap>` under high write concurrency. Trade-off: harder to iterate atomically. | No — overkill for demo |
/// | `Cow<'_, str>` | Avoids cloning strings when ownership is conditional. Useful in larger apps where some paths return borrowed data. | No — all paths clone |
/// | `clone()` minimization | Every `.clone()` on a `Todo` heap-allocates a new `String`. In hot paths, consider returning `Arc<Todo>` to share ownership without copying. | Noted — acceptable here |
#[derive(Clone)]
pub struct AppState {
    /// In-memory todo storage. Key = todo id.
    pub todos: Arc<RwLock<HashMap<u64, Todo>>>,
    /// Auto-incrementing id counter.
    ///
    /// Uses `AtomicU64` instead of `RwLock<u64>` — a single `fetch_add`
    /// is lock-free and wait-free, much faster than acquiring a write lock.
    /// `Ordering::Relaxed` is sufficient because we only need the counter
    /// to produce unique ids, not to synchronize other memory accesses.
    pub next_id: Arc<AtomicU64>,
}

impl AppState {
    /// Creates new empty application state.
    pub fn new() -> Self {
        Self {
            todos: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Error handling
// ---------------------------------------------------------------------------

/// Centralized error type for all handlers.
///
/// Implements `IntoResponse` so axum converts each variant to the right
/// HTTP status code automatically. Handlers return `Result<T, AppError>`.
///
/// # Derive rationale
///
/// * `Debug` - Required by `std::error::Error` trait; also enables `{:?}` in logs
/// * `thiserror::Error` - Proc macro that auto-generates two trait impls:
///   - `std::fmt::Display` — from `#[error("...")]` on each variant
///   - `std::error::Error` — the standard error trait
///
///   Without it, you'd manually write both impls (see below).
///
/// # What thiserror generates (equivalent hand-written code)
///
/// ```rust,ignore
/// impl std::fmt::Display for AppError {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         match self {
///             AppError::NotFound(msg) => write!(f, "not found: {msg}"),
///             AppError::BadRequest(msg) => write!(f, "bad request: {msg}"),
///         }
///     }
/// }
/// impl std::error::Error for AppError {}
/// ```
///
/// # thiserror vs anyhow (repo convention)
///
/// * `thiserror` — **define** error types (library/handler code, like here)
/// * `anyhow` — **propagate** errors with context (application-level, e.g. `main()`)
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// 404 — resource not found.
    #[error("not found: {0}")]
    NotFound(String),

    /// 400 — invalid request.
    #[error("bad request: {0}")]
    BadRequest(String),
}

/// Maps each error variant to an HTTP status code + JSON body.
///
/// This is the single place where errors become HTTP responses.
/// Adding a new error = add one variant + one match arm.
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // Match on `self` (owned) to move the String directly,
        // avoiding the unnecessary `.clone()` that `match &self` requires.
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };
        let body = serde_json::json!({ "error": message });
        (status, Json(body)).into_response()
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// LIST — GET /todos
///
/// Returns all todos, optionally filtered by `?completed=true` or
/// `?completed=false`.
#[utoipa::path(
    get,
    path = "/todos",
    params(
        ("completed" = Option<bool>, Query, description = "Filter by completion status")
    ),
    responses(
        (status = 200, description = "List of todos", body = Vec<Todo>)
    ),
    tag = "Todos"
)]
pub async fn list_todos(
    State(state): State<AppState>,
    Query(filter): Query<TodoFilter>,
) -> Json<Vec<Todo>> {
    let todos = state.todos.read().await;
    // Collect into a Vec, applying the optional filter.
    // .values() iterates over the HashMap values (Todo items).
    // Pre-allocate with_capacity — we hold the read lock so todos.len() is
    // the upper bound. Avoids reallocations during collect.
    let mut result: Vec<Todo> = Vec::with_capacity(todos.len());
    result.extend(
        todos
            .values()
            // is_none_or (stable since Rust 1.82): returns true when the Option
            // is None, or when the predicate returns true for the Some value.
            // Replaces the verbose match { Some(c) => ..., None => true }.
            .filter(|t| filter.completed.is_none_or(|c| t.completed == c))
            .cloned(),
    );
    // Sort by id for consistent ordering (HashMap has no order).
    result.sort_by_key(|t| t.id);
    Json(result)
}

/// GET ONE — GET /todos/:id
///
/// Returns a single todo by id, or 404 if not found.
#[utoipa::path(
    get,
    path = "/todos/{id}",
    params(
        ("id" = u64, Path, description = "Todo ID")
    ),
    responses(
        (status = 200, description = "Todo found", body = Todo),
        (status = 404, description = "Todo not found")
    ),
    tag = "Todos"
)]
pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Todo>, AppError> {
    let todos = state.todos.read().await;
    let todo = todos
        .get(&id)
        // ok_or_else takes a closure (|| = zero parameters) that runs ONLY on None.
        // Why not ok_or? ok_or(expr) evaluates expr eagerly — the format! would
        // allocate a String on every call, even when the todo exists.
        // ok_or_else(|| expr) is lazy — the closure runs only when .get() returns None,
        // avoiding a heap allocation on the happy path.
        .ok_or_else(|| AppError::NotFound(format!("todo {id} not found")))?;
    Ok(Json(todo.clone()))
}

/// CREATE — POST /todos
///
/// Creates a new todo. Returns 201 Created with the created todo.
#[utoipa::path(
    post,
    path = "/todos",
    request_body = CreateTodo,
    responses(
        (status = 201, description = "Todo created", body = Todo),
        (status = 400, description = "Invalid request (empty title)")
    ),
    tag = "Todos"
)]
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    if payload.title.trim().is_empty() {
        return Err(AppError::BadRequest("title cannot be empty".to_string()));
    }

    // Atomically increment the id counter and get the previous value.
    // fetch_add is lock-free — no RwLock contention, no await point.
    // Relaxed ordering: we only need uniqueness, not memory synchronization.
    let id = state.next_id.fetch_add(1, Ordering::Relaxed);

    let todo = Todo {
        id,
        title: payload.title,
        completed: payload.completed,
    };

    state.todos.write().await.insert(id, todo.clone());
    // 201 Created — REST convention for successful resource creation.
    Ok((StatusCode::CREATED, Json(todo)))
}

/// FULL UPDATE — PUT /todos/:id
///
/// Replaces all fields of an existing todo. Returns 404 if not found.
/// All fields in the request body are required.
#[utoipa::path(
    put,
    path = "/todos/{id}",
    params(
        ("id" = u64, Path, description = "Todo ID")
    ),
    request_body = UpdateTodo,
    responses(
        (status = 200, description = "Todo fully replaced", body = Todo),
        (status = 404, description = "Todo not found")
    ),
    tag = "Todos"
)]
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let mut todos = state.todos.write().await;
    let todo = todos
        .get_mut(&id)
        .ok_or_else(|| AppError::NotFound(format!("todo {id} not found")))?;

    // Full replacement — all fields are overwritten.
    todo.title = payload.title;
    todo.completed = payload.completed;

    Ok(Json(todo.clone()))
}

/// PARTIAL UPDATE — PATCH /todos/:id
///
/// Updates only the provided fields. Omitted fields keep their current
/// values. Returns 404 if not found.
///
/// This is the key difference from PUT:
/// - PUT = "here is the complete new state" (all fields required)
/// - PATCH = "update only these fields" (all fields optional)
#[utoipa::path(
    patch,
    path = "/todos/{id}",
    params(
        ("id" = u64, Path, description = "Todo ID")
    ),
    request_body = PatchTodo,
    responses(
        (status = 200, description = "Todo partially updated", body = Todo),
        (status = 404, description = "Todo not found")
    ),
    tag = "Todos"
)]
pub async fn patch_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<PatchTodo>,
) -> Result<Json<Todo>, AppError> {
    let mut todos = state.todos.write().await;
    let todo = todos
        .get_mut(&id)
        .ok_or_else(|| AppError::NotFound(format!("todo {id} not found")))?;

    // Only update fields that were provided.
    // This is the in-memory equivalent of SQL's COALESCE pattern.
    if let Some(title) = payload.title {
        todo.title = title;
    }
    if let Some(completed) = payload.completed {
        todo.completed = completed;
    }

    Ok(Json(todo.clone()))
}

/// DELETE — DELETE /todos/:id
///
/// Removes a todo. Returns 204 No Content on success, 404 if not found.
#[utoipa::path(
    delete,
    path = "/todos/{id}",
    params(
        ("id" = u64, Path, description = "Todo ID")
    ),
    responses(
        (status = 204, description = "Todo deleted"),
        (status = 404, description = "Todo not found")
    ),
    tag = "Todos"
)]
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, AppError> {
    let mut todos = state.todos.write().await;
    todos
        .remove(&id)
        .ok_or_else(|| AppError::NotFound(format!("todo {id} not found")))?;
    // 204 No Content — REST convention for successful deletion.
    // No response body is sent.
    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

/// OpenAPI documentation definition.
///
/// Lists all handler functions under `paths` and all request/response
/// types under `schemas`. utoipa generates the full OpenAPI 3.1 spec
/// at compile time from these annotations.
#[derive(OpenApi)]
#[openapi(
    paths(list_todos, get_todo, create_todo, update_todo, patch_todo, delete_todo),
    components(schemas(Todo, CreateTodo, UpdateTodo, PatchTodo)),
    tags(
        (name = "Todos", description = "Todo CRUD operations")
    ),
    info(
        title = "Todo API",
        version = "0.1.0",
        description = "In-memory Todo CRUD API built with axum"
    )
)]
struct ApiDoc;

/// Builds the axum router with all todo routes, CORS middleware, and Swagger UI.
///
/// CORS (Cross-Origin Resource Sharing) is required so a frontend dev server
/// running on a different port can call our API on `http://localhost:3000`.
/// Without it, the browser blocks cross-origin requests.
///
/// Swagger UI is served at `/swagger-ui` — browse and test endpoints interactively.
pub fn create_router(state: AppState) -> Router {
    // CorsLayer::permissive() allows all origins, methods, and headers.
    // In production, restrict to specific origins:
    //   CorsLayer::new().allow_origin("https://myapp.com".parse().unwrap())
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // "/todos" handles GET (list) and POST (create).
        .route("/todos", get(list_todos).post(create_todo))
        // "/todos/:id" handles GET (single), PUT, PATCH, and DELETE.
        .route(
            "/todos/{id}",
            get(get_todo)
                .put(update_todo)
                .patch(patch_todo)
                .delete(delete_todo),
        )
        // Swagger UI — interactive API docs at /swagger-ui
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .with_state(state)
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the tracing subscriber so `info!`, `error!`, etc. produce output.
    // In production, replace with a more advanced setup (e.g. tracing-subscriber
    // with JSON formatting, OpenTelemetry layers, etc.).
    tracing_subscriber::fmt::init();

    let state = AppState::new();
    let app = create_router(state);

    let addr = "0.0.0.0:3000";
    info!("Listening on http://{addr}");
    info!("Swagger UI: http://{addr}/swagger-ui");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    /// Helper — creates a test server with empty state.
    fn test_server() -> TestServer {
        let state = AppState::new();
        let app = create_router(state);
        TestServer::new(app)
    }

    /// Helper — creates a todo via POST and returns the response body.
    async fn create_test_todo(server: &TestServer, title: &str) -> Todo {
        server
            .post("/todos")
            .json(&serde_json::json!({ "title": title }))
            .await
            .json::<Todo>()
    }

    #[tokio::test]
    async fn test_create_todo() {
        let server = test_server();

        let res = server
            .post("/todos")
            .json(&serde_json::json!({ "title": "Buy milk" }))
            .await;

        res.assert_status(StatusCode::CREATED);
        let todo: Todo = res.json();
        assert_eq!(todo.title, "Buy milk");
        assert!(!todo.completed);
        assert_eq!(todo.id, 1);
    }

    #[tokio::test]
    async fn test_create_todo_empty_title_returns_400() {
        let server = test_server();

        let res = server
            .post("/todos")
            .json(&serde_json::json!({ "title": "  " }))
            .await;

        res.assert_status(StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_list_todos() {
        let server = test_server();
        create_test_todo(&server, "First").await;
        create_test_todo(&server, "Second").await;

        let res = server.get("/todos").await;
        res.assert_status_ok();
        let todos: Vec<Todo> = res.json();
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].title, "First");
        assert_eq!(todos[1].title, "Second");
    }

    #[tokio::test]
    async fn test_list_todos_filter_completed() {
        let server = test_server();
        create_test_todo(&server, "Done task").await;
        create_test_todo(&server, "Pending task").await;

        // Mark first todo as completed via PUT
        server
            .put("/todos/1")
            .json(&serde_json::json!({
                "title": "Done task",
                "completed": true
            }))
            .await;

        // Filter completed=true
        let res = server.get("/todos?completed=true").await;
        let todos: Vec<Todo> = res.json();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, "Done task");

        // Filter completed=false
        let res = server.get("/todos?completed=false").await;
        let todos: Vec<Todo> = res.json();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, "Pending task");
    }

    #[tokio::test]
    async fn test_get_todo() {
        let server = test_server();
        create_test_todo(&server, "Read docs").await;

        let res = server.get("/todos/1").await;
        res.assert_status_ok();
        let todo: Todo = res.json();
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "Read docs");
    }

    #[tokio::test]
    async fn test_get_todo_not_found() {
        let server = test_server();

        let res = server.get("/todos/999").await;
        res.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_update_todo_put() {
        let server = test_server();
        create_test_todo(&server, "Original").await;

        let res = server
            .put("/todos/1")
            .json(&serde_json::json!({
                "title": "Updated",
                "completed": true
            }))
            .await;

        res.assert_status_ok();
        let todo: Todo = res.json();
        assert_eq!(todo.title, "Updated");
        assert!(todo.completed);
    }

    #[tokio::test]
    async fn test_update_todo_put_not_found() {
        let server = test_server();

        let res = server
            .put("/todos/999")
            .json(&serde_json::json!({
                "title": "Nope",
                "completed": false
            }))
            .await;

        res.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_patch_todo_partial_update() {
        let server = test_server();
        create_test_todo(&server, "Original title").await;

        // Patch only completed — title should remain unchanged.
        let res = server
            .patch("/todos/1")
            .json(&serde_json::json!({ "completed": true }))
            .await;

        res.assert_status_ok();
        let todo: Todo = res.json();
        assert_eq!(todo.title, "Original title"); // unchanged
        assert!(todo.completed); // updated

        // Patch only title — completed should remain true.
        let res = server
            .patch("/todos/1")
            .json(&serde_json::json!({ "title": "New title" }))
            .await;

        let todo: Todo = res.json();
        assert_eq!(todo.title, "New title"); // updated
        assert!(todo.completed); // unchanged
    }

    #[tokio::test]
    async fn test_patch_todo_not_found() {
        let server = test_server();

        let res = server
            .patch("/todos/999")
            .json(&serde_json::json!({ "completed": true }))
            .await;

        res.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_todo() {
        let server = test_server();
        create_test_todo(&server, "To delete").await;

        let res = server.delete("/todos/1").await;
        res.assert_status(StatusCode::NO_CONTENT);

        // Verify it's gone
        let res = server.get("/todos/1").await;
        res.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_todo_not_found() {
        let server = test_server();

        let res = server.delete("/todos/999").await;
        res.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_auto_increment_ids() {
        let server = test_server();
        let t1 = create_test_todo(&server, "First").await;
        let t2 = create_test_todo(&server, "Second").await;
        let t3 = create_test_todo(&server, "Third").await;

        assert_eq!(t1.id, 1);
        assert_eq!(t2.id, 2);
        assert_eq!(t3.id, 3);
    }
}
