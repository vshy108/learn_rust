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
npm install axios
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
TypeScript Axios functions (e.g., `fetchTodos`, `createTodo`,
`updateTodo`, `patchTodo`, `deleteTodo`).

All examples use a shared `axios.create({ baseURL })` instance —
configure interceptors (auth, logging, retry) in one place.

### 5. Example: minimal App.tsx (React 19)

Uses `useOptimistic`, `useActionState`, `useTransition`, form `action`,
and Axios for HTTP calls. Follows the correct React 19 patterns:

- `useActionState` returns **3 values**: `[state, submitAction, isPending]`
- Form `action` takes a **wrapper function** that calls `addOptimistic()` first, then `submitAction(formData)`
- `isPending` from `useActionState` disables the button directly (no `useFormStatus` child needed here)
- Discriminated union action for `useOptimistic` reducer handles both toggle and delete

```tsx
import { useState, useOptimistic, useTransition, useActionState, useEffect } from "react";
import axios from "axios";

interface Todo {
  id: number;
  title: string;
  completed: boolean;
}

// Shared axios instance — baseURL configured once.
// Add interceptors here for auth headers, error logging, retry, etc.
const api = axios.create({ baseURL: "http://localhost:3000" });

// --- API functions ---
// Axios auto-parses JSON (res.data), auto-throws on non-2xx,
// and auto-sets Content-Type for object bodies.

async function fetchTodos(): Promise<Todo[]> {
  const { data } = await api.get<Todo[]>("/todos");
  return data;
}

async function apiCreateTodo(title: string): Promise<Todo> {
  const { data } = await api.post<Todo>("/todos", { title });
  return data;
}

async function apiPatchTodo(
  id: number,
  body: { completed?: boolean },
): Promise<Todo> {
  const { data } = await api.patch<Todo>(`/todos/${id}`, body);
  return data;
}

async function apiDeleteTodo(id: number): Promise<void> {
  await api.delete(`/todos/${id}`);
}

// --- Types ---

// Discriminated union keeps state machine transitions explicit.
// The `status` field narrows which `message` interpretation applies.
type SubmissionState =
  | { readonly status: "idle"; readonly message: string }
  | { readonly status: "success"; readonly message: string }
  | { readonly status: "error"; readonly message: string };

const initialSubmission: SubmissionState = {
  status: "idle",
  message: "Add a todo to see optimistic UI.",
};

// Discriminated union action for useOptimistic reducer.
// One reducer handles both toggle and delete — exhaustive switch ensures
// new action types cannot slip through unhandled.
type OptimisticAction =
  | { type: "add"; title: string }
  | { type: "toggle"; todo: Todo }
  | { type: "delete"; id: number };

// --- App ---

function App() {
  const [todos, setTodos] = useState<Todo[]>([]);

  useEffect(() => {
    fetchTodos().then(setTodos);
  }, []);

  // useOptimistic(realState, reducer) — optimistic layer auto-resets
  // when real `todos` state changes.
  const [optimisticTodos, addOptimistic] = useOptimistic(
    todos,
    (state: Todo[], action: OptimisticAction) => {
      switch (action.type) {
        case "add":
          return [{ id: 0, title: action.title, completed: false }, ...state];
        case "toggle":
          return state.map((t) =>
            t.id === action.todo.id ? { ...t, completed: !t.completed } : t,
          );
        case "delete":
          return state.filter((t) => t.id !== action.id);
      }
    },
  );

  // useActionState returns [state, submitAction, isPending]:
  //   state        = latest return value of the async action
  //   submitAction = call with FormData to trigger the action
  //   isPending    = true while the action is running
  const [submissionState, submitAction, isPending] = useActionState(
    async (
      _prev: SubmissionState,
      formData: FormData,
    ): Promise<SubmissionState> => {
      try {
        // Re-parse FormData here (don't trust the optimistic path).
        // This function is the authoritative save path.
        const title = formData.get("title") as string;
        if (!title.trim()) {
          return { status: "error", message: "Title is required." };
        }
        const todo = await apiCreateTodo(title);
        setTodos((prev) => [todo, ...prev]);
        return {
          status: "success",
          message: `Created "${todo.title}" (id: ${todo.id}).`,
        };
      } catch (e) {
        return {
          status: "error",
          message: e instanceof Error ? e.message : "Unknown error",
        };
      }
    },
    initialSubmission,
  );

  // Wrapper handler: optimistic update first, then delegate to submitAction.
  // The form `action` prop calls this, NOT `submitAction` directly.
  function handleCreateAction(formData: FormData) {
    const title = formData.get("title") as string;
    if (title?.trim()) {
      addOptimistic({ type: "add", title: title.trim() });
    }
    // submitAction owns the real lifecycle (pending flag, state updates).
    submitAction(formData);
  }

  const [, startTransition] = useTransition();

  const handleToggle = (todo: Todo) => {
    startTransition(async () => {
      addOptimistic({ type: "toggle", todo });
      const updated = await apiPatchTodo(todo.id, {
        completed: !todo.completed,
      });
      setTodos((prev) =>
        prev.map((t) => (t.id === updated.id ? updated : t)),
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

      {/* action={handleCreateAction} — NOT action={submitAction} */}
      <form action={handleCreateAction}>
        <input name="title" placeholder="What needs to be done?" />
        {/* isPending from useActionState — no useFormStatus child needed */}
        <button type="submit" disabled={isPending}>
          {isPending ? "Adding..." : "Add"}
        </button>
      </form>
      <p>{submissionState.message}</p>

      <ul>
        {optimisticTodos.map((t) => (
          <li key={t.id}>
            <button onClick={() => handleToggle(t)}>
              {t.completed ? "[x]" : "[ ]"}
            </button>
            {t.title}
            {t.id === 0 ? " (saving...)" : ""}
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
| `useActionState` | App (create form) | Returns `[state, submitAction, isPending]` — owns async form lifecycle |
| `useOptimistic` | App | Instant add/toggle/delete via discriminated union reducer |
| `useTransition` | App (toggle/delete) | Wraps async work so UI stays responsive |

### When to use `useFormStatus`

`useFormStatus` reads the pending state of the **parent** `<form>`. It's only
needed when the submit button lives in a **separate child component** that
cannot access `isPending` from `useActionState`. In this example the button
is in the same component, so `isPending` is used directly.

```tsx
// Only needed when SubmitButton is a separate component:
import { useFormStatus } from "react-dom";

function SubmitButton() {
  const { pending } = useFormStatus(); // reads parent <form>'s state
  return <button disabled={pending}>{pending ? "Adding..." : "Add"}</button>;
}
```

## Architecture Notes

- **State**: `Arc<RwLock<HashMap<u64, Todo>>>` — concurrent reads,
  exclusive writes, no database needed.
- **Error handling**: `AppError` enum with `IntoResponse` impl maps each
  variant to an HTTP status code. Handlers return `Result<T, AppError>`.
- **CORS**: `CorsLayer` allows all origins for development. In production,
  restrict to your frontend domain.
- **PUT vs PATCH**: PUT requires all fields (full replacement). PATCH
  accepts optional fields (partial update). Both are demonstrated.
