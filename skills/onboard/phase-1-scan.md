# Phase 1 — Intake + Scan

Free-text intake of target paths, then read-only Bash sweep of declarative
artefacts. Output: one structured summary per project.

## 1a — Ask for the path(s)

Emit a regular message (NOT AskUserQuestion):

> Give me the project path(s) to onboard. Accepts:
> - single path: `~/Projects/MyApp/`
> - glob scope:  `~/Projects/keit-*`
> - list:        `~/Projects/App1/, ~/Projects/App2/`
>
> Reply in one line.

Resolve the reply into `PATHS` (one or more absolute paths). Expand `~`.
Expand the glob via `ls -d <glob> 2>/dev/null`. If zero paths resolve →
tell the user, ask again. Never fall through.

## 1b — Scope-granularity click (AskUserQuestion, conditional)

Only emit this call if `len(PATHS) > 1`. For a single project, skip to 1c.

```json
{
  "questions": [
    {
      "question": "Scope granularity for the N projects?",
      "header": "Scope",
      "multiSelect": false,
      "options": [
        {"label": "Per-project (independent)", "description": "Scan+propose+apply each project separately — different configs per project"},
        {"label": "Bulk same-config",          "description": "Scan each; propose a SHARED config; apply the same agent/hook set to all matching projects"},
        {"label": "Mixed",                     "description": "Scan each, propose per-project, but offer a bulk-apply shortcut at confirm-time"}
      ]
    }
  ]
}
```

Store as `GRANULARITY`. Default (single path) → `per-project`.

## 1c — Read-only scan (Bash, per project)

For each path in `PATHS`, run a single Bash sweep. Every command must be
read-only (no mkdir, no touch, no >). Absolute paths only.

```bash
P="<absolute-project-path>"

# Framework / package manifests
ls -1 "$P"/package.json "$P"/Cargo.toml "$P"/pyproject.toml \
      "$P"/pubspec.yaml "$P"/go.mod "$P"/Package.swift \
      "$P"/requirements.txt 2>/dev/null

# CI
ls -1 "$P"/.github/workflows/*.yml "$P"/.github/workflows/*.yaml \
      "$P"/.forgejo/workflows/*.yml "$P"/.gitea/workflows/*.yml \
      "$P"/.gitlab-ci.yml "$P"/.circleci/config.yml 2>/dev/null

# Deploy
ls -1 "$P"/docker-compose*.yml "$P"/docker-compose*.yaml \
      "$P"/Dockerfile "$P"/Dockerfile.* \
      "$P"/fly.toml "$P"/wrangler.toml "$P"/modal.toml \
      "$P"/render.yaml "$P"/vercel.json 2>/dev/null

# Tests
ls -1d "$P"/tests "$P"/test "$P"/__tests__ "$P"/spec 2>/dev/null
find "$P" -maxdepth 3 -type f \
     \( -name "*_test.go" -o -name "*.test.ts" -o -name "*.test.tsx" \
        -o -name "*.test.js" -o -name "*.spec.ts" -o -name "test_*.py" \) \
     2>/dev/null | head -5

# README (first 100 lines — purpose extraction)
head -n 100 "$P"/README.md 2>/dev/null

# Recent activity (log ONLY; no writes — git is BLOCKED for state change
# but read-only log is the only inspection primitive available)
git -C "$P" log --oneline -20 2>/dev/null

# Env surface — schema files ONLY, never actual secrets
ls -1 "$P"/.env.example "$P"/.env.template \
      "$P"/secrets/*.env.example 2>/dev/null
grep -h '^[A-Z][A-Z0-9_]*=' \
     "$P"/.env.example "$P"/.env.template 2>/dev/null | \
     cut -d= -f1 | sort -u
```

Capture each block of output (even if empty — empty IS a signal).

## 1d — Structured summary per project

Produce a markdown summary per project in the conversation (do NOT write
to disk). Shape:

```
## Scan: <project-path>
Stack:        <package.json → Node/<framework> | Cargo.toml → Rust | pyproject → Python | pubspec → Flutter | go.mod → Go | Package.swift → Swift | NONE_DETECTED>
CI:           <github-actions | forgejo-actions | gitlab-ci | circleci | NONE>
Deploy hints: <docker-compose | Dockerfile | fly.io | cloudflare-workers | modal | vercel | NONE>
Tests:        <tests/ dir | test files found | NONE>
README:       <first-sentence summary, verbatim quoted>
Git activity: <N commits in last 20 log lines | NO_GIT>
Env vars:     <KEY1, KEY2, ... (names only) | NONE>
```

## Verify-criterion

- `PATHS` is non-empty and every entry resolves to a directory.
- Per-project summary is present for each path in `PATHS`.
- No actual secret values appear in the summary — only KEY names.
- Absent signals are recorded as `NONE` / `NONE_DETECTED`, never invented.
- If scan surfaces conflicting evidence (e.g. both `package.json` AND
  `Cargo.toml`), list BOTH in the summary — do not pick one silently;
  Phase 2 handles disambiguation.
