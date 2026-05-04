---
title: rules_cmd.rs
path: kei-decompose/src/rules_cmd.rs
dna_hash: sha256:8674358386175856
language: rust
size_loc: 184
generated: by-keidocs
---

# kei-decompose/src/rules_cmd.rs

`decompose-rules` CLI subcommand implementation.

Walks `<rules-dir>/*.md`, `specialty/*.md`, and `projects/*.md`
(depth ≤ 2), parses each rule file into `RuleFragment`s, writes each
fragment body to `<frags-dir>/<rule>__<section>.md` (a real file), and
registers each fragment in `kei-registry` with that real path.

Path convention: `<frags-dir>/<rule-slug>__<section-slug>.md`
Double-underscore separates slugs (shell-safe; `::` is not a valid path
component). This ensures `_assembler` can `fs::read_to_string` the path.

Constructor Pattern: this cube owns the walk + write + register loop.
Parsing lives in `parsers::rule`. Registry API in `kei_registry`.
Migration (rebuild) lives in `rules_rebuild`.

## Public API

- Counters returned after a full run.
- `pub fn run` — Entry point called from `main.rs`.
- `pub fn fragment_path` — Canonical fragment file: `<frags_dir>/<rule>__<section>.md`.
- `pub fn write_fragment_file` — Write body to disk only if content differs from existing file.
- `pub fn ensure_dir` — Create directory (and parents) if absent.

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: anyhow, crate, kei_registry, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
