---
title: server.rs
path: kei-llm-llamacpp/src/server.rs
dna_hash: sha256:75a09264da063912
language: rust
size_loc: 83
generated: by-keidocs
---

# kei-llm-llamacpp/src/server.rs

Server — spawn `llama-server` and return a managed handle.

Default --host 127.0.0.1 ALWAYS. Non-localhost host strings are
rejected with `Error::InvalidHost`; this primitive is a daemon
spawner, never a remote-exposure tool.

## Public API

- Inputs to a `server` invocation.
- JSON-friendly summary returned by the CLI on spawn.
- `pub fn validate_host` — Reject anything that isn't an allow-listed loopback host.
- `pub fn build_server_args` — Build the argv for `llama-server -m <model> --host <host> --port <port>`.
- `pub fn info_from_handle` — Build a ServerInfo from a handle + opts.
- Validate host then spawn `llama-server`. Caller owns the handle;

## Related

- parent: `kei-llm-llamacpp/Cargo.toml`
- imports: crate, serde, std

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
