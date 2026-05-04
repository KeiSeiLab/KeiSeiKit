---
title: generate.rs
path: kei-forge/src/generate.rs
dna_hash: sha256:17704f160eb788ff
language: rust
size_loc: 159
generated: by-keidocs
---

# kei-forge/src/generate.rs

Atom-scaffolding generator — pure Rust templating.

Reads the five templates in `<repo>/_templates/atom/`, substitutes the
six placeholder tokens (`__CRATE__`, `__CRATE_SNAKE__`, `__VERB__`,
`__VERB_SNAKE__`, `__KIND__`, `__DESCRIPTION__`), and writes the
resulting files into `<repo>/_primitives/_rust/<crate>/`.

No shell-out. No sed. The Rust string replace cannot be coerced into
executing a secondary expression, so the description-injection attack
class defended by `form::validate_description` is structurally gone —
the whitelist stays as defence-in-depth, not the primary barrier.

Atomicity: every file written is accumulated in a rollback list; on
any write failure the accumulator is flushed (files deleted best-
effort) before the error surfaces. Matches new-atom.sh's `trap ERR`.

## Public API

- Structured failure modes returned by the pure-Rust generator.
- `<repo>/_primitives/_rust/<crate>/` does not exist.
- One of the five target files already exists — refuse to overwrite.
- `<repo>/_templates/atom/` missing or a template file unreadable.
- Filesystem I/O failed mid-write.
- Result of a scaffolding attempt — wire-compatible with the previous
- `pub fn repo_root` — Locate the repo root by walking up from CARGO_MANIFEST_DIR until we
- `pub fn forge` — Thin wrapper the HTTP layer calls — discovers repo root, invokes the
- `pub fn generate_atom` — Core entry point — pure fn over (req, root), exposed for unit tests.
- Write one file, register in the rollback list, rollback on error.
- Render path relative to repo-root for the JSON response.

## Related

- parent: `kei-forge/Cargo.toml`
- imports: crate, paths, placeholders, rollback, serde, std

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
