# PRE-DEV GATE — three checks before any new code

This gate runs ONCE before you write a single line of new code on a non-trivial change. Skipping it is the most common cause of overlapping rewrites, dependency drift, and silent duplication.

## 1. Analogues check — does this already exist?

Before designing your own solution, search the project + its direct dependencies for an existing one. Use `Grep` / `Glob` for symbols and patterns; use the keimd graph index (`keimd related <file>`, `keimd search <query>`) for semantic relatedness.

- Search the symbol you'd name (function / type / struct).
- Search adjacent verb forms (`scan_*`, `parse_*`, `*_handler`).
- Read the README and `_primitives/MANIFEST.toml` (or equivalent index) for cubes that already cover this concern.

If a usable analogue exists, **prefer reusing or extending it** over a parallel implementation. Branching the codebase on the same concern produces shotgun-surgery later.

## 2. Stack compatibility — does the new dep belong?

If your change pulls a new dependency, check it against the project's existing stack BEFORE adding to `Cargo.toml` / `package.json` / `pyproject.toml`:

- **Language match** — does the dep's language fit the project's default? In Rust-first projects, a Python-only dep needs a stated exception.
- **Maintenance signal** — last release date, open-issue count, transitive dep count.
- **Conflict with existing deps** — runtime conflicts (two HTTP clients, two TLS stacks, two async runtimes) are silent foot-guns.
- **License** — Apache-2.0 / MIT / BSD-3 are safe; AGPL / SSPL / proprietary need explicit approval.

If the dep doesn't fit, prefer the existing stack's idiomatic primitive even if it's slightly less convenient.

## 3. Duplication check — are you about to recreate something?

The architecture-overlay incident (a single file ballooned 227 → 354 LOC purely from "fix" patches that duplicated the formula they were supposed to repair) is the canonical warning. Before adding new code on top of existing code, ask:

- Am I patching around a problem instead of fixing it at the root?
- Is this new function logically the same as one already in the codebase, just with different phrasing?
- Is my change adding a third copy of a constant / config value / regex that should live in one place?

If yes → STOP and refactor at the root before adding the new behaviour.

## Failing the gate

If ANY check fails, stop and reconsider. The cheapest pivot is at this gate; every layer downstream (commit, review, audit, deploy) is more expensive to walk back. Do not proceed to implementation while one of the three checks is unresolved.

The gate is paired with **Plan Mode First** — you write the plan AFTER this gate (so the plan reflects what already exists), not before.
