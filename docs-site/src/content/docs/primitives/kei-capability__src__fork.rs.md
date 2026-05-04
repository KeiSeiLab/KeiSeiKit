---
title: fork.rs
path: kei-capability/src/fork.rs
dna_hash: sha256:f06f2427ff6a15c7
language: rust
size_loc: 200
generated: by-keidocs
---

# kei-capability/src/fork.rs

`kei-capability fork <source> --as <new-name>` — copy+rewrite a capability.

Reads `_capabilities/<src-cat>/<src-slug>/{capability.toml, text.md}`,
validates the new `<cat>::<slug>` name, creates the target directory,
writes a rewritten `capability.toml` (new name + `[lineage]` block),
and copies `text.md` byte-identical.

Constructor Pattern: one cube = fork copy+rewrite. No subcommand dispatch.

## Public API

- Summary returned to the CLI / tests after a successful fork.
- `pub fn run_fork` — Run the fork operation against a kit root.
- Split `<cat>::<slug>` and validate both halves through the shared regex.
- Parse source capability.toml, rewrite `[capability].name`, insert a
- `pub fn current_iso_utc` — Current UTC time as `YYYY-MM-DDTHH:MM:SSZ`. No chrono dep — minimal

## Related

- parent: `kei-capability/Cargo.toml`
- imports: anyhow, kei_agent_runtime, std, toml

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
