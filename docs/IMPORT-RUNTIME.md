# kei-import — Foreign-Project Ingestion Runtime

> Take any Rust / TypeScript / Python / Go repository and decompose it into
> KeiSeiKit's substrate so its primitives, skills, and behavioural fragments
> live alongside yours under a single DNA index, single registry, single
> assembler.

## What it does

`kei-import <repo>` runs four phases in sequence and produces:

```
<output-dir>/
├── plan.md              # migration plan, foundation-first ordering
├── gap_report.md        # confident matches + weak signals + unmatched
├── skills/              # SKILL.md fragments extracted from README + docs/
├── executor-plan.json   # per-phase agent prompts (handed to Claude Code)
└── (registered rows in ~/.claude/registry.sqlite)
```

The substrate the imported repo gets folded into is the same one your
existing kit blocks live in. Once imported, foreign primitives are first-
class composition targets in the assembler, queryable in the encyclopedia,
referenceable from agent manifests as `rule_blocks` or as substrate
`blocks` entries.

## Quick start

```bash
# Local path
kei-import ~/code/some-repo --non-interactive

# URL (will git clone to a tempdir, then process)
kei-import https://github.com/foo/bar.git --non-interactive --keep-clone

# Custom output location
kei-import . --output-dir ./my-import-out

# Drier — see what would happen
kei-import . --dry-run
```

After it finishes:

```bash
# Inspect the plan
$EDITOR ./kei-import-output/plan.md

# Inspect what got registered in the registry
sqlite3 ~/.claude/registry.sqlite \
  "SELECT block_type, COUNT(*) FROM blocks GROUP BY block_type"

# Generate the human-readable encyclopedia
kei-registry encyclopedia --output docs/DNA-INDEX.md
```

## Phase-by-phase breakdown

### Phase 1 — walk + identify

`kei-import-project decompose <PATH>`

Walks the tree (respects `.gitignore`-style ignore list), classifies files
by extension, finds language manifests (`Cargo.toml`, `package.json`,
`pyproject.toml` / `setup.py`, `go.mod`), groups source files into
**modules**.

Output: a markdown table listing each module + kind + root + source-file
count.

### Phase 2 — match traits

`kei-import-project map <PATH> [--threshold 0.5] [--format markdown|json]`

For each module, parses Rust source (regex-based fingerprint extraction —
public method names, `impl Trait for Type` blocks, `use` paths), matches
against the 12 KeiSeiKit-runtime-core trait patterns:

```
ComputeProvider · AuthProvider · NotifyChannel · GitBackend
LlmBackend · ServiceManager · MemoryBackend · Scheduler
NetworkMode · Backup · CostGuard · Observability
```

Confidence score `0.6 * methods_matched + 0.4 * keywords_matched`.

Output: per-module suggested trait + confidence + matched method names.
Modules below threshold land in `## Modules below threshold`.

### Phase 3 — extract skills

`kei-import-project extract-skills <PATH> [--project-slug NAME] [--dry-run]`

Walks `README.md` + `docs/**/*.md`, splits each on H2 headings using the
priority parser chain (`architecture` → `research` → `new_project` →
fallback H2 splitter), produces an `ExtractedSkill` per fragment with:

- frontmatter `name: <project>::<section>`
- frontmatter `description: <first-200-chars-of-body>`
- body verbatim from the source markdown

Each fragment:

1. Written to `<fragments-dir>/<project>__<source>__<section>.md` as a
   valid `SKILL.md` (frontmatter + body).
2. Registered in `kei-registry` as `BlockType::Skill` with DNA composed
   via `kei-shared::compose_dna`.

Idempotent: identical body → no rewrite, no new registry row. Changed
body → supersede chain.

### Phase 4 — generate migration plan

`kei-import-project plan <PATH> [--threshold 0.5] [--output PATH]`

Clusters Phase 2's `MapEntry` results by trait family, assigns phase IDs
using foundation-first heuristic:

```
P0.x — foundation:    MemoryBackend, AuthProvider, ServiceManager
P1.x — core:          ComputeProvider, GitBackend, NetworkMode
P2.x — services:      NotifyChannel, LlmBackend, Scheduler
P3.x — application:   CostGuard, Backup, Observability
Pwip.x — needs review (confidence 0.3 ≤ x < threshold)
```

Output: HERMES-MIGRATION-PLAN.md-style markdown with STATUS BANNER + phase
table + per-phase detail (modules + verify-gate criteria) + unmatched +
follow-up.

### Phase 5 — phase executor (semi-automatic)

`kei-import-project execute <PLAN-PATH> [--ledger-db PATH] [--prereg]
                                       [--format markdown|json]`

Reads `plan.md`, generates one **agent prompt JSON** per phase. Each
prompt is a ready-to-paste body for `Agent({ subagent_type: ..., prompt:
... })` invocations from inside Claude Code (or any MCP-compatible
runtime).

The executor itself does **not** spawn agents — that lives outside this
crate, since spawning requires the host runtime's Agent tool. Use
`--prereg` to write a `queued` row per phase to `kei-ledger` ahead of
spawning, so progress is trackable.

After each agent returns:

```bash
kei-ledger done <row-id>      # success
kei-ledger fail <row-id> --reason '<msg>'   # failure
```

## End-to-end example

For a synthetic repo with one Rust crate:

```bash
$ kei-import ~/code/foreign-store --non-interactive

Phase 1 (walk + identify)...
Phase 1 complete: 1 modules.

Phase 2 (map traits)...
Phase 2 complete: 1 analyses.

Phase 3 (extract-skills)...
Phase 3 complete: 1 skills.

Phase 4 (plan)...
Phase 4 complete. Output: ./kei-import-output

Phase 5: deferred. Run `kei-import-project execute
./kei-import-output/plan.md`.

$ ls ./kei-import-output/
executor-plan.json   gap_report.md   plan.md   skills/
```

For a real repo (e.g. an open-source crate from crates.io):

```bash
$ kei-import https://github.com/dtolnay/anyhow.git --non-interactive

Phase 1 (walk + identify)...
Phase 1 complete: 4 modules.

Phase 2 (map traits)...
Phase 2 complete: 4 analyses.

Phase 3 (extract-skills)...
Phase 3 complete: 7 skills.

Phase 4 (plan)...
Phase 4 complete. Output: ./kei-import-output
```

## Guarantees

**Idempotent** — re-running on the same repo produces no new registry
rows. Changed source files → supersede chain entries.

**Append-only DNA** — every block ever registered keeps its row. Old
versions stay queryable via `kei-registry list --include-superseded`.

**Substrate-native** — imported skills end up in the same SQLite the
assembler queries when composing agent .md files. Set `rule_blocks =
["foreign-store::overview"]` in any `_manifests/<agent>.toml` and the
assembler injects the imported fragment alongside your own substrate.

**No public push** — registry stays in `~/.claude/registry.sqlite` and
the import output stays in your local working tree. Foreign code never
gets pushed back to a remote without your explicit `git add`.

## Limits & caveats

**Heuristic matching, not LLM** — Phase 2 uses regex pattern matching on
public Rust method names and dependency paths. False positives possible
on modules with method names that coincidentally match (e.g. a CRUD
helper module matching `MemoryBackend`). Manual review of the gap report
is the safety net.

**Rust-focused** — TypeScript / Python / Go modules are *identified*
(Phase 1 finds them) but not *trait-matched* (Phase 2 only parses Rust).
Future extension; not a Phase-1 release.

**Manifests skipped** — `_manifests/<agent>.toml` files are agent
declarations, not blocks. The importer ignores them; replicate them by
hand in your kit if you want the foreign agents to live alongside your
own.

**Phase 5 is plan-only** — see "Phase 5" above. The executor produces
prompt JSON; Agent invocation lives in the host runtime. If you want a
single-command import that also spawns agents, write a thin shell wrapper
that loops over the prompts JSON and pipes each into your `Agent({...})`
caller.

**License hygiene** — imported skills inherit their source repo's license.
Check the foreign repo's `LICENSE` before redistribution. KeiSeiKit
itself is Apache 2.0.

## Output schema (executor-plan.json)

```json
{
  "phases": [
    {
      "phase_id": "P0.1",
      "trait_family": "MemoryBackend",
      "agent_type": "code-implementer-rust",
      "modules": ["foreign-mem-sled", "foreign-mem-pg"],
      "prompt_text": "...full prompt...",
      "estimated_tokens_in": 4000,
      "estimated_tokens_out": 1500
    }
  ]
}
```

Each `prompt_text` follows the substrate-aware format:
- "MUST NOT invoke git/gh/bash beyond cargo check/test"
- "Implement the {trait} trait for these foreign modules"
- "Verify: cargo check + STATUS-TRUTH MARKER per RULE 0.16"
- Constructor Pattern budgets stated explicitly

## Related primitives

- **`kei-registry encyclopedia`** — generate `docs/DNA-INDEX.md` showing
  every imported block alongside your own substrate
- **`kei-decompose`** — the markdown decomposer used by Phase 3 to split
  README/docs into fragments
- **`kei-skill-importer`** — separate crate, parses external skill formats
  (OpenClaw / Cline / Cursor / Claude Code / Kimi)
- **`kei-shared::compose_dna`** — DNA wire format used to identify each
  imported block
- **`auto-register-on-edit.sh`** hook — once installed, edits to imported
  fragments auto-refresh their registry rows
- **`auto-encyclopedia-refresh.sh`** hook — same edits also refresh
  `docs/DNA-INDEX.md` so the committed encyclopedia tracks live state

## Troubleshooting

**"git clone failed"** — URL inputs require `git` on PATH. Tempdir is
auto-cleaned on exit unless `--keep-clone` is passed.

**"no modules identified"** — The repo has no recognised manifest at the
root (`Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`). Try
`--threshold 0.0` to see all modules including weak signals, or run
`decompose` directly to confirm the walker found anything.

**"no traits matched"** — Phase 2 is Rust-only. For TS / Python / Go
repos, Phase 2 produces empty matches; Phase 3 + 4 still run. Plan will
list every module under "unmatched."

**Plan is "scaffolding" everywhere** — That's the initial state per
RULE 0.16 STATUS-TRUTH MARKER conventions. Phase 5 executor's agents
upgrade phases to `partial` or `functional` as they land.

**Registry already has rows from a previous import** — Idempotent. Re-
running on the same repo path produces no new rows. The `path` column on
each row distinguishes which repo each block came from.

## Related docs

- [`docs/INSTALL.md`](./INSTALL.md) — kit-wide install paths
- [`docs/DNA-INDEX.md`](./DNA-INDEX.md) — current DNA encyclopedia
  (auto-refreshed on every substrate edit)
- [`docs/RULES-AS-BLOCKS.md`](./RULES-AS-BLOCKS.md) — Wave 14 rules-as-
  substrate (the same composition pattern Phase 3 builds on)
- [`HERMES-MIGRATION-PLAN.md`](../HERMES-MIGRATION-PLAN.md) — reference
  manual port of an external project (the format Phase 4 emits)
