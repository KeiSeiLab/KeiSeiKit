#!/usr/bin/env bash
# tomd — universal non-native-format → markdown converter.
# First-class primitive. Universal non-native-format → markdown converter.
# Install path: $HOME/.claude/agents/_primitives/tomd.sh.
# Deps: pandoc, python3, jq. Optional: pymupdf4llm, openpyxl, tesseract.

set -euo pipefail

check_deps() {
  local missing=()
  command -v python3 >/dev/null 2>&1 || missing+=("python3 (system)")
  command -v jq      >/dev/null 2>&1 || missing+=("jq      (brew install jq)")
  if [ "${#missing[@]}" -gt 0 ]; then
    echo "[tomd] missing core prerequisites:" >&2
    for m in "${missing[@]}"; do echo "[tomd]   - $m" >&2; done
    echo "[tomd] hint: brew install jq && pip3 install pymupdf4llm openpyxl" >&2
    exit 1
  fi
}
need_pandoc() {
  command -v pandoc >/dev/null 2>&1 && return 0
  echo "[tomd] pandoc required for this format. Install: brew install pandoc" >&2
  exit 1
}

detect_format() {
  local f="$1"
  [ "$f" = "-" ] && { echo stdin; return; }
  local l; l=$(printf '%s' "$f" | tr '[:upper:]' '[:lower:]')
  case "$l" in
    *.pdf)  echo pdf ;; *.docx) echo docx ;; *.doc) echo doc ;;
    *.html|*.htm) echo html ;; *.pptx) echo pptx ;;
    *.xlsx) echo xlsx ;; *.csv)  echo csv ;;
    *.json) echo "fence:json" ;; *.yaml|*.yml) echo "fence:yaml" ;;
    *.xml)  echo "fence:xml" ;; *.toml) echo "fence:toml" ;; *.sql) echo "fence:sql" ;;
    *.png|*.jpg|*.jpeg|*.gif|*.webp|*.svg) echo image ;;
    *.py)   echo "fence:python" ;; *.go) echo "fence:go" ;;
    *.ts|*.tsx) echo "fence:typescript" ;; *.js|*.jsx) echo "fence:javascript" ;;
    *.rs)   echo "fence:rust" ;; *.c|*.h) echo "fence:c" ;; *.cpp|*.hpp) echo "fence:cpp" ;;
    *.swift) echo "fence:swift" ;; *.sh|*.bash|*.zsh) echo "fence:bash" ;;
    *.zig)  echo "fence:zig" ;; *.md) echo md ;; *) echo text ;;
  esac
}

convert_pdf() {
  python3 - "$1" <<'PYEOF'
import sys
p=sys.argv[1]
try: import pymupdf4llm; print(pymupdf4llm.to_markdown(p))
except ImportError:
    try:
        import fitz; doc=fitz.open(p)
        for page in doc: print(page.get_text("text")); print()
    except ImportError:
        sys.stderr.write("[tomd] pdf: pip3 install pymupdf4llm\n"); sys.exit(1)
PYEOF
}

convert_pandoc() {
  need_pandoc
  local from="${2:-}"
  if [ -n "$from" ]; then pandoc -f "$from" -t markdown --wrap=none "$1"
  else pandoc -t markdown --wrap=none "$1"; fi
}

convert_doc() {
  if ! command -v textutil >/dev/null 2>&1; then
    echo "[tomd] .doc: textutil not available (macOS only). Convert to .docx first." >&2
    exit 1
  fi
  need_pandoc
  local tmp; tmp=$(mktemp /tmp/tomd-XXXX.html)
  textutil -convert html -output "$tmp" "$1"
  pandoc -f html -t markdown --wrap=none "$tmp"; rm -f "$tmp"
}

convert_csv() {
  python3 - "$1" <<'PYEOF'
import csv, sys
with open(sys.argv[1]) as f: rows=list(csv.reader(f))
if not rows: sys.exit(0)
print('| '+' | '.join(rows[0])+' |')
print('|'+'|'.join(['---']*len(rows[0]))+'|')
for r in rows[1:]: print('| '+' | '.join(r)+' |')
PYEOF
}

convert_xlsx() {
  python3 - "$1" <<'PYEOF'
import sys
try: import openpyxl
except ImportError: sys.stderr.write("[tomd] xlsx: pip3 install openpyxl\n"); sys.exit(1)
wb=openpyxl.load_workbook(sys.argv[1], data_only=True)
for name in wb.sheetnames:
    ws=wb[name]; print(f"## Sheet: {name}\n")
    rows=list(ws.iter_rows(values_only=True))
    if not rows: continue
    h=[str(c) if c is not None else '' for c in rows[0]]
    print('| '+' | '.join(h)+' |'); print('|'+'|'.join(['---']*len(h))+'|')
    for r in rows[1:]:
        print('| '+' | '.join(str(c) if c is not None else '' for c in r)+' |')
    print()
PYEOF
}

fence() { echo "\`\`\`$1"; cat "$2"; echo '```'; }

convert_json() {
  echo '```json'
  if [ "$1" = "-" ]; then jq '.' 2>/dev/null || cat
  else jq '.' "$1" 2>/dev/null || cat "$1"; fi
  echo '```'
}

convert_image() {
  echo "![$(basename "$1")]($1)"
  if command -v tesseract >/dev/null 2>&1; then
    echo; echo "**OCR:**"; echo '```'
    tesseract "$1" stdout 2>/dev/null || echo "(failed)"
    echo '```'
  fi
}

convert_stdin() {
  local c; c=$(cat)
  if printf '%s' "$c" | jq '.' >/dev/null 2>&1; then
    echo '```json'; printf '%s' "$c" | jq '.'; echo '```'
  else printf '%s\n' "$c"; fi
}

convert_single() {
  local f="$1" fmt; fmt=$(detect_format "$f")
  case "$fmt" in
    pdf)        convert_pdf "$f" ;;
    docx)       convert_pandoc "$f" ;;
    doc)        convert_doc "$f" ;;
    html)       convert_pandoc "$f" html ;;
    pptx)       convert_pandoc "$f" ;;
    xlsx)       convert_xlsx "$f" ;;
    csv)        convert_csv "$f" ;;
    fence:json) convert_json "$f" ;;
    fence:*)    fence "${fmt#fence:}" "$f" ;;
    image)      convert_image "$f" ;;
    md|text)    cat "$f" ;;
    stdin)      convert_stdin ;;
    *)          echo "[tomd] unknown format for $f" >&2; return 1 ;;
  esac
}

convert_dir() {
  local dir="${1:-.}" outdir="${1:-.}/_md" count=0
  mkdir -p "$outdir"
  while IFS= read -r -d '' file; do
    local fmt; fmt=$(detect_format "$file")
    { [ "$fmt" = "text" ] || [ "$fmt" = "md" ]; } && continue
    local out="$outdir/$(basename "${file%.*}").md"
    echo "[tomd]   $file -> $out" >&2
    convert_single "$file" > "$out" 2>/dev/null && count=$((count+1)) || true
  done < <(find "$dir" -maxdepth 2 -type f -not -path '*/_md/*' -not -path '*/.git/*' -print0)
  echo "[tomd] converted $count files -> $outdir" >&2
}

usage() { cat <<'EOF'
Usage: tomd <file> [--output out.md]
       tomd --dir <folder>
       cat file | tomd -

Converts non-native formats to markdown (for LLM ingestion).
Formats: PDF DOCX DOC HTML PPTX XLSX CSV JSON YAML XML TOML SQL
         images(+OCR) code(py/go/ts/rs/c/swift/sh/zig) text
EOF
}

check_deps
case "${1:-}" in
  -h|--help|help) usage; exit 0 ;;
  --dir)          convert_dir "${2:-.}"; exit 0 ;;
  "")             usage; exit 1 ;;
esac

output=""; file="$1"; shift
while [ $# -gt 0 ]; do
  case "$1" in --output) output="$2"; shift 2 ;; *) shift ;; esac
done
if [ -n "$output" ]; then convert_single "$file" > "$output"; echo "[tomd] wrote: $output" >&2
else convert_single "$file"; fi
