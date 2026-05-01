#!/bin/sh
# auto-register-on-edit.sh — PostToolUse:Edit|Write — auto-register substrate blocks.
# Severity: warn (exit 0; advisory only)
#
# When a substrate file is edited (skill / hook / block / capability / role /
# primitive crate source), call the right kei-registry register command to
# refresh that block's row. Closes the auto-loop:
#
#   skill SKILL.md edit → kei-registry register-skill <path>
#   hook .sh edit → kei-registry register-hook <path>
#   _blocks/*.md edit → kei-registry index-substrate (broad — kept simple)
#   _capabilities/* edit → kei-registry index-substrate
#   _roles/* edit → kei-registry index-substrate
#   _primitives/_rust/<crate>/ edit → kei-import-project register <crate-root>
#
# Companions:
# - decompose-rules-on-edit.sh (Wave 14) handles ~/.claude/rules/*.md
# - assemble-agents.sh handles _manifests/*.toml
# This hook handles the rest.
#
# Bypass: AUTO_REGISTER_BYPASS=1.

[ "${AUTO_REGISTER_BYPASS:-0}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0

INPUT=$(cat 2>/dev/null || true)
FILE=$(printf '%s' "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)
[ -z "$FILE" ] && exit 0

# Only fire on files inside KeiSeiKit-public (substrate dirs)
case "$FILE" in
    */KeiSeiKit-public/*) ;;
    *) exit 0 ;;
esac

# Resolve binaries; bail silently if absent (bootstrap-friendly)
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

KIP=$(command -v kei-import-project 2>/dev/null)
if [ -z "$KIP" ]; then
    for path in \
        "$HOME/.cargo/bin/kei-import-project" \
        "$HOME/Projects/KeiSeiKit-public/_primitives/_rust/target/release/kei-import-project"
    do
        [ -x "$path" ] && KIP="$path" && break
    done
fi

# Dispatch by file path
case "$FILE" in
    */skills/*/SKILL.md)
        # Walk up to skill folder (parent of SKILL.md)
        SKILL_DIR=$(dirname "$FILE")
        "$KR" register-skill "$SKILL_DIR" >/dev/null 2>&1 || true
        ;;
    */hooks/*.sh)
        "$KR" register-hook "$FILE" >/dev/null 2>&1 || true
        ;;
    */_blocks/*.md|*/_capabilities/*|*/_roles/*)
        # Broad refresh — substrate composition affects neighboring rows
        # Find KeiSeiKit-public root
        ROOT=$(echo "$FILE" | sed -E 's|(.*/KeiSeiKit-public)/.*|\1|')
        "$KR" index-substrate "$ROOT" >/dev/null 2>&1 || true
        ;;
    */_primitives/_rust/*/src/*|*/_primitives/_rust/*/Cargo.toml)
        # Primitive crate edit — re-register that crate via kei-import-project
        [ -z "$KIP" ] && exit 0
        # Walk up to crate root (find Cargo.toml's directory)
        CRATE_DIR=$(echo "$FILE" | sed -E 's|(.*/_primitives/_rust/[^/]+).*|\1|')
        if [ -d "$CRATE_DIR" ] && [ -f "$CRATE_DIR/Cargo.toml" ]; then
            # Register just this crate's root — kei-import-project register
            # walks the path to identify modules, finds this single crate.
            "$KIP" register "$CRATE_DIR" >/dev/null 2>&1 || true
        fi
        ;;
    *) exit 0 ;;
esac

exit 0
