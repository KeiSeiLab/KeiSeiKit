---
name: new-agent
description: Generate a new project-specialist agent manifest via interactive wizard. Asks stack/deploy/domain questions via AskUserQuestion, composes blocks, writes manifest, generates .md.
---

# New Agent — Project-Specialist Wizard

You are creating a new project-specialist agent for the KeiSeiKit Constructor-Pattern agent fleet.

The fleet lives at `~/.claude/agents/`:
- `_manifests/*.toml` — source of truth (one per agent)
- `_blocks/*.md` — reusable building blocks
- `_templates/specialist.toml.template` — the template you will fill
- `_assembler/target/release/assemble` — Rust binary: manifest + blocks → .md
- `<name>.md` — generated agent file (a hook blocks direct edits)

Goal: interactive wizard → fill template → validate → assemble in-place → report.

---

## Phase 1 — Option-picker questions (AskUserQuestion, ONE call)

Send ALL FOUR questions in a SINGLE `AskUserQuestion` invocation so the user picks them in one pass. Use `multiSelect: false` for every question. Do NOT use free-text here.

```json
{
  "questions": [
    {
      "question": "Project stack?",
      "header": "Stack",
      "multiSelect": false,
      "options": [
        {"label": "Rust CLI",                 "description": "Binary + clap/argh, local or piped tooling"},
        {"label": "Rust server (axum)",       "description": "HTTP/WebSocket service, tokio runtime"},
        {"label": "Swift macOS (SPM)",        "description": "Menubar / desktop app, SPM executable, -Xlinker"},
        {"label": "Swift iOS",                "description": "iOS app, Xcode project, App Store target"},
        {"label": "Flutter",                  "description": "Cross-platform mobile/web, Dart + Riverpod"},
        {"label": "FastAPI + PostgreSQL",     "description": "Python backend, SQLAlchemy, uvicorn"},
        {"label": "Next.js",                  "description": "TS + React, Vercel/Cloudflare Pages deploy"},
        {"label": "Go server",                "description": "net/http or gin, statically linked binary"},
        {"label": "Embedded (STM32/ESP32)",   "description": "C / Rust no_std firmware, on-device flashing"},
        {"label": "Python ML",                "description": "PyTorch/JAX training, large-param models"}
      ]
    },
    {
      "question": "Deploy target?",
      "header": "Deploy",
      "multiSelect": false,
      "options": [
        {"label": "Local only (banned-public)", "description": "Never deployed — proprietary ML weights / offensive / kernel / client-confidential"},
        {"label": "AWS EC2",                    "description": "Instance, Elastic IP, SSH, docker-compose"},
        {"label": "Cloudflare Workers",         "description": "Edge, Workers + Pages + KV, wrangler deploy"},
        {"label": "Modal",                      "description": "Serverless GPU, retries + volumes, anti-stop guard"},
        {"label": "Docker self-hosted",         "description": "Any VPS, docker-compose, nginx"},
        {"label": "None yet",                   "description": "Greenfield, no deploy decided"}
      ]
    },
    {
      "question": "Uses paid APIs?",
      "header": "Paid APIs",
      "multiSelect": false,
      "options": [
        {"label": "Yes", "description": "fal.ai / OpenAI / Anthropic / Apify / ElevenLabs / similar — cost guard needed"},
        {"label": "No",  "description": "No paid external calls"}
      ]
    },
    {
      "question": "Contains ML training/inference?",
      "header": "ML",
      "multiSelect": false,
      "options": [
        {"label": "Yes", "description": "LLM / training run / inference — math-first + ml-protocol apply"},
        {"label": "No",  "description": "No ML components"}
      ]
    }
  ]
}
```

Store answers as `Q1`, `Q2`, `Q3`, `Q4`.

---

## Phase 1b — Follow-up (AskUserQuestion, ALWAYS run — one call)

This call ALWAYS runs. Q6 (scrapers) is independent of Q2/Q3, so every agent answers it. Q5 defaults "No" only if user explicitly picks so.

```json
{
  "questions": [
    {
      "question": "Has credentials / secrets?",
      "header": "Secrets",
      "multiSelect": false,
      "options": [
        {"label": "Yes", "description": "API keys / SSH keys / DB creds — never echo, reference paths only"},
        {"label": "No",  "description": "No secrets handled"}
      ]
    },
    {
      "question": "Uses scrapers (data extraction)?",
      "header": "Scrapers",
      "multiSelect": false,
      "options": [
        {"label": "None",             "description": "No scraping / data extraction"},
        {"label": "Free-tier only",   "description": "YouTube API v3, GitHub GraphQL, Telegram Telethon, Twitter twscrape — Tier 1"},
        {"label": "Paid tier",        "description": "Apify / Bright Data — HIGH GDPR + cost risk, kei-cost-guardian mandatory"}
      ]
    }
  ]
}
```

Store as `Q5`, `Q6`.

---

## Phase 2 — Free-text prompts (regular message, NOT AskUserQuestion)

Ask the user to reply in one message with all four fields. Use this exact prompt:

> Now give me four lines:
> 1. Project slug (lowercase, `[a-z0-9-]{3,30}`, e.g. `myapp` — proposed agent name will be `kei-<slug>-specialist` unless you override in Phase 3.5)
> 2. One-line description (shown to the orchestrator)
> 3. Project path (e.g. `~/Projects/MyApp/`)
> 4. 3-5 domain gotchas / constraints (one per line; these become the forbidden-domain list)

Validate the slug:
- Lowercase `[a-z0-9-]+` only, 3-30 chars.
- Invalid → report the regex failure and re-ask that single line; do not fall through.

Build:
- Default proposed `name = "kei-<slug>-specialist"` — matches the KeiSeiKit kit-prefix convention. The user confirms or overrides in Phase 3.5 below; the value written to the manifest is the one picked there.
- `memory_project = "<slug>-project.md"`
- `project_claudemd = "<project-path>/CLAUDE.md"` (preserve the `~/` prefix if the user gave it — the assembler does not expand tildes; manifests across the fleet keep `~/` literals)

---

## Phase 3 — Compose the manifest

### 3.1 Compute `blocks` array

ALWAYS include (in this order):
1. `baseline` (OBLIGATORY)
2. `evidence-grading` (OBLIGATORY)
3. `memory-protocol` (OBLIGATORY)
4. `rule-pre-dev-gate` (project-specialists write code → need it)

Then stack block based on Q1 (one of):
- Rust CLI → `stack-rust-cli`
- Rust server (axum) → `stack-rust-axum`
- Swift macOS (SPM) → `stack-swift-spm`
- Swift iOS → `stack-swift-ios`
- Flutter → `stack-flutter`
- FastAPI + PostgreSQL → `stack-fastapi-postgres`
- Next.js → `stack-nextjs`
- Go server → `stack-go-server`
- Embedded (STM32/ESP32) → `stack-embedded-stm32`
- Python ML → `stack-python-ml`

Then deploy block based on Q2 (skip for "None yet"):
- Local only (banned-public) → `deploy-local-only`
- AWS EC2 → `deploy-aws-ec2`
- Cloudflare Workers → `deploy-cloudflare`
- Modal → `deploy-modal`
- Docker self-hosted → `deploy-docker`

Then conditional domain blocks:
- If Q3 == Yes → append `domain-paid-apis`
- If Q4 == Yes → append `domain-ml-training` then `rule-math-first`
- If Q5 == Yes → append `domain-has-secrets`
- If Q6 == "Free-tier only" → append `scraper-free-tier` then `scraper-unified-output`
- If Q6 == "Paid tier" → append `scraper-paid-tier` then `scraper-free-tier` then `scraper-unified-output`; ALSO append `domain-paid-apis` if not already present (Q3 != Yes)

### 3.2 Pre-flight: verify every block exists on disk

Before writing the manifest, for each block in the computed list, check
`~/.claude/agents/_blocks/<block>.md` exists. Use a single Bash call:

```bash
for b in baseline evidence-grading memory-protocol rule-pre-dev-gate <rest>; do
  [ -f ~/.claude/agents/_blocks/$b.md ] || echo "MISSING: $b"
done
```

If any block is missing:
- Do NOT silently drop the block (constructive-only rule).
- Report the missing block(s) to the user with three constructive paths:
  (A) Create the missing block now (10 min) — offer to scaffold it from `baseline.md`.
  (B) Proceed WITHOUT the block and document the gap in the manifest comment.
  (C) Abort and come back once the parallel block-creation work lands.
- Wait for user approval before proceeding.

### 3.3 Compute `handoffs`

ALWAYS include:
- `kei-code-implementer` — "generic dev work outside this project's stack"
- `kei-critic` — "anti-pattern / Constructor Pattern sweep on diffs >200 LOC"
- `kei-validator` — "fact-check / citation sanity before commit"

Conditional additions:
- Q3 == Yes → add `kei-cost-guardian` — "paid API run — pricing + cost estimate + dashboard check"
- Q4 == Yes → add `kei-ml-implementer` — "numerical experiment / training run" AND `kei-ml-researcher` — "literature / prior art lookup"
- Q2 != "None yet" → add `kei-infra-implementer` — "node provisioning / deploy / SSH / container ops"
- Q1 is Rust/Swift/Go (any variant) → add `kei-security-auditor` — "crypto / key handling / network / memory review"
- Q6 == "Paid tier" → add `kei-cost-guardian` if not already present (paid scraping = cost risk even when Q3 == No) — "paid scraper run — Apify/Bright Data pricing + cost estimate + dashboard check"

### 3.4 Compute `references.extra`

Start from:
- The project's CLAUDE.md (Q2 path + `CLAUDE.md`)

Conditional:
- Q5 == Yes → a note like `"<project-path>/secrets/  (NEVER read into chat)"`

### 3.5 Compute remaining fields

`tools` — sensible project-specialist default:
```
"Glob", "Grep", "Read", "Edit", "Write", "Bash", "Agent", "TaskCreate", "TaskUpdate", "TaskList", "TaskGet"
```

`model` — `"opus"` (default; do not ask unless user overrode).

`role` — 2-3 sentences you compose from the answers. Template:

> You are the {slug} specialist. You own {one-line description}. Stack: {Q1}. Deploy: {Q2 or "local-only"}. Hand off generic dev work to `kei-code-implementer`, numerical work to `kei-ml-implementer` (if ML), and infra to `kei-infra-implementer` (if deployed).

`domain_in` — 5-8 lines derived from Q1/Q2 stack/deploy specifics. Use the
existing generic manifests in `_manifests/` as shape references. If unsure,
include at minimum: stack location, memory paths, deploy target, and any
cost/paid-API context.

`forbidden_domain` — start with the user's Phase-2 gotcha list (one line each,
verbatim, quoted), then append the standard hardlines every project-specialist
inherits:
- `"\`git push\` to public hosting for any sensitive-IP project"`
- `"\`git add -A\` — stage specific files only"`
- If Q5 == Yes: `"Echoing any secret from <secrets-dir> in chat — reference paths only"`
- If Q4 == Yes: `"Running paid training without cost estimate + single-variant verify first"`
- If Q6 == "Paid tier": `"Paid scraper batch >100 items without \`kei-cost-guardian\` pre-run cost estimate"` + `"LinkedIn paid scrape without legal-review sign-off (BGH Germany Nov 2024 GDPR risk)"`
- Files >200 LOC without decomposition (Constructor Pattern).
- Picking a non-default language without a documented reason.

`output_extra_fields` — 5-8 lines; reasonable defaults:
- `"Stack touched: <{Q1}>"`
- `"Deploy impact: <{Q2} — yes/no>"`
- `"Language: <Rust | <other> + reason>"`
- `"Tests added / updated: <path:test_name — pass/fail — reproduce cmd>"`
- `"Checkpoints: <commit-sha> — <description>"`
- If Q3 == Yes: `"Cost estimate: <$N.NN | N/A>"`
- If Q4 == Yes: `"Math-first expression: <1-3 lines LaTeX>"`
- If Q5 == Yes: `"Secrets referenced only (no echo): yes/no"`

---

## Phase 3.5 — Final name confirmation (AskUserQuestion, ONE call)

Before writing the manifest, give the user one explicit chance to confirm or override the agent name. Send this `AskUserQuestion`. Substitute the literal slug from Phase 2 into every option label so the user sees, for example, `kei-myapp-specialist` (NOT the literal `kei-<slug>-specialist` placeholder).

```json
{
  "questions": [
    {
      "question": "Use proposed agent name?",
      "header": "Name",
      "multiSelect": false,
      "options": [
        {"label": "kei-<slug>-specialist", "description": "Proposed default — matches KeiSeiKit kit-prefix convention"},
        {"label": "<slug>-specialist",     "description": "Without kei- prefix (user-namespace, won't collide with kit names)"},
        {"label": "Specify custom name",   "description": "Enter arbitrary name as one free-text line (must match [a-z0-9-]{3,40})"}
      ]
    }
  ]
}
```

Resolve the final name as follows:

- **`kei-<slug>-specialist`** — use as-is.
- **`<slug>-specialist`** — use as-is.
- **Specify custom name** — follow up with ONE free-text prompt: `Enter the agent name (lowercase [a-z0-9-], 3-40 chars, no double-dash, no leading/trailing dash).`
  Validate strictly:
  - Regex: `^[a-z0-9]([a-z0-9-]*[a-z0-9])?$` (forbids leading/trailing `-`).
  - Length: 3-40 chars.
  - No `--` anywhere.
  - Invalid → report the failing check and re-ask the same question; do NOT fall through to a default.
  - No `-specialist` suffix is auto-appended. Whatever the user types IS the final name.

Store the resolved value as `FINAL_NAME`. All subsequent phases use `FINAL_NAME` in place of `<slug>-specialist` when writing the manifest, running the assembler, and reporting.

---

## Phase 3.6 — Cognitive modes (AskUserQuestion, ONE call, optional)

Cognitive-mode blocks (`_blocks/mode-*.md`) add a behavioural skew to the generated agent. They compose — multi-selection is expected. **Default: pick NONE** if unsure; modes are not free (each lands verbatim in the prompt).

See `_blocks/mode-matrix.md` for the recommended starting set per agent role.

```json
{
  "questions": [
    {
      "question": "Add cognitive mode blocks?",
      "header": "Modes",
      "multiSelect": true,
      "options": [
        {"label": "skeptic — doubt-first",            "description": "Ask 'what's the evidence?' on every claim. Good for critics, validators, researchers."},
        {"label": "devils-advocate — steel-man opposite", "description": "Name the strongest objection before agreeing. Good for security auditors, code reviewers."},
        {"label": "minimalist — subtract over add",    "description": "Justify every addition against existing code. Good for refactor, architect, ml-implementer."},
        {"label": "maximalist — 10× version",          "description": "Return both maximum and minimum bounds. Good for brainstorm / concept-exploration agents."},
        {"label": "first-principles — derive from invariants", "description": "Cite the physical / mathematical constraint, not 'best practice'. Good for architects, physicists, systems-designers."}
      ]
    }
  ]
}
```

Resolve: map each selected label to its block name (`mode-skeptic`, `mode-devils-advocate`, `mode-minimalist`, `mode-maximalist`, `mode-first-principles`) and APPEND them to the `blocks` array computed in Phase 3.1 — after the stack/deploy/domain blocks, in the order the user picked them.

If the user selected nothing — skip. The manifest is valid with zero mode blocks (the 12 existing kit manifests ship that way).

Pre-flight still applies: Phase 3.2 existence-check must cover any mode blocks added here.

---

## Phase 4 — Fill the template + write the manifest

1. Read `~/.claude/agents/_templates/specialist.toml.template` via the Read tool.
2. Replace EVERY `{{PLACEHOLDER}}` with the computed value. For list-shaped
   placeholders (BLOCKS, DOMAIN_IN, FORBIDDEN_DOMAIN, OUTPUT_EXTRA, REFERENCES):
   produce TOML-quoted strings, one per line, with trailing commas,
   4-space indented — match the style of the existing manifests in `_manifests/`.
3. For `TOOLS`: produce a comma-separated list of quoted strings inline, e.g.
   `"Glob", "Grep", "Read", "Edit", "Write", "Bash"` (no trailing comma).
4. For `HANDOFFS`: emit a sequence of `[[handoff]]` tables, one per downstream
   agent, each with `target = "..."` and `trigger = "..."`, separated by blank
   lines. Example:
   ```toml
   [[handoff]]
   target = "kei-code-implementer"
   trigger = "generic dev work outside this project's stack"

   [[handoff]]
   target = "kei-critic"
   trigger = "anti-pattern / Constructor Pattern sweep on diffs >200 LOC"
   ```
5. Write the filled manifest to
   `~/.claude/agents/_manifests/<FINAL_NAME>.toml` via the Write tool.
   (The `name = "..."` field inside the manifest MUST also equal `FINAL_NAME` —
   the assembler uses this as the single source of truth for both the generated
   `.md` filename and the frontmatter `name:` value.)

CRITICAL invariants (re-check before Write):
- All top-level keys appear BEFORE any `[[handoff]]`.
- All `[[handoff]]` tables appear BEFORE `[references]`.
- `[references]` is the FINAL section (nothing after it except the closing `]`).
- `blocks` contains all three obligatory blocks: `baseline`, `evidence-grading`, `memory-protocol`.
- At least one `[[handoff]]` is present.
- `domain_in` and `forbidden_domain` each have >=1 entry.

---

## Phase 5 — Validate + assemble

Run validate first, assemble only on success:

```bash
~/.claude/agents/_assembler/target/release/assemble --validate ~/.claude/agents/_manifests/<FINAL_NAME>.toml
```

If validate FAILS:
- Show the error verbatim to the user.
- Do NOT run `--in-place`.
- Offer constructive paths:
  (A) I fix the manifest and re-validate — show the diff first.
  (B) You edit the manifest yourself; I re-validate after.
  (C) Abort and delete the manifest.

If validate PASSES:

```bash
~/.claude/agents/_assembler/target/release/assemble --in-place ~/.claude/agents/_manifests/<FINAL_NAME>.toml
```

This writes `~/.claude/agents/<FINAL_NAME>.md` (the generated agent file).

---

## Phase 6 — Report

Show a concise block to the user. `<FINAL_NAME>` is the name resolved in Phase 3.5 (default `kei-<slug>-specialist`, or the user's override).

```
Agent generated: <FINAL_NAME>
  Blocks:    baseline, evidence-grading, memory-protocol, rule-pre-dev-gate,
             <stack>, <deploy or —>, <domain blocks>
  Handoffs:  kei-code-implementer, kei-critic, kei-validator, <conditional ones>
  Manifest:  ~/.claude/agents/_manifests/<FINAL_NAME>.toml
  Generated: ~/.claude/agents/<FINAL_NAME>.md
  Memory:    ~/.claude/memory/<slug>-project.md (not yet created — adjust path if your memory layout differs)

Edit the MANIFEST, not the .md — the no-hand-edit-agents hook will block direct .md edits.
```

---

## Phase 8 — Project bridges (optional, click-only)

After reporting the new agent, offer to generate cross-tool bridge files for the project's working tree (so Cursor, Copilot, Aider, Windsurf, Junie, Continue, Gemini/Antigravity, Replit, Codex CLI, Warp, Zed all see the same Constructor-Pattern ruleset). Send this `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Generate cross-tool bridges for this project?",
      "header": "Bridges",
      "multiSelect": false,
      "options": [
        {"label": "Yes — all 11",        "description": "Cursor (legacy + MDC), Copilot, Codex, Windsurf, Junie, Continue, Aider, Replit, Antigravity/Gemini, Warp, Zed — one Constructor-Pattern ruleset across every AI coding tool"},
        {"label": "Yes — AGENTS.md only", "description": "Minimal — only the universal AGENTS.md that most modern tools read"},
        {"label": "No — skip",            "description": "Agent-only install; user will generate bridges later via install.sh --with-bridges or _bridges/emit.sh"}
      ]
    }
  ]
}
```

Resolve:

- **Yes — all 11** — invoke:
  ```bash
  ~/.claude/agents/_bridges/emit.sh "<project-path-from-Q2>"
  ```
  Use the project path the user gave in Phase 2. The helper auto-derives `PROJECT_NAME` from the directory basename and `PROJECT_DESCRIPTION` from the first non-blank line of the project's `CLAUDE.md` or `README.md`.

- **Yes — AGENTS.md only** — invoke the same helper with the `--only` filter:
  ```bash
  ~/.claude/agents/_bridges/emit.sh --only AGENTS.md "<project-path-from-Q2>"
  ```

- **No — skip** — print the exact command the user can run later and fall through to Phase 7:
  ```
  To generate bridges later:
    cd <project-path> && ~/.claude/agents/_bridges/emit.sh "$PWD"
  Or at install time:
    cd <project-path> && <kit-repo>/install.sh --with-bridges
  ```

All three options are idempotent — existing bridge files in the project are skipped, never overwritten. Report which files were created / skipped verbatim from `emit.sh` output.

---

## Phase 7 — Suggested next steps (print, do NOT execute without ask)

Offer as a final block the user can copy-paste:

```bash
# 1. Create project memory file (adjust path to your memory layout)
touch ~/.claude/memory/<slug>-project.md

# 2. Add one-line entry to your MEMORY.md index under "## Projects"
#    e.g. [[<slug>-project]] — <one-line description>

# 3. Commit the new agent
cd ~/.claude && git add \
  agents/_manifests/<FINAL_NAME>.toml \
  agents/<FINAL_NAME>.md \
  && git commit -m "feat: new agent <FINAL_NAME>"
```

Ask the user: "Run the three commands now? (yes / edit first / skip)"
- yes → execute via Bash.
- edit first → stop; they will run manually after editing.
- skip → stop.

---

## Rules (apply throughout the wizard)

- NO DOWNGRADE: every failure mode above must return constructive paths, not "can't do it".
- PLAN MODE FIRST: this skill IS the plan — each phase is a step with a verify-criterion.
- Constructor Pattern: the generated manifest must stay <200 lines; if your composition exceeds that, split a block instead of stuffing more into the manifest.
- Surgical changes: do NOT touch other manifests, other agents, or unrelated files during the wizard.
- Never fabricate block names or handoff targets that don't exist on disk — verify via Phase 3.2 before writing.
