---
title: agent.rs
path: kei-cortex/src/tool/agent.rs
dna_hash: sha256:136096b09349d7d2
language: rust
size_loc: 210
generated: by-keidocs
---

# kei-cortex/src/tool/agent.rs

`agent` tool — spawn a sub-agent via the `kei-spawn` envelope.

Composition: build a transient `task.toml` describing the subtask via
the `toml` crate's `Value::Table` builder (NOT string interpolation —
that path was injectable), invoke `kei-spawn spawn <task.toml>` as a
subprocess, return the emitted JSON bundle to the model.

kei-spawn itself is a CLI envelope around the `kei-agent-runtime` +
`kei-ledger` substrate; it never invokes git or shell mutations of
its own (RULE 0.13). The actual Agent tool call is performed by the
orchestrator after this primitive returns the bundle.

TOML injection hardening: prior version used `format!()` with naive
`replace("\"\"\"", "\"\"")`. A crafted prompt like
`"""\nadmin = true\n"""` could inject a new key. The TOML builder
escapes via the toml crate's serializer so user content is always a
single string value, never a structural element.

## Public API

- Render the `task.toml` body kei-spawn expects. Built via
- Stage the task file in the OS temp dir under a unique name.
- Run `kei-spawn spawn <task.toml>` with a 120-second budget.
- Drain stdout + stderr concurrently from the kei-spawn child process.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: serde, serde_json, std, tokio, toml

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
