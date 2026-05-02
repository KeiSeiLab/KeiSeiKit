# Convergence Plan — KeiSeiKit Substrate Unification

**Date:** 2026-04-23
**Status:** Synthesis of two parallel convergence audits + user directive "постарайся унифицировать"
**Hypothesis (user):** "If substrate is right, the primitive count converges, not expands"

---

## Headline

| Layer | Current | After Consolidation | Cut |
|---|---|---|---|
| Atom crates | 25 | 15-17 | −36% |
| Agent capabilities | 11 | 7 | −36% |
| Skills | 39 | ~28 | −28% |
| **Total** | **75** | **~52** | **−31%** |

Less aggressive than user's gut (~35) because audit shows genuine domain-disjointness in some clusters (setup pipelines, motion/animation). User's hypothesis partially confirmed — substrate does converge, but only where LOC overlap is real. Where it isn't, we get shared blocks + preamble extraction instead.

---

## Convergence by layer

### Atoms (25 → 15-17)

Source: `docs/CONVERGENCE-AUDIT-ATOMS.md` (architect, E2).

| Consolidation | From | To | When | Effort |
|---|---|---|---|---|
| **SQLite-CRUD engine** | 6 crates (kei-chat/content/social/sage/task/crossdomain-store) | 1 `kei-entity-store` engine + 6 thin schema plugins | **parallel with Stream B** (not before) | 2-3 days |
| **Provisioner unification** | 3 shells (provision-hetzner, provision-vultr, harden-base) | 1 `kei-provision <backend>` Rust (harden-base stays — different lifecycle) | **v0.24 now** | 0.5-1 day |
| **Frontend meta-dispatcher** | 4 shells (design-scrape, live-preview, figma-tokens, frontend-inspect) | thin `kei-frontend fetch\|analyze` shim, keep 4 impls underneath | post-unlock, optional | 0.5 day |
| **4 unmapped LBM-port crates** | unknown | audit after mapping | post-unlock (2026-06-03+) | TBD |

**Evidence:** `Store::open` is byte-identical across 5 crates (`kei-chat-store/src/store.rs:13-21` et al). FTS re-index literal-same body. Deps 7-liner identical. ~500 LOC duplication removed.

**Timing constraint (RULE 0.5):** DO NOT consolidate SQLite-CRUD before Stream B atom refactor of remaining 24 crates lands. Fragment-then-aggregate dual-direction = merge hell.

### Capabilities (11 → 7)

Source: `docs/CONVERGENCE-AUDIT-CAPS.md` (critic, E2-E4).

| Consolidation | From | To | When |
|---|---|---|---|
| **scope::path-filter** | scope::files-whitelist + scope::files-denylist | `scope::path-filter { mode: include\|exclude, patterns: [...] }` | post-unlock (schema amendment) |
| **quality::cargo-green** | quality::cargo-check-green + quality::tests-green | `quality::cargo-green { verbs: [check, test, clippy, fmt] }` — future-proofs clippy + fmt | post-unlock |

**Kept separate with rationale:**
- `output::report-format` + `output::severity-grade` — different validator shapes (section headers vs per-finding enum). Merge into generic `output::validator` pushes complexity down without saving files.
- `tools::read-only` + `tools::cargo-only-bash` — different gate mechanisms (tool-name allow-list vs argv regex). Rename for clarity only: `tools::read-only` → `tools::deny-tools`, `tools::cargo-only-bash` → `tools::bash-allowlist`.
- `policy::no-git-ops` — collapse `policy::` + `safety::` namespace ONLY IF no planned `policy::` additions. Check `_manifests/*.toml` before collapsing.

**Final 7:** `safety::no-git-ops`, `safety::no-dep-bump`, `scope::path-filter`, `quality::constructor-pattern`, `quality::cargo-green`, `output::report-format`, `output::severity-grade`, `tools::deny-tools`, `tools::bash-allowlist` — actually 9 if we keep policy as safety namespace + don't merge tools. Honest count: **7-9 depending on namespace-merge decision**.

### Skills (39 → ~28)

Source: same audit.

| Consolidation | From | To | When |
|---|---|---|---|
| **/audit `<target>`** | /seo-audit + /a11y-audit + /perf-audit + /responsive-audit + /pr-review + /security-review (6) | 1 skill + checklist registry `checklists/<target>.md` | post-unlock — **largest LOC win (~50%)** |
| **Deprecate /site-builder** | /site-builder (subset of /site-create) | `[DEPRECATED: use /site-create]` header | pre-unlock |
| **Deprecate research presets** | /competitor-analysis + /design-inspiration (both are /research Phase-5 specializations) | Document as `/research --angle=competitors` / `--angle=design-refs` | pre-unlock |
| **/animate gateway** | motion-design + scroll-animation + web-effects + ai-animation | Add top-level `/animate` router (40 LOC). Keep 4 existing (domain-disjoint matrices) | pre-unlock |

**Kept separate with rationale:**
- Setup pipelines (ci-scaffold, auth-setup, observability-setup, docs-scaffold, schema-design) — block sets disjoint (`ci-*.md` vs `auth-*.md`). Unified `/scaffold <domain>` would just be a switch. Reject consolidation. Extract `_blocks/pipeline-5phase-template.md` instead.
- Motion/animation family — each has different decision matrix over different libraries. Mega-skill would be 1000+ LOC.

---

## Execution plan

### 🟢 Pre-unlock quick wins (non-breaking, ship NOW)

These land during the current schema-lock window (before 2026-05-14 for agent substrate; before 2026-06-03 for atom substrate). Zero risk of breaking the contract.

| # | Task | Effort | Locks affected |
|---|---|---|---|
| 1 | Extract `_blocks/pipeline-5phase-template.md` — shared preamble for ci/auth/obs/docs/schema pipeline skills | 0.5 day | none |
| 2 | Extract `_blocks/rule-pure-click-contract.md` — AskUserQuestion contract referenced by 5+ skills | 0.5 day | none |
| 3 | Rename capabilities for clarity (`cargo-only-bash` → `bash-allowlist`, `read-only` → `deny-tools`). Keep old names as deprecated aliases | 0.25 day | none (additive) |
| 4 | Deprecate `/site-builder` with `[DEPRECATED: use /site-create]` header | 0.1 day | none |
| 5 | Deprecate `/competitor-analysis` + `/design-inspiration` as presets pointing to `/research` | 0.1 day | none |
| 6 | Add `/animate` gateway skill (40 LOC router) | 0.25 day | none |
| **7** ✓ | **Cluster 2: Provisioner unification `kei-provision <backend>`** | **1 day** | **atom substrate — but additive, not removing existing scripts** |

**Total pre-unlock: ~3 days**. All non-breaking, all safe to parallel-agent.

### 🟡 Mid-cycle (parallel with Stream B continued refactor)

During the substrate v1 atom-lock window as remaining 24 crates get their atom layout (Stream B extension).

| # | Task | Effort |
|---|---|---|
| 8 | **Cluster 1: SQLite-CRUD engine extraction** — `kei-entity-store` + 6 plugins. Piggyback on Stream B atom decomposition | 2-3 days |
| 9 | Enumerate + audit remaining 4 v0.14 LBM-port crates not in the 6 | 0.5 day audit |

### 🔴 Post-unlock (after 2026-06-03 for atoms, 2026-05-14 for agents)

Requires schema amendment. Non-trivial validation before merging.

| # | Task | Effort | Validation required |
|---|---|---|---|
| 10 | `scope::path-filter { mode, patterns }` — merge whitelist + denylist | 1 day | Read `_assembler/src/` first to confirm capability-fragment injection supports parameterized mode |
| 11 | `quality::cargo-green { verbs }` — merge check + test + future clippy/fmt | 1 day | Same |
| 12 | `/audit <target>` with `checklists/<target>.md` registry | 2-3 days | Refactor 5 existing audit skills to data + data-driven skill shell |
| 13 | Cluster 3: `kei-frontend fetch\|analyze` meta-dispatcher (OPTIONAL) | 0.5 day | None — optional |
| 14 | Evaluate policy:: + safety:: namespace merge | 0.25 day | Check planned `policy::` additions first |

### Total estimated wall-time with parallel execution

- Pre-unlock wave: **~3 days** (7 tasks, fully parallel-safe)
- Mid-cycle: **~3 days** (piggyback timing)
- Post-unlock: **~5-6 days** (4 tasks, some parallel)

**Grand total: ~12 days over 6-week window = 4 days of my own work / week average.**

---

## Shared-block extraction (bonus, non-counted)

Not a consolidation per se — a DRY refactor. Each saved LOC is LOC that was duplicated across 5+ skills.

| Block | Extracted from | LOC saved |
|---|---|---|
| `_blocks/pipeline-5phase-template.md` | 5 pipeline skills | ~150 |
| `_blocks/rule-pure-click-contract.md` | 5+ skills | ~50 |
| `_blocks/evidence-grading.md` (already exists?) | RESOLVED 2026-05-02: file exists, not duplicated | TBD |

---

## Validation checklist before consolidation merge

Before accepting any task from §Post-unlock:

1. Read `_assembler/src/` — confirm capability-fragment injection handles parameterized mode (`mode: include|exclude`)
2. Grep all shipped manifests for dependencies on the to-be-renamed capability names
3. Grep all skills for references to deprecated skill names
4. `tests/substrate_integration.sh` passes post-change (no regression)
5. `tests/hook_wiring_integration.sh` passes post-change
6. New test for each consolidation: tempfile fixture → new unified primitive → expected behaviour both modes

---

## Honest audit gaps disclosed by agents

**Atom audit gaps:**
- Did not enumerate 4 of the 10 v0.14 LBM-port crates → Cluster 4 grade dropped from E2 to E3 for those 4
- LOC estimates ±20%
- Did not read `atoms/*.md` — if Stream B already encodes plug-in split, Cluster 1 may be partly in-flight

**Capability audit gaps:**
- Could not read `_capabilities/*.md` directly (Read tool enumeration issue) — verdicts based on manifest references + architecture doc
- C1-C5 verdicts inherit this — re-validate by reading `_assembler/src/` before schema amend

**Both audits graded E2-E4.** Not E1 (which would require running the consolidation and measuring). Take with appropriate humility.

---

## Convergence wisdom reference

**User's hypothesis was right directionally, calibrated wrong in magnitude.**
- Predicted 75 → ~35 (−53%)
- Actual 75 → ~52 (−31%)

Gap is skills: user assumed 39 → ~18, reality 39 → ~28. Why? Because domain-disjoint content (setup pipelines, motion/animation) doesn't compress via generic wrappers. It compresses via *shared preamble blocks* instead — which is a different technique (block composition vs verb parameterization).

**Distinguish:**
- **Verb parameterization** — when content is homogeneous, extract verb + param (scope::path-filter, quality::cargo-green, /audit <target>). Works when LOC overlap >50%.
- **Block extraction** — when structure is homogeneous but content heterogeneous, extract shared block and reference it (_blocks/pipeline-5phase-template.md). Works when preamble overlap >50% but body differs.
- **Subset deprecation** — when one skill is a strict subset of another (/site-builder ⊆ /site-create). Rare but clean.

All three techniques apply in this plan. Substrate doesn't always compress via the same primitive.
