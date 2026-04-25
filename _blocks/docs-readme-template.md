# DOCS — Public `README.md` scaffold

`README.md` is the first file a new reader (human OR agent) opens. One file, nine sections, in this order. Keep ≤ 300 lines; longer material lives in `docs/`.

**Nine-section template:**

```markdown
# {{PROJECT_NAME}}

> One-line pitch (what + why, ≤ 100 chars).

[![CI](badge)](link) [![License](badge)](link) [![Version](badge)](link)

## What
2-3 sentences: what it does, who it's for. No marketing adjectives.

## Why
2-3 sentences: problem this solves, alternatives considered, why this one.
Link to the relevant ADR: [DECISIONS.md](DECISIONS.md#adr-nnnn).

## Install
```bash
# Primary path — the 90% case
<one command>
```

**Prerequisites:** <language X >= vN, OS constraints, system deps>.

<If needed: alternative install methods in `docs/install.md`.>

## Usage
Smallest working example. Copy-pasteable.
```bash
<command producing visible output>
```

Link to a richer quickstart in `docs/quickstart.md` if >20 lines.

## Development
```bash
git clone <repo>
cd <repo>
<bootstrap command, e.g. cargo build>
<test command>
```

Project layout:
- `src/` — implementation
- `tests/` — integration tests
- `docs/` — long-form docs
- `{{STACK}}-specific notes → link>

## Deploy
Target: **{{DEPLOY}}**. One-liner: `<deploy command>`.
Full runbook: `docs/runbooks/deploy.md`.

## Architecture
One paragraph + one Mermaid diagram (see `_blocks/docs-architecture-diagrams.md`). Detail in `docs/architecture.md`.

## Contributing
- Issue tracker: <url>
- Commit convention: Conventional Commits (see `_blocks/git-conventions` in kit)
- PR checklist: `docs/CONTRIBUTING.md`

## License
<SPDX id> — see [LICENSE](LICENSE).
```

**Rules:**
- No secrets (RULE 0.8). No literal tokens.
- Install command must be ONE command for the happy path.
- Every "see docs/X" link must resolve — scaffolder verifies or creates the target.
- If the project is private / not publicly deployable (banned list per `rules/security.md`), mark the repo header with `<!-- PRIVATE — do not publish -->` and omit public badges.

**Source:** standard-readme spec (RichardLitt/standard-readme) [E4]; GitHub "About READMEs" [E4].
