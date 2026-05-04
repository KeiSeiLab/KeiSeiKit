---
title: main.rs
path: kei-tlog/src/main.rs
dna_hash: sha256:f64791aed2e43cd0
language: rust
size_loc: 229
generated: by-keidocs
---

# kei-tlog/src/main.rs

`kei-tlog` — atomar time-logger (RULE 0.17 enforcement primitive).

Three subcommands:
start <name>        — emit a `start` line to journal, print epoch on stdout
stop  <name>        — match the most recent `start` for `<name>` and emit `stop`+duration
wrap  <name> -- cmd — `start` → run `cmd` → `stop`. Exit code passes through.

Journal: `$KEI_TLOG_JOURNAL` (default `~/.claude/memory/time-metrics/tasks.jsonl`).

All output is JSONL; every line is one of:
{"kind":"start","name":..,"start_epoch":..,"ts":..}
{"kind":"stop","name":..,"start_epoch":..,"end_epoch":..,"duration_s":..,"exit":..,"ts":..}

Constructor Pattern: one file, ≤200 LOC, no dependencies beyond serde_json + std.

## Related

- parent: `kei-tlog/Cargo.toml`
- imports: std

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
