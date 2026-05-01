#!/usr/bin/env bash
# Auto-generate marketplace src/data/*.ts from real public kit content.
# Run from repo root; targets ~/Projects/keisei-marketplace/src/data/.
#
# Usage:
#   ./tools/marketplace-sync/sync.sh

set -uo pipefail
# Note: -e disabled — grep+head+sed pipelines may legitimately return
# non-zero when a Cargo.toml has no `description` field; we handle that
# case explicitly via the `[ -z "$DESC" ]` fallback inside each loop.

KIT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")"/../.. && pwd)"
MARKETPLACE_DIR="${MARKETPLACE_DIR:-$HOME/Projects/keisei-marketplace}"
DATA_DIR="$MARKETPLACE_DIR/src/data"

if [ ! -d "$DATA_DIR" ]; then
  echo "error: marketplace data dir not found: $DATA_DIR" >&2
  exit 1
fi

echo "Kit:         $KIT_DIR"
echo "Marketplace: $MARKETPLACE_DIR"
echo ""

# --- primitives.ts ---
echo "▶ Generating primitives.ts from Cargo workspace..."
WORKSPACE_TOML="$KIT_DIR/_primitives/_rust/Cargo.toml"
PRIMS=()
while IFS= read -r line; do PRIMS+=("$line"); done < <(awk '/^members = \[/,/^]/' "$WORKSPACE_TOML" | grep -oE '"[^"]+"' | tr -d '"' | sort -u)
TOTAL=${#PRIMS[@]}
echo "  Found $TOTAL workspace members"

cat > "$DATA_DIR/primitives.ts" <<HEADER
/**
 * PRIMITIVES — auto-generated from KeiSeiKit-public/_primitives/_rust workspace.
 * SSoT for /primitives, /primitives/[slug].
 *
 * Regenerate with: ./tools/marketplace-sync/sync.sh
 * Last sync: $(date -u +%Y-%m-%dT%H:%M:%SZ)
 */

export type Primitive = {
  slug: string;
  name: string;
  tagline: string;
  description: string;
  language: "Rust" | "Bash" | "Shell + Rust";
  binSize?: string;
  ruleRefs?: string[];
  featured?: boolean;
};

export const PRIMITIVES: Primitive[] = [
HEADER

for p in "${PRIMS[@]}"; do
  CARGO="$KIT_DIR/_primitives/_rust/$p/Cargo.toml"
  [ -f "$CARGO" ] || continue
  DESC=$(grep -E "^description" "$CARGO" 2>/dev/null | head -1 | sed 's/description = "//; s/"$//; s/\\/\\\\/g; s/"/\\"/g')
  [ -z "$DESC" ] && DESC="Rust primitive in the KeiSeiKit substrate."
  cat >> "$DATA_DIR/primitives.ts" <<ENTRY
  {
    slug: "$p",
    name: "$p",
    tagline: "${DESC:0:80}",
    description: "$DESC",
    language: "Rust",
  },
ENTRY
done

cat >> "$DATA_DIR/primitives.ts" <<'FOOTER'
];

export const PRIMITIVES_BY_SLUG = Object.fromEntries(
  PRIMITIVES.map((p) => [p.slug, p])
);
FOOTER
echo "  Wrote $(grep -c '^  {' "$DATA_DIR/primitives.ts") primitive entries"

# --- skills.ts ---
echo ""
echo "▶ Generating skills.ts from skills/ dir..."
SKILLS=()
while IFS= read -r line; do SKILLS+=("$line"); done < <(ls "$KIT_DIR/skills/" 2>/dev/null | sort -u)
SCOUNT=${#SKILLS[@]}
echo "  Found $SCOUNT skill directories"

cat > "$DATA_DIR/skills.ts" <<HEADER
/**
 * SKILLS — auto-generated from KeiSeiKit-public/skills/.
 * SSoT for /skills, /skills/[slug].
 *
 * Regenerate with: ./tools/marketplace-sync/sync.sh
 * Last sync: $(date -u +%Y-%m-%dT%H:%M:%SZ)
 */

export type Skill = {
  slug: string;
  name: string;
  tagline: string;
  description: string;
  command: string;
};

export const SKILLS: Skill[] = [
HEADER

for s in "${SKILLS[@]}"; do
  SK="$KIT_DIR/skills/$s/SKILL.md"
  [ -f "$SK" ] || SK="$KIT_DIR/skills/$s/skill.md"
  [ -f "$SK" ] || continue
  TAG=$(awk '/^description:/{ sub(/^description: */,""); gsub(/"/,"\\\""); print; exit }' "$SK" | head -c 200)
  [ -z "$TAG" ] && TAG=$(awk '/^# /{ sub(/^# */,""); gsub(/"/,"\\\""); print; exit }' "$SK" | head -c 100)
  [ -z "$TAG" ] && TAG="Skill in the KeiSeiKit substrate."
  cat >> "$DATA_DIR/skills.ts" <<ENTRY
  {
    slug: "$s",
    name: "$s",
    tagline: "${TAG:0:100}",
    description: "$TAG",
    command: "/$s",
  },
ENTRY
done

cat >> "$DATA_DIR/skills.ts" <<'FOOTER'
];

export const SKILLS_BY_SLUG = Object.fromEntries(
  SKILLS.map((s) => [s.slug, s])
);
FOOTER
echo "  Wrote $(grep -c '^  {' "$DATA_DIR/skills.ts") skill entries"

# --- hooks.ts ---
echo ""
echo "▶ Generating hooks.ts from hooks/ dir..."
HOOKS=()
while IFS= read -r line; do HOOKS+=("$line"); done < <(ls "$KIT_DIR/hooks/"*.sh 2>/dev/null | xargs -n1 basename | sed 's/.sh$//' | sort -u)
HCOUNT=${#HOOKS[@]}
echo "  Found $HCOUNT hooks"

cat > "$DATA_DIR/hooks.ts" <<HEADER
/**
 * HOOKS — auto-generated from KeiSeiKit-public/hooks/.
 * SSoT for /hooks, /hooks/[slug].
 *
 * Regenerate with: ./tools/marketplace-sync/sync.sh
 * Last sync: $(date -u +%Y-%m-%dT%H:%M:%SZ)
 */

export type Hook = {
  slug: string;
  name: string;
  description: string;
  event?: string;
};

export const HOOKS: Hook[] = [
HEADER

for h in "${HOOKS[@]}"; do
  HF="$KIT_DIR/hooks/$h.sh"
  [ -f "$HF" ] || continue
  DESC=$(awk 'NR>1 && /^# / && !/^#!/ { sub(/^# */,""); gsub(/"/,"\\\""); print; exit }' "$HF" | head -c 150)
  [ -z "$DESC" ] && DESC="Hook in the KeiSeiKit runtime."
  cat >> "$DATA_DIR/hooks.ts" <<ENTRY
  {
    slug: "$h",
    name: "$h",
    description: "$DESC",
  },
ENTRY
done

cat >> "$DATA_DIR/hooks.ts" <<'FOOTER'
];

export const HOOKS_BY_SLUG = Object.fromEntries(
  HOOKS.map((h) => [h.slug, h])
);
FOOTER
echo "  Wrote $(grep -c '^  {' "$DATA_DIR/hooks.ts") hook entries"

# --- agents.ts (from manifests) ---
echo ""
echo "▶ Generating agents.ts from _manifests/..."
AGENTS=()
while IFS= read -r line; do AGENTS+=("$line"); done < <(ls "$KIT_DIR/_manifests/"*.toml 2>/dev/null | xargs -n1 basename | sed 's/.toml$//' | sort -u)
ACOUNT=${#AGENTS[@]}
echo "  Found $ACOUNT agent manifests"

cat > "$DATA_DIR/agents.ts" <<HEADER
/**
 * AGENTS — auto-generated from KeiSeiKit-public/_manifests/.
 * SSoT for /agents, /agents/[slug].
 *
 * Regenerate with: ./tools/marketplace-sync/sync.sh
 * Last sync: $(date -u +%Y-%m-%dT%H:%M:%SZ)
 */

export type Agent = {
  slug: string;
  name: string;
  description: string;
  substrate_role?: string;
  kind: "hub" | "atomar" | "discipline";
};

export const AGENTS: Agent[] = [
HEADER

for a in "${AGENTS[@]}"; do
  M="$KIT_DIR/_manifests/$a.toml"
  [ -f "$M" ] || continue
  DESC=$(awk '/^description = / { sub(/^description = "/,""); sub(/"$/,""); gsub(/"/,"\\\""); print; exit }' "$M")
  ROLE=$(awk '/^substrate_role = / { sub(/^substrate_role = "/,""); sub(/"$/,""); print; exit }' "$M")
  KIND="hub"
  case "$a" in
    code-implementer-*|validator-*|critic-*|security-auditor-*|infra-implementer-*|researcher-*) KIND="atomar" ;;
  esac
  cat >> "$DATA_DIR/agents.ts" <<ENTRY
  {
    slug: "$a",
    name: "$a",
    description: "$DESC",
    substrate_role: "$ROLE",
    kind: "$KIND",
  },
ENTRY
done

cat >> "$DATA_DIR/agents.ts" <<'FOOTER'
];

export const AGENTS_BY_SLUG = Object.fromEntries(
  AGENTS.map((a) => [a.slug, a])
);
FOOTER
echo "  Wrote $(grep -c '^  {' "$DATA_DIR/agents.ts") agent entries"

echo ""
echo "═══ Summary ═══"
echo "  primitives.ts: $(grep -c '^  {' "$DATA_DIR/primitives.ts") entries"
echo "  skills.ts:     $(grep -c '^  {' "$DATA_DIR/skills.ts") entries"
echo "  hooks.ts:      $(grep -c '^  {' "$DATA_DIR/hooks.ts") entries"
echo "  agents.ts:     $(grep -c '^  {' "$DATA_DIR/agents.ts") entries"
