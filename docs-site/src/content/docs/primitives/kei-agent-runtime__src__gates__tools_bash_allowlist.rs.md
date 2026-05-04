---
title: tools_bash_allowlist.rs
path: kei-agent-runtime/src/gates/tools_bash_allowlist.rs
dna_hash: sha256:e3f64228c9f6d917
language: rust
size_loc: 27
generated: by-keidocs
---

# kei-agent-runtime/src/gates/tools_bash_allowlist.rs

`tools::bash-allowlist` — PreToolUse:Bash denies commands not matching
one of the configured allowlist regexes.

Renamed from `tools::cargo-only-bash` in v0.17. Old name still resolves
via registry alias.

As of v0.18 convergence wave: thin const wrapper over `PatternGate`.

## Related

- parent: `kei-agent-runtime/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
