---
title: main.rs
path: kei-cortex/src/main.rs
dna_hash: sha256:66988ce2c12ef072
language: rust
size_loc: 168
generated: by-keidocs
---

# kei-cortex/src/main.rs

`kei-cortex` CLI — `serve` subcommand starts the daemon.

Token is auto-generated on first launch if missing. The daemon binds to
`127.0.0.1:<port>` only; public binding is forbidden by design.

## Public API

- Start the daemon on 127.0.0.1.
- Directory containing bundled Cubism sample rigs (haru/, mao/, hiyori/,
- Process working directory used to discover CLAUDE.md / AGENTS.md
- Project root used as the chroot for `/fs/list` + `/term` and as
- Default LLM provider used when the chat request omits `?provider=`.
- SQLite database for per-turn token-event telemetry. Defaults to
- SECURITY: refuse to start when bound to a non-loopback address with a
- Emit a warning if the live2d samples dir is missing. Not fatal — the

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: anyhow, clap, kei_cortex, std, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
