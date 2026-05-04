---
title: validate.rs
path: kei-agent-runtime/src/validate.rs
dna_hash: sha256:890854fadd54fbe5
language: rust
size_loc: 175
generated: by-keidocs
---

# kei-agent-runtime/src/validate.rs

Agent-id validator — HIGH-security path-traversal defence.

Every `agent_id` flowing from task.toml (or auto-gen) into a filesystem
path sink MUST pass `validate_agent_id` first. Without this gate, a
hostile task.toml with `agent-id = "../../../etc/foo"` reaches
`tasks/<agent-id>/` and writes arbitrary paths.

Rules (enforced in order, first failure wins):
- non-empty, length ≤ 64
- ASCII-only, matches `^[A-Za-z0-9][A-Za-z0-9_.-]*$`
- rejects `/`, `\`, `..`, leading `.`, leading `-`, NUL, `:`,
whitespace, non-ASCII
- rejects Windows-reserved names (case-insensitive):
CON, PRN, AUX, NUL, COM1-9, LPT1-9

Also hosts `autogen_agent_id` (moved from prepare.rs) so the auto-gen
output passes the validator by construction.

## Public API

- `pub const MAX_AGENT_ID_LEN` — Maximum permitted `agent_id` length (bytes = chars, since ASCII-only).
- Typed error — the sole failure variant of `validate_agent_id`.
- `pub fn validate_agent_id` — Validate an `agent_id` before it reaches any filesystem path.
- `pub fn autogen_agent_id` — Auto-generate a fresh `agent_id` whose output is validator-clean.
- `pub fn slugify_role` — Slugify a role name into the validator's allowed class.

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: std, thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
