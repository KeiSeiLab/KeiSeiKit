---
title: prepare_smoke.rs
path: kei-agent-runtime/tests/prepare_smoke.rs
dna_hash: sha256:75b8a1652ebefc4d
language: rust
size_loc: 120
generated: by-keidocs
---

# kei-agent-runtime/tests/prepare_smoke.rs

Prepare smoke — validates orchestrator-facing wrapper.

Three fixtures per task spec:
1. Happy path — valid task.toml → AgentInvocation fully populated
2. Unknown role → clear error (role lookup fails)
3. Non-spawnable role (git-ops) → explicit refusal + RULE 0.13 pointer

## Related

- parent: `kei-agent-runtime/tests`
- imports: kei_agent_runtime, tempfile

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
