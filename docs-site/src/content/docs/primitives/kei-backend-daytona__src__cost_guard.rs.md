---
title: cost_guard.rs
path: kei-backend-daytona/src/cost_guard.rs
dna_hash: sha256:db08ea6be77e8f14
language: rust
size_loc: 117
generated: by-keidocs
---

# kei-backend-daytona/src/cost_guard.rs

Free-tier cost guard for the Daytona backend.

Daytona's free tier covers **2 concurrent sandboxes** with **30-min idle
hibernate**. Anything past that is paid. Before any `create_sandbox`
call, `pre_create_check` lists existing sandboxes and counts the ones
that consume quota (`Running | Hibernated | Stopped | Pending`). If the
count is at or above `cap`, the call is blocked with a structured
error — kei-cost-guardian will eventually consume this signal directly.

## Public API

- `pub const FREE_TIER_CAP` — Daytona free-tier concurrent-sandbox cap.
- Error returned when a creation would exceed the configured cap.
- Number of sandboxes currently consuming quota.
- Cap the call would have crossed.
- True if `state` consumes a quota slot on the Daytona side.
- Block a sandbox creation when `count(quota-consuming sandboxes) >= cap`.

## Related

- parent: `kei-backend-daytona/Cargo.toml`
- imports: crate, std

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
