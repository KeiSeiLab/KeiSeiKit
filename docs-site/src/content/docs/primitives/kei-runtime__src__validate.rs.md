---
title: validate.rs
path: kei-runtime/src/validate.rs
dna_hash: sha256:77168f0f5e86ce57
language: rust
size_loc: 165
generated: by-keidocs
---

# kei-runtime/src/validate.rs

JSON Schema draft-07 validation wrappers.

Thin façade over the `jsonschema` crate (v0.18). Reads schema from disk
per call. Returns a single, readable error message.

SSRF / IMDS hardening:
- `default-features = false` on `jsonschema` — no `resolve-http` feature.
- Custom `LocalFileResolver` replaces the default. It rejects any URL
whose scheme isn't `file://` and any path outside the schema's own
directory (anchored at the schema file's parent).

## Public API

- `pub fn validate_input` — Validate `input` against JSON Schema at `schema_path`.
- `pub fn validate_output` — Validate `output` against JSON Schema at `schema_path`.
- `$ref` resolver that rejects every scheme except `file://`, AND rejects
- Walk up from root to find workspace's `_schemas/fragments/`. Returns

## Related

- parent: `kei-runtime/Cargo.toml`
- imports: jsonschema, serde_json, std, url

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
