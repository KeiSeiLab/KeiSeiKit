#!/usr/bin/env bash

# Runtime gate (hooks-control skill / KEI_DISABLED_HOOKS / KEI_HOOK_PROFILE).
_KEI_LIB="$(dirname "$0")/_lib/gate.sh"; if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "alignment-check" || exit 0; fi
# ALIGNMENT CHECK HOOK
# Fires on UserPromptSubmit when comparison/experiment keywords detected.
# THREE-TIME REPEAT BUG: exp6, exp24-28, basecaller — all forgot alignment.

INPUT=$(cat)
PROMPT=$(printf '%s' "$INPUT" | jq -r '.prompt // empty' 2>/dev/null)
[ -z "$PROMPT" ] && exit 0

# Detect comparison/experiment keywords
if echo "$PROMPT" | grep -qiE 'compar|delta|divergen|versus|vs\b|difference|запуск|experiment|exp[0-9]|прогон|basecall|сравн|два генома|two genome'; then
  cat <<'HOOK'
{"hookSpecificOutput":{"hookEventName":"UserPromptSubmit","additionalContext":"<alignment-check>\n⚠️ ALIGNMENT CHECK (E14/E20 — failed 3 times!)\n\nBefore ANY comparison between two data streams:\n1. Are they ALIGNED? (MAFFT for genomes, PAF for signal, CIGAR for reads)\n2. How do you KNOW? Show the alignment file/proof.\n3. Does position[i] in stream A = position[i] in stream B?\n\nHistory: exp6 (25%→141x after MAFFT), exp24-28 (25%→60% after PAF alignment).\nCost of forgetting: 8 wasted experiments, ~5 hours.\n\nIf comparing genomes → MAFFT align first.\nIf comparing signal→base → use PAF/segmentation first.\nIf unsure → STOP and ask.\n</alignment-check>"}}
HOOK
fi

exit 0
