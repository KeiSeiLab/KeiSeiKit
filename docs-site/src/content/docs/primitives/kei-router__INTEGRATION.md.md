---
title: INTEGRATION
path: kei-router/INTEGRATION.md
dna_hash: sha256:72180d5aae69ccd1
language: markdown
size_loc: 111
generated: by-keidocs
---

# kei-router/INTEGRATION.md

## Public API

- `kei-router LLM provider — INTEGRATION.md` — > v0.40 Wave 32. Multi-provider LLM abstraction. This file is the orchestrator
- `Three-step orchestrator wire-in (kei-cortex)` — ### 1. CLI flag on the daemon (`kei-cortex/src/main.rs`)
- `Errors to surface to clients` — `kei_router::LlmError` enum maps to:
- `Cost-based dispatch (optional, v0.40+)` — When the caller provides a token estimate, `LlmRouter::cheapest_for_estimated_tokens`
- `Testing the integration` — After the three steps above, run the existing kei-cortex tests AND:
- `Constructor Pattern compliance` — | File                                  | LOC | Cubes                |

## Related

- parent: `kei-router`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
