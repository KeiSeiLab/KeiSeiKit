---
title: lib.rs
path: kei-model/src/lib.rs
dna_hash: sha256:2c538ca73c725bcd
language: rust
size_loc: 31
generated: by-keidocs
---

# kei-model/src/lib.rs

kei-model — universal model registry + selector.

Replaces hardcoded `MODEL` / `DEFAULT_MODEL` constants in kei-cortex,
kei-router, and kei-spawn with a SSoT TOML catalog (`data/models.toml`)
plus role-default routing (`data/selectors.toml`). Pure compute, no
async, no network — siblings depend on this primitive, not the other way.

## Subcommands
* `list`       — filter catalog by provider/cap/status/role
* `resolve`    — pick cheapest active model for role+budget+caps
* `price`      — estimate micro-cent cost for a token budget
* `providers`  — list distinct providers with active/deprecated counts
* `fallback`   — walk fallback chain until None or cycle

## RULE 0.4 — Pricing
All seed pricing rows ship `status = "placeholder"` with rates of 0.
Real provider rates land in a follow-up verification commit. Callers
checking cost MUST consult `pricing.status` before quoting numbers.

## Related

- parent: `kei-model/Cargo.toml`

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
