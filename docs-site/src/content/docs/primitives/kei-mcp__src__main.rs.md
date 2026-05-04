---
title: main.rs
path: kei-mcp/src/main.rs
dna_hash: sha256:a40eac873cfa72d7
language: rust
size_loc: 123
generated: by-keidocs
---

# kei-mcp/src/main.rs

kei-mcp binary — stdio JSON-RPC loop.

Flow:
1. Read stdin line-by-line (each line is one JSON-RPC request).
2. Cap each line at `MAX_MESSAGE_BYTES`; oversize lines emit a
`-32700 parse error` reply and the loop continues.
3. Parse each line; on parse error emit a `-32700 parse error` reply.
4. Dispatch to the matching async handler; serialise the response
back as one stdout line.
5. On stdin EOF, exit cleanly (graceful shutdown).

Discipline:
- stdout: ONLY JSON-RPC frames (line-delimited).
- stderr: ALL logging / warnings.
- One request per line, one response per line. No batching for now.

## Public API

- JSON-RPC `parse error` code per the spec.
- Default atom-registry root: `$KEI_MCP_ATOMS_ROOT` or `_primitives/_rust`.
- Parse + dispatch one stdin line. On parse failure produce a JSON-RPC
- Build a `-32700` reply for an oversize line. We deliberately do NOT

## Related

- parent: `kei-mcp/Cargo.toml`
- imports: anyhow, kei_mcp, serde_json, std, tokio

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
