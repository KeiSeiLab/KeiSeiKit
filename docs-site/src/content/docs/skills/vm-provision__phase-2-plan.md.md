---
title: phase-2-plan
path: vm-provision/phase-2-plan.md
dna_hash: sha256:87e3f45e967712dd
language: markdown
size_loc: 137
generated: by-keidocs
---

# vm-provision/phase-2-plan.md

## Public API

- `Phase 2 — Plan Mode Doc` — > Goal: produce a written, user-approved plan (RULE 0.5) that enumerates
- `2.a — Synthesise the plan` — Write `<run-dir>/plan.md` (where `<run-dir>` is `./.keisei/vm-provision/<timestamp>/`) with
- `VM-Provision Plan — <timestamp>` — 
- `Intent` — <INTENT one-line>
- `Target` — - Provider: <PROVIDER>
- `Access` — - Admin user:  <ADMIN_USER>       # default keiadmin
- `Ports to allow (ufw + provider cloud firewall)` — <APP_PORTS — list>
- `TLS` — - Host:   <TLS_HOST or none>
- `Hardening steps (harden-base.sh)` — - apt update + upgrade
- `Verification (hard gate before handoff)` — - ssh-check  → exit 0
- `Rollback` — - `_primitives/provision-<provider>.sh destroy <VM_NAME>` — 1-command destroy.
- `Cost estimate` — <Plan price per month from PROVIDER pricing page; CITE>
- `2.b — Build the `firewall-intent.yaml`` — Write `<run-dir>/firewall-intent.yaml`:
- `2.c — AskUserQuestion (customise ports, TLS, admin name)` — One `AskUserQuestion` call with up to 4 questions:
- `2.d — Present the plan for approval` — Render `plan.md` in chat. Ask ONE final AskUserQuestion:
- `2.e — Verify criterion` — - [ ] `plan.md` written to `<run-dir>/plan.md`.

## Related

- parent: `vm-provision`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
