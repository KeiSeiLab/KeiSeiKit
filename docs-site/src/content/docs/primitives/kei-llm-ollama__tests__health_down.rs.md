---
title: health_down.rs
path: kei-llm-ollama/tests/health_down.rs
dna_hash: sha256:ad59e0d9add6f3ef
language: rust
size_loc: 24
generated: by-keidocs
---

# kei-llm-ollama/tests/health_down.rs

`is_running` returns `false` when nothing listens on the given URL.

We bind a TCP listener on an ephemeral loopback port, drop it (releasing the
port), and point the client at that now-unbound URL. The connection refuses
immediately on every modern OS — we don't have to wait for any timeout.

## Related

- parent: `kei-llm-ollama/tests`
- imports: kei_llm_ollama, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
