#!/usr/bin/env bash
# build-index.sh — регенерация _blocks/INDEX.md из *.md.
#
# Использование:
#   cd ~/Projects/KeiSeiKit-public/_blocks && bash build-index.sh
#   # или из любого места:
#   bash $(git rev-parse --show-toplevel)/_blocks/build-index.sh
#
# Что делает:
#   1. Сканит _blocks/*.md (исключая README.md и INDEX.md).
#   2. Группирует по префиксу (api-, auth-, ci-, db-, deploy-, ...).
#   3. Для каждого блока берёт первую H1-строку как описание.
#   4. Пишет INDEX.md с разбиением по 14 категориям + "Прочие".
#
# Безопасно перезапускать — детерминированный output.

set -euo pipefail

# Запускаемся всегда из _blocks/.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

CATEGORIES=(api auth ci db deploy docs domain mode obs path rule scraper security stack test)

OUT="INDEX.md"
TMP="${OUT}.tmp.$$"
trap 'rm -f "$TMP"' EXIT

{
  printf '# Реестр блоков KeiSeiKit\n\n'
  printf '> SSoT для assembler. Все блоки доступные для `blocks = [...]` в `_manifests/<agent>.toml`.\n'
  printf '> Авто-генерируется из `_blocks/*.md` через `bash build-index.sh`.\n'
  printf '> Каждый файл = атомарный кубик (Constructor Pattern).\n\n'
  printf 'Пример:\n```toml\nblocks = ["baseline", "rule-pre-dev-gate", "api-anthropic"]\n```\n\n'
  printf '## По категориям\n\n'

  for cat in "${CATEGORIES[@]}"; do
    upper=$(echo "$cat" | tr '[:lower:]' '[:upper:]')
    files=$(ls 2>/dev/null | grep -E "^${cat}(-|\.).*\.md$" || true)
    [ -z "$files" ] && continue
    printf '### %s\n\n' "$upper"
    while IFS= read -r f; do
      [ -z "$f" ] && continue
      name="${f%.md}"
      desc=$(awk '/^# / { sub(/^# /, ""); print; exit }' "$f" 2>/dev/null || true)
      [ -z "$desc" ] && desc="(no title)"
      printf -- '- `%s` — %s\n' "$name" "$desc"
    done <<< "$files"
    printf '\n'
  done

  printf '### Прочие (без категорийного префикса)\n\n'
  while IFS= read -r f; do
    name="${f%.md}"
    case "$name" in
      api-*|auth-*|ci-*|db-*|deploy-*|docs-*|domain-*|mode-*|obs-*|path-*|rule-*|scraper-*|security-*|stack-*|test-*|README|INDEX) continue ;;
    esac
    desc=$(awk '/^# / { sub(/^# /, ""); print; exit }' "$f" 2>/dev/null || true)
    [ -z "$desc" ] && desc="(no title)"
    printf -- '- `%s` — %s\n' "$name" "$desc"
  done < <(ls *.md)

  total=$(ls *.md | grep -vE '^(README|INDEX)\.md$' | wc -l | tr -d ' ')
  printf '\n---\n\nВсего блоков: %d.\n' "$total"
  printf 'Перегенерация: `bash _blocks/build-index.sh`.\n'
} > "$TMP"

mv "$TMP" "$OUT"
trap - EXIT

echo "✓ $OUT regenerated"
wc -l "$OUT"
