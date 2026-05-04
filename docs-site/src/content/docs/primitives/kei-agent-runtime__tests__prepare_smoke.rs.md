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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
