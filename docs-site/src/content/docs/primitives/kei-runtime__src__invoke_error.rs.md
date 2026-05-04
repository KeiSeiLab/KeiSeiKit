---
title: invoke_error.rs
path: kei-runtime/src/invoke_error.rs
dna_hash: sha256:702ae644627a93b8
language: rust
size_loc: 54
generated: by-keidocs
---

# kei-runtime/src/invoke_error.rs

Typed errors for the atom-invocation runtime.

Constructor Pattern: extracted from `invoke.rs` so the runtime
parent file stays under 200 LOC after Wave 44d added bounded-read
capture + truncation handling.

## Public API

- `crate_name` in atom YAML failed the `kei-*` allowlist check.
- Crate binary is missing from both `KEI_RUNTIME_BIN_DIR` and `PATH`.
- Subprocess exited non-zero — propagate the atom's own exit code.
- IO / spawn failure (not a non-zero exit from the child).
- Atom's stdout was not parseable as JSON.
- Legacy escape — atom not yet migrated to `run-atom` protocol.

## Related

- parent: `kei-runtime/Cargo.toml`

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
