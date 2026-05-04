---
title: phase-3-provision
path: vm-provision/phase-3-provision.md
dna_hash: sha256:5ebdfa9cefb5acdb
language: markdown
size_loc: 107
generated: by-keidocs
---

# vm-provision/phase-3-provision.md

## Public API

- `Phase 3 — Provision + SSH First Contact` — > Goal: create the VM via the right `_primitives/provision-<provider>.sh`,
- `3.a — Render cloud-init user-data` — Copy `_blocks/deploy-vps-generic.md`'s `cloud-init.yaml` template to
- `3.b — Choose provisioner + run` — Dispatch by `PROVIDER`:
- `3.c — SSH first contact (TOFU)` — ```bash
- `3.d — AskUserQuestion (confirm IP + ready to harden)` — One `AskUserQuestion`:
- `3.e — Verify criterion` — - [ ] `VM_IP` set.
- `3.f — Constructive-fail paths` — - **Create returned no IP (provisioner exit 2).** Root cause likely API

## Related

- parent: `vm-provision`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
