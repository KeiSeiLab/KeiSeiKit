---
title: skills_via_registry.rs
path: kei-mcp/tests/skills_via_registry.rs
dna_hash: sha256:c64667f23e666a06
language: rust
size_loc: 177
generated: by-keidocs
---

# kei-mcp/tests/skills_via_registry.rs

Integration test — `resources/list` and `resources/read` flow through
the `kei-skills` `SkillRegistry` (Phase 3.1 SSoT).

Walks ancestors of `CARGO_MANIFEST_DIR` to find the repo root's
`skills/` directory (KeiSeiKit corpus, ~45 SKILL.md files at time of
writing). Skips the test if the dir cannot be located — keeps the
suite green on isolated checkouts that don't carry the skills tree.

Asserts:
1. `resources/list` returns ≥ 30 entries (corpus headroom over 45).
2. Three known-valid skill names — `research`, `refactor`, `onboard`
— are reachable both from the list and via `resources/read`.
3. `resources/read` for a bogus skill returns an error envelope.
4. `resources/list` URIs match `skill://<name>` shape and carry a
non-empty `description`.

## Public API

- Locate `skills/` by walking up from `CARGO_MANIFEST_DIR`. Returns

## Related

- parent: `kei-mcp/tests`
- imports: kei_mcp, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
