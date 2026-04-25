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

# Filter the git-ls-files list with grep BEFORE handing to rsync.
# rsync's `--exclude` is ignored when `--files-from` is used (its glob
# matcher only fires for directory traversal, not for explicit-file mode).
# So we pre-strip every path that would leak Genesis-laden data.
EXCLUDED_PATTERNS='^(_primitives/_rust/frustration-matrix/data/corpus/|_primitives/_rust/frustration-matrix/data/labeled-training-set\.jsonl|_primitives/_rust/frustration-matrix/data/firmwares/|\.claude/worktrees/|_forks/)'
( cd "$SRC" && git ls-files ) | grep -vE "$EXCLUDED_PATTERNS" > "$TMP_LIST"

rsync -a --files-from="$TMP_LIST" "$SRC/" "$DST/"

# Drop public-side files that no longer exist (or are now excluded) on src.
# We can't use `rsync --delete` with `--files-from` reliably (it limits to
# the listed paths); do it explicitly via diff against the destination tree.
DST_LIST="$(mktemp)"
DELETIONS="$(mktemp)"
trap "rm -f $TMP_LIST $DST_LIST $DELETIONS" EXIT
( cd "$DST" && git ls-files ) > "$DST_LIST"
# Files in DST that are not in the new (filtered) source list AND are not
# locally-managed public-only artefacts (sync script, mirror docs, sync
# meta) get deleted.
LOCAL_ONLY='^(tools/sync-public\.sh|PUBLIC-MIRROR\.md|_primitives/_rust/frustration-matrix/data/README\.md)$'
{ comm -23 <(sort "$DST_LIST") <(sort "$TMP_LIST") || true; } \
    | { grep -vE "$LOCAL_ONLY" || true; } > "$DELETIONS"
if [ -s "$DELETIONS" ]; then
    while IFS= read -r f; do
        [ -n "$f" ] && rm -f "$DST/$f"
    done < "$DELETIONS"
fi

# ---- 2. sed substitutions ----
echo "==> applying scrub rules"
# Rules are run in order — order matters. Each rule is a `sed -E` script.
# More-specific rules first; the catch-all "Genesis " (with trailing space)
# rule at the bottom must NOT run before the structured ones.
SUBSTITUTIONS=(
    # Multi-token leak: "Port of Genesis `compute_firmware_v2.py` + `deep_firmware.py`. Theorem"
    's#Port of Genesis `[a-z_.0-9]+\.py`( \+ `[a-z_.0-9]+\.py`)?\. Theorem#Theorem#g'
    # "Ports Genesis `<file>.py` / `<file>.py`. Output is..." (CLI subcommand docstring form)
    's#Ports Genesis `[a-z_.0-9]+\.py`( / `[a-z_.0-9]+\.py`)?\. ##g'
    # Bare reference: "Genesis `<file>.py` /  `<file>.py`" with no trailing period
    's#Genesis `[a-z_.0-9]+\.py`( / `[a-z_.0-9]+\.py`)?##g'
    # Genesis `<file>.py` line N reference
    's#Genesis `[a-z_.0-9]+\.py` line [0-9]+:?##g'
    # E-grade citations referencing Genesis or source
    's#\[E[0-9] VERIFIED: Genesis [^]]*\]##g'
    's#\[E[0-9] VERIFIED: source\]##g'
    # Genesis HISTORY.md section/line refs (most specific first)
    's#`~/Projects/KeiLab/Genesis/firmware/HISTORY\.md` §[0-9]+ l\.[0-9]+#internal calibration#g'
    's#`~/Projects/KeiLab/Genesis/firmware/HISTORY\.md`#internal calibration#g'
    's#~/Projects/KeiLab/Genesis/firmware/HISTORY\.md##g'
    's#Genesis HISTORY\.md §[0-9-]+#internal calibration#g'
    's#Genesis HISTORY\.md#internal calibration notes#g'
    's#`~/Projects/KeiLab/Genesis/`##g'
    's#~/Projects/KeiLab/Genesis/##g'
    # Phase-N labels that imply Genesis project structure
    's#Genesis Phase 3 compression ratio#internal compression-ratio target#g'
    's#Genesis Phase [0-9]+ entropy curve#internal entropy-curve calibration#g'
    's#Genesis Phase [0-9]+#internal phase#g'
    # Catch-all attribution rewrites
    's#DERIVED from Genesis#DERIVED from internal#g'
    's#Genesis dissolves those#as ill-posed#g'
    's#Genesis-clean: no normalize\(S\), no Frobenius, no rank-r decomposition\.##g'
    's#Genesis-clean: no [a-zA-Z_(),.0-9 -]+\.##g'
    's#for KeiLab / bio work#for any pre-registered experiment work#g'
    # Trailing whitespace artefacts left by stripping
    's#  +$##g'
    's#^(//!|//|/\*\*?)\s*$##g'
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
# Match any Genesis/KeiLab/HISTORY.md reference that is NOT legitimate
# author attribution (README.md / docs/PHILOSOPHY.md / docs/ARCHITECTURE.md
# carry "KeiLab" as the org name and that's allowed).
RESIDUAL=$(grep -rniE "Genesis [A-Z]|Genesis \`|Port of Genesis|HISTORY\.md|/KeiLab/Genesis|deep_firmware|compute_firmware" \
    "$DST/_primitives/_rust/" \
    --include="*.rs" --include="*.toml" 2>/dev/null \
    | grep -vE "target/|^Binary file" || true)

if [ -n "$RESIDUAL" ]; then
    echo "==> RESIDUAL Genesis content — sync ABORTED:"
    echo "$RESIDUAL"
    echo
    echo "Add a substitution rule for the leak above and re-run."
    echo "(See SUBSTITUTIONS array above. Stage stays unstashed for inspection.)"
    exit 3
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
