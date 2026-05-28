set -e
# shellcheck shell=bash
# lib-profile.sh — MANIFEST.toml parser + profile resolver.
#
# Tiny awk-based TOML reader with optional Python fallback for robustness.
# Two shapes used:
#   1. profile.<name> = ["a", "b", ...]
#   2. [primitive.<name>] kind/file/crate/deps/desc
#
# If tomllib (python3.11+) or toml is available, prefer it. Otherwise awk.
#
# Requires: $MANIFEST (set by install.sh).
# Requires: err from lib-log.sh.

have_python_toml() {
  if command -v python3 >/dev/null 2>&1; then
    python3 -c 'import tomllib' >/dev/null 2>&1 && return 0
    python3 -c 'import toml' >/dev/null 2>&1 && return 0
  fi
  return 1
}

# Generic one-line-array TOML reader. Echoes space-separated values of
#   [<table>]
#   <key> = ["a", "b", ...]
# python-tomllib preferred; awk fallback handles one-line arrays only.
# Usage: _toml_array <file> <table> <key>
_toml_array() {
  local file="$1" table="$2" key="$3"
  [ -f "$file" ] || return 1
  if have_python_toml; then
    python3 - "$file" "$table" "$key" <<'PY' 2>/dev/null || return 1
import sys
try:
    import tomllib
    mode = "rb"
except ImportError:
    import toml as tomllib
    mode = "r"
path, table, key = sys.argv[1], sys.argv[2], sys.argv[3]
with open(path, mode) as f:
    data = tomllib.load(f)
vals = data.get(table, {}).get(key)
if vals is None:
    sys.exit(2)
print(" ".join(vals))
PY
  else
    awk -v table="$table" -v key="$key" '
      $0 ~ "^\\[" table "\\]" { in_t=1; next }
      /^\[/ { in_t=0 }
      in_t && $0 ~ "^[[:space:]]*" key "[[:space:]]*=" {
        line = $0
        sub(/^[^\[]*\[/, "", line)
        sub(/\].*$/, "", line)
        gsub(/"/, "", line)
        gsub(/,/, " ", line)
        print line
        exit
      }
    ' "$file"
  fi
}

# Echo space-separated primitive names for a given profile.
# Usage: profile_members <profile-name>
profile_members() {
  local profile="$1"
  [ -f "$MANIFEST" ] || { err "MANIFEST.toml not found at $MANIFEST"; return 1; }
  _toml_array "$MANIFEST" "profile" "$profile"
}

# Echo a field of a primitive. Usage: primitive_field <name> <field>
#   field ∈ { kind, file, crate, desc, deps }
primitive_field() {
  local name="$1" field="$2"
  [ -f "$MANIFEST" ] || return 1
  if have_python_toml; then
    python3 - "$MANIFEST" "$name" "$field" <<'PY' 2>/dev/null
import sys
try:
    import tomllib
    mode = "rb"
except ImportError:
    import toml as tomllib
    mode = "r"
path, name, field = sys.argv[1], sys.argv[2], sys.argv[3]
with open(path, mode) as f:
    data = tomllib.load(f) if mode == "rb" else tomllib.load(f)
p = data.get("primitive", {}).get(name)
if p is None:
    sys.exit(2)
v = p.get(field, "")
if isinstance(v, list):
    print("; ".join(v))
else:
    print(v)
PY
  else
    awk -v pname="$name" -v fname="$field" '
      $0 ~ "^\\[primitive\\." pname "\\]" { in_p=1; next }
      /^\[/ && in_p { in_p=0 }
      in_p && $0 ~ "^[[:space:]]*" fname "[[:space:]]*=" {
        line = $0
        sub(/^[^=]*=[[:space:]]*/, "", line)
        gsub(/^"/, "", line)
        gsub(/"$/, "", line)
        print line
        exit
      }
    ' "$MANIFEST"
  fi
}

# Echo all primitive names defined in MANIFEST.
all_primitive_names() {
  [ -f "$MANIFEST" ] || return 1
  awk '
    /^\[primitive\./ {
      name = $0
      sub(/^\[primitive\./, "", name)
      sub(/\]$/, "", name)
      print name
    }
  ' "$MANIFEST"
}
