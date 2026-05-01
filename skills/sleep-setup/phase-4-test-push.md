# Phase 4 — Test push (verify write access)

Write a tiny marker file, call `kei-sleep-sync.sh`, let the user confirm
the commit landed in the remote.

## 4a — Write a test marker

```bash
touch ~/.claude/memory/sync-repo/traces/.sleep-setup-test
```

The marker file is tracked the same way real traces are (so it tests the
real `git add traces/` path used at session end).

## 4b — Invoke the sync helper

```bash
~/.claude/agents/_primitives/kei-sleep-sync.sh
```

Capture exit code. The helper is designed to be silent on success; capture
`~/.claude/memory/sync-errors.log` as well — if it gained a new line in
the last 60s, surface that line to chat.

## 4c — Show expected commit to user

Read `HEAD`'s commit message from the local mirror:

```bash
( cd ~/.claude/memory/sync-repo && git log -1 --pretty=format:'%h %s' )
```

Print this commit to chat as "expected to appear on your remote:".

## 4d — Confirm click

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Do you see this commit on the remote (refresh the repo page)?",
      "header": "Test push",
      "multiSelect": false,
      "options": [
        {"label": "Yes, commit is there",       "description": "Proceed to schedule"},
        {"label": "No, not showing up",         "description": "Show diagnostics + 2-3 fix paths"},
        {"label": "Skip — I'll check later",    "description": "Mark as UNVERIFIED, continue"}
      ]
    }
  ]
}
```

Handle each option:
- `Yes`   → set `TEST_VERIFIED = true`, clean up marker, proceed to Phase 5.
- `No`    → print the diagnostic block below; re-emit the click.
- `Skip`  → set `TEST_VERIFIED = false`, proceed to Phase 5.

## 4e — Diagnostic block (when user says "not showing up")

Render constructively per RULE -1:

```
Three things to check:
  1. Deploy key write-access — GitHub/GitLab/Bitbucket default to READ,
     you must tick the write box explicitly.
  2. Default branch — your repo must have a 'main' branch; if it has
     'master' or nothing at all the push target is missing.
  3. SSH reachability — run:
       ssh -i ~/.ssh/keisei-memory-sync -T git@<host>
     and confirm the auth banner shows your repo account.
If all three look correct, check ~/.claude/memory/sync-errors.log.
```

## 4f — Cleanup marker

Regardless of branch:

```bash
rm -f ~/.claude/memory/sync-repo/traces/.sleep-setup-test
```

## Verify-criterion

- Exactly ONE `AskUserQuestion` (plus loops on the "No" branch).
- `TEST_VERIFIED` is either `true` or `false` (both acceptable; only
  "Abort" terminates the wizard, and that option doesn't exist here).
