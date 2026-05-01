#!/bin/sh
# regen-counts.sh — regenerate README.md counts from sources of truth.
# Markers: <!-- count:NAME -->VAL<!-- /count:NAME -->
# Sources: _primitives/MANIFEST.toml, _primitives/_rust/Cargo.toml, filesystem.
# Usage: ./scripts/regen-counts.sh [--check]
# POSIX sh; no arrays, no bashisms; no yq/jq/python hard deps.

set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
README="$ROOT/README.md"
MANIFEST="$ROOT/_primitives/MANIFEST.toml"
CARGO="$ROOT/_primitives/_rust/Cargo.toml"

die() { printf 'regen-counts: %s\n' "$*" >&2; exit 2; }

count_rust_crates() {
  awk '
    /^\[workspace\]/          { in_ws=1; next }
    /^\[/                     { in_ws=0 }
    in_ws && /members *= *\[/ { in_arr=1 }
    in_arr                    { total += gsub(/"[^"]+"/, "&"); if (index($0, "]")) in_arr=0 }
    END                       { print total+0 }
  ' "$CARGO"
}

count_primitive_kind() {
  awk -v want="$1" '
    /^\[primitive\./ { in_block=1; next }
    /^\[/            { in_block=0 }
    in_block && $0 ~ "^kind *= *\"" want "\"" { n++; in_block=0 }
    END { print n+0 }
  ' "$MANIFEST"
}

count_profile() {
  awk -v key="$1" '
    /^\[profile\]/ { in_p=1; next }
    /^\[/          { in_p=0 }
    in_p && $1 == key && $2 == "=" {
      line=$0; sub(/^[^\[]*\[/, "", line); sub(/\].*$/, "", line)
      print gsub(/"[^"]+"/, "&", line) + 0; exit
    }
  ' "$MANIFEST"
}

count_files() { eval "$1" | wc -l | tr -d ' '; }

RUST_CRATES=$(count_rust_crates)
RUST_PRIMITIVES=$(count_primitive_kind rust)
SHELL_PRIMITIVES=$(count_primitive_kind shell)
TOTAL_PRIMITIVES=$((RUST_PRIMITIVES + SHELL_PRIMITIVES))
SKILLS=$(count_files "find '$ROOT/skills' -maxdepth 2 -name SKILL.md")
HOOKS=$(count_files "find '$ROOT/hooks' -maxdepth 1 -name '*.sh'")
BLOCKS=$(count_files "find '$ROOT/_blocks' -maxdepth 1 -name '*.md' -not -name README.md")
AGENTS=$(count_files "find '$ROOT/_manifests' -maxdepth 1 -name 'kei-*.toml'")
BRIDGES=$(count_files "find '$ROOT/_bridges' -maxdepth 1 \( -name '*.tmpl' -o -name '*.mdc' \)")
PROFILE_FULL=$(count_profile full)
PROFILE_MCP=$(count_profile mcp)
PROFILE_DEV=$(count_profile dev)
PROFILE_OPS=$(count_profile ops)
PROFILE_FRONTEND=$(count_profile frontend)
PROFILE_CORE=$(count_profile core)
LBM_PORTS=10   # hand-maintained: v0.14 LBM port semantic group

[ "$RUST_CRATES" = "$RUST_PRIMITIVES" ] || \
  printf 'regen-counts: WARN Cargo members (%s) != MANIFEST rust kind (%s)\n' \
    "$RUST_CRATES" "$RUST_PRIMITIVES" >&2

apply_markers() {
  awk \
    -v m_rc="$RUST_CRATES"       -v m_rp="$RUST_PRIMITIVES" \
    -v m_sp="$SHELL_PRIMITIVES"  -v m_tp="$TOTAL_PRIMITIVES" \
    -v m_sk="$SKILLS"            -v m_hk="$HOOKS" \
    -v m_bl="$BLOCKS"            -v m_ag="$AGENTS" \
    -v m_br="$BRIDGES" \
    -v m_pf="$PROFILE_FULL"      -v m_pm="$PROFILE_MCP" \
    -v m_pd="$PROFILE_DEV"       -v m_po="$PROFILE_OPS" \
    -v m_pr="$PROFILE_FRONTEND"  -v m_pc="$PROFILE_CORE" \
    -v m_lb="$LBM_PORTS" '
    function sub_marker(name, val,    re) {
      re = "<!-- count:" name " -->[^<]*<!-- /count:" name " -->"
      gsub(re, "<!-- count:" name " -->" val "<!-- /count:" name " -->")
    }
    {
      sub_marker("RUST_CRATES",      m_rc); sub_marker("RUST_PRIMITIVES",  m_rp)
      sub_marker("SHELL_PRIMITIVES", m_sp); sub_marker("TOTAL_PRIMITIVES", m_tp)
      sub_marker("SKILLS",           m_sk); sub_marker("HOOKS",            m_hk)
      sub_marker("BLOCKS",           m_bl); sub_marker("AGENTS",           m_ag)
      sub_marker("BRIDGES",          m_br); sub_marker("PROFILE_FULL",     m_pf)
      sub_marker("PROFILE_MCP",      m_pm); sub_marker("PROFILE_DEV",      m_pd)
      sub_marker("PROFILE_OPS",      m_po); sub_marker("PROFILE_FRONTEND", m_pr)
      sub_marker("PROFILE_CORE",     m_pc); sub_marker("LBM_PORTS",        m_lb)
      print
    }
  '
}

mode="${1:-write}"
[ -f "$README" ] || die "README.md not found at $README"

tmp=$(mktemp -t regen-counts.XXXXXX) || die "mktemp failed"
trap 'rm -f "$tmp"' EXIT INT TERM
apply_markers <"$README" >"$tmp"

if [ "$mode" = "--check" ]; then
  if cmp -s "$README" "$tmp"; then
    echo "regen-counts: no drift"; exit 0
  fi
  echo "regen-counts: DRIFT DETECTED" >&2
  diff -u "$README" "$tmp" >&2 || true
  exit 1
fi

cp "$tmp" "$README"
printf 'regen-counts: README updated (crates=%s skills=%s hooks=%s blocks=%s prims=%s)\n' \
  "$RUST_CRATES" "$SKILLS" "$HOOKS" "$BLOCKS" "$TOTAL_PRIMITIVES"
