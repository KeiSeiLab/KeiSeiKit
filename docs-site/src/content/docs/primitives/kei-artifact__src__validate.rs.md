---
title: validate.rs
path: kei-artifact/src/validate.rs
dna_hash: sha256:070b88348a1e1c81
language: rust
size_loc: 268
generated: by-keidocs
---

# kei-artifact/src/validate.rs

Minimal JSON Schema validator — strict subset of draft 2020-12.

Keyword support (chosen for the 5 built-in schemas):
- `type` (object, array, string, integer, number, boolean, null)
- `required` (array of property names)
- `properties` (object → sub-schema)
- `additionalProperties` (bool; default true, we set false on ours)
- `enum` (array of allowed scalar values)
- `items` (sub-schema for array elements)
- `minLength` (integer) / `minItems` (integer) / `minimum` (number)

Intentionally NOT supported: $ref, oneOf/anyOf/allOf, patternProperties,
format validation, conditional schemas. The 5 built-in schemas are written
to avoid needing those — keeps the validator under 200 LOC and removes the
40+ transitive-dep `jsonschema` crate.

RULE 0.4 note: draft 2020-12 is the current JSON Schema standard
[VERIFIED: https://json-schema.org/draft/2020-12 — spec page].
This implementation is a strict subset — any schema author sticking to
the keywords above gets draft-2020-12-compatible semantics.

## Public API

- `pub fn validate_content` — Top-level entry. Returns `Ok(())` on pass, `Err(msg)` with a path-style
- Keywords the minimal validator knows about. Used by `warn_unsupported_keywords`
- `pub fn warn_unsupported_keywords` — Emit a stderr warning for each schema keyword this validator does not

## Related

- parent: `kei-artifact/Cargo.toml`
- imports: serde_json

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
