---
title: main.rs
path: kei-cleanup/src/main.rs
dna_hash: sha256:8ffe5bb526567598
language: rust
size_loc: 161
generated: by-keidocs
---

# kei-cleanup/src/main.rs

kei-cleanup CLI entrypoint — see CLEANUP-RUNTIME-SPEC.md §3.4.

v0.1 flags:
* positional PATH (default `.`) — workspace root
* `--json FILE`     — emit serialised CleanupReport
* `--quiet`         — suppress TTY pretty output
* `--fail-on LEVEL` — exit 1 if findings ≥ level (high|medium|low)
* `--only LIST`     — comma-separated scanner whitelist

## Public API

- Workspace root (default: current directory).
- Emit JSON report to FILE.
- Suppress TTY output.
- Exit 1 if findings ≥ this severity (high|medium|low).
- Comma-list of scanner names to run.
- Emit findings as predicate rows to a kei-registry SQLite DB.

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: anyhow, clap, kei_cleanup, std

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
