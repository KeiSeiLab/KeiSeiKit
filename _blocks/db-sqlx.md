# DB — SQLx (Rust) patterns

Use when the project is Rust and needs a SQL-first (not ORM) query layer with compile-time checking. Pairs with `stack-rust-axum`, `stack-rust-cli`. [E4 — expert assessment]

**Core versions:** `sqlx = "0.8"` (current as of 2026-04) with features `runtime-tokio`, `tls-rustls`, and one of `postgres` / `sqlite` / `mysql`. Never mix `runtime-async-std` and `runtime-tokio` — they clash at link time. [UNVERIFIED: verify latest on crates.io before pinning]

**Compile-time checked queries:**
```rust
let row = sqlx::query!("SELECT id, name FROM users WHERE id = $1", user_id)
    .fetch_one(&pool).await?;
```
Requires either:
- `DATABASE_URL` env set during `cargo build` (live DB) — convenient in dev, brittle in CI.
- **Offline mode** (recommended for CI): `cargo sqlx prepare` commits `.sqlx/query-*.json` to the repo, then CI builds with `SQLX_OFFLINE=true` and no DB access.

**Connection pool:**
```rust
let pool = sqlx::postgres::PgPoolOptions::new()
    .max_connections(20)                  // tune to server max_connections / replica count
    .acquire_timeout(Duration::from_secs(3))
    .connect(&database_url).await?;
```
Single `PgPool` per process, `Arc`-cloned into handlers. Don't open per-request.

**Migrations:**
```rust
sqlx::migrate!("./migrations").run(&pool).await?;
```
Built-in runner reads `YYYYMMDDHHMMSS_<name>.sql` files. For richer UX (up/down, status, create scaffolding) use the `kei-migrate` primitive in this kit.

**Transactions:**
```rust
let mut tx = pool.begin().await?;
sqlx::query!("...").execute(&mut *tx).await?;
sqlx::query!("...").execute(&mut *tx).await?;
tx.commit().await?;                       // explicit; Drop = rollback
```

**Forbidden:** `sqlx::query` (non-macro) with untrusted input without `bind()` — that's string concat, i.e. SQL injection; `.unwrap()` on DB calls in prod paths; enabling both `runtime-tokio` and `runtime-async-std`; committing a live `DATABASE_URL` to `.env.example`.
