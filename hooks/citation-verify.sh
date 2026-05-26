#!/bin/bash

# Runtime gate (hooks-control skill / KEI_DISABLED_HOOKS / KEI_HOOK_PROFILE).
_KEI_LIB="$(dirname "$0")/_lib/gate.sh"; if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "citation-verify" || exit 0; fi
# PreToolUse(Edit|Write) — block unverified academic citations
#
# Rule 0.5 NO HALLUCINATION enforcer.
# Triggers on Edit.new_string / Write.content containing academic citation
# patterns. Blocks unless explicit [VERIFIED: <url>] / [UNVERIFIED] /
# retraction context is present.
#
# Incident log (session 68f86858, 2026-04-18) — all fabricated:
#   Alpay-Jorgensen-Levanony 2022 JMP 63:062104
#   Jaffe-Jäkel 2014 CMP 325
#   Letac 2023 Lecture Notes "Cauchy Distributions on Euclidean Spaces and Groups"
#   Archibald-Kratz-Meerschaert-Sabatelli 2001

set -u

INPUT=$(cat)
TOOL=$(printf '%s' "$INPUT" | jq -r '.tool_name // empty' 2>/dev/null)
CONTENT=""

case "$TOOL" in
    Edit)
        CONTENT=$(printf '%s' "$INPUT" | jq -r '.tool_input.new_string // empty' 2>/dev/null)
        ;;
    Write)
        CONTENT=$(printf '%s' "$INPUT" | jq -r '.tool_input.content // empty' 2>/dev/null)
        ;;
    *)
        exit 0
        ;;
esac

[ -z "$CONTENT" ] && exit 0

# Short content unlikely to contain citation blocks
[ "${#CONTENT}" -lt 40 ] && exit 0

# Pattern A — hyphenated multi-author + year: "Foo-Bar 2014", "Alpay-Jorgensen-Levanony 2022"
PAT_A='[A-Z][a-zÀ-ÿ]+(-[A-Z][a-zÀ-ÿ]+){1,3}[, ]+(\(?(19|20)[0-9]{2}\)?)'
# Pattern B — journal+vol:page: "JMP 63:062104", "CMP 325", "Ann. Math. 149"
PAT_B='(J\. Math\. Phys\.|JMP|Comm\. Math\. Phys\.|CMP|Proc\. Amer\. Math\. Soc\.|Proc\. AMS|Ann\. Math\.|Ann\. Statist\.|J\. Multivariate Anal\.|JMVA|Phys\. Rev\. [DEL]|PRL|Adv\. Theor\. Math\. Phys\.|ATMP|J\. Geom\. Phys\.|JGP|Rev\. Mod\. Phys\.|Lett\. Math\. Phys\.|LMP)[  ]*\*?\*?[  ]*[0-9]{1,4}'
# Pattern C — "et al. YYYY" or "and X YYYY"
PAT_C='[A-Z][a-zÀ-ÿ]+( et al\.?| and [A-Z][a-zÀ-ÿ]+)[, ]+(\(?(19|20)[0-9]{2}\)?)'

HITS_A=$(printf '%s' "$CONTENT" | grep -oE "$PAT_A" 2>/dev/null | head -5 || true)
HITS_B=$(printf '%s' "$CONTENT" | grep -oE "$PAT_B" 2>/dev/null | head -5 || true)
HITS_C=$(printf '%s' "$CONTENT" | grep -oE "$PAT_C" 2>/dev/null | head -5 || true)

ALL_HITS=$(printf '%s\n%s\n%s' "$HITS_A" "$HITS_B" "$HITS_C" | grep -v '^$' || true)
[ -z "$ALL_HITS" ] && exit 0

# Allowlist: explicit verification or retraction context
ALLOW_REGEX='\[VERIFIED:|\[UNVERIFIED\]|\[FABRICATED|\[RETRACTED|\[MISATTRIBUTED|FABRICATED|RETRACTED 2026|MISATTRIBUTED|NOT FOUND|unverifiable|misattributed|does NOT exist|do NOT EXIST|are fabricated|were fabricated|\[HOOK-BYPASS:[[:space:]]*citation-verify'
if printf '%s' "$CONTENT" | grep -qE "$ALLOW_REGEX"; then
    exit 0
fi

N=$(printf '%s\n' "$ALL_HITS" | wc -l | tr -d ' ')

cat >&2 <<EOF
[citation-verify] BLOCKED — $N academic citation pattern(s) without verification marker.

Sample matches:
$(printf '%s\n' "$ALL_HITS" | head -6)

RULE 0.5 NO HALLUCINATION. Prior fabrications (session 68f86858, 2026-04-18):
  • "Alpay-Jorgensen-Levanony 2022 JMP 63:062104" → real is 2026 67:022302 (2 authors)
  • "Jaffe-Jäkel 2014 CMP 325" → real is 3-author Jaffe-Jäkel-Martinez CMP 329
  • "Letac 2023 Lecture Notes" → not in DBLP/arXiv/HAL
  • "Archibald-Kratz-Meerschaert-Sabatelli 2001" → no trace

TO PROCEED (any one):
  1) WebFetch/WebSearch the citation → paste DOI/URL as [VERIFIED: <url>] inline
  2) Mark as [UNVERIFIED] and do NOT use as proof
  3) If this is a retraction/audit, include literal: FABRICATED / RETRACTED / MISATTRIBUTED / unverifiable / NOT FOUND

Bypass (emergency only): prepend content with "[HOOK-BYPASS: citation-verify <reason>]".
EOF

# Exit code 2 = blocking error surfaced to model
exit 2
