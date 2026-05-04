---
title: main.rs
path: kei-ledger/src/main.rs
dna_hash: sha256:79c4e81d069d7d50
language: rust
size_loc: 233
generated: by-keidocs
---

# kei-ledger/src/main.rs

kei-ledger — CLI dispatcher.

Single responsibility: parse args, dispatch to ledger ops, format output.
Storage: `~/.claude/agents/ledger.sqlite` (or $KEI_LEDGER_DB override).

Module tree: this binary depends on the `kei_ledger` library crate
(defined in `src/lib.rs`). The CLI dispatcher holds clap shapes and
glue only — every operation forwards to a library function.

## Public API

- Override ledger path (default: $KEI_LEDGER_DB or ~/.claude/agents/ledger.sqlite)
- Create the ledger file + schema if missing.
- Log a new running agent.
- Branch name (<=256 chars).
- Parent branch (<=256 chars).
- Layer G DNA fingerprint (optional; kept blank for legacy callers).
- DNA / human id of the agent that spawned this fork (v4 lineage).
- DNA of the forked-from agent, if this is itself a fork (v4 lineage).
- Mark a running agent as done.
- Mark a running agent as failed.
- Mark a done/failed agent as merged.
- List agents, optionally filtered by status.
- Print parent -> children tree starting at a root agent id.
- Validate required artefact bundle for a given branch's agent.
- List agents whose fork_parent_id OR creator_id equals the given DNA.
- Aggregate skill_invocations for Phase D nightly decisions.
- Unix-second lower bound (default: now - 30 days).
- Output format: json or markdown (default: markdown).
- Record a skill invocation in `skill_invocations` (schema v8+).
- Skill name as registered in `~/.claude/skills/<name>/SKILL.md`.
- 1 = succeeded, 0 = bailed/failed.
- Optional agent invocation that triggered this skill.
- Optional trajectory id for skill-chain grouping.
- Wall-clock duration in milliseconds (≥ 0).
- Record cost-tracking metadata (v6+) for an existing agent row.
- Agent id (matches `fork ... <id>`).
- Cost in cents (integer ≥ 0). Capped at i64::MAX on extreme values.
- Provider name, e.g. "anthropic".
- Model name, e.g. "claude-haiku-4-5-20251001".
- Overwrite previous cost (legacy semantics). Without this flag,
- clap value_parser caps branch/parent length at MAX_BRANCH_LEN (audit L1).

## Related

- parent: `kei-ledger/Cargo.toml`
- imports: clap, dispatch, kei_ledger, std

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
