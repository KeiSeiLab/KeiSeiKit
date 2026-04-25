# STACK — Rust CLI / tooling

Cargo workspace. Default language — no language justification needed.

**Layout:**
- Workspace root `Cargo.toml` declares `members = [...]`; one crate per cube.
- Binaries under `<crate>/src/bin/*.rs`; library root `<crate>/src/lib.rs`.
- Integration tests in `<crate>/tests/*.rs`; unit tests inline with `#[cfg(test)]`.

**Hard invariants:**
- File > 200 LOC → split (Constructor Pattern). Function > 30 LOC → split.
- `clippy::pedantic` in CI; warnings = errors on `main`.
- `thiserror` for library error enums, `anyhow::Result` for binaries only. Never `Box<dyn Error>` in new code.
- NO `.unwrap()` / `.expect()` in prod paths. Allowed in tests and one-shot scripts flagged `// SCRIPT`.
- Benchmarks live under `benches/` with `cargo bench` (Criterion) and the documented number is ALWAYS from `cargo test --release` / `cargo bench` — never debug timings.

**CI gate:**
```
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --release
```

**Pre-commit:** `cargo fmt && cargo clippy --fix --allow-dirty && cargo test`.

**Forbidden:** `Rc<RefCell<...>>` in hot paths (use `&mut` or `Arc<Mutex<_>>`); `unsafe` without a `// SAFETY:` comment explaining the invariant; panic-on-parse in library crates.
