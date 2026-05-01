# Phase 5 — Architecture proposal (math-first)

Compose the architecture by following `_blocks/rule-math-first.md`.

## 5a — Expression first

One to three lines describing which primitives combine, in which order,
with which invariants. Use this shape:

```
artefact = compose(block_A, block_B, ..., block_N)
where block_* ∈ {_blocks/, newly drafted, skills/, _manifests/}
invariant: <one-line, e.g. "every cube <200 LOC, every handoff verified">
```

If the architecture requires parsing binary document formats (PDF / DOCX /
XLSX / PPTX / CSV), reference the `tomd` primitive
(`_primitives/tomd.sh`) instead of rolling custom parsing — RULE "reuse
over rewrite". The PreToolUse(Read) hook `tomd-preread.sh` already redirects
Claude to the converted markdown transparently.

### Pipeline / primitive cross-refs (reuse before rewrite)

If the user's task maps onto an existing hub-and-spoke pipeline, recommend
it instead of composing from scratch. Each pipeline is itself discoverable
via Phase 3 grep, but surface it explicitly so the user sees the option:

- VM / server provisioning → `/vm-provision` + `ssh-check` + `firewall-diff`
- Database schema design → `/schema-design` + `kei-migrate` (PG/SQLite/MySQL)
- Metrics + logs observability → `/observability-setup` + `metrics-scrape` + `log-ship`
- Authentication / session / JWT / OAuth → `/auth-setup`
- CI/CD workflow scaffolding → `/ci-scaffold` + `kei-ci-lint`
- REST / GraphQL / gRPC API contract → `/api-design`
- Doc site + changelog automation → `/docs-scaffold` + `kei-changelog`
- Test matrix (unit / integration / e2e / visual) → `/test-matrix`
- Frontend site / UI WYSIWYD loop → `/site-create` + `mock-render` + `visual-diff` + `tokens-sync`
- Multi-agent project bootstrap → `/new-project` + `kei-ledger` (RULE 0.12 fork tracking)
- Typed artifact handoff between agents → `kei-artifact` (v0.15: schema-validated spec→plan→patch→review chain instead of prose hints). If your architecture spans multiple agents and the output of one is the input of another, declare `produces_artifact` / `expects_artifact` in the manifest and emit via `kei-artifact emit`.

One-line per reference, click-discoverable, no duplication of pipeline logic.

## 5b — What is UNNECESSARY?

For each block listed, justify why it's in. If a block can be removed
without losing the user's goal — remove it. Derive-first: explicit claim
"this is the minimal decomposition, nothing removable". Follow the checklist
from `_blocks/rule-math-first.md`:

- Learned parameters / free knobs? WHY? Determined by input?
- Separate blocks for similar concerns? WHY? Can a single block cover both?
- Gate / wrapper layers? WHY? Is a direct reference enough?

## 5c — Constructor Pattern check

Each output cube must be single-concern, file < 200 LOC, function < 30 LOC.
If the proposed assembly violates this, split before proceeding.

## 5d — Count

Show the numbers explicitly in the preview:
- New files: N
- Edits to existing files: M
- Total lines of markdown to be written: L

## 5e — Preview + confirm

Preview as plain text in chat, then:

```json
{
  "questions": [
    {
      "question": "Architecture OK?",
      "header": "Architecture",
      "multiSelect": false,
      "options": [
        {"label": "Confirm",              "description": "Proceed to Phase 6 block augmentation (if any gaps) then Phase 7 assembly"},
        {"label": "Revise component N",   "description": "One component's decomposition or reuse choice is wrong — reply with one free-text line"},
        {"label": "Remove something",     "description": "You see a block that's not strictly necessary — reply which one"},
        {"label": "Abort",                "description": "Stop — nothing gets written"}
      ]
    }
  ]
}
```

On `Revise` / `Remove` → ONE free-text prompt, regenerate the architecture,
re-preview.

## Verify-criterion

- User clicked Confirm.
- The expression (5a) is present and < 3 lines.
- The "what is unnecessary" pass (5b) has been applied and is visible in the
  preview.
- Constructor Pattern check (5c) passed.
