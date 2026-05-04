---
title: runners.rs
path: kei-frustration-loop/src/runners.rs
dna_hash: sha256:ecc6292d2922dc53
language: rust
size_loc: 149
generated: by-keidocs
---

# kei-frustration-loop/src/runners.rs

Loop CLI runners — `bootstrap` / `nightly-scan` / `feedback` /
`auto-train` / `personalize`.

Constructor Pattern: each runner is a thin shim that adapts the parsed
args to a domain cube and prints a JSON record. No business logic
lives here; the cubes own corpus prep, classification, threshold check.

## Public API

- `pub fn run_bootstrap` — Run the install-time bootstrap and print a JSON `BootstrapReport`.
- `pub fn run_nightly` — Run the Phase-0 nightly scan and print a JSON `ScanReport`.
- `pub fn run_feedback` — Append one feedback row and surface the retrain recommendation.
- Build the `Feedback` struct from CLI fragments. Splits parse + struct
- `pub fn run_auto_train` — Trigger an `auto_train` run and print the resulting `TrainReport`.
- `pub fn run_personalize` — Inspect which firmware will be used for `--user`.
- `pub fn resolve_home` — Resolve `home` argument: explicit `--home` wins; falls back to `$HOME`.
- Print one JSON record to stdout (newline-terminated). Used by every loop
- Wall-clock now in Unix seconds. 0 if the system clock is broken.

## Related

- parent: `kei-frustration-loop/Cargo.toml`
- imports: anyhow, crate, frustration_matrix, serde, std

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
