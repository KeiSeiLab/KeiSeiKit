---
title: discovery.rs
path: kei-llm-llamacpp/src/discovery.rs
dna_hash: sha256:0cde278264d020d8
language: rust
size_loc: 89
generated: by-keidocs
---

# kei-llm-llamacpp/src/discovery.rs

Discovery — locate `llama-cli` / `llama-server` on PATH.

ENV override `KEI_LLAMA_CPP_DIR` lets non-standard installs point at
a custom directory (binaries inside it take precedence over PATH).
Version detection runs `<bin> --version` through the Runner trait and
parses the first numeric token after "version".

## Public API

- Discovered binaries + version (None if neither found).
- `pub fn any_found` — True if at least one of llama-cli / llama-server is present.
- Locate both binaries (PATH + ENV override) and ask the runner for
- Resolve a binary by name. ENV override wins over PATH.
- Run `<path> --version` and pull the version string. We accept
- `pub fn parse_version` — Best-effort version parse. Looks for "version <X>" or "v<X>" or any

## Related

- parent: `kei-llm-llamacpp/Cargo.toml`
- imports: crate, regex, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
