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

# Echo space-separated primitive names for a given profile.
# Usage: profile_members <profile-name>
profile_members() {
  local profile="$1"
  [ -f "$MANIFEST" ] || { err "MANIFEST.toml not found at $MANIFEST"; return 1; }
  if have_python_toml; then
    python3 - "$MANIFEST" "$profile" <<'PY' 2>/dev/null || return 1
import sys
try:
    import tomllib
    mode = "rb"
except ImportError:
    import toml as tomllib
    mode = "r"
path, prof = sys.argv[1], sys.argv[2]
with open(path, mode) as f:
    data = tomllib.load(f) if mode == "rb" else tomllib.load(f)
members = data.get("profile", {}).get(prof)
if members is None:
    sys.exit(2)
print(" ".join(members))
PY
  else
    # awk fallback — only handles `profile.<name> = [...]` on one line
    awk -v prof="$profile" '
      /^\[profile\]/ { in_profile=1; next }
      /^\[/ && !/^\[profile\]/ { in_profile=0 }
      in_profile && $0 ~ "^[[:space:]]*" prof "[[:space:]]*=" {
        line = $0
        sub(/^[^\[]*\[/, "", line)
        sub(/\].*$/, "", line)
        gsub(/"/, "", line)
        gsub(/,/, " ", line)
        print line
        exit
      }
    ' "$MANIFEST"
  fi
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
