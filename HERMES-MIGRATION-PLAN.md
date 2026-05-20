# Hermes → KeiSeiKit Migration Plan

> Source: NousResearch/hermes-agent (MIT, Python+TS, ~645K LOC, 2684 files).
> Local clone: `/tmp/hermes-research/hermes-agent/`.
> Research: 7 parallel Explore agents, 2026-04-28.
> Author: orchestrator session synthesis.

---

## STATUS BANNER (post-audit, 2026-04-28 — RULE 0.16 self-application)

> **SCAFFOLDING SHIPPED — ~52% functional coverage across 7 phases.**
> Honest reconciliation after `feat/hermes-batch-2026-04-28` audit by 7 kei-critic agents.

| Phase | Goal coverage | Status (RULE 0.16) | cargo-check | Top remaining gap |
|---|---|---|---|---|
| P0.2 export-trajectories | 55% | partial | PASS | 3-turn hardcode, `From::Tool` never used, 832 LOC vs ≤200 budget |
| P0.3 README Hermes column | 70% | partial | n/a | Verified TRUE [E1 source] — no edits required after Hermes claim re-grep |
| P1.1 OpenAI-compat | **25%** | **scaffolding** | PASS (after fix) | Echo stubs in all handlers; real `chat_stream::run_loop_stream` exists at `handlers/chat.rs:13` but unwired; `main.rs:98` lacks `into_make_service_with_connect_info` |
| P1.2 Daytona | 55% | partial | PASS | No Modal backend in repo to compose alongside; REST paths unverified vs Daytona OpenAPI; FileSync not wired into acquire/release |
| P2.1 injection-guard | 55% | partial-wrong-wire | PASS | Wired to `cmd_backlog --add` (RULE 0.14 CRUD), NOT to `ingest::insert_event` or `kei-pet::memory` (real memory writes) |
| P2.2 memory-nudge | **25%** | **dead-code** | PASS | Zero callers in handlers; `Invoker` trait has no production impl; `MemoryStore` Arc not plumbed; `from_context` returns invoker=None → `spawn_review` early-returns |
| P3.1 kei-skills | 30% | dead-code | PASS | Zero downstream consumers; kei-mcp re-implements skills-as-MCP via raw walkdir, bypassing kei-skills entirely |
| P3.4 kei-ledger v8 | 80% | partial-write-only | PASS | Real SQL + 5 funcs + 6 tests; no caller until Phase D nightly job built |
| P4.1 kei-gateway | 40% | scaffolding | PASS | 9 `todo!()` panics in TG/Discord/Slack adapters; only CLI real; `agent_cache` field DEAD in runner; blake3 hash unused in production path |
| P4.2 kei-cron-scheduler | **85%** | **functional** | PASS | Parser+job+runner real, no stubs. Minor: 4 `matches!` no-op tests need `assert!`; 3 scheduling abstractions in kit (smell) |

**Hermes "no auto-extraction" claim re-verified [E1 source code]**: no edits required to README footnote or §"Honest delta vs Hermes". Verification by exhaustive grep of `/tmp/hermes-research/hermes-agent/` for `extract_skill`, `auto_save_skill`, post-task hooks, plus inspection of sister `NousResearch/hermes-agent-self-evolution` (DSPy+GEPA prompt optimization, NOT trajectory→skill extraction; separate repo, no integration).

**RULE 0.16 SHIPPED-VS-FUNCTIONAL DRIFT** codified 2026-04-28 in response to this audit. Three layers: agent STATUS-TRUTH MARKER footer + `~/.claude/hooks/agent-stub-scan.sh` (WARN 7d → ENFORCE) + orchestrator pre-commit cargo gate. Belt+suspenders+chastity-belt against repeating this drift.

**Functional follow-ups (in priority order)** to take any phase from `partial`/`scaffolding` to `functional`:
- P1.1.b: wire `chat_stream::run_loop_stream` into OpenAI handlers (~4-8h) — biggest user-visible win
- P2.1.b: re-wire injection_guard to `ingest::insert_event` + `kei-pet::memory` real write paths (~2h)
- P2.2.b: implement `Invoker` for `kei-anthropic` + plumb `MemoryStore` Arc + call `maybe_trigger` from chat handler (~1d)
- P3.1.b: replace kei-mcp's raw walkdir with `kei_skills::SkillRegistry` consumer (~3-4h)
- P0.2.b: parse chatlog into multi-turn ShareGPT (split on tool boundaries, emit `From::Tool`) (~1d)
- P4.1.b: real teloxide / serenity / slack-morphism adapter implementations (3-4d each)

---

---

## TL;DR — what we take, what we drop

| Hermes feature | Verdict | Effort | KeiSei gap? |
|---|---|---|---|
| OpenAI-compat `/v1/chat/completions` + `/v1/responses` (axum) | **P0 TAKE** | 16-25h | yes — instant frontend ecosystem |
| Daytona backend (real hibernation, not Modal-style) | **P0 TAKE** | 1-2 days | yes — Modal-only today |
| ShareGPT JSONL trajectory export from `kei-ledger` | **P0 TAKE** | 2 days | yes — community RL distribution |
| Multi-platform gateway (TG/Discord/Slack/CLI single process) | **P1 TAKE** | 10-12 days MVP | yes — adapters separate today |
| `croniter` for recurring `/schedule` (interval + cron-expr) | **P1 TAKE** | 1-2 days | yes — only one-shot today |
| Memory injection scanner (block "ignore previous" etc.) | **P1 TAKE** | 3-4 days | **security gap** |
| Periodic-nudge background memory review (every N turns) | **P1 TAKE** | 1-2 weeks | yes — runtime curation |
| `MemoryProvider` plugin trait (8+ external memory backends) | **P2 EVAL** | 2-3 weeks | yes — but our SQLite better than their builtin |
| **Phase D learning loop** (auto trajectory→skill, real self-improvement) | **P0 BUILD** | 3-5 weeks | **we go FURTHER than Hermes** |
| Plug KeiSei skills into Hermes agentskills.io taps | **P1 TAKE** | 1 day | distribution win, zero lock-in |
| ACP (agent-client-protocol) wrapper for kei-mcp | **SKIP** | — | wrong layer; ACP = editor↔agent, MCP = agent↔tool |
| Honcho integration | **P3 LATER** | unknown | external SaaS dependency |
| `delegate_task` ThreadPoolExecutor (in-process subagents) | **SKIP** | — | conflicts with RULE 0.12 worktree+ledger model |
| Atropos RL submodule | **SKIP** | — | we don't train models |
| Trajectory compressor | **P2 EVAL** | unknown | only if we add long-context summarization |

---

## Honest assessment of Hermes

**Architecture quality**: Mid. Files are massive — `run_agent.py` is **13,268 LOC**, `gateway/run.py` **11,760 LOC**, `cli.py` **11,388 LOC**. That's the opposite of our Constructor Pattern (≤200 LOC/file). **Porting means decomposing, not copying.**

**Marketing vs reality**:
- "Self-improving learning loop" — **CRUD on markdown files with manual triggers**. No automatic trajectory→skill extraction. No success-rate tracking. No background evaluator. The mechanism is `agent.write_skill_file(yaml + md)` plus `agent.patch_skill(fuzzy_replace)`. The README sells more than the code delivers.
- "Daytona AND Modal hibernate" — **only Daytona truly hibernates**. Modal volumes persist; Modal sandboxes always cold-start.
- "FTS5 full-text search" — **applies to external Honcho only**, not builtin memory. Builtin uses substring matching on markdown.

**Where Hermes IS strong**:
- Cross-platform user continuity via deterministic session-key hash (one function, ~170 LOC) — clean and correct
- 6 execution backends with pluggable interface
- Rich gateway (15+ platforms, race-condition handling via interrupt/queue/steer modes)
- OpenAI-compat HTTP server with SSE + tool-progress events to prevent hallucination during tool calls
- MemoryProvider ABC plugin discovery — clean trait surface
- Injection scanning on memory writes (security awareness we lack)

**Where KeiSei is already strong (don't regress)**:
- Constructor Pattern enforcement (≤200 LOC/file, ≤30 LOC/function)
- DNA per-run, kei-ledger fork model (RULE 0.12)
- SQLite + FTS5 + TF-IDF + pattern co-access in `kei-memory` (Hermes builtin has nothing comparable)
- Sleep-layer A/B/C (incubation / REM / deep-sleep NREM) — Hermes has no equivalent
- Ed25519 client identity / blake3(pubkey) → user_id
- Rust core, ≤2 MB binaries, type safety

---

## Detailed migration roadmap

### Phase 0 — distribution + visibility (1 week, low risk)

Goal: get KeiSei in front of users without changing core code.

**P0.1 — Plug KeiSei skills into Hermes hub** (1 day)
- Create `github.com/KeiSei84/keisei-skills` mirror in agentskills.io format (YAML frontmatter + SKILL.md)
- Document `extra_taps` install instruction in our README
- Effect: any Hermes / OpenClaw / Cursor user discovers our 45 skills via `hermes /skills search ...`

**P0.2 — ShareGPT JSONL exporter from `kei-ledger`** (2 days)
- New Rust binary `kei-export-trajectories` in `_primitives/_rust/`
- Reads `~/.claude/agents/ledger.sqlite` + chatlog files
- Emits `.jsonl` with `{conversations: [{from: system|human|gpt|tool, value}], tool_stats, prompt_index, completed}`
- ≤200 LOC, single binary, follows Constructor Pattern
- Effect: KeiSei users contribute training data to community RL ecosystems

**P0.3 — README honest competitor table update** (30 min)
- Add Hermes column to comparison table (the closest peer, not LangChain)
- Acknowledge what they do better (multi-platform gateway, plugins) — don't oversell
- Effect: trust signal for engineer-readers

### Phase 1 — frontend ecosystem unlock (2 weeks, medium risk)

Goal: any OpenAI-compatible UI talks to `kei-cortex`.

**P1.1 — OpenAI-compat HTTP routes in `kei-cortex`** (16-25h)

Add to `_primitives/_rust/kei-cortex/src/`:
```
routes/v1_chat_completions.rs   (~180 LOC)  POST /v1/chat/completions
routes/v1_responses.rs          (~180 LOC)  POST /v1/responses (stateful)
routes/v1_models.rs             (~80 LOC)   GET /v1/models
routes/v1_runs.rs               (~180 LOC)  POST /v1/runs + GET /events + POST /stop
routes/sse_streaming.rs         (~150 LOC)  tokio mpsc → axum::response::Sse
auth/bearer_token.rs            (~80 LOC)   hmac::compare via API_SERVER_KEY env
tool_translation/openai_to_kei.rs (~150 LOC) function-call schema mapping
```

Reference: Hermes `gateway/platforms/api_server.py:1-22, 1042-1172, 2620-2640`.

**Tool-progress event** (Hermes #6972) — emit `event: kei.tool.progress` during long tool calls so client doesn't hallucinate "model fell silent". Do this. It's free and we already track it in `kei-ledger`.

**Auth** — bearer + `hmac::compare_digest` against env var. If unset, allow local-only (matches Hermes default).

**Acceptance test**: Open WebUI / LobeChat / LibreChat / NextChat / ChatBox all connect and stream replies through `kei-cortex` with tool calls visible mid-stream.

**P1.2 — Daytona backend addition** (1-2 days)

Add to `_primitives/_rust/` a new crate `kei-backend-daytona`:
- Wraps Daytona REST API (the SDK is Python-only; we use HTTP directly)
- Implements `Backend` trait alongside our existing Modal backend
- Hibernation: GET /sandbox/{name} → 200 → POST /sandbox/{name}/start; on 404 → create fresh
- Volume mount: `~/.keiseikit` rsync'd before/after

Reference: Hermes `tools/environments/daytona.py:30-120`.

**Cost note**: Daytona free tier = 2 sandboxes, 30min idle hibernate. Beyond that — paid. Add to `kei-cost-guardian` checklist.

### Phase 2 — security + memory hardening (2-3 weeks, low risk)

**P2.1 — Memory injection scanner** (3-4 days)

Add `_primitives/_rust/kei-memory/src/injection_guard.rs` (~200 LOC):
- Pattern set: `"ignore previous"`, `"you are now"`, `"system:"`, `"<\\|im_start\\|>"`, curl/wget with `Authorization`/`api_key` substrings, SSH-key dump patterns, base64-encoded blobs >1KB, invisible unicode (zero-width chars, RTL override)
- Block at WRITE path in `kei-memory::store::add()` — return `Err(InjectionDetected{pattern, line})`
- Bypass: `KEI_MEMORY_SKIP_GUARD=1` (logged with reason)

Reference: Hermes `tools/memory_tool.py:90-102`.

**Test**: feed 50 known prompt-injection samples from PromptGuard / PI-Bench → expect ≥45 blocks.

**P2.2 — Periodic-nudge background memory review** (1-2 weeks)

Add to `kei-cortex` agent loop:
- Counter `_turns_since_memory_review` increments every agent turn
- At threshold `memory_nudge_interval` (default 10), spawn detached tokio task:
  - New ephemeral `Agent` with `enabled_tools=["memory_search","memory_add","memory_replace"]`, max 8 iterations, `quiet_mode=true`
  - Conversation snapshot from parent (via `Arc<RwLock<Vec<Turn>>>`)
  - Prompt: "Review the conversation. Save user-revealed facts about themselves OR explicit behavior preferences. Otherwise reply 'Nothing to save.' and stop."
  - Writes go to `kei-memory` directly via `Arc<MemoryStore>`
- Parent prints `💾 <action summary>` on completion

Reference: Hermes `run_agent.py:3147-3156, 3267-3390, 9740-9750`.

**Frozen-snapshot pattern**: memory injected into system prompt is frozen at session start. Background reviews mutate disk store but NOT the in-flight system prompt — preserves prefix cache (which is critical for cost on Anthropic's prompt-caching).

### Phase 3 — Phase D learning loop (KeiSei goes BEYOND Hermes) (4-6 weeks, high value)

**P3.1 — Skill format compatibility** (3 days)

Adopt Hermes / agentskills.io SKILL.md format:
```yaml
---
name: <slug>
description: <≤1024 chars>
category: <optional>
---

## Overview
...
## Process
1. ...
```

Add `kei-skills` crate (~600 LOC across 5 files):
- `format.rs` — YAML frontmatter + body parser (use `serde_yaml`)
- `validator.rs` — frontmatter required-field check (port `tools/skills_tool.py:172-208`)
- `patcher.rs` — fuzzy find-replace (port `fuzzy_match.py`; or use `similar` crate's diff)
- `loader.rs` — read `~/.keiseikit/skills/**/SKILL.md` at daemon start
- `registry.rs` — name-keyed in-memory store, hot-reload via inotify/fsevents

Also: `kei-skills` and Hermes interop is bidirectional — same on-disk format, same `extra_taps` distribution.

**P3.2 — Trajectory→skill auto-extraction** (2-3 weeks)

This is **THE feature Hermes claims but doesn't implement**. We build it for real.

Trigger conditions (codified in `kei-skills/src/extraction_trigger.rs`):
- Phase B (REM consolidation) just finished
- Trajectory has ≥5 tool calls AND completed=true AND total turns ≥4
- No existing skill matches >85% similarity (via embedding)
- OR explicit user opt-in via `/extract-skill` slash command

Extraction pipeline:
1. Phase B emits trajectory chunk → enqueued in `~/.keiseikit/sleep-queue/skill-extraction/`
2. `kei-skills` extractor (during Phase D, see below) loads chunk
3. Calls Anthropic / OpenRouter with prompt:
   ```
   Extract a reusable procedural skill from this task trajectory.
   Output ONLY YAML frontmatter + markdown body in agentskills.io format.
   Frontmatter: {name: <slug>, description: <≤1024 chars>, category: <one of: code-review|debugging|deploy|...>}.
   Body sections: ## Overview, ## Process (numbered), ## Pitfalls, ## Examples (verbatim from trajectory).
   ```
4. Validate output, write to `~/.keiseikit/skills/<category>/<name>/SKILL.md` atomically
5. Append to `kei-ledger` with extraction metadata (parent task ID, success metric, char count)

**P3.3 — Phase D: nightly skill self-improvement** (1-2 weeks)

Adds 4th sleep-layer phase (after A incubation / B REM / C deep-sleep NREM):

Phase D = procedural consolidation. Runs LAST in nightly cycle. Per-skill workflow:
1. Query `kei-ledger` for last-30-days usage of skill `S` (count, success_rate, time-since-last-use)
2. **If success_rate < 60% AND usage_count > 5** → re-extraction trigger
3. **If skill never used in 30 days** → archive to `~/.keiseikit/skills/_archive/`
4. **If usage > 20 AND success_rate > 90%** → mark "validated" in frontmatter (`stability: validated`)

Phase D runs Modal/Daytona serverless to keep local-Mac uninterrupted at 03:00 local. Budget: 30 min/night, 5 skills max per cycle (matches Phase B greedy-pack pattern).

**P3.4 — Skill metrics in `kei-ledger`** (3 days)

New table:
```sql
CREATE TABLE skill_invocations (
  id INTEGER PRIMARY KEY,
  skill_name TEXT NOT NULL,
  ts INTEGER NOT NULL,
  agent_id TEXT,
  success INTEGER NOT NULL,  -- 0/1, derived from agent's review.md
  trajectory_id TEXT,
  duration_ms INTEGER
);
CREATE INDEX idx_skill_invocations_name_ts ON skill_invocations(skill_name, ts);
```

Tracked at agent-loop level when skill is loaded into context.

### Phase 4 — multi-platform gateway (3 weeks, medium-high risk)

**P4.1 — Unified gateway crate** (10-12 days MVP, 14-16 days prod)

New crate `_primitives/_rust/kei-gateway/` with Constructor-decomposed adapters:

```
src/
  message.rs        (~150 LOC)  MessageEvent struct (text, source, media_urls, ts)
  session_key.rs    (~170 LOC)  build_session_key() — port hash function
  session_store.rs  (~180 LOC)  SQLite + LRU cache (sqlx + lru crates)
  router.rs         (~140 LOC)  DeliveryRouter — fan-out by platform
  guard.rs          (~150 LOC)  Per-session asyncio.Event equivalent (tokio Mutex<bool>)
  agent_cache.rs    (~150 LOC)  LRU<session_key, Arc<Agent>> with TTL
  runner.rs         (~180 LOC)  GatewayRunner — orchestrates adapters

adapters/
  base.rs           (~200 LOC)  PlatformAdapter trait
  telegram.rs       (~200 LOC)  teloxide
  discord.rs        (~200 LOC)  serenity
  slack.rs          (~200 LOC)  slack-morphism
  cli.rs            (~150 LOC)  stdin/stdout async loop
  whatsapp.rs       (~200 LOC)  axum webhook + twilio crate (later)
  signal.rs         (~200 LOC)  signal-cli subprocess bridge (later)
```

**Interrupt mode (default)**: incoming message during running agent → call `agent.interrupt(text)` → enqueue. Reference: Hermes `gateway/run.py:1678-1729`.

**Race-condition guard**: per-`session_key` `tokio::sync::Mutex<bool>` (acquired before agent run, released on completion). Stale-lock heal at adapter level if 30s stuck.

**Cross-platform user-id linking**: same `user_id` (e.g. linked TG account + Discord OAuth) → same session_key → same memory. Optional `~/.keiseikit/user_aliases.toml` for manual mapping.

**P4.2 — `croniter` for recurring `/schedule`** (1-2 days)

Add `cron` Rust crate dep. Extend `kei-sleep-queue.sh` (or replace with `kei-scheduler` Rust binary) to support:
- One-shot: `2026-05-01T14:00`, `30m`, `2h`, `1d`
- Interval: `every 30m`, `every 2h`
- Cron expr: `0 9 * * 1-5` (weekdays 9am)

Persistence: `~/.keiseikit/scheduler/jobs.json` (atomic temp+rename, fcntl locking).

Reference: Hermes `cron/jobs.py:102-209`.

### Phase 5 — optional / decide later

**P5.1 — `MemoryProvider` plugin trait** (2-3 weeks) — DEFER

Hermes has 8 external providers. Honcho is interesting (peer modeling) but requires SaaS dep. Mem0 is local-friendly. Decision: defer until ≥2 users explicitly request alternative memory backend. Our SQLite+FTS5+TF-IDF is already richer than Hermes builtin.

**P5.2 — Honcho integration** — DEFER until P5.1 (no point integrating one provider if no plugin trait).

**P5.3 — Trajectory compressor** — DEFER. Only useful when `kei-cortex` chats exceed 64K context. Current token budgets are fine.

**SKIP — ACP wrapper for kei-mcp**. Wrong abstraction layer. ACP = editor↔agent (Zed-like surface), MCP = agent↔tool. If we ever build a KeiSei-as-agent server (rather than substrate), revisit.

**SKIP — `delegate_task` ThreadPoolExecutor**. Hermes uses in-process threads with restricted toolsets. We have RULE 0.12 worktree+ledger fork — durable, auditable, parallel via real OS isolation. The Hermes pattern is a downgrade for us.

**SKIP — Atropos**. RL-training submodule. We're a substrate, not a model trainer.

---

## Sequencing & risk

### Recommended order (12-14 weeks total)

```
Week 1     P0.1 hub-tap + P0.2 trajectory-export + P0.3 README       ← distribution
Weeks 2-3  P1.1 OpenAI-compat axum routes                            ← frontend unlock
Week 4     P1.2 Daytona backend                                      ← cheap hibernation
Weeks 5-6  P2.1 injection scanner + P2.2 nudge memory review         ← security + UX
Weeks 7-9  P3.1 skill format + P3.2 trajectory→skill extraction      ← Phase D core
Weeks 10-11 P3.3 Phase D nightly + P3.4 skill metrics                ← Phase D close
Weeks 12-14 P4.1 gateway crate + P4.2 croniter scheduler             ← multi-platform
```

### Risks (severity • mitigation)

- **HIGH** Constructor-Pattern violation by porting Hermes 1:1 (their files are 11K+ LOC). **Mitigation**: every PR must pass our `≤200 LOC/file` pre-commit hook. Decomposition is part of the work, not a follow-up.
- **HIGH** Daytona free tier exhausted under load. **Mitigation**: `kei-cost-guardian` pre-launch gate; if hit, fall back to Modal volumes (no hibernation, but works).
- **MEDIUM** OpenAI-compat surface drift (OpenAI changes spec faster than we can chase). **Mitigation**: pin to `2024-10-01` schema; add CI test against Open WebUI client weekly.
- **MEDIUM** Phase D runaway extraction (1000 skills, none useful). **Mitigation**: hard cap 50 active skills total; archive policy in P3.3; user can `/skills prune`.
- **LOW** Cross-platform user-id linking false positives. **Mitigation**: opt-in via explicit `user_aliases.toml`, no auto-linking on similar names.
- **LOW** TG/Discord crate breaking changes. **Mitigation**: pin versions; `cargo deny` in CI.

### Phase D vs Hermes — why we win

| Dimension | Hermes "learning loop" | KeiSei Phase D (P3) |
|---|---|---|
| Trigger | Manual (agent calls `skill_manage(create)`) | Automatic (post-Phase-B) |
| Storage | YAML+MD on disk | YAML+MD on disk (compatible) |
| Improvement | Manual fuzzy patch | Auto re-extraction at success_rate <60% |
| Metrics | None | usage_count, success_rate, last_used |
| Archive | Never (skills accumulate forever) | 30-day-unused → `_archive/` |
| Validation | None | `stability: validated` after 20+ uses with >90% success |
| Compute | None | Modal/Daytona serverless, 30 min/night, 5 skills/cycle |

We ship the feature their README claims. Honest delta in marketing.

---

## Licensing

- All Hermes-derived code is MIT-licensed → free to copy with attribution.
- Apache-2.0 patent grant covers original kit additions (see LICENSE).

---

## Approval gates

Per RULE 0.5 (plan-mode-first), each phase requires explicit user `proceed` before code:

1. **Phase 0** (distribution) — low risk, recommend immediate proceed
2. **Phase 1** (OpenAI-compat + Daytona) — mid risk, review API-surface choices
3. **Phase 2** (memory hardening) — low risk, recommend immediate proceed
4. **Phase 3** (Phase D learning loop) — **HIGH STRATEGIC** — author-policy review FIRST, then proceed
5. **Phase 4** (gateway) — mid risk, scope-confirm before crate cluster spawn
6. **Phase 5** (optional) — re-evaluate after Phases 0-4 ship

Per RULE 0.13 (orchestrator branch first), each phase = orchestrator-created branch (`feat/p0-1-hub-taps`, `feat/p1-1-openai-compat`, etc.), agents only write files, orchestrator commits.

---

## Sources

- `/tmp/hermes-research/hermes-agent/` (NousResearch/hermes-agent @ HEAD, 2026-04-28)
- `~/Projects/KeiSeiKit/` (local, public mirror github.com/KeiSeiLab/KeiSeiKit-1.0)
- 7 parallel Explore agents, 2026-04-28 session.
