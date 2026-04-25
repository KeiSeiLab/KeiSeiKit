#!/bin/sh
# error-spike-detector.sh — PostToolUse:* hook (RULE 0.14).
#
# Maintains a rolling 20-tool-call window of (timestamp, error_flag) pairs.
# When ≥3 errors land in the window, append to audit-backlog.md. NEVER
# blocks: every exit path is `exit 0`. Silent-first: stderr advisory only
# fires after 10 sessions are in the memory store.

command -v jq >/dev/null 2>&1 || exit 0

_KEI_LIB="$(dirname "$0")/_lib/gate.sh"
if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "error-spike-detector" || exit 0; fi

set -eu

input="$(cat)"

is_err=$(printf '%s' "$input" | jq -r '.tool_response.is_error // empty' 2>/dev/null || true)
msg=$(printf '%s' "$input" | jq -r '.tool_response.content // .tool_response // empty' 2>/dev/null || true)

# Classify current call as errored if explicit is_error=true OR message
# matches a common failure signature. Empty tool_response → treat as OK.
flag=0
case "$is_err" in
    true|True|TRUE|1) flag=1 ;;
esac
if [ "$flag" -eq 0 ] && [ -n "$msg" ]; then
    case "$(printf '%s' "$msg" | tr 'A-Z' 'a-z')" in
        *error:*|*failed*|*panic*|*denied*) flag=1 ;;
    esac
fi

window="${HOME}/.claude/memory/error-window.txt"
backlog="${HOME}/.claude/memory/audit-backlog.md"
mkdir -p "$(dirname "$window")" 2>/dev/null || exit 0

# Append this tool call to the rolling window.
ts=$(date +%s)
printf '%s %s\n' "$ts" "$flag" >> "$window"

# Trim to last 20 lines.
tmp="${window}.tmp.$$"
tail -n 20 "$window" > "$tmp" 2>/dev/null || true
mv -f "$tmp" "$window" 2>/dev/null || true

# Count errors in the window.
err_ct=$(awk '$2==1' "$window" 2>/dev/null | wc -l | tr -d ' ')
err_ct=${err_ct:-0}

if [ "$err_ct" -lt 3 ]; then
    exit 0
fi

# Classify the spike by greping the recent error messages. Best-effort.
pattern="unclassified"
if [ -n "$msg" ]; then
    case "$(printf '%s' "$msg" | tr 'A-Z' 'a-z')" in
        *permission*denied*) pattern="permission_denied" ;;
        *worktree*)          pattern="worktree_error" ;;
        *cargo*workspace*)   pattern="cargo_workspace" ;;
        *panic*)             pattern="panic" ;;
        *timeout*)           pattern="timeout" ;;
    esac
fi

# Ensure the backlog file exists.
if [ ! -f "$backlog" ]; then
    {
        printf '# Audit Backlog\n\n'
        printf '<!-- session_count: 0 -->\n\n'
    } > "$backlog"
fi

iso=$(date -u +%Y-%m-%dT%H:%M)
printf -- '- [ERROR-SPIKE %s] %s errors in last 20 tool calls. Pattern: %s\n' \
    "$iso" "$err_ct" "$pattern" >> "$backlog"

# Reset window so we do not re-fire on every subsequent call.
: > "$window"

# Silent-first: advisory stderr only after 10 sessions.
session_count=$(grep -Eo 'session_count: [0-9]+' "$backlog" 2>/dev/null | head -1 | awk '{print $2}')
session_count=${session_count:-0}
if [ "$session_count" -ge 10 ]; then
    printf 'kei-memory: error spike detected (%s in 20) — see %s\n' \
        "$err_ct" "$backlog" >&2
fi

exit 0
