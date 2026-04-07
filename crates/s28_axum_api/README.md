# s28_axum_api — Full CRUD REST API with Axum

A complete REST API example using [axum](https://docs.rs/axum) with an
in-memory Todo store. Demonstrates all HTTP methods, centralized error
handling, and CORS.

## Prerequisites

- Rust toolchain (stable)
- No database required — uses an in-memory `HashMap`

## Build & Run

```bash
# Build
cargo build -p s28_axum_api

# Run (starts on http://localhost:3000)
cargo run -p s28_axum_api
```

## Run Tests

```bash
cargo test -p s28_axum_api
```

## API Reference

| Method  | Path          | Request Body          | Response        | Status         |
|---------|---------------|-----------------------|-----------------|----------------|
| GET     | /todos        | —                     | `Todo[]`        | 200 OK         |
| GET     | /todos?completed=true | —               | `Todo[]`        | 200 OK         |
| GET     | /todos/:id    | —                     | `Todo`          | 200 / 404      |
| POST    | /todos        | `{ title, completed? }` | `Todo`        | 201 Created    |
| PUT     | /todos/:id    | `{ title, completed }` | `Todo`         | 200 / 404      |
| PATCH   | /todos/:id    | `{ title?, completed? }` | `Todo`       | 200 / 404      |
| DELETE  | /todos/:id    | —                     | —               | 204 / 404      |

### Todo JSON Shape

```json
{
  "id": 1,
  "title": "Buy milk",
  "completed": false
}
```

### Error JSON Shape

```json
{
  "error": "todo 999 not found"
}
```

## curl Examples

### Create a todo

```bash
curl -s -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Buy milk"}' | jq
```

```json
{
  "id": 1,
  "title": "Buy milk",
  "completed": false
}
```

### List all todos

```bash
curl -s http://localhost:3000/todos | jq
```

```json
[
  { "id": 1, "title": "Buy milk", "completed": false }
]
```

### List with filter

```bash
curl -s "http://localhost:3000/todos?completed=false" | jq
```

### Get single todo

```bash
curl -s http://localhost:3000/todos/1 | jq
```

### Get non-existent todo (404)

```bash
curl -s -w "\nHTTP %{http_code}\n" http://localhost:3000/todos/999
```

```
{"error":"todo 999 not found"}
HTTP 404
```

### Full update (PUT)

```bash
curl -s -X PUT http://localhost:3000/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"title": "Buy almond milk", "completed": true}' | jq
```

### Partial update (PATCH) — toggle completed only

```bash
curl -s -X PATCH http://localhost:3000/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"completed": true}' | jq
```

### Delete a todo

```bash
curl -s -X DELETE http://localhost:3000/todos/1 -w "\nHTTP %{http_code}\n"
```

```
HTTP 204
```

## Architecture Notes

- **State**: `Arc<RwLock<HashMap<u64, Todo>>>` — concurrent reads,
  exclusive writes, no database needed. In-memory storage is intentional
  for learning — data resets on restart.
- **Error handling**: `AppError` enum with `IntoResponse` impl maps each
  variant to an HTTP status code. Handlers return `Result<T, AppError>`.
- **PUT vs PATCH**: PUT requires all fields (full replacement). PATCH
  accepts optional fields (partial update). Both are demonstrated.

> **CORS**: `CorsLayer` allows all origins for development. In production,
> restrict to your frontend domain to prevent unauthorized cross-origin requests.
