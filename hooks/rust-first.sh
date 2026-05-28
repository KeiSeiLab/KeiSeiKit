#!/usr/bin/env bash
set -e

# Runtime gate (hooks-control skill / KEI_DISABLED_HOOKS / KEI_HOOK_PROFILE).
_KEI_LIB="$(dirname "$0")/_lib/gate.sh"; if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "rust-first" || exit 0; fi
# RULE 0.2 — RUST FIRST reminder hook.
#
# Fires on UserPromptSubmit. Detects keywords indicating language choice
# or new development, and injects the rust-first rule into next-turn context.
#
# Soft reminder, not a hard block. The rule itself is enforced by Claude
# consulting ~/.claude/rules/rust-first.md before writing code.
#
# Keyword sources:
#   - English patterns: inline regex below
#   - Non-English patterns (Russian etc): ~/.claude/hooks/ru/rust-first-keywords.txt
#     One keyword or phrase per line, lowercase, substring match.
#
# Exit behavior:
#   - No match: exit 0 silently
#   - Match: print a JSON block with additionalContext, exit 0

PROMPT=$(jq -r '.prompt // empty' 2>/dev/null)
[ -z "$PROMPT" ] && exit 0

# Lowercase (macOS Bash 3 — use tr, not ${var,,})
PROMPT_LC=$(printf '%s' "$PROMPT" | tr '[:upper:]' '[:lower:]')

MATCH=0

# --- English language-choice patterns ------------------------------------
if printf '%s' "$PROMPT_LC" | grep -qE '(new project|start a project|choose (a )?language|choose (the )?stack|what stack|which stack|stack choice|rewrite .* in (python|go|js|javascript|typescript|swift|ruby|c\+\+)|should (we|i) use (python|go|js|javascript|typescript|swift|ruby|rust)|what language|pick a language|rust or (python|go|js)|python or rust|go or rust)'; then
  MATCH=1
fi

# --- Non-English keywords via sidecar file -------------------------------
SIDECAR="$HOME/.claude/hooks/ru/rust-first-keywords.txt"
if [ "$MATCH" -eq 0 ] && [ -f "$SIDECAR" ]; then
  # Substring match: each non-empty non-comment line
  while IFS= read -r KW; do
    # Skip empty lines and comments
    case "$KW" in
      ''|'#'*) continue ;;
    esac
    if printf '%s' "$PROMPT_LC" | grep -qF -- "$KW"; then
      MATCH=1
      break
    fi
  done < "$SIDECAR"
fi

if [ "$MATCH" -eq 0 ]; then
  exit 0
fi

# --- Inject additional context ------------------------------------------
cat <<'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "<rust-first-reminder>\nRULE 0.2 RUST FIRST: language-choice keyword detected in user prompt.\n\nBefore proposing any stack, consult ~/.claude/rules/rust-first.md.\n\nDefault is Rust for all new code. A non-Rust choice requires an explicit architectural reason from the allowed list: (1) large ML training over 10M params, (2) existing language-locked project being extended, (3) platform-native UI, (4) browser/DOM runtime, (5) under-50-line throwaway script, (6) external binding only in Python/JS, (7) explicit user override with stated reason.\n\nNot acceptable reasons: 'Python is the ML language', 'I know Python better', 'faster iteration in Python', 'matplotlib easier', 'we will rewrite in Rust later', 'just a prototype', 'libraries'.\n\nDocument the chosen exception in project DECISIONS.md.\n\nIf the user has already chosen another language, respect their call but note the reason in project memory. Otherwise default to Rust and state the architectural reason the exception does not apply.\n</rust-first-reminder>"
  }
}
EOF

exit 0
