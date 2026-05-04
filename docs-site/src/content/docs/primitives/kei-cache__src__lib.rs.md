---
title: lib.rs
path: kei-cache/src/lib.rs
dna_hash: sha256:319a5988ad522b97
language: rust
size_loc: 120
generated: by-keidocs
---

# kei-cache/src/lib.rs

kei-cache — deterministic caching primitive for pure atom invocations.

Entry point is [`wrap_with`]: given a cache [`rusqlite::Connection`], an
[`exec::AtomExecutor`], an atom id, JSON input, and a TTL, either
return the cached payload or invoke the executor, store the result,
and return it.

Key derivation lives in [`key`]. Storage lives in [`store`]. Invocation
on miss lives in [`exec`]. `lib.rs` only composes them — it owns no
persistent state.

## Public API

- Outcome of a [`wrap_with`] call.
- `pub trait Cache` — Cache trait — library-level API for downstream consumers.
- Fetch from the cache; `None` if absent or expired.
- Store `payload` under `key` with TTL (seconds).
- `pub struct SqliteCache` — SQLite-backed cache impl. Holds a borrowed [`Connection`].
- `pub fn wrap_with` — Top-level wrap: lookup → return on hit, invoke + store on miss.

## Related

- parent: `kei-cache/Cargo.toml`
- imports: anyhow, rusqlite, serde_json, std, tempfile

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
