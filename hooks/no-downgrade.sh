#!/usr/bin/env bash

# Runtime gate (hooks-control skill / KEI_DISABLED_HOOKS / KEI_HOOK_PROFILE).
_KEI_LIB="$(dirname "$0")/_lib/gate.sh"; if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "no-downgrade" || exit 0; fi
# RULE -1 NO DOWNGRADE / CONSTRUCTIVE ONLY (2026-04-15 LOCK) enforcement.
#
# Detects downgrade-style phrases in Write/Edit content without accompanying
# constructive rescue (3 solution paths). Warn-level — not a hard block.
#
# Trigger: PreToolUse matcher = "Write|Edit"
# Severity: warn (exit 0 + stderr message). Upgrade to enforce (exit 2) on
# second recurrence per RULE 0.10 escalation ladder.
#
# Bypass: export RULE_M1_BYPASS=1 (visible, per-invocation).

set -u

# Bypass
[[ "${RULE_M1_BYPASS:-0}" == "1" ]] && exit 0

PAYLOAD=$(cat)

TOOL_NAME=$(echo "$PAYLOAD" | jq -r '.tool_name // ""' 2>/dev/null)
[[ "$TOOL_NAME" != "Write" && "$TOOL_NAME" != "Edit" ]] && exit 0

# Extract content being written (Write: content; Edit: new_string)
CONTENT=$(echo "$PAYLOAD" | jq -r '.tool_input.content // .tool_input.new_string // ""' 2>/dev/null)
[[ -z "$CONTENT" ]] && exit 0

FILE_PATH=$(echo "$PAYLOAD" | jq -r '.tool_input.file_path // ""' 2>/dev/null)

# Only check docs/memos/chatlogs — skip source code where "failed" is a legit token
# (tests, error enums, status fields, etc.). Path heuristic:
case "$FILE_PATH" in
    *.md|*.txt|*.rst) ;;                # docs — do check
    *chatlogs*|*memory*|*report*) ;;    # memo paths — do check
    *) exit 0 ;;                         # source code / configs — skip
esac

# Downgrade triggers (case-insensitive, word-boundary where possible)
# derived: incident catalog from 2026-04-14 chatlogs + 2026-04-24 live session
TRIGGERS='\b(failed|refuted|doesn.?t work|downgrade|accept as limitation|не работает|не сработало|провалился|не удалось|tautolog(y|ical)|rejected?|dismiss|give up|отказываемся|отступаем|неудача|провал|это (всё\s+)?что мы)\b'

# Constructive rescue markers — if ANY of these present, downgrade is OK
# because the agent provided solution paths (RULE -1 compliance).
RESCUE='(three paths|3 paths|variant A|option A|вариант[аы]?\s+решения|solution paths?|constructive|recommend [AB]|три пути|можем попробовать|proposed fix|root cause.*fix|альтернативный путь|next step|решения\s*:)'

HAS_TRIGGER=$(echo "$CONTENT" | grep -ciE "$TRIGGERS" || true)
HAS_RESCUE=$(echo "$CONTENT" | grep -ciE "$RESCUE" || true)

if [[ "$HAS_TRIGGER" -gt 0 && "$HAS_RESCUE" -eq 0 ]]; then
    # Find one concrete offending line for diagnostics
    OFFENDING=$(echo "$CONTENT" | grep -iE "$TRIGGERS" | head -1 | cut -c1-120)
    cat >&2 <<EOF
[RULE -1 WARN] downgrade-style phrase detected without constructive follow-up.
  file: ${FILE_PATH:-<stdin>}
  offending: ${OFFENDING}
  required: present ≥2 solution paths + recommendation (see ~/.claude/rules/no-downgrade-constructive.md).
  bypass: RULE_M1_BYPASS=1 (if the phrase is a legitimate data label, not a conclusion).
EOF
fi

exit 0
