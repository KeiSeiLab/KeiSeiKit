---
title: main.rs
path: kei-tty/src/main.rs
dna_hash: sha256:b997b97c725ad889
language: rust
size_loc: 146
generated: by-keidocs
---

# kei-tty/src/main.rs

`kei-tty` binary entry point.

Two subcommands:

* `chat`  — interactive ratatui TUI (default mode for power users).
* `send`  — one-shot: read message from `--message` or stdin, stream
response to stdout, exit. Pipe-friendly.

Daemon URL is read from `KEI_DAEMON_URL` (default
`http://127.0.0.1:9797`). Bearer token is read from
`~/.keisei/cortex.token` (created by `keisei daemon init`); on first run
the file may not yet exist — we surface a clear error rather than
crashing.

## Public API

- Default cortex daemon URL when `KEI_DAEMON_URL` is unset.
- Default user_id for the cortex `pet` route — `keisei` daemon creates
- Interactive TUI session (default mode).
- One-shot: send a single message and stream the reply to stdout.
- Message body. If omitted, read from stdin.
- Read the bearer token from `~/.keisei/cortex.token`. The keisei daemon
- Enter the TUI: alternate screen + raw mode, run the event loop, then
- One-shot mode: drains the SSE stream and prints token text to stdout.
- Resolve `--message` or stdin into a non-empty string.
- Stream-event renderer for `send` mode (one event → stdout write).

## Related

- parent: `kei-tty/Cargo.toml`
- imports: anyhow, clap, crossterm, kei_tty, ratatui, std

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
