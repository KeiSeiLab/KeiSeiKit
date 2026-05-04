---
title: capability.rs
path: kei-agent-runtime/src/capability.rs
dna_hash: sha256:b83ba312f76776fa
language: rust
size_loc: 141
generated: by-keidocs
---

# kei-agent-runtime/src/capability.rs

Capability trait + context / result types.

Per schema §Capability trait contract (Rust). One trait, dispatched by
string name via `registry::get()`. Gates return `GateDecision`;
verifies return `VerifyResult`. Defaults are no-op so gate-only and
verify-only capabilities omit the other half.

## Public API

- `pub trait Capability` — Shared Capability trait. Gate + verify methods both default to no-op
- Namespaced capability name: `<category>::<slug>` (e.g. `policy::no-git-ops`).
- PreToolUse gate; called by `kei-capability check <name>`.
- On-return verify; called by `kei-capability verify <name>`.
- `pub struct GateContext` — Context passed to `Capability::check()` — constructed by the hook binary
- Gate outcome. `Deny` exits 2 in the hook binary; `Allow`/`NotApplicable` exit 0.
- `pub struct VerifyContext` — Context passed to `Capability::verify()` — constructed from env vars by the
- `pub fn run_dir` — Active run dir: simulated-merge path if present, otherwise the worktree.
- Verify result. `Fail` exits non-zero in the hook binary.
- Verify execution mode. Orchestrator splits `Both` into two sequential
- Parsed task.toml. Subset used by gates + verifies; parser lives in

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: serde, serde_json, std

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
