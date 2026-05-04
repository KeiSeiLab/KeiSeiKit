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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
