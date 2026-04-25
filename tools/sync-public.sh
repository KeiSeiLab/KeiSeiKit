#!/usr/bin/env bash
#
# sync-public.sh — port new logic from private KeiSeiKit into this public mirror.
#
# Usage:
#   tools/sync-public.sh                    # default --src ~/Projects/KeiSeiKit
#   tools/sync-public.sh --src /path/to/private --dry-run
#   tools/sync-public.sh --src /path/to/private --confirm
#
# What it does:
#   1. rsync diff from $SRC/ → ./   (excluding artefacts + Genesis-laden data)
#   2. apply deterministic sed substitutions to scrub Genesis/KeiLab refs
#   3. show staged diff for human review
#   4. on --confirm: git add + commit; on --dry-run or default: stop after diff
#
# Push is NEVER automatic. Run `git push github main` manually after review.
#
# Symbol substitution rules (extend by editing $SUBSTITUTIONS below):
#   "Genesis HISTORY.md"           → "internal calibration"
#   "Port of Genesis ..."          → "Theorem-backed implementation of ..."
#   "DERIVED from Genesis"         → "DERIVED from internal"
#   "Genesis Phase N"              → "internal phase"
#   "~/Projects/KeiLab/Genesis/"   → ""  (path strip)
#   "Genesis-clean: no normalize"  → ""  (line strip)
#   "Genesis dissolves those"      → "as ill-posed"
#
# Files that NEVER ship to public (stripped at rsync layer):
#   - _primitives/_rust/frustration-matrix/data/corpus/
#   - _primitives/_rust/frustration-matrix/data/labeled-training-set.jsonl
#   - _primitives/_rust/frustration-matrix/data/firmwares/
#   - .claude/worktrees/
#   - _forks/
#   - target/
#   - node_modules/

set -euo pipefail

SRC="${HOME}/Projects/KeiSeiKit"
DST="${HOME}/Projects/KeiSeiKit-public"
MODE="dry-run"

while [ $# -gt 0 ]; do
    case "$1" in
        --src) SRC="$2"; shift 2 ;;
        --dst) DST="$2"; shift 2 ;;
        --confirm) MODE="confirm"; shift ;;
        --dry-run) MODE="dry-run"; shift ;;
        *) echo "unknown arg: $1" >&2; exit 2 ;;
    esac
done

if [ ! -d "$SRC/.git" ]; then echo "no .git in $SRC" >&2; exit 1; fi
if [ ! -d "$DST/.git" ]; then echo "no .git in $DST (run init first)" >&2; exit 1; fi

cd "$DST"

# ---- 1. rsync content (mirror tracked files only, no artefacts) ----
echo "==> rsync $SRC → $DST (tracked files only)"
TMP_LIST="$(mktemp)"
trap "rm -f $TMP_LIST" EXIT
( cd "$SRC" && git ls-files ) > "$TMP_LIST"

rsync -a --delete --files-from="$TMP_LIST" \
    --exclude='_primitives/_rust/frustration-matrix/data/corpus/' \
    --exclude='_primitives/_rust/frustration-matrix/data/labeled-training-set.jsonl' \
    --exclude='_primitives/_rust/frustration-matrix/data/firmwares/' \
    --exclude='.claude/worktrees/' \
    --exclude='_forks/' \
    "$SRC/" "$DST/"

# ---- 2. sed substitutions ----
echo "==> applying scrub rules"
SUBSTITUTIONS=(
    's|Port of Genesis `[a-z_.0-9]\+\.py` + `[a-z_.0-9]\+\.py`\. Theorem|Theorem|g'
    's|Genesis HISTORY\.md §[0-9-]\+|internal calibration|g'
    's|`~/Projects/KeiLab/Genesis/firmware/HISTORY\.md`|internal calibration|g'
    's|DERIVED from Genesis|DERIVED from internal|g'
    's|`Genesis-clean: no [^`]*`|implementation note removed|g'
    's|Genesis-clean: no normalize.*$||g'
    's|Genesis dissolves those|as ill-posed|g'
    's|Genesis Phase 3 compression ratio|internal compression-ratio target|g'
    's|Genesis Phase 5|internal phase|g'
    's|\[E2 VERIFIED: Genesis [^]]*\]||g'
    's|\[E1 VERIFIED: source\]||g'
    's|for KeiLab / bio work|for any pre-registered experiment work|g'
    's|`~/Projects/KeiLab/Genesis/`||g'
    's|Genesis HISTORY\.md|internal calibration notes|g'
)

# Targets: only files we control, not data dir
TARGETS=$(find "$DST/_primitives/_rust/frustration-matrix" \
                "$DST/_primitives/_rust/kei-memory" \
                -type f \( -name "*.rs" -o -name "*.toml" \) 2>/dev/null || true)

for f in $TARGETS; do
    for rule in "${SUBSTITUTIONS[@]}"; do
        sed -i.bak -E "$rule" "$f" 2>/dev/null || true
    done
    rm -f "$f.bak"
done

# ---- 3. validate: no residual Genesis/KeiLab content leaks ----
echo "==> validating clean"
RESIDUAL=$(grep -rniE "Genesis (HISTORY|Phase|firmware|paradigm|paths|compute_firmware|deep_firmware)" \
    "$DST/_primitives/_rust/" \
    --include="*.rs" --include="*.toml" 2>/dev/null \
    | grep -vE "target/|^Binary file" || true)

if [ -n "$RESIDUAL" ]; then
    echo "==> RESIDUAL Genesis content content (review before commit):"
    echo "$RESIDUAL"
    echo
fi

# ---- 4. git diff + decide ----
git -C "$DST" add -A
DIFF_LINES=$(git -C "$DST" diff --cached --stat | tail -1 || echo "no changes")
echo "==> staged: $DIFF_LINES"
git -C "$DST" diff --cached --stat | tail -20

if [ "$MODE" != "confirm" ]; then
    echo
    echo "==> dry-run complete. Re-run with --confirm to commit."
    echo "==> push with: git -C $DST push github main"
    exit 0
fi

# ---- 5. commit ----
HEAD_SRC=$(git -C "$SRC" rev-parse --short HEAD)
git -C "$DST" commit -m "sync: import logic from private @ ${HEAD_SRC}

$(git -C "$DST" diff --cached --stat | tail -10)

Co-Authored-By: sync-public.sh <noreply@keilab.io>"

echo
echo "==> committed. Push with: git -C $DST push github main"
