---
title: term.rs
path: kei-cortex/src/handlers/term.rs
dna_hash: sha256:fda10fa29af560d2
language: rust
size_loc: 157
generated: by-keidocs
---

# kei-cortex/src/handlers/term.rs

`WS /api/v1/cortex/term?cwd=<path>` — bidirectional terminal pipe.

Spawns the user's `$SHELL` in a PTY rooted at `cwd`. WS frames in feed
the PTY stdin; PTY stdout is forwarded as WS BINARY frames (xterm.js
`term.write(Uint8Array)` consumes them losslessly — UTF-8 multi-byte
sequences and ANSI escapes split across reads no longer get mangled).

Path safety: `cwd` MUST resolve inside `state.config().project_root` —
mirrors the `/fs/list` chroot. Symlinks are not followed.

Wave 44b fixes:
- F-HIGH-2: Origin header check before WS upgrade (CSWSH defence).
- F-MED-2: PTY child kill+wait via `PtyBag::Drop`.
- F-MED-5: `Message::Binary` instead of UTF-8 lossy text frames.
- MISS-2: PTY reader is `tokio::spawn_blocking` + cancel token, no
leaked OS threads on disconnect.

## Public API

- WebSocket upgrade entry point. Validates `Origin` and `cwd` BEFORE
- Reject WebSocket upgrades whose `Origin` header is missing, literally
- Resolve the requested cwd; defaults to project root when unset. Rejects
- Per-connection driver: spawn shell + bridge bytes both ways. The
- Bidirectional bridge: PTY-stdout → WS-out (binary frames) and WS-in →

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, futures, serde, std, tokio

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
