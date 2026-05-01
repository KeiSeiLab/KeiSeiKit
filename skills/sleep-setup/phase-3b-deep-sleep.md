# Phase 3b — Deep-sleep NREM configuration (v0.13.0)

Collect three pure-click decisions for Phase C (system consolidation):
cadence, fork mode, store backend. All three are `AskUserQuestion`
batches — zero free text (frequency "custom" is the single exception).

## Mode-dependent behaviour

If `SLEEP_MODE == local-only` (set in Phase 0), the fork mode question
in §3b.2 gets an EXTRA third option `plan+local-patch` that applies
auto-resolvable changes directly to `~/.claude/` files (after user
confirm at morning) instead of committing to a git branch. Cadence
(§3b.1) and store backend (§3b.3) are still asked, but the store
backend defaults to `filesystem` for local-only — the user can still
pick another if they want a secondary backup path.

For `remote-only` / `hybrid` the fork mode stays as the original 2
options (plan only / plan + fork branch).

## 3b.1 — Deep-sleep cadence

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "How often should deep-sleep run (system consolidation — detect conflicts across rules, hooks, blocks, memory)?",
      "header": "Deep-sleep cadence",
      "multiSelect": false,
      "options": [
        {"label": "Never (disable)",                 "description": "Phase C skipped forever"},
        {"label": "Every 14 days (low-load)",        "description": "Minimal churn; rare consolidation"},
        {"label": "Every 7 days (Recommended)",      "description": "Weekly Sunday — default"},
        {"label": "Every 3 days",                     "description": "Tighter loop for active refactors"},
        {"label": "Every day (heavy-load only)",     "description": "May be overkill for most users"},
        {"label": "Custom (N days, free-text)",      "description": "Enter integer on next prompt"}
      ]
    }
  ]
}
```

Store as `DEEP_SLEEP_CRON_DAYS`:
- Never → `0`
- Every 14 → `14`
- Every 7 → `7`
- Every 3 → `3`
- Every day → `1`
- Custom → emit follow-up freeText prompt, parse integer, clamp to
  `1..=90`. Reject non-integer with retry.

## 3b.2 — Fork output mode

For `SLEEP_MODE ∈ {remote-only, hybrid}`, emit ONE `AskUserQuestion`
with 2 options:

```json
{
  "questions": [
    {
      "question": "Fork output with applied changes?",
      "header": "Deep-sleep fork",
      "multiSelect": false,
      "options": [
        {"label": "Plan only (Recommended)",  "description": "Read markdown in the morning; decide by hand"},
        {"label": "Plan + fork branch",       "description": "Also generate deep-sleep/YYYY-MM-DD branch for git review"}
      ]
    }
  ]
}
```

For `SLEEP_MODE == local-only`, emit ONE `AskUserQuestion` with 3
options (adds `plan+local-patch`):

```json
{
  "questions": [
    {
      "question": "Fork output with applied changes?",
      "header": "Deep-sleep fork (local mode)",
      "multiSelect": false,
      "options": [
        {"label": "Plan only (Recommended)",   "description": "Read markdown in the morning; decide by hand"},
        {"label": "Plan + fork branch",        "description": "Also generate deep-sleep/YYYY-MM-DD branch (needs a local git repo under ~/.claude/)"},
        {"label": "Plan + local-patch",        "description": "Auto-resolvable changes applied directly to ~/.claude/ after morning confirm — no git branch needed"}
      ]
    }
  ]
}
```

Store as `DEEP_SLEEP_WITH_FORK` ∈ {0, 1, 2}:
- `0` — Plan only
- `1` — Plan + fork branch
- `2` — Plan + local-patch (local-only mode only)

## 3b.3 — Memory-repo backend

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Memory-repo backend?",
      "header": "Store backend",
      "multiSelect": false,
      "options": [
        {"label": "GitHub private (simplest)",   "description": "github.com with deploy key or PAT"},
        {"label": "Forgejo self-hosted",          "description": "Same wire protocol; different base URL"},
        {"label": "Gitea self-hosted",            "description": "Same wire protocol as Forgejo"},
        {"label": "Filesystem only (no remote)",  "description": "Local .git; no push; survives laptop only"},
        {"label": "S3-compatible (R2/MinIO/AWS)", "description": "Object storage — MVP stub in v0.13.0"}
      ]
    }
  ]
}
```

Store as `STORE_BACKEND` ∈ {github, forgejo, gitea, filesystem, s3}.

## 3b.4 — Write store config

Call `kei-store init <STORE_BACKEND> --url <REPO_URL>` which writes
`~/.claude/agents/_primitives/store-config.toml` with:

```toml
[active]
backend = "<STORE_BACKEND>"
local_path = "~/.claude/memory/sync-repo"

[<STORE_BACKEND>]
url = "<REPO_URL>"
ssh_key_env = "KEI_MEMORY_SSH_KEY"
pat_env = "KEI_MEMORY_PAT"
```

Secrets (SSH key path, PAT) remain in `~/.claude/secrets/.env` per
RULE 0.8. The config file stores only env-var NAMES.

For `filesystem` backend skip the URL step entirely (no remote).
For `s3` also prompt for `endpoint`, `bucket`, `region` via three
free-text fields (one-off — unavoidable; S3 has no SSH-like default).

## 3b.5 — Verify-criterion

- `DEEP_SLEEP_CRON_DAYS ∈ {0,1,3,7,14, or 1..=90}` for custom.
- `DEEP_SLEEP_WITH_FORK ∈ {0, 1}` for `remote-only` / `hybrid`;
  `∈ {0, 1, 2}` for `local-only` (2 = plan+local-patch).
- `STORE_BACKEND ∈ {github, forgejo, gitea, filesystem, s3}`.
- `~/.claude/agents/_primitives/store-config.toml` exists and has
  the active backend set.
- Exactly THREE `AskUserQuestion` batches in this phase (plus one
  follow-up free-text iff the user picked Custom for cadence, and up
  to three free-text fields iff S3 was picked).
