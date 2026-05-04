---
title: resolve.rs
path: kei-pipe/src/resolve.rs
dna_hash: sha256:803c5b7364d2664d
language: rust
size_loc: 128
generated: by-keidocs
---

# kei-pipe/src/resolve.rs

`$step-id.path.to.field` resolver.

A reference is a string of the form `$<step-id>[.<segment>]*` where a
segment is either a dotted key (`foo`) or a numeric index (`0`). The
leading segment after the step id is matched against the step's
returned `{"atom":..., "result":...}` envelope the same way a caller
would type it — including an explicit `result.` prefix.

A string that is `$step.path.to.field` verbatim is substituted by the
referenced value (which may itself be a JSON object, array, number).
Strings that merely contain a `$` somewhere else are left alone.

## Public API

- `pub fn resolve_input` — Walk `input` recursively, replacing every `$step.path` string with the

## Related

- parent: `kei-pipe/Cargo.toml`
- imports: serde_json, std

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
