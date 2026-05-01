# Phase 3 — Run setup script, hand off deploy key

Run the imperative helper and hand the public-key material to the user.

## 3a — Invoke `kei-sleep-setup.sh`

Run the primitive non-interactively with `REPO_URL` pre-supplied:

```bash
KEI_MEMORY_REPO_URL="<REPO_URL>" \
  ~/.claude/agents/_primitives/kei-sleep-setup.sh
```

Capture stdout + stderr. The script:
1. Generates `~/.ssh/keisei-memory-sync` if missing.
2. Prints the `.pub` contents and fingerprint.
3. Scaffolds `~/.claude/memory/sync-repo/` and writes config + env refs.
4. Tests SSH auth against the host (advisory).

If the script exits non-zero, surface its stderr directly to chat and
abort the wizard. Do NOT retry silently.

## 3b — Render deploy-key block to chat

The script already printed the key + fingerprint to its stdout. Echo
that block back to the user verbatim, prefaced with:

```
Add this key as a DEPLOY KEY with WRITE access to <REPO_URL>.
GitHub:    Settings → Deploy keys → Add deploy key ("Allow write access")
GitLab:    Settings → Repository → Deploy keys → Enable with write access
Bitbucket: Repository settings → Access keys → Add key (write)
Self-host: check your provider's "deploy key" or "access key" feature
```

NEVER show the private key. The `.pub` file is safe to display.

## 3c — Confirm click

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Have you added the deploy key to the repo with WRITE access?",
      "header": "Deploy key",
      "multiSelect": false,
      "options": [
        {"label": "Yes, it's added",      "description": "Proceed to a test push"},
        {"label": "Show me the key again", "description": "Re-print the public key + fingerprint"},
        {"label": "Abort",                "description": "Cancel — re-run /sleep-setup later"}
      ]
    }
  ]
}
```

Handle each option:
- `Yes`         → set `KEY_ADDED = true`, proceed to Phase 4.
- `Show again`  → re-print the block from 3b, re-emit this click.
- `Abort`       → print "aborted — re-run /sleep-setup later"; exit.

## Verify-criterion

- `~/.ssh/keisei-memory-sync(.pub)` exist.
- `~/.claude/memory/sync-repo/.git/` exists.
- `~/.claude/secrets/.env` contains all three `KEI_MEMORY_*` refs.
- `KEY_ADDED == true`.
- Exactly ONE `AskUserQuestion` (plus loops on "Show me again").
