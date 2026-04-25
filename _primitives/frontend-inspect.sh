#!/usr/bin/env sh
# frontend-inspect — scan a project directory and report what it is:
# framework (Astro/Next/SvelteKit/Vite-React), styling (Tailwind/CSS-Modules/
# styled-components), UI-component count, and package-manager lockfile.
#
# USAGE
#   frontend-inspect [<dir>]              # default: current directory
#   frontend-inspect <dir> --json         # machine-readable JSON output

set -eu

DIR="${1:-.}"
JSON=0
[ "${2:-}" = "--json" ] && JSON=1

usage() {
  cat <<'EOF'
Usage: frontend-inspect [<dir>] [--json]

Reports:
  - Framework (astro / next / sveltekit / vite-react / static / unknown)
  - Styling (tailwind4 / tailwind3 / css-modules / plain)
  - Package manager (npm / pnpm / yarn / bun)
  - Component file count (.tsx / .vue / .svelte / .astro)
  - Contains tests? (yes/no)
EOF
}

[ "$DIR" = "-h" ] || [ "$DIR" = "--help" ] && { usage; exit 0; }
[ -d "$DIR" ] || { echo "frontend-inspect: $DIR not a directory" >&2; exit 1; }

PKG="$DIR/package.json"

has_dep() {
  # $1 = dep name
  [ -f "$PKG" ] || return 1
  if command -v jq >/dev/null 2>&1; then
    jq -e --arg d "$1" '(.dependencies[$d] // .devDependencies[$d] // null) != null' "$PKG" >/dev/null 2>&1
  else
    grep -q "\"$1\"" "$PKG" 2>/dev/null
  fi
}

detect_framework() {
  if has_dep astro;      then echo astro;      return; fi
  if has_dep next;       then echo next;       return; fi
  if has_dep "@sveltejs/kit"; then echo sveltekit; return; fi
  if has_dep vite && has_dep react; then echo vite-react; return; fi
  if has_dep vite && has_dep vue;   then echo vite-vue;   return; fi
  if has_dep vite;       then echo vite;       return; fi
  [ -f "$DIR/index.html" ] && echo static && return
  echo unknown
}

detect_styling() {
  if has_dep tailwindcss; then
    # Tailwind 4 has `@theme` in CSS and no tailwind.config.js, usually; rough heuristic:
    if [ -f "$DIR/tailwind.config.ts" ] || [ -f "$DIR/tailwind.config.js" ] || [ -f "$DIR/tailwind.config.mjs" ]; then
      echo tailwind3
    else
      echo tailwind4
    fi
    return
  fi
  if has_dep "styled-components"; then echo styled-components; return; fi
  if find "$DIR/src" -maxdepth 3 -name '*.module.css' -print -quit 2>/dev/null | grep -q .; then
    echo css-modules
    return
  fi
  echo plain
}

detect_pm() {
  [ -f "$DIR/pnpm-lock.yaml" ] && echo pnpm && return
  [ -f "$DIR/yarn.lock" ]       && echo yarn && return
  [ -f "$DIR/bun.lockb" ]       && echo bun && return
  [ -f "$DIR/package-lock.json" ] && echo npm && return
  echo none
}

count_components() {
  find "$DIR/src" -type f \( -name '*.tsx' -o -name '*.vue' -o -name '*.svelte' -o -name '*.astro' \) 2>/dev/null | wc -l | tr -d ' '
}

has_tests() {
  if [ -f "$PKG" ] && (has_dep vitest || has_dep jest || has_dep "@playwright/test"); then
    echo yes
  else
    echo no
  fi
}

FW="$(detect_framework)"
ST="$(detect_styling)"
PM="$(detect_pm)"
CC="$(count_components)"
TS="$(has_tests)"

if [ "$JSON" = "1" ]; then
  printf '{"dir":"%s","framework":"%s","styling":"%s","pm":"%s","components":%s,"tests":"%s"}\n' \
    "$DIR" "$FW" "$ST" "$PM" "$CC" "$TS"
else
  printf "dir:        %s\n" "$DIR"
  printf "framework:  %s\n" "$FW"
  printf "styling:    %s\n" "$ST"
  printf "pm:         %s\n" "$PM"
  printf "components: %s\n" "$CC"
  printf "tests:      %s\n" "$TS"
fi
