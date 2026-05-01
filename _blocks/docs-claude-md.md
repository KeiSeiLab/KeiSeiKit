# DOCS — `CLAUDE.md` (project bootstrap template)

A per-project `CLAUDE.md` answers one question: *what does a Claude agent need to know in the first 30 seconds on this repo?* It is read before any code work. Keep it under ~150 lines.

**Canonical sections (in this order):**

1. **Project one-liner** — name, domain, status (`active | maintenance | archived`), primary stack, public-surface flag.
2. **Architecture** — 2-5 bullets + optional Mermaid block. Layer names match the code tree. If a layer diagram helps, `_blocks/docs-architecture-diagrams.md` has the patterns.
3. **Stack / dependencies** — language(s), major frameworks, DB, queue, deploy target. One line per item.
4. **Constraints** — API rate limits, licensing, cost tiers, platform quirks (e.g. "Flux 2 Pro zero-config", "SPM needs `-Xlinker`").
5. **Known issues** — bugs that aren't fixable now, workarounds, tickets. Keep dated.
6. **Test invariants** — how tests are run (`cargo test --release`, `pytest`, `flutter test`), coverage floor, which tests are load-bearing.
7. **Commands cheatsheet** — 5-8 commands the agent will type most: build, test, lint, deploy, format.
8. **Secrets / credentials** — env var NAMES only (RULE 0.8). Never literal tokens. Path: `secrets/*.env`.
9. **Related files** — `DECISIONS.md`, `HOTPATHS.md`, `TODO.md`, runbooks.

**Placeholders used by `kei-docs-scaffold.sh`:**
`{{PROJECT_NAME}}`, `{{STACK}}`, `{{DEPLOY}}`, `{{PRIMARY_LANGUAGE}}`, `{{TEST_CMD}}`.

**Forbidden:**
- Copying the umbrella `~/.claude/CLAUDE.md` here — link to it, do not duplicate.
- Storing API tokens / private URLs (use `secrets/*.env`).
- Marketing prose. Every line must be actionable by the agent.

**Source:** Anthropic Claude Code docs — `claude.ai/code` project-memory convention (E4). Karpathy viral CLAUDE.md (forrestchang/andrej-karpathy-skills, 15K+ stars) [E4].
