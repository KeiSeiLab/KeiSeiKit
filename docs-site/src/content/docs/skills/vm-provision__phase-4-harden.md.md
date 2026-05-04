---
title: phase-4-harden
path: vm-provision/phase-4-harden.md
dna_hash: sha256:2f6c4e44492ea89f
language: markdown
size_loc: 109
generated: by-keidocs
---

# vm-provision/phase-4-harden.md

## Public API

- `Phase 4 — Harden via `harden-base.sh`` — > Goal: run `_primitives/harden-base.sh` on the VM, over SSH, idempotently.
- `4.a — Ship the script` — The script lives on the workstation; copy to the VM and run with `sudo`:
- `4.b — Stream logs` — `harden-base.sh` logs to stderr with timestamps. Capture to
- `4.c — Post-hardening live-check` — After exit 0, SSH back in and confirm:
- `4.d — AskUserQuestion (ready to verify?)` — One `AskUserQuestion`:
- `4.e — Verify criterion` — - [ ] `harden-base.sh` exited 0.
- `4.f — Non-obvious failure modes` — - **`systemctl reload ssh` fails because `sshd -t` rejects the drop-in.**

## Related

- parent: `vm-provision`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
