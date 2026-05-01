---
name: research
description: Deep research on any topic using parallel agents, web search, and cross-referencing. Use when user asks to research, investigate, or deeply analyze a topic, technology, library, or concept. Triggers on keywords like "research", "investigate", "deep dive", "find out everything about". Supports `--angle` presets for common specializations (competitors, design-refs).
argument-hint: <topic or question> [--angle=competitors|design-refs]
---

## Angle presets (Phase-5 specializations)

`/research` is the source-of-truth skill for deep research. Two common
specializations of Phase 5 are available as `--angle` presets — pick via
`AskUserQuestion` in Phase 1 (or surfaced directly from the CLI):

| Angle | Phase-5 focus | Replaces (deprecated) |
|---|---|---|
| `--angle=competitors` | `practical` + `arch-analyst` + `trends` weighted; kill-list includes marketing-only competitor pages | `/competitor-analysis` |
| `--angle=design-refs` | `web-researcher` weighted toward Awwwards/Godly/SiteInspire; `arch-analyst` weighted toward visual archetype + motion tier; `kei-critic` checks stale AI-slop patterns | `/design-inspiration` |
| (none — default) | Full 9-angle verification (numbers / critique / competitors / docs / stacks / competitor-arch / academic / intersections / trends) | — |

The preset is a Phase-1 question option; it does not add a new pipeline —
it just re-weights the Phase-5 teammate mix. Users who want the old
standalone skills can still invoke `/competitor-analysis` or
`/design-inspiration` (both route here).


> **Role-tag convention.** Names like `web-researcher`, `meta-critic`,
> `arch-analyst`, `{component}-critic` that appear later in this skill are
> ad-hoc role tags passed to the generic `kei-researcher` subagent inside
> its prompt — they are NOT separate manifests in the kit. Do not grep for
> them in `_manifests/`; they will not be found. The only manifest behind
> every research teammate is `kei-researcher`.

# Deep Research Skill

You are conducting deep research on: $ARGUMENTS

> [REQUIRES: optional knowledge vault] This skill persists research output to
> a markdown knowledge vault at `$KNOWLEDGE_VAULT` (conventional default:
> `$HOME/Projects/KnowledgeVault/`). If you don't maintain a vault, use any
> stable directory (e.g. `$HOME/research/`) — the skill will create
> subdirectories as needed. Skip the vault-read phases if empty.
>
> [REQUIRES: team tools] The skill uses Claude Code's team orchestration
> tools (`TeamCreate`, `TaskCreate`, `TaskUpdate`, `TeamDelete`). If your
> Claude Code install lacks team tooling, run the phases sequentially with
> single agents instead of parallel teammates.
>
> [OPTIONAL: project registry] If you maintain a project registry (e.g. in
> `$HOME/.claude/CLAUDE.md` or `$HOME/.claude/memory/MEMORY.md`), the
> `intersections` agent in Phase 5 reads it to find cross-project links.
> Skip that agent if you have no registry.

## PHASE -1: MANDATORY USER CHOICE — Variant + Control Level

> **BEFORE any work**, ask the user two questions via `AskUserQuestion` (terminal option-picker, NOT free-text).
> **Send BOTH questions in ONE `AskUserQuestion` call** (questions array of 2) so the user picks both in one prompt.

### Question 1 — "Research depth variant"

Options (pick ONE):

- **A — Light (2-stage, ~15 min)** — Discovery wave → verification pass. 3-5 agents total. Structured report, no graph, no 9-angle verification. Use for: quick fact-checks, tool comparisons, shallow context.
- **B — Standard (8-phase, ~40 min)** — Current default. Discovery (5 agents) → cross-ref → report → vault save → **9-angle verification wave** → synthesis → graph indexing → cleanup. Full report + graph.json + intersections.md. Use for: technology choice, architectural decisions, competitor mapping.
- **C — Deep Decomposition (wave-based, ~1-2h)** — Wave 0 decomposes question → Wave 1 parallel angle exploration per component → Wave 2 "two-touches rule" expansion (find what was missed; expand each finding via 2 touches upstream/downstream) → Wave 3 cross-analysis + re-expansion of found paths. Each agent writes its own file. Mandatory self-cleanup at end. 15-30+ agents. Outputs component-report-per-file + master synthesis + graph + intersection matrix. Use for: deep-domain research, new domain entry, major strategic decisions.

### Question 2 — "Control level"

Options (pick ONE):

- **1 — Full control** — Lead confirms EVERY agent spawn before it fires. You (lead) present a preview of the next wave's tasks and wait for user approval. Safest; slowest. Use when topic is sensitive, cost-critical, or decomposition untrusted.
- **2 — Confirm branches only** — Lead spawns Wave 1 + discovery automatically. Confirms ONLY at (a) verification-wave start, (b) two-touches expansion branches, (c) any NEW domain a finding opens. Middle ground.
- **3 — Max autonomy** — Lead self-approves all waves + branches via internal critique + confidence grading. User sees final report only. Fastest; requires trust. Use when topic well-scoped and user is away.

### How to send (one AskUserQuestion call, 2 questions)

```json
{
  "questions": [
    {
      "question": "Research depth variant?",
      "header": "Depth",
      "multiSelect": false,
      "options": [
        {"label": "A — Light (2-stage, ~15 min)",            "description": "Discovery + verification. 3-5 agents. Report only."},
        {"label": "B — Standard (8-phase, ~40 min)",          "description": "Full current flow. 9-angle verification. Graph + intersections."},
        {"label": "C — Deep decomposition (wave-based, ~1-2h)", "description": "Decompose → per-component → two-touches → cross-analysis. 15-30+ agents."}
      ]
    },
    {
      "question": "Control level?",
      "header": "Control",
      "multiSelect": false,
      "options": [
        {"label": "1 — Full control (confirm each spawn)", "description": "You approve every agent before it fires."},
        {"label": "2 — Confirm branches only",             "description": "Auto discovery. Ask before verification + new branches."},
        {"label": "3 — Max autonomy",                      "description": "Self-approve via internal critique. Final report only."}
      ]
    }
  ]
}
```

Route to matching section based on pair {A|B|C, 1|2|3}:
- A → "## Variant A — Light" (below)
- B → original 8-phase flow (current Phase 0-8)
- C → "## Variant C — Deep Decomposition" (below)

Control-level logic applies to ALL variants:
- **L1** — before EVERY `TaskCreate` + agent spawn, invoke `AskUserQuestion` with options {Approve | Modify | Skip}
- **L2** — auto-spawn discovery waves; `AskUserQuestion` ONLY before verification waves + new-branch spawns
- **L3** — no user prompts; apply `kei-critic` teammate self-evaluation on each wave output before proceeding

---

## Variant A — Light (2-stage)

**Phase 0:** Check knowledge vault (same as standard).

**Phase 1:** Spawn 3 teammates in parallel:
- `web-researcher` — WebSearch/WebFetch top 10 findings
- `kei-critic` — limitations, alternatives, failure stories
- `practical` — real-world use cases + prior art

**Phase 2:** Cross-reference + verify + confidence grading (you, lead).

**Phase 3:** Structured report + save to `$KNOWLEDGE_VAULT/research/{topic}-light/`.

**Phase 4:** Cleanup — `TeamDelete`. No graph, no 9-angle, no two-touches.

---

## Variant C — Deep Decomposition (wave-based)

### Wave 0 — Decomposition (lead-only, no spawn)

Break the research question into 3-7 orthogonal components. Each component must be:
- Independently explorable (no circular dependency)
- Concrete (not "the whole topic" recursively)
- Evidence-gradable

Save decomposition to `$KNOWLEDGE_VAULT/research/{topic}/00-decomposition.md`. **If L1 control: show decomposition to user via `AskUserQuestion` with options {Approve | Request changes | Abort} before Wave 1.**

### Wave 1 — Per-component angle exploration (parallel spawn)

For EACH component from Wave 0, spawn 3-5 angle-specific agents:
- `{component}-web` — WebSearch/WebFetch for this component only
- `{component}-critic` — issues/limitations specific to this component
- `{component}-practical` — real-world examples of this component
- `{component}-docs` — official documentation (conditional)
- `{component}-academic` — papers/arXiv (conditional, only for tech/ML/math)

**Typical spawn count: 5 components × 4 angles = 20 agents.** Each writes to `{topic}/wave1-{component}-{angle}.md`.

**If L1 control: ask user before spawning each component's wave (5 prompts).**
**If L2 control: auto-spawn all Wave 1.**
**If L3 control: auto-spawn all, self-critique each output.**

### Wave 2 — Two-touches rule expansion

For EACH Wave 1 finding, apply "two-touches rule":
- **Touch 1** — What does this finding directly depend on? (upstream)
- **Touch 2** — What does this finding enable or block? (downstream)

Both touches MUST be explored — not just adjacent facts, but second-order consequences. If Wave 1 mentioned "Rust async uses tokio", Touch 1 = "what tokio depends on (mio, futures-rs)", Touch 2 = "what consuming tokio affects (debuggability, compile time, binary size)".

Spawn one agent per (finding × two-touches) pair, or bundle by component. Each writes to `wave2-{component}-expansion.md`.

**Also run a dedicated `{component}-gaps` agent per component:** "what was NOT covered in Wave 1 for this component? What angle is missing? What should have been asked but wasn't?"

**If L1 control: user approves expansion list.**
**If L2 control: user approves NEW domains opened by expansion.**
**If L3 control: self-evaluate which branches are worth expanding; auto-approve.**

### Wave 3 — Cross-analysis + re-expansion

Now that Waves 1-2 have generated 30-50 .md files, run 5-7 synthesis agents:
1. `cross-analyst` — which findings across components CONFLICT? Build conflict matrix.
2. `gap-closer` — which gaps from Wave 2 `-gaps` agents are still open? Can any be closed with one more agent?
3. `integration-mapper` — how do components connect to each other + to existing projects (read Project Registry)?
4. `evidence-auditor` — re-grade ALL claims across all files. Downgrade unverified. Flag `[DISPUTED]`.
5. `timing-analyst` — what's urgent, what's deferred, what's already obsolete?
6. `meta-critic` — final adversarial pass. "Is this research actually useful or just a pile of data?"

Any finding marked for re-expansion: spawn one more agent for that specific thread.

**If L1 control: user sees synthesis plan, approves.**
**If L2 control: user approves re-expansion branches.**
**If L3 control: auto-proceed.**

### Wave 4 — Master synthesis (lead)

Lead writes master document at `{topic}/MASTER-REPORT.md` containing:
- Executive summary (2-3 sentences)
- Per-component findings (linked to wave1/wave2 files)
- Cross-component conflicts + resolutions
- Project-registry intersections
- Evidence grades + confidence matrix
- Timing recommendation
- Open gaps (what COULDN'T be answered even with deep decomposition)

Plus `graph.json` cumulative update + `intersections.md`.

### Wave 5 — Cleanup (mandatory — no orphaned agents or stale files left behind)

1. `shutdown_request` to ALL teammates
2. `TeamDelete`
3. Verify no orphaned agents
4. Commit all research files to git (in knowledge vault repo if versioned)
5. Update `$KNOWLEDGE_VAULT/knowledge/research-graph.json` master index
6. Append one-line entry to `$KNOWLEDGE_VAULT/knowledge/research-index.md`

---

## CRITICAL: Team-Based Orchestration

> **ALWAYS create a team for research.** This is mandatory, not optional.
>
> **Step 1:** Call `TeamCreate` with `team_name: "research-{topic-slug}"` (e.g., "research-rust-async-runtimes")
> **Step 2:** Create tasks via `TaskCreate` for each research angle
> **Step 3:** Spawn named teammates via `Task` tool with `team_name` parameter
> **Step 4:** Assign tasks to teammates via `TaskUpdate`
> **Step 5:** Teammates work, mark tasks completed, go idle
> **Step 6:** You (lead) synthesize results, create Phase 2 tasks, assign to idle teammates
> **Step 7:** After all phases — `shutdown_request` to all teammates, then `TeamDelete`
>
> **WHY teams:** Task lists give visibility. Named teammates can be re-assigned across phases.
> Idle teammates from Phase 1 get reused in Phase 5 — no wasted spawns.
>
> **Teammate naming convention:**
> - Phase 1: `web-researcher`, `code-explorer`, `kei-critic`, `practical`, `docs`
> - Phase 5: reuse same teammates with new tasks (they keep context!)
> - If topic doesn't need code exploration, skip `code-explorer` — spawn only what's needed
>
> **Execution pattern:**
> 1. Phase 0 — you do this yourself (Read vault files)
> 2. TeamCreate + TaskCreate for Phase 1 tasks
> 3. Spawn 3-5 teammates IN PARALLEL (single message, multiple Task calls with team_name)
> 4. Assign Phase 1 tasks to teammates
> 5. Wait for teammates to complete (they send messages when done)
> 6. Phase 2-3 — you do this yourself (cross-reference + report)
> 7. Phase 4 — you do this yourself (save to vault)
> 8. Create Phase 5 tasks, assign to EXISTING idle teammates (reuse, don't spawn new)
> 9. If Phase 5 needs >5 agents, spawn additional teammates
> 10. Wait for Phase 5 completion
> 11. Phase 6-7 — you do this yourself (synthesis + graph)
> 12. Shutdown all teammates, TeamDelete
>
> **NEVER use `run_in_background: true`** — you need their results to proceed.

## Process

### Phase 0: Check knowledge vault Vault

Before web search, check existing knowledge:
1. Read `$KNOWLEDGE_VAULT/knowledge/` MOC notes for relevant existing findings
2. Search `$KNOWLEDGE_VAULT/research/` for related prior research
3. Check `$KNOWLEDGE_VAULT/knowledge/wrong-paths.md` for known dead ends on this topic
4. Use findings to focus web search on GAPS, not re-research known facts

### Phase 1: Parallel Discovery (3-5 teammates)

Create tasks and assign to teammates:

1. **web-researcher** — Use WebSearch + WebFetch to find latest articles, docs, repos, benchmarks. Return top 10 findings with URLs.

2. **code-explorer** — If topic relates to code/library, search the codebase and npm/pypi/github for implementations, examples, patterns. [CONDITIONAL: skip if not code-related]

3. **kei-critic** — Search for criticisms, limitations, known issues, alternatives. Find "X vs Y" comparisons, migration guides, deprecation notices.

4. **practical** — Find real-world usage examples, case studies, production stories. Check GitHub issues, Stack Overflow, blog posts.

5. **docs** — If a library/framework, fetch official docs. Use Context7 MCP if available for versioned docs. [CONDITIONAL: skip if not a library/framework]

### Phase 2: Cross-Reference & Validate

After teammates report back:
1. Cross-reference findings — what do multiple sources agree on?
2. Flag contradictions between sources
3. Identify gaps — what wasn't found?
4. Check dates — is information current?
5. Rate confidence for each finding (high/medium/low)

### Phase 3: Structured Report

Present findings as:

```
## Research: [Topic]

### Summary (2-3 sentences)

### Key Findings
1. [Finding] — confidence: X% — [source]
2. ...

### Architecture/How It Works
[If applicable — diagrams, data flow]

### Pros & Cons
| Pros | Cons |
|------|------|
| ... | ... |

### Alternatives Compared
| Feature | Option A | Option B | Option C |
|---------|----------|----------|----------|

### Recommendations
[Based on findings, what's the best path]

### Sources
- [URL] — [what was found]
```

### Phase 4: Save to knowledge vault + Memory

If research reveals important patterns or decisions:
- Save full research to `$KNOWLEDGE_VAULT/research/{topic}/` with YAML frontmatter and [[wikilinks]]
- Update relevant MOC notes in `$KNOWLEDGE_VAULT/knowledge/`:
  - New dead end → `wrong-paths.md`
  - New pattern → `code-patterns.md`
  - API finding → `api-integrations.md`
  - Architecture insight → `architecture-decisions.md`
- Save key findings to memory topic file
- Update MEMORY.md index if new project/technology

---

### Phase 5: 9-Angle Verification

> Re-verification of EVERYTHING found so far. Reuse idle teammates from Phase 1 + spawn new if needed.
> Goal: find errors in Phase 1-4, close gaps, open new vectors.

Create new tasks and assign to teammates (reuse existing first, spawn additional if >5 needed):

1. **web-researcher** (reuse) → **Numbers** — Verify ALL numbers, unit economics, metrics, benchmarks. Recalculate independently. Find primary sources for each figure. If numbers don't agree — mark `[DISPUTED]`.

2. **kei-critic** (reuse) → **Critique** — Devil's advocate. Find ALL reasons this WON'T work. Worst-case scenarios. Legal risks. Ethical problems. What skeptics say. Real failure stories.

3. **practical** (reuse) → **Competitors** — Find ALL competitors (not only the obvious ones). Check: market map, Crunchbase, ProductHunt, G2/Capterra. Who launched in the last 6 months? Who died? Who pivoted?

4. **docs** (reuse) → **Doc verification** — Re-read official docs, changelogs, migration guides, deprecation notices. Find undocumented features, breaking changes, roadmap items. Verify docs match real behaviour.

5. **code-explorer** (reuse) → **Tech stacks** — Deep analysis of technologies: versions, compatibility, license (MIT/GPL/proprietary), community health (stars, contributors, last commit), alternatives. Vendor lock-in risk.

6. **arch-analyst** (spawn new) → **Competitor architecture** — For each competitor: tech stack, architectural patterns, API design, infra, open-source components. Weak points. What's hard to replicate (moat).

7. **academic** (spawn new) → **Academic papers** — [CONDITIONAL: only for tech/ML/math topics] arXiv, Google Scholar, Semantic Scholar, IEEE. Original papers. Skip if topic is business/SaaS.

8. **intersections** (spawn new) → **Intersection branches** — Read Project Registry from `$HOME/.claude/CLAUDE.md` or `$HOME/.claude/memory/MEMORY.md` (if maintained). For each project: is there an intersection? Can we reuse code/knowledge/infra? What NEW directions does this research open? Skip if no registry.

9. **trends** (spawn new) → **Trends & timing** — Where is the market/technology moving? Hype cycle position. Regulation. Timing — too early or too late? Window of opportunity.

### Phase 6: Verification Synthesis

After all teammates report back:

1. **Conflict Matrix** — Build a table: where do Phase 1 and Phase 5 disagree? For each conflict: which source is more reliable?
2. **Evidence Upgrade** — Re-grade evidence (`[E1]`-`[E6]`) based on verification
3. **Kill List** — Which findings from Phase 1-4 turned out false/inaccurate? Remove from report
4. **New Findings** — What did verification find that discovery missed?
5. **Updated Report** — Update structured report from Phase 3 with verified data

### Phase 7: Graph Indexing

Build knowledge graph and save to knowledge vault:

1. **Generate `graph.json`** — Save to `$KNOWLEDGE_VAULT/research/{topic}/graph.json`:
```json
{
  "topic": "research topic",
  "date": "YYYY-MM-DD",
  "nodes": [
    {"id": "node-1", "label": "Name", "type": "technology|company|concept|person|project", "evidence": "E1-E6", "confidence": 85}
  ],
  "edges": [
    {"from": "node-1", "to": "node-2", "relation": "competes_with|depends_on|enables|blocks|intersects|replaces", "weight": 0.9, "evidence": "E1-E6"}
  ],
  "clusters": [
    {"id": "cluster-1", "label": "Cluster Name", "nodes": ["node-1", "node-2"]}
  ]
}
```

2. **Update knowledge vault wikilinks** — Ensure all nodes have corresponding `[[wikilinks]]` in research notes and MOC files

3. **Intersection Map** — Generate `intersections.md` showing connections to existing projects:
```
## Intersections with Project Registry
- [[project-name]] ↔ [finding] — potential: high/medium/low — action: [what to do]
```

4. **Update Master Graph** — Append new nodes/edges to `$KNOWLEDGE_VAULT/knowledge/research-graph.json` (cumulative index across all research sessions)

### Phase 8: Cleanup

1. Send `shutdown_request` to ALL teammates
2. Wait for shutdown confirmations
3. Call `TeamDelete` to clean up team and task list

---

## Rules
- NEVER present unverified claims as facts
- Always cite sources with URLs
- Flag when information might be outdated (>6 months)
- Present multiple viewpoints, not just one
- Evidence grade (`[E1]`-`[E6]`) for each major claim. Suggested scale: E1=primary source confirmed, E2=reproducible / multi-source agreement, E3=synthetic benchmark, E4=expert assessment / docs analysis, E5=theoretical hypothesis, E6=single unverified or stale (>6mo) source.
- Confidence percentage for each major claim
- Phase 5 verification is MANDATORY — skip only if user explicitly says "quick research"
- Graph indexing runs AFTER verification, not before (verified data only goes into graph)
- **TeamCreate is MANDATORY** — no research without a team
- **Reuse teammates across phases** — don't spawn new when idle ones exist
- **TeamDelete at the end** — always clean up
