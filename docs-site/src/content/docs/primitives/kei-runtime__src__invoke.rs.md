---
title: invoke.rs
path: kei-runtime/src/invoke.rs
dna_hash: sha256:3905f54650250802
language: rust
size_loc: 159
generated: by-keidocs
---

# kei-runtime/src/invoke.rs

Atom invocation — executes atoms by spawning `<crate> run-atom <verb>`.

Flow:
1. Discover atom → get `crate_name` + `verb` from `AtomMeta`
2. Validate input JSON against the atom's `input_schema`
3. Resolve the binary at `<KEI_RUNTIME_BIN_DIR>/<crate>` or `PATH`
4. Spawn `<crate> run-atom <verb>` with input on stdin
5. Parse stdout as JSON → `Output { atom, result }`
6. Propagate exit codes: 0 ok, 2 atom-error, 127 not-found, 1 IO

`NotImplemented` is retained as a rare corner-case escape (e.g. an atom
whose crate has not yet been migrated to the `run-atom` protocol).

## Public API

- `pub const OUTPUT_CAP` — Max bytes we read from subprocess stdout/stderr to guard against
- Parsed output of an invoked atom. `result` is the raw JSON the atom wrote.
- `pub fn invoke` — Invoke an atom by full ID with a JSON input string.
- Validate `name` matches `^kei-[a-z][a-z0-9-]+$`; rejects path traversal and injection chars.
- Write the atom's input JSON to the child's stdin, dropping the handle
- Map a `Captured` (potentially-truncated) child result into our typed
- Resolve `<crate_name>` as binary: first `$KEI_RUNTIME_BIN_DIR/<name>`, then `$PATH`.

## Related

- parent: `kei-runtime/Cargo.toml`
- imports: crate, serde, serde_json, std

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
