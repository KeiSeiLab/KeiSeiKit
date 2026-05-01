# Wave 44b — orchestrator integration notes

Disjoint scope of this fork: `handlers/tool_apply.rs` + `handlers/term.rs`
plus their newly-extracted Constructor-Pattern siblings. Wave 44a is
expected to land in parallel with `tool/atomic_io.rs` extraction; do NOT
merge the two without reading the conflicts below.

## Files in this fork

- **NEW** `handlers/tool_apply_atomic.rs` (173 LOC) — symlink-safe
  atomic write (`O_NOFOLLOW` openat + post-rename canonical re-check).
- **NEW** `handlers/term_pty.rs` (123 LOC) — PtyBag struct + Drop +
  spawn_pty + reader (tokio::spawn_blocking + AtomicBool cancel).
- **NEW** `handlers/tool_apply_symlink_test.rs` (100 LOC) —
  symlink-escape regression suite (4 tests).
- **NEW** `handlers/tool_apply_write_test.rs` (89 LOC) — extracted
  write-tool branch tests (5 tests; was inline in tool_apply_test.rs).
- **MODIFIED** `handlers/tool_apply.rs` (200 LOC, was 199) — switched
  to `atomic_write_nofollow`, returns `Resolved { path, root_canon }`
  from `resolve_under_root`. `atomic_write` removed.
- **MODIFIED** `handlers/term.rs` (157 LOC, was 178) — added
  `validate_origin`, switched `Message::Text(lossy)` →
  `Message::Binary(bytes)`, delegated PTY lifecycle to `term_pty.rs`,
  removed in-place `child.kill()` cleanup (now in PtyBag::Drop).
- **MODIFIED** `handlers/term_test.rs` — added Origin validation
  tests (6 cases) + tokio-runtime PTY drop smoke test.
- **MODIFIED** `handlers/tool_apply_test.rs` (167 LOC, was 224) —
  helpers exposed as `pub(super)` so sibling test cubes can reuse;
  write-branch + symlink tests extracted to siblings.
- **MODIFIED** `handlers/tool_apply_INTEGRATION.md` — F-CRIT-4 note.
- **MODIFIED** `handlers/mod.rs` — `mod tool_apply_atomic;` and
  `mod term_pty;` (both private — only siblings need them).
- **MODIFIED** `kei-cortex/Cargo.toml` — `nix = "0.29"` (default
  features off, `fs` feature on).
- **MODIFIED** `_primitives/_rust/Cargo.toml` (workspace) — `nix`
  workspace dep declaration.

## Conflict with wave44a (`tool/atomic_io.rs` extraction)

Wave 44a extracts the existing `tool::write::atomic_write` into a
`tool/atomic_io.rs` cube. Wave 44b adds `atomic_write_nofollow` LOCAL
to `handlers/tool_apply_atomic.rs` because the symlink-safe version
needs:

- `nix` crate access
- `OwnedFd` + raw-fd plumbing
- post-rename canonical re-check against `AppError::Forbidden`

These deps don't belong in the simple `tool::write::atomic_write`.

**Recommended merge action:** keep both. Wave 44a's `tool/atomic_io.rs`
serves the agentic loop's read/edit/write tools. Wave 44b's
`handlers/tool_apply_atomic.rs::atomic_write_nofollow` serves the
trusted `/tool/apply` HTTP endpoint. After both land, the orchestrator
may consider relocating Wave 44b's function to
`tool/atomic_io.rs::atomic_write_nofollow` and changing the
`super::tool_apply_atomic::` import in `tool_apply.rs` to
`crate::tool::atomic_io::atomic_write_nofollow`. The post-write
canonical re-check (which is what makes 44b distinct) should stay
exposed as a separate helper or kept inline in the handler if the
orchestrator wants `tool/atomic_io.rs` to remain agent-loop-only.

## Ledger of behavioural changes (for QA)

1. `/tool/apply` now refuses overwrites that would canonicalize
   outside `project_root` even if the resolution-time check passed.
2. `/tool/apply` writes are mode `0o600` (was implementation-defined
   default of `tokio::fs::write`).
3. `/term` WS upgrade now requires an exact `Origin` header match
   against `cors_origin`. This breaks any current cortex-ui code that
   used a different origin (e.g. `localhost:5173` vs the configured
   `cors_origin`). Fix is to align the dev server origin with what
   the daemon was launched with via `--cors-origin`.
4. `/term` WS frames are now BINARY. xterm.js handles
   `term.write(Uint8Array)` natively; if the UI ever decoded
   `Message::Text` server-side and concatenated, that path needs
   updating to consume `event.data instanceof ArrayBuffer`.
5. `/term` no longer leaks zombie shell processes per disconnected
   session.
6. `/term` no longer leaks an OS thread per session (reader is now
   a `tokio::spawn_blocking` task with cooperative cancellation).

## Testing

- `cargo test -p kei-cortex --lib` should run all tool_apply/term
  tests including the new symlink + Origin + PTY-drop suites.
- Symlink tests use `std::os::unix::fs::symlink` — Unix-only. They
  will not compile on Windows; the `kei-cortex` crate has no Windows
  CI runners today, so this is acceptable.
- The PTY-drop smoke test uses `tokio::time::sleep` for 50 ms. It is
  not a strict timing test — it's a "nothing panics on Drop" smoke.
