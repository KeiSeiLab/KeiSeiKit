---
title: list.rs
path: keisei/src/list.rs
dna_hash: sha256:555f7f03a6ae35ac
language: rust
size_loc: 65
generated: by-keidocs
---

# keisei/src/list.rs

`keisei list-adapters` — read-only dump of every registered adapter
and its detection state on this host.

Constructor Pattern: single responsibility — render a tabular view.
No state mutation, no config touches.

v0.21: adapters can advertise multiple scopes — we show the user-scope
config path by default (the fan-out target for `keisei mount`), plus a
trailing `scopes=...` column so an operator can see which adapters can
also take a `--scope=project` attach.

## Related

- parent: `keisei/Cargo.toml`
- imports: crate

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
