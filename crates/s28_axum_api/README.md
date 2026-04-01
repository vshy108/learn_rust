# s28_axum_api — Full CRUD REST API with Axum

A complete REST API example using [axum](https://docs.rs/axum) with an
in-memory Todo store. Demonstrates all HTTP methods, centralized error
handling, CORS, and includes React+TypeScript calling examples as doc
comments on each handler.

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

## React + TypeScript Setup

Each handler in `src/main.rs` includes a complete React+TypeScript example
as a doc comment. To try them against the running API:

### 1. Create a React app

```bash
npm create vite@latest todo-frontend -- --template react-ts
cd todo-frontend
npm install
```

### 2. Start the dev server

```bash
npm run dev
# Runs on http://localhost:5173
```

### 3. CORS is already configured

The Rust API includes `CorsLayer` middleware, so the React dev server
on `localhost:5173` can call `localhost:3000` without a proxy.

### 4. Copy the API functions

See the doc comments on each handler in `src/main.rs` for ready-to-use
TypeScript `fetch()` functions (e.g., `fetchTodos`, `createTodo`,
`updateTodo`, `patchTodo`, `deleteTodo`).

### 5. Example: minimal App.tsx (React 19)

Uses `useOptimistic`, `useActionState`, `useFormStatus`, and form `action`.

```tsx
import { useState, useOptimistic, useTransition, useActionState } from "react";
import { useFormStatus } from "react-dom";

interface Todo {
  id: number;
  title: string;
  completed: boolean;
}

const API = "http://localhost:3000";

// --- API functions ---

async function fetchTodos(): Promise<Todo[]> {
  const res = await fetch(`${API}/todos`);
  return res.json();
}

async function apiCreateTodo(title: string): Promise<Todo> {
  const res = await fetch(`${API}/todos`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ title }),
  });
  if (!res.ok) throw new Error("Failed to create todo");
  return res.json();
}

async function apiPatchTodo(
  id: number,
  data: { completed?: boolean }
): Promise<Todo> {
  const res = await fetch(`${API}/todos/${id}`, {
    method: "PATCH",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(data),
  });
  return res.json();
}

async function apiDeleteTodo(id: number): Promise<void> {
  await fetch(`${API}/todos/${id}`, { method: "DELETE" });
}

// --- Components ---

// useFormStatus reads the pending state of the parent <form>.
// Must be rendered as a child of <form>, not in the same component
// that defines the form action.
function SubmitButton() {
  const { pending } = useFormStatus();
  return (
    <button type="submit" disabled={pending}>
      {pending ? "Adding..." : "Add"}
    </button>
  );
}

// useActionState manages form action + state in one hook.
// Returns [state, formAction, isPending]:
//   state      = return value of the async action (error message or null)
//   formAction = pass to <form action={...}> (replaces onSubmit)
//   isPending  = true while the action is running
function CreateForm({ onCreated }: { onCreated: (t: Todo) => void }) {
  const [error, formAction] = useActionState(
    async (_prev: string | null, formData: FormData) => {
      const title = formData.get("title") as string;
      if (!title.trim()) return "Title is required";
      try {
        const todo = await apiCreateTodo(title);
        onCreated(todo);
        return null;
      } catch (e) {
        return (e as Error).message;
      }
    },
    null
  );
  return (
    <form action={formAction}>
      <input name="title" placeholder="What needs to be done?" />
      <SubmitButton />
      {error && <p style={{ color: "red" }}>{error}</p>}
    </form>
  );
}

function App() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [loaded, setLoaded] = useState(false);

  // Initial load (still useEffect for simplicity —
  // see handler doc comments for use() + Suspense approach)
  if (!loaded) {
    fetchTodos().then((t) => {
      setTodos(t);
      setLoaded(true);
    });
  }

  // useOptimistic gives an "optimistic" copy of todos that updates
  // instantly, then auto-resets when the real `todos` state changes.
  const [optimisticTodos, addOptimistic] = useOptimistic(
    todos,
    (state: Todo[], action: { type: "toggle"; todo: Todo } | { type: "delete"; id: number }) => {
      if (action.type === "toggle") {
        return state.map((t) =>
          t.id === action.todo.id ? { ...t, completed: !t.completed } : t
        );
      }
      return state.filter((t) => t.id !== action.id);
    }
  );

  const [, startTransition] = useTransition();

  const handleToggle = (todo: Todo) => {
    startTransition(async () => {
      addOptimistic({ type: "toggle", todo });
      const updated = await apiPatchTodo(todo.id, {
        completed: !todo.completed,
      });
      setTodos((prev) =>
        prev.map((t) => (t.id === updated.id ? updated : t))
      );
    });
  };

  const handleDelete = (id: number) => {
    startTransition(async () => {
      addOptimistic({ type: "delete", id });
      await apiDeleteTodo(id);
      setTodos((prev) => prev.filter((t) => t.id !== id));
    });
  };

  return (
    <div>
      <h1>Todos</h1>
      <CreateForm
        onCreated={(todo) => setTodos((prev) => [...prev, todo])}
      />
      <ul>
        {optimisticTodos.map((t) => (
          <li key={t.id}>
            <button onClick={() => handleToggle(t)}>
              {t.completed ? "[x]" : "[ ]"}
            </button>
            {t.title}
            <button onClick={() => handleDelete(t.id)}>Delete</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
```

### React 19 hooks used

| Hook | Where | Purpose |
|---|---|---|
| `useActionState` | CreateForm | Async form action + error state in one hook |
| `useFormStatus` | SubmitButton | Shows "Adding..." while form is pending |
| `useOptimistic` | App | Instant toggle/delete before server responds |
| `useTransition` | App | Wraps async work so UI stays responsive |

## Architecture Notes

- **State**: `Arc<RwLock<HashMap<u64, Todo>>>` — concurrent reads,
  exclusive writes, no database needed.
- **Error handling**: `AppError` enum with `IntoResponse` impl maps each
  variant to an HTTP status code. Handlers return `Result<T, AppError>`.
- **CORS**: `CorsLayer` allows all origins for development. In production,
  restrict to your frontend domain.
- **PUT vs PATCH**: PUT requires all fields (full replacement). PATCH
  accepts optional fields (partial update). Both are demonstrated.
