#!/usr/bin/env bash
# _bridges/emit.sh — render cross-tool bridge templates into a target dir.
# Usage:
#   emit.sh <target-dir> [project-name] [project-description]
#   emit.sh --only <output-path> <target-dir> [project-name] [project-description]
# Idempotent: files that already exist are skipped, not overwritten.

set -euo pipefail

BRIDGES_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

ONLY=""
if [ "${1:-}" = "--only" ]; then
  ONLY="${2:-}"
  [ -n "$ONLY" ] || { echo "error: --only requires an output path" >&2; exit 2; }
  shift 2
fi

TARGET="${1:-}"
[ -n "$TARGET" ] || { echo "usage: emit.sh [--only <path>] <target-dir> [name] [description]" >&2; exit 2; }
[ -d "$TARGET" ] || { echo "error: target dir does not exist: $TARGET" >&2; exit 2; }
TARGET="$(cd "$TARGET" && pwd)"

NAME="${2:-$(basename "$TARGET")}"
DESC="${3:-}"
if [ -z "$DESC" ]; then
  for f in "$TARGET/CLAUDE.md" "$TARGET/README.md"; do
    if [ -f "$f" ]; then
      DESC="$(grep -m1 -E '^[[:space:]]*[^#[:space:]].*' "$f" | sed 's/^[[:space:]]*//' || true)"
      [ -n "$DESC" ] && break
    fi
  done
  [ -n "$DESC" ] || DESC="No description"
fi

YEAR="$(date +%Y)"; MONTH="$(date +%m)"; DATE="$(date +%Y-%m-%d)"

# tmpl|output-rel-path (11 entries, matches _bridges/README.md table)
MAP="cursorrules.tmpl|.cursorrules
agents-md.tmpl|AGENTS.md
copilot.tmpl|.github/copilot-instructions.md
cursor-mdc.tmpl|.cursor/rules/main.mdc
windsurf.tmpl|.windsurf/rules/main.md
junie.tmpl|.junie/guidelines.md
continue.tmpl|.continue/rules/main.md
gemini.tmpl|GEMINI.md
aider-conventions.tmpl|CONVENTIONS.md
aider-conf.tmpl|.aider.conf.yml
replit.tmpl|replit.md"

# sed-escape a replacement string (handles & / \ newlines)
sed_escape() { printf '%s' "$1" | sed -e 's/[\\&/]/\\&/g' -e 's/$/\\/' -e '$ s/\\$//'; }

created=0; skipped=0
while IFS='|' read -r tmpl out; do
  [ -n "$tmpl" ] || continue
  [ -n "$ONLY" ] && [ "$out" != "$ONLY" ] && continue
  src="$BRIDGES_DIR/$tmpl"
  dst="$TARGET/$out"
  [ -f "$src" ] || { echo "error: missing template $src" >&2; exit 3; }
  if [ -e "$dst" ]; then
    echo "skipped: $out (exists)"
    skipped=$((skipped+1))
    continue
  fi
  mkdir -p "$(dirname "$dst")"
  sed -e "s/{{PROJECT_NAME}}/$(sed_escape "$NAME")/g" \
      -e "s/{{PROJECT_DESCRIPTION}}/$(sed_escape "$DESC")/g" \
      -e "s/{{YEAR}}/$YEAR/g" \
      -e "s/{{MONTH}}/$MONTH/g" \
      -e "s/{{DATE}}/$DATE/g" \
      "$src" > "$dst"
  echo "created: $out"
  created=$((created+1))
done <<< "$MAP"

echo "bridges: $created created, $skipped skipped"
