---
title: mod.rs
path: kei-export-trajectories/tests/common/mod.rs
dna_hash: sha256:852625207e6efecd
language: rust
size_loc: 105
generated: by-keidocs
---

# kei-export-trajectories/tests/common/mod.rs

Shared fixture builders for golden tests.

Two synthetic agents:
- agent-a: 'done', 3 successful tool events (Read x2, Bash x1)
- agent-b: 'failed', 1 failed tool event (Bash)

Their started_ts are 1700000100 and 1700000300 respectively, so any
cutoff at or below 1700000100 includes both.

## Public API

- `pub fn build_ledger` — Create a minimal `agents`-schema sqlite at `path` with two synthetic
- `pub fn build_memory` — Create a minimal `events`-schema sqlite at `path` with 4 events
- `pub fn build_artefacts` — Create `.claude/agents/{agent-a,agent-b}/{spec.md,chatlog.md}` under

## Related

- parent: `kei-export-trajectories/tests/common`
- imports: rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
