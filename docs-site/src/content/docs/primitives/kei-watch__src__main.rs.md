---
title: main.rs
path: kei-watch/src/main.rs
dna_hash: sha256:f2e4cc732f2ce1be
language: rust
size_loc: 136
generated: by-keidocs
---

# kei-watch/src/main.rs

kei-watch CLI — streams canonical FS events as JSON Lines.

Usage:
```text
kei-watch watch --path <DIR> [--recursive] [--timeout-ms <N>]
```

Each event is one JSON object per line, flushed per event:
`{"kind":"Modified","path":"/abs/path","from":null,"ts":1712345678}`.

Exits after `--timeout-ms` of no activity. Without the flag, runs
until killed (Ctrl-C).

## Public API

- Watch a path and emit JSON-line events to stdout.
- Path to watch (file or directory).
- Recurse into subdirectories.
- Exit after this many ms without activity. Omit → run forever.

## Related

- parent: `kei-watch/Cargo.toml`
- imports: clap, kei_watch, std

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
