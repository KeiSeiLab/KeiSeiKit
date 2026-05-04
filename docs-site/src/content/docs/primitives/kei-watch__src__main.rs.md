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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
