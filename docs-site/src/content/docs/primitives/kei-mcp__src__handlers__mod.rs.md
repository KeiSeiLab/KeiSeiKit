---
title: mod.rs
path: kei-mcp/src/handlers/mod.rs
dna_hash: sha256:68f1b25f4c6f1a68
language: rust
size_loc: 35
generated: by-keidocs
---

# kei-mcp/src/handlers/mod.rs

MCP method handlers — one file per method-family.

`dispatch` is the single entry point; it parses the method string,
delegates to the matching family handler, and folds the result back into
a JSON-RPC envelope.

`dispatch` is `async` because `tools/call` shells out to atom binaries
through `tokio::process::Command` with a 60s timeout (MISS-4). The other
method handlers are pure / synchronous; they are awaited as cheap no-op
futures by the matcher below.

## Public API

- Async dispatcher — picks the right handler based on the method.

## Related

- parent: `kei-mcp/Cargo.toml`
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
