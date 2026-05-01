#!/bin/sh
# auto-encyclopedia-refresh.sh — PostToolUse:Edit|Write — refresh DNA-INDEX.md.
# Severity: warn (exit 0; advisory only)
#
# Triggers when a substrate file changes (companion to auto-register-on-edit.sh).
# After the registry row is refreshed, regenerate docs/DNA-INDEX.md so the
# committed encyclopedia stays in sync with the live registry state.
#
# Idempotent: kei-registry encyclopedia is read-only over the registry,
# write-only over the output file. Same registry state → same bytes.
#
# Bypass: AUTO_ENCYCLOPEDIA_BYPASS=1.

[ "${AUTO_ENCYCLOPEDIA_BYPASS:-0}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0

INPUT=$(cat 2>/dev/null || true)
FILE=$(printf '%s' "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)
[ -z "$FILE" ] && exit 0

# Only fire on substrate edits inside KeiSeiKit-public — same scope as
# auto-register-on-edit.sh. Skip if the touched file is the encyclopedia
# itself (avoids infinite loop).
case "$FILE" in
    */KeiSeiKit-public/docs/DNA-INDEX.md) exit 0 ;;
    */KeiSeiKit-public/skills/*/SKILL.md) ;;
    */KeiSeiKit-public/hooks/*.sh) ;;
    */KeiSeiKit-public/_blocks/*.md) ;;
    */KeiSeiKit-public/_capabilities/*) ;;
    */KeiSeiKit-public/_roles/*) ;;
    */KeiSeiKit-public/_primitives/_rust/*/src/*) ;;
    */KeiSeiKit-public/_primitives/_rust/*/Cargo.toml) ;;
    *) exit 0 ;;
esac

# Resolve binary
KR=$(command -v kei-registry 2>/dev/null)
if [ -z "$KR" ]; then
    for path in \
        "$HOME/.cargo/bin/kei-registry" \
        "$HOME/Projects/KeiSeiKit-public/_primitives/_rust/target/release/kei-registry"
    do
        [ -x "$path" ] && KR="$path" && break
    done
fi
[ -z "$KR" ] && exit 0

# Resolve KeiSeiKit-public root
ROOT=$(echo "$FILE" | sed -E 's|(.*/KeiSeiKit-public)/.*|\1|')
[ -d "$ROOT/docs" ] || exit 0

# Regenerate encyclopedia. Output to repo's docs/DNA-INDEX.md so a
# subsequent git diff shows the live state vs last-committed baseline.
"$KR" encyclopedia --output "$ROOT/docs/DNA-INDEX.md" >/dev/null 2>&1 || true

exit 0
