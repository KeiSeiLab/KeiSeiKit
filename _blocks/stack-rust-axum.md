# STACK — Rust HTTP server (axum + tokio + sqlx)

Default web stack — no language justification needed.

**Versions:** axum 0.7+, tokio 1.x (`rt-multi-thread`), sqlx 0.7+ (NOT diesel — async-first), tower 0.4+ for middleware.

**App shape:**
- `AppState` struct → `Arc<AppState>` → `Router::with_state(state)`. No globals.
- Handlers take `State<Arc<AppState>>`, extractors typed, return `Result<impl IntoResponse, AppError>`.
- `AppError` = single `thiserror` enum with `IntoResponse` impl → maps to HTTP status + JSON body.
- `#[tokio::main]` ONLY in the binary crate. Library crates never pin a runtime.

**Middleware stack (order matters):**
1. `TraceLayer` (tower-http) — request id + span
2. `CorsLayer` — explicit allow-list, never `Any` in prod
3. `TimeoutLayer` — hard cap per route
4. `CompressionLayer`
5. Auth middleware (custom) — short-circuits on 401

**Crypto:** Ed25519 for signing (`ed25519-dalek`); never roll your own. Secrets from env at startup, never in code.

**sqlx:** queries use `sqlx::query!` / `query_as!` macros (compile-time checked against live DB). Migrations under `migrations/` managed by `sqlx-cli`. NEVER string-concat SQL.

**Forbidden:** `unwrap()` in handler paths, `sqlx::query()` with runtime strings, blocking calls (`std::fs::read`) without `spawn_blocking`, `#[tokio::main]` in lib crates (caller chooses runtime).
