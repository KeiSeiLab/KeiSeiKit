---
title: main.rs
path: kei-scheduler/src/main.rs
dna_hash: sha256:7b3ef4fa475ce217
language: rust
size_loc: 141
generated: by-keidocs
---

# kei-scheduler/src/main.rs

kei-scheduler CLI — schedule / cancel / list-due / mark-run / tick.

Exit-code contract:
- 0 — success
- 1 — IO / storage / usage
- 2 — validation (bad trigger kind / spec / unknown id)

## Public API

- Insert a new scheduled task.
- Cancel a task by id.
- Print due tasks as a JSON array (reads `now = Utc::now`).
- Record a run's exit code and advance next_run_at.
- Convenience: `list-due` for the current wall clock.
- Print one task as JSON.

## Related

- parent: `kei-scheduler/Cargo.toml`
- imports: chrono, clap, kei_scheduler, std

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
