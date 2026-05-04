---
title: error.rs
path: kei-llm-llamacpp/src/error.rs
dna_hash: sha256:67e5404cfc73883f
language: rust
size_loc: 82
generated: by-keidocs
---

# kei-llm-llamacpp/src/error.rs

Error enum — exit-code carrier for the kei-llm-llamacpp CLI.

Exit-code mapping (locked by spec):
0 success   — never reached via Error
1 IO/parse  — SpawnFailed, ParseFailed
2 not-found — BinaryNotFound, ModelNotFound
3 process   — NonZeroExit
4 timeout   — Timeout
5 security  — InvalidHost (non-localhost rejected)

## Public API

- `pub fn exit_code` — Map this error to the locked CLI exit code.

## Related

- parent: `kei-llm-llamacpp/Cargo.toml`
- imports: std

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
