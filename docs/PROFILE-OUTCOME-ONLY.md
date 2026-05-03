# `outcome-only` install profile

> Five-file pitch: install the outcome-tracking primitive without
> committing to anything else. No daemon, no Forgejo, no launchd, no
> hundred Rust crates, no `no-github-push` hook, no agent generation.
> If you do not like what `~/.claude/agents/ledger.sqlite` collects,
> the uninstall is a four-line shell paste at the bottom.

## What gets installed

| # | Path                                                   | Source                                | LOC |
|---|--------------------------------------------------------|---------------------------------------|-----|
| 1 | `~/.claude/hooks/agent-outcome-backfill.sh`            | `hooks/agent-outcome-backfill.sh`     | 140 |
| 2 | `~/.claude/hooks/error-spike-detector.sh`              | `hooks/error-spike-detector.sh`       |  89 |
| 3 | `~/.claude/agents/ledger.sqlite`                       | `install/sql/outcome-only-schema.sql` (or `kei-ledger init`) | n/a |
| 4 | one appended line in `~/.claude/CLAUDE.md`             | the STATUS-TRUTH MARKER instruction   |   1 |
| 5 | `_primitives/_rust/kei-model-router/target/release/kei-model-router` (deferred) | `_primitives/_rust/kei-model-router/` | n/a |

Plus a jq-merge of two hooks into `~/.claude/settings.json`:
- `PostToolUse:Agent` → `agent-outcome-backfill.sh`
- `PostToolUse:*` → `error-spike-detector.sh`

`./install.sh --profile=outcome-only --dry-run` prints exactly this
list and exits 0 without writing.

## What does NOT get installed

- 102 Rust crates (cortex, frustration-loop, sleep-layer, …)
- 67 skills, 37 agent manifests, 82 substrate blocks
- `kei-cortex` HTTP / WS daemon
- Forgejo, dev hub, Datasette, restic, mdbook, gdrive-import
- launchd plists (`disk-reclaim`, sleep-layer cron)
- `no-github-push.sh` hook (or any other Bash gate)
- substrate PATH wiring (no edits to your shell rc files)

If you later want any of those, the kit is incremental: re-run
`./install.sh --profile=core` (or heavier) and the outcome-only state
is preserved verbatim — both paths share `~/.claude/hooks/` and
`~/.claude/agents/ledger.sqlite`.

## How `kei-model-router` activates

The router is a posterior decision rule keyed on per-task-class DNA
plus a Beta posterior over `(success, total)` in `agents.outcome`.
Until you accumulate ~100 outcome rows, the router falls back to
"behaviour unchanged" — every spawn keeps whatever model the agent
manifest declares.

After ~100 rows the posterior dominates the prior and the router
starts producing concrete recommendations. You opt in by adding
`kei-model-router` to a `PreToolUse:Agent` hook later — that step is
**not** done by this profile. You stay in observe-only mode by default.

If `cargo` is on PATH at install time the binary is built into
`_primitives/_rust/kei-model-router/target/release/`. If `cargo` is
missing the build is skipped silently and the install is still
considered complete; rebuild later with:

```bash
cd _primitives/_rust/kei-model-router && cargo build --release
```

## Privacy posture

All outcome rows live in `~/.claude/agents/ledger.sqlite`. They never
leave the machine — no sync hook, no remote-push, no telemetry.
Inspect with:

```bash
sqlite3 ~/.claude/agents/ledger.sqlite \
  "SELECT id, branch, status, outcome, stubs_count, started_ts FROM agents
   ORDER BY started_ts DESC LIMIT 20;"
```

Uncomfortable with the file? `rm` it; the next install or agent run
recreates an empty schema, no other side effects.

## Uninstall

```bash
rm -f ~/.claude/hooks/agent-outcome-backfill.sh
rm -f ~/.claude/hooks/error-spike-detector.sh
rm -f ~/.claude/agents/ledger.sqlite
rm -f ~/.claude/memory/time-metrics/agent-toolstats.jsonl
# CLAUDE.md cleanup — portable across BSD (macOS) and GNU sed via awk.
# The original line `sed -i.bak '/.../,+1 d'` used the GNU `,+N` address
# extension which BSD sed does NOT support; on macOS it silently no-ops,
# leaving an orphan instruction. The awk recipe below works on both.
awk 'BEGIN{skip=0} /<!-- outcome-only profile \(KeiSeiKit\) -->/ {skip=2; next} skip>0 {skip--; next} {print}' \
    ~/.claude/CLAUDE.md > ~/.claude/CLAUDE.md.tmp \
    && mv ~/.claude/CLAUDE.md.tmp ~/.claude/CLAUDE.md
```

Both hooks exit 0 immediately when their target script is missing, so
the `~/.claude/settings.json` jq-merge entries are harmless after
`rm`. To scrub those too, drop `agent-outcome-backfill.sh` /
`error-spike-detector.sh` lines from `settings.json` by hand.

The 5th `rm` removes the sidecar telemetry JSONL the backfill hook
writes (per-agent token counts + tool stats; local-only, no network
egress, but worth deleting if you uninstalled for privacy reasons).

## Why this profile exists

A kit with 100 crates / Forgejo / launchd plists is too heavy to
evaluate. A pitch you can read in four minutes and trial in five is
not. This profile is the answer to "what is the smallest version of
KeiSeiKit that still demonstrates the outcome loop?" — and nothing more.
