---
title: phase-3-deploy-key
path: sleep-setup/phase-3-deploy-key.md
dna_hash: sha256:35014a92d2a139e9
language: markdown
size_loc: 70
generated: by-keidocs
---

# sleep-setup/phase-3-deploy-key.md

## Public API

- `Phase 3 — Run setup script, hand off deploy key` — Run the imperative helper and hand the public-key material to the user.
- `3a — Invoke `kei-sleep-setup.sh`` — Run the primitive non-interactively with `REPO_URL` pre-supplied:
- `3b — Render deploy-key block to chat` — The script already printed the key + fingerprint to its stdout. Echo
- `3c — Confirm click` — Emit ONE `AskUserQuestion`:
- `Verify-criterion` — - `~/.ssh/keisei-memory-sync(.pub)` exist.

## Related

- parent: `sleep-setup`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
