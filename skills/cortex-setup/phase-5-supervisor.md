# Phase 5 — Supervisor (launchd / systemd / manual)

Register the daemon to auto-start at login so the user doesn't need to
invoke `kei-cortex` manually each time. macOS uses `launchd` with a
user-agent plist; Linux uses a `systemd --user` unit. Both subcommands
are idempotent per the primitive contract.

The user sees ONE click: install supervisor vs skip (manual start).

## 5a — Supervisor choice

ONE `AskUserQuestion` (branching on `OS` from Phase 0):

```json
{
  "questions": [
    {
      "question": "Install the supervisor so kei-cortex auto-starts at login?",
      "header": "Supervisor",
      "multiSelect": false,
      "options": [
        {"label": "Install (recommended)", "description": "<launchd plist on macOS | systemd --user unit on linux>; auto-starts at login, restarts on crash, logs to <log path>"},
        {"label": "Skip (manual start)",   "description": "No auto-start — run `<BIN_PATH> --port <CORTEX_PORT> --token-file ~/.keisei/cortex.token` manually when you want the daemon"}
      ]
    }
  ]
}
```

Substitute the bracketed placeholders per `OS`:

- `macos`: `launchd plist on macOS` and `~/Library/Logs/keisei-cortex.log`.
- `linux`: `systemd --user unit on linux` and
  `journalctl --user -u keisei-cortex -n 20`.

Handle:

- `Install (recommended)` → proceed to 5b.
- `Skip (manual start)`   → set `SUPERVISOR = "manual"` and skip to 5e.

## 5b — Invoke install subcommand

Branch on `OS`:

### macOS

```bash
"$SETUP_PATH" launchd-install "$BIN_PATH" "$TOKEN_PATH" "$UI_DIST"
INSTALL_EXIT=$?
```

### Linux

```bash
"$SETUP_PATH" systemd-install "$BIN_PATH" "$TOKEN_PATH" "$UI_DIST"
INSTALL_EXIT=$?
```

Note: we pass `TOKEN_PATH` (not the value) and `UI_DIST` (not a URL) —
RULE 0.8 means the supervisor unit reads the token from disk at
daemon start, never embeds it in the plist / unit file.

On non-zero exit, capture stderr and emit 5d (retry/skip/abort click).

## 5c — Verify the supervisor is live

### macOS

```bash
launchctl list | grep app.keisei.cortex | head -1
```

Expected output: a line with three columns — PID (integer or `-`),
exit-code (integer), and the label `app.keisei.cortex`. A numeric PID
confirms the daemon is running. `-` means registered but not yet
started; launchd will start it shortly, so treat that as success.

If `grep` finds nothing → verification failed. Emit 5d.

### Linux

```bash
systemctl --user status keisei-cortex --no-pager -l | head -20
```

Expected: an `Active: active (running)` line. Anything else (inactive,
failed, dead) → verification failed. Emit 5d.

Also run:

```bash
systemctl --user is-enabled keisei-cortex
```

Expected `enabled`. If not, log a note — this means the unit will NOT
restart at next login. The user can fix manually with
`systemctl --user enable keisei-cortex`.

## 5d — Retry / skip on verification failure

ONE `AskUserQuestion` with last 10 lines of stderr in context:

```json
{
  "questions": [
    {
      "question": "Supervisor install or verification failed. What next?",
      "header": "Supervisor failed",
      "multiSelect": false,
      "options": [
        {"label": "Retry",           "description": "Re-run launchd-install / systemd-install — fixes transient permission issues"},
        {"label": "Skip (manual)",   "description": "Continue without supervisor — you'll run the daemon manually"},
        {"label": "Abort",           "description": "Stop the wizard here — re-run /cortex-setup later"}
      ]
    }
  ]
}
```

Handle:

- `Retry`  → re-run 5b + 5c. Max 2 retries.
- `Skip (manual)` → set `SUPERVISOR = "manual"`, continue to 5e.
- `Abort`  → exit the wizard with `Aborted. Re-run /cortex-setup later.`

## 5e — Emit outcome summary

On success, print:

### macOS

```
Supervisor:  launchd   (label: app.keisei.cortex)
Log file:    ~/Library/Logs/keisei-cortex.log
Tail logs:   tail -f ~/Library/Logs/keisei-cortex.log
Uninstall:   launchctl unload ~/Library/LaunchAgents/app.keisei.cortex.plist
```

Set `SUPERVISOR = "launchd"`.

### Linux

```
Supervisor:  systemd (user)  (unit: keisei-cortex.service)
Log file:    systemd journal (no flat file)
Tail logs:   journalctl --user -u keisei-cortex -f
Uninstall:   systemctl --user disable --now keisei-cortex
```

Set `SUPERVISOR = "systemd"`.

### Manual (skip chosen)

```
Supervisor:  manual  (no auto-start)
Start:       <BIN_PATH> --port <CORTEX_PORT> --token-file ~/.keisei/cortex.token
Stop:        Ctrl-C in the terminal where it's running
```

Set `SUPERVISOR = "manual"`.

## 5f — Post-install sanity probe (advisory)

Optionally probe the daemon's `/healthz` endpoint to confirm it's
actually listening. Non-blocking — purely informational.

```bash
sleep 1  # give launchd/systemd a moment to start
if command -v curl >/dev/null 2>&1; then
  if curl -sf -o /dev/null "http://127.0.0.1:${CORTEX_PORT}/healthz"; then
    echo "Daemon healthz: ok  (http://127.0.0.1:${CORTEX_PORT}/healthz)"
  else
    echo "Daemon healthz: not responding yet — it may still be starting; Phase 6 will open the UI anyway."
  fi
fi
```

Do NOT gate on this. The supervisor may still be spinning up; the user
can retry via browser in Phase 6.

## Verify-criterion

- `SUPERVISOR ∈ {launchd, systemd, manual}`.
- If `launchd` → `launchctl list | grep app.keisei.cortex` found a line.
- If `systemd` → `systemctl --user status keisei-cortex` reported
  `active (running)` (or `Retry`/`Skip` chosen).
- Exactly 1 `AskUserQuestion` in the happy path (5a). Up to 2 more if
  install fails and the user retries (5d).
- Log file path printed in the summary matches the subcommand's contract.
