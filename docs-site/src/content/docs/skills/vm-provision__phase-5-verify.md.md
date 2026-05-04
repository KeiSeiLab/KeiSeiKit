---
title: phase-5-verify
path: vm-provision/phase-5-verify.md
dna_hash: sha256:39b82412068d76c1
language: markdown
size_loc: 112
generated: by-keidocs
---

# vm-provision/phase-5-verify.md

## Public API

- `Phase 5 — Verification Hard Gate (`ssh-check` + `firewall-diff`)` — > Goal: fail-closed verification. Phase 6 refuses to run unless BOTH
- `5.a — Pull config artefacts from the VM` — ```bash
- `5.b — Run `ssh-check`` — ```bash
- `5.c — Run `firewall-diff`` — ```bash
- `5.d — Decision tree` — | `ssh-check` | `firewall-diff` | Action |
- `5.e — The AskUserQuestion` — Exactly ONE AskUserQuestion, gated on the decision tree above:
- `5.f — Verify criterion` — - [ ] `ssh-check` exit 0.
- `5.g — Non-obvious pitfalls` — - **sshd_config.d drop-in not loaded.** Debian 12's default

## Related

- parent: `vm-provision`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
