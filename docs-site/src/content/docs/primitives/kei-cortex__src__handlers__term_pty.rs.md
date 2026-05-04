---
title: term_pty.rs
path: kei-cortex/src/handlers/term_pty.rs
dna_hash: sha256:6b26232c940521d6
language: rust
size_loc: 140
generated: by-keidocs
---

# kei-cortex/src/handlers/term_pty.rs

PTY lifecycle for the `/term` WebSocket handler.

Owns the spawned shell child + its reader/writer halves and tears them
down deterministically on `Drop`. Wave 44b fixes:

- F-MED-2: `child.kill()` is now followed by `child.wait()` so we don't
leak a zombie per disconnected session.
- MISS-2: the PTY reader runs on `tokio::task::spawn_blocking` (instead
of a bare `std::thread::spawn`) and watches a shared `AtomicBool`
cancellation flag, so daemon shutdown / WS disconnect cleanly stops
the reader instead of leaking a thread per session.

## Public API

- PTY initial size — sane default; client resize ack lands in v0.41.
- Read chunk size from PTY stdout. Anything larger fragments without
- `pub struct PtyBag` — Bag of PTY handles + bookkeeping we keep alive for the connection
- `pub fn spawn_pty` — Spawn `$SHELL` (or `/bin/sh` fallback) in a PTY anchored at `cwd` and
- SECURITY: drop ALL inherited env so daemon secrets (KEI_AUTH_KEY,
- Forward PTY stdout bytes to the WS sender via a bounded channel. Reads

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: portable_pty, std, tokio

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
