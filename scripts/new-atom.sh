#!/usr/bin/env bash
# new-atom.sh — scaffold a new atom per SUBSTRATE-SCHEMA.md
#
# Usage:
#   scripts/new-atom.sh <crate> <verb> [kind]
#
# Example:
#   scripts/new-atom.sh kei-task add-dependency command
#
# Kinds (per schema §Atom kinds): command | query | stream | transform
# Default kind: command

set -euo pipefail

CRATE="${1:?usage: new-atom.sh <crate> <verb> [kind]}"
VERB="${2:?usage: new-atom.sh <crate> <verb> [kind]}"
KIND="${3:-command}"

# Validate kind against schema
case "$KIND" in
  command|query|stream|transform) ;;
  *) echo "error: kind must be one of: command, query, stream, transform" >&2; exit 1 ;;
esac

# Validate verb naming (kebab-case, lowercase)
if ! [[ "$VERB" =~ ^[a-z][a-z0-9]*(-[a-z0-9]+)*$ ]]; then
  echo "error: verb must be lowercase kebab-case (got '$VERB')" >&2
  exit 1
fi

# Repo root = two dirs up from this script
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CRATE_DIR="$ROOT/_primitives/_rust/$CRATE"
TEMPLATE_DIR="$ROOT/_templates/atom"

if [ ! -d "$CRATE_DIR" ]; then
  echo "error: crate directory not found: $CRATE_DIR" >&2
  echo "hint: create the crate first (e.g. via 'cargo new --lib $CRATE_DIR')" >&2
  exit 1
fi

VERB_SNAKE="${VERB//-/_}"
CRATE_SNAKE="${CRATE//-/_}"

# Target files
MD_OUT="$CRATE_DIR/atoms/$VERB.md"
IN_OUT="$CRATE_DIR/atoms/schemas/$VERB-input.json"
OUT_OUT="$CRATE_DIR/atoms/schemas/$VERB-output.json"
RS_OUT="$CRATE_DIR/src/atoms/$VERB_SNAKE.rs"
TEST_OUT="$CRATE_DIR/tests/${VERB_SNAKE}_smoke.rs"

# Refuse to overwrite
for f in "$MD_OUT" "$IN_OUT" "$OUT_OUT" "$RS_OUT" "$TEST_OUT"; do
  if [ -e "$f" ]; then
    echo "error: file already exists: $f" >&2
    echo "hint: pick a different verb, or delete the existing file first" >&2
    exit 1
  fi
done

# Prompt for description (stdin-friendly, non-interactive if piped)
if [ -t 0 ]; then
  read -rp "One-line description: " DESCRIPTION
else
  DESCRIPTION="${ATOM_DESCRIPTION:-TODO: add description}"
fi
# Escape for sed — forward-slash is our delimiter; strip any the user typed
DESCRIPTION_ESCAPED="${DESCRIPTION//\//\\/}"

mkdir -p "$CRATE_DIR/atoms/schemas" "$CRATE_DIR/src/atoms" "$CRATE_DIR/tests"

# Track what we wrote so we can roll back on failure
CREATED=()

substitute() {
  local src="$1" dest="$2"
  sed \
    -e "s/__CRATE__/$CRATE/g" \
    -e "s/__CRATE_SNAKE__/$CRATE_SNAKE/g" \
    -e "s/__VERB__/$VERB/g" \
    -e "s/__VERB_SNAKE__/$VERB_SNAKE/g" \
    -e "s/__KIND__/$KIND/g" \
    -e "s/__DESCRIPTION__/$DESCRIPTION_ESCAPED/g" \
    "$src" > "$dest"
  CREATED+=("$dest")
}

rollback() {
  echo "rolling back — removing ${#CREATED[@]} generated files..." >&2
  for f in "${CREATED[@]}"; do
    rm -f "$f"
  done
}

trap rollback ERR

substitute "$TEMPLATE_DIR/atoms/__VERB__.md.template"                    "$MD_OUT"
substitute "$TEMPLATE_DIR/atoms/schemas/__VERB__-input.json.template"    "$IN_OUT"
substitute "$TEMPLATE_DIR/atoms/schemas/__VERB__-output.json.template"   "$OUT_OUT"
substitute "$TEMPLATE_DIR/src/atoms/__VERB_SNAKE__.rs.template"          "$RS_OUT"
substitute "$TEMPLATE_DIR/tests/__VERB_SNAKE___smoke.rs.template"        "$TEST_OUT"

# Registering the atom module in src/atoms/mod.rs is left to Stream B
# refactor — on a freshly templated crate, src/atoms/mod.rs may not exist
# yet. The generator refuses to guess where to append.

trap - ERR

echo ""
echo "✓ Scaffolded atom $CRATE::$VERB ($KIND)"
echo ""
echo "Files created:"
for f in "${CREATED[@]}"; do
  echo "  ${f#$ROOT/}"
done
echo ""
echo "Next steps:"
echo "  1. Edit atoms/$VERB.md — fill description, examples, related[] wikilinks"
echo "  2. Edit atoms/schemas/$VERB-{input,output}.json — declare actual fields"
echo "  3. Implement src/atoms/$VERB_SNAKE.rs — replace NotImplemented with real logic"
echo "  4. Register: add 'pub mod $VERB_SNAKE;' to src/atoms/mod.rs"
echo "  5. cargo check -p $CRATE"
echo "  6. (once kei-schema-lint ships) kei-schema-lint $CRATE"
