#!/bin/sh
# auto-dev-guard.sh — PostToolUse:Edit|Write advisory hook.
#
# Triggers a frontend-validator pass after meaningful Edit/Write on
# frontend files (.tsx, .ts, .svelte, .vue, .dart) OR DB-layer files
# (migrations/*.sql, src/db/**, src/types/**, prisma/schema.prisma,
# drizzle.config.*).
#
# DOES NOT block. Emits stderr advisory pointing the user at /dev-guard
# OR auto-spawns a single-shot validator pass (advisory mode) if
# `kei-db-contract` binary is on PATH.
#
# Skip-on-trivial: if Edit's diff is < 30 LOC, skip. Avoid spawn-fatigue.
#
# Bypass: KEI_DISABLED_HOOKS contains "auto-dev-guard" OR
#         KEI_HOOK_PROFILE in {advisory-off, minimal, off}.

command -v jq >/dev/null 2>&1 || exit 0

_KEI_LIB="$(dirname "$0")/_lib/gate.sh"
if [ -r "$_KEI_LIB" ]; then
    . "$_KEI_LIB"
    kei_hook_gate "auto-dev-guard" || exit 0
fi

set -eu

input="$(cat)"

# --- Detect tool + file_path ---
tool=$(printf '%s' "$input" | jq -r '.tool_name // empty' 2>/dev/null || true)
file=$(printf '%s' "$input" | jq -r '.tool_input.file_path // empty' 2>/dev/null || true)
[ -n "$file" ] || exit 0

# --- Match frontend / DB-layer patterns ---
match=0
case "$file" in
    *.tsx|*.ts|*.svelte|*.vue|*.dart) match=1 ;;
    */migrations/*.sql) match=1 ;;
    */src/db/*|*/src/types/*) match=1 ;;
    */prisma/schema.prisma|*/drizzle.config.*) match=1 ;;
esac
[ "$match" = "1" ] || exit 0

# --- Trivial-edit gate: skip if change is small ---
# For Edit tool, count line delta. For Write, count total lines.
delta=0
if [ "$tool" = "Edit" ]; then
    old_str=$(printf '%s' "$input" | jq -r '.tool_input.old_string // empty' 2>/dev/null || true)
    new_str=$(printf '%s' "$input" | jq -r '.tool_input.new_string // empty' 2>/dev/null || true)
    old_lines=$(printf '%s' "$old_str" | wc -l 2>/dev/null | tr -d ' ')
    new_lines=$(printf '%s' "$new_str" | wc -l 2>/dev/null | tr -d ' ')
    delta=$((new_lines > old_lines ? new_lines - old_lines : old_lines - new_lines))
elif [ "$tool" = "Write" ]; then
    content=$(printf '%s' "$input" | jq -r '.tool_input.content // empty' 2>/dev/null || true)
    delta=$(printf '%s' "$content" | wc -l 2>/dev/null | tr -d ' ')
fi
[ "$delta" -ge 30 ] || exit 0

# --- Resolve project root from file path ---
# Walk up from $file until we find package.json / pubspec.yaml / Cargo.toml.
project_root=""
dir=$(dirname "$file")
for _ in 1 2 3 4 5 6 7 8; do
    if [ -f "$dir/package.json" ] || [ -f "$dir/pubspec.yaml" ]; then
        project_root="$dir"
        break
    fi
    parent=$(dirname "$dir")
    [ "$parent" = "$dir" ] && break
    dir="$parent"
done

# --- Emit advisory + (optionally) run kei-db-contract for DB drift ---
echo "[auto-dev-guard] Frontend-relevant edit: $(basename "$file") ($delta LOC delta)" >&2

if [ -n "$project_root" ] && command -v kei-db-contract >/dev/null 2>&1; then
    # Only run DB contract check when DB-layer files were edited
    case "$file" in
        */migrations/*.sql|*/src/db/*|*/src/types/*|*/prisma/schema.prisma|*/drizzle.config.*)
            drift_json=$(kei-db-contract "$project_root" --output json 2>/dev/null || true)
            if [ -n "$drift_json" ]; then
                drift_count=$(printf '%s' "$drift_json" | jq -r '.drift_count // 0' 2>/dev/null || echo 0)
                if [ "$drift_count" -gt 0 ]; then
                    echo "[auto-dev-guard] DB-contract drift: $drift_count table(s)." >&2
                    echo "[auto-dev-guard] Run /dev-guard or 'kei-db-contract $project_root' for details." >&2
                fi
            fi
            ;;
    esac
fi

# Suggest /dev-guard for manual full pass
case "$file" in
    *.tsx|*.ts|*.svelte|*.vue|*.dart)
        echo "[auto-dev-guard] Tip: /dev-guard for full TS / lint / DB / visual pass." >&2
        ;;
esac

exit 0
