# Sleep Layer & Session Self-Audit

Day sessions → overnight consolidation → morning report. Three nightly phases on an Anthropic-cloud agent, plus an always-on session retrospective.

---

## The nightly cycle at a glance

The sleep layer is a **three-phase nightly cycle** on an Anthropic-cloud agent. The three phases run in order on the same scheduled trigger.

```
                          YOUR NIGHT
        ┌──────────────────────────────────────────────────────┐
Day →→→ │  Phase A            Phase B            Phase C       │ →→→ Morning
        │  INCUBATION         REM                NREM          │
        │  "sleep on it"      consolidation      deep-sleep    │
        │  v0.12.0            v0.11.0            v0.13.0       │
        │  (queued tasks)     (trace patterns)   (conflict     │
        │                                         refactor)    │
        └──────────────────────────────────────────────────────┘
              ↓                   ↓                  ↓
         sleep-results/     reports/sleep-*.md  sleep-deep/*.md
         <uuid>.md          (always)            (every N days)
```

**Biological analog.** Your Mac is the hippocampus (fast, stateful, volatile — captures raw episodes). The memory-repo is the transport layer. The cloud agent is the neocortex (slow, stateless, generalising). The morning `git pull` is the recall. Phase A mirrors the "sleep on it" insight effect (Wagner et al. 2004, *Nature* 427:352–355 [VERIFIED: doi:10.1038/nature02223]; the original study did not isolate a specific stage — secondary literature attributes the effect primarily to slow-wave sleep, our mapping is loose). Phase B mirrors REM dream-state pattern extraction. Phase C mirrors NREM slow-wave system consolidation.

**Phase interaction rules (important):**

- A `marathon` task in Phase A (8-hour budget, 1 task only) **owns the whole night** — Phases B and C are skipped for that night. Traces are append-only, so the next night's Phase B picks up the skipped backlog.
- Phase C only fires when today is a multiple of `DEEP_SLEEP_CRON_DAYS` (default 7) counted from your install date. Anchor lives in `sync-repo/reports/install-anchor.txt`.
- The morning report is for **HUMAN review**. It is NEVER auto-injected into a Claude Code session. Any rule or hook that emerges from it is installed via `/escalate-recurrence` — not by the cloud agent.

Governed end-to-end by 5 in `~/.claude/rules/sleep-layer.md`.

## Session self-audit (4)

KeiSeiKit auto-analyzes sessions on 3 triggers:

- **Stop event** — session ended; `session-end-dump.sh` archives the JSONL trace and ingests it into `kei-memory`.
- **Milestone commits** — `git commit -m "feat:"` / `"refactor:"` / `git merge`; `milestone-commit-hook.sh` appends a one-line session summary to `~/.claude/memory/audit-backlog.md`.
- **Error spike** — 3+ errors in the last 20 tool calls; `error-spike-detector.sh` tags the pattern and logs it.

Findings surface via click-only `AskUserQuestion`, routing to `/escalate-recurrence` (codify rule + wiki + hook), `/debug-deep` (5-phase RCA), or the audit backlog (log-only). **Silent-first**: the first 10 sessions log only — prompts activate from session 11 onward so the memory store has a useful baseline before it interrupts you. Counter lives in `~/.claude/memory/audit-backlog.md` as `<!-- session_count: N -->`.

Manual trigger: `/self-audit` skill (same flow, invoked on demand).

Requires the `kei-memory` primitive. Included in the `dev` and `full` profiles; otherwise add via `./install.sh --add=kei-memory`.

## Cloud REM sync (v0.11.0) — Phase B

Run a nightly "sleep" cycle on Anthropic's cloud — no laptop, no infra, no DevOps.

**How it works:**
- Each session: your Mac pushes trace JSONL to a private git repo you control
- 03:00 local time: a remote Claude Code agent clones the repo, analyzes the last 24h of traces, writes `reports/sleep-YYYY-MM-DD.md`, and commits back
- Next morning: `git pull` and read the consolidated findings

**Current state (2026-05-03) — what Phase B does and does not do:**

Phase B currently writes a markdown report at
`~/Projects/KeiSeiKit-public/reports/sleep-YYYY-MM-DD.md` (or the
equivalent path inside your sync-repo). The report is intended to be
**read by a human**.

**Auto-codification of rules from sleep insights is not yet
implemented.** The ContractDoc designates `/escalate-recurrence` as
the manual codification path — when you read the morning report and
spot a pattern worth turning into a rule, you invoke that skill by
hand.

When auto-codification lands, the loop will be:

```
Phase B detects pattern → opens AskUserQuestion →
  on user-confirm → writes rule + hook stub
```

This is tracked as a separate atomar; until then, Phase B is
report-only and codification is human-in-the-loop. This matches the
sleep-layer rule's "no feedback loop into agent state" invariant —
nothing the cloud agent writes is auto-injected into a session.

**Setup (one-time, ~5 min):**

1. Create an empty private repo on GitHub / GitLab / Bitbucket / self-hosted Forgejo
2. In Claude Code run `/sleep-setup`
3. The wizard generates an SSH deploy key → you paste it into the repo's deploy-key settings with WRITE access
4. The wizard emits a ready-to-paste `/schedule create` command, converted to your local 03:00 in UTC

After that, the sleep cycle runs every night automatically. The morning report is yours to read — nothing is auto-injected back into any session.

**Requires** the `kei-memory` primitive (shipped in the `dev` and `full` profiles; add via `./install.sh --add=kei-memory` otherwise). Sleep-sync scripts themselves are installed unconditionally and stay dormant until you opt in via `/sleep-setup`.

Opt in at install time with `./install.sh --with-sleep-sync` (TTY-only). Governed by 5 in `~/.claude/rules/sleep-layer.md`.

## Sleep on it (incubation, v0.12.0) — Phase A

Defer a hard question or research task to the nightly remote agent: run `/sleep-on-it`, fill in one free-text field plus three clicks (type / priority / format), submit. The task lands in `sync-repo/sleep-queue/` and the nightly agent processes it before REM consolidation.

Priority maps to a wall-clock budget. Pick the one that matches the task's difficulty:

| Priority | Budget | When to pick |
|---|---|---|
| Quick | 15 min, this night | Simple questions, fast lookups |
| Standard | 60 min, this night | Default, medium research |
| Deep | 4 hours, this night | Serious derivations, thorough prior-art |
| Marathon | Full night (up to 8 h), **1 task only** | Hard equations, full autonomy; Phase B REM skipped that night |
| Weekly batch | 60 min, next Sunday UTC | Non-urgent research |

Checkpointing: Standard / Deep / Marathon runs commit a `.partial.md` every 20–30 minutes, so if the cloud session is cut short you still get the partial on morning pull.

Typical use:
- "Should I use a continuous-time net for memory re-ranker?" → deep-research → architectural recommendation by morning
- "Compare SvelteKit vs Astro vs Next.js App Router for the kit's landing" → comparative study
- "Derive closed form for an attractor on a Stiefel manifold" → marathon mode, full night of autonomous derivation
- "What patterns in audit-backlog have highest impact?" → pattern analysis

Results in `sync-repo/sleep-results/<uuid>.md`, linked from the next morning's REM report. Biological analog: the REM-sleep "sleep on it" effect (Wagner et al. 2004, *Nature*). Queue mutations go through the `kei-sleep-queue` helper.

## Deep-sleep NREM consolidation (v0.13.0) — Phase C

A third nightly phase — **Phase C** — runs after REM on a user-chosen cadence (default: every 7 days). Biological analog: NREM slow-wave-sleep system consolidation. The remote agent scans your memory-repo for conflicts across rules, hooks, `_blocks/`, and memory (contradictory directives, overlapping hook matchers, >70%-duplicate blocks, orphaned wikilinks, Constructor-Pattern violations) and produces a structured refactor plan.

**4-primitive pipeline, in order:**

```
kei-conflict-scan  →  kei-refactor-engine  →  kei-graph-check  (via kei-store transport)
 (detect)             (propose)                 (verify)         (read/write memory-repo)
```

1. `kei-conflict-scan` reads `_rules/`, `hooks/hooks.json`, `_blocks/`, and `memory/` and emits a typed conflict list (name-collision, matcher-overlap, duplicate-block, orphan-wikilink, CP-violation).
2. `kei-refactor-engine` groups conflicts by safe-to-auto-resolve vs `requires_human_decision` and writes the plan + auto-resolve markdown.
3. `kei-graph-check` walks every wikilink / block-ref / handoff-ref in the proposed state; if anything fails to resolve, the fork branch is blocked and the plan is annotated.
4. `kei-store` is the transport — reads the pre-state from your GitHub / Forgejo / Gitea / FS / S3 backend and writes the two output files back atomically.

**Concrete example** (real category, paraphrased):

> Conflict detected: hook `.sh` (PreToolUse:Bash, matcher `git push`) and rule file `patents.md` (§"Never reference unfiled applications") both govern the same risk surface — a github push containing private language. The hook blocks on URL; the rule blocks on content. Suggested refactor: keep both (they are complementary), but add a cross-ref from `patents.md` to the hook so a future reader sees the two-layer defence. Auto-resolvable (pure documentation edit, no behaviour change). Written to `YYYY-MM-DD-autoresolve.md` for human review.

Two output modes, chosen once in `/sleep-setup` Phase 3b:

- **Plan only** (default) — markdown report in `sync-repo/sleep-deep/YYYY-MM-DD-plan.md`. Read in the morning, decide what to merge by hand.
- **Plan + fork** — same plan plus an auto-resolve review markdown (`YYYY-MM-DD-autoresolve.md`) listing the auto-resolvable conflicts with WHY / EXAMPLE / TRADEOFF per item. You open each file in an editor, apply the suggested change, commit on a `deep-sleep/YYYY-MM-DD` branch, then let the graph-check gate verify the wikilinks still resolve.

> v0.14.1 retraction: earlier README claimed a `git apply`-ready patch. The engine cannot synthesise real unified-diff hunks without reading the source files — that would risk fabricated edits (RULE 0.4). The autoresolve file is now plain markdown reviewed and applied by hand; the "fork" path only automates the rename/move class of ops, not content edits.

**Zero-conflict guarantee:** any conflict the engine marks `requires_human_decision` is EXCLUDED from the auto-resolve markdown and listed plainly in the plan. No silent auto-apply of ambiguous changes.

**Store backends** (picked in Phase 3b, consumed via the new `kei-store` trait):

| Backend | Status | Notes |
|---|---|---|
| GitHub private | production | SSH deploy key or PAT; default |
| Forgejo self-hosted | production | Same wire protocol as GitHub |
| Gitea self-hosted | production | Same wire protocol |
| Filesystem only | production | Local `.git`; no push; fastest |
| S3 / R2 / MinIO | production (v0.21, behind `s3` feature) | Real GetObject / PutObject / ListObjectsV2 via `aws-sdk-s3`. Build with `cargo build -p kei-store --features s3` and set `[s3] bucket = "..."` in `store-config.toml`. AWS default credential chain (env vars → `~/.aws/credentials` → IMDS). Custom endpoint for R2 / MinIO / Wasabi via `KEI_STORE_S3_ENDPOINT` env or `s3.endpoint` TOML field. Binary grows ~5 MB when the feature is on. Omit the feature OR omit `s3.bucket` to fall back to the v0.14 local-manifest stub (still gated by `KEI_STORE_ALLOW_S3_STUB=1`). |

Requires the new `kei-conflict-scan`, `kei-refactor-engine`, `kei-graph-check`, and `kei-store` primitives (shipped in the `dev` and `full` profiles). Governed by the Phase C extension of 5 in `~/.claude/rules/sleep-layer.md`.
