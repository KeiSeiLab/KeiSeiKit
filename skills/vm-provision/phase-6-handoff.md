# Phase 6 — Handoff + Final Report

> Goal: emit a single, complete report and (optionally) hand off to
> `/web-deploy` or `/auth-setup`. No further mutation to the VM from this
> skill.
> **Verify criterion:** final report emitted; all Phase-1..5 artefacts
> listed with absolute paths; next-skill dispatch (if any) announced.

---

## 6.a — Artefact ledger

Collect and surface:

- `<run-dir>/plan.md`                      — Phase 2
- `<run-dir>/cloud-init.yaml`              — Phase 3 input
- `<run-dir>/firewall-intent.yaml`         — Phase 2 source of truth
- `<run-dir>/harden.log`                   — Phase 4 stderr
- `<run-dir>/post-harden.txt`              — Phase 4 systemctl snapshot
- `<run-dir>/sshd_config` + `sshd_config.d/` — Phase 5 input (captured)
- `<run-dir>/ufw-status.txt`               — Phase 5 input (captured)
- `<run-dir>/ssh-check.json`               — Phase 5 output
- `<run-dir>/firewall-diff.json`           — Phase 5 output

Every path must exist on disk before emitting the report. Missing
artefact = bug in an earlier phase; STOP and surface the gap.

---

## 6.b — Final report

```
=== /VM-PROVISION REPORT ===
Intent:        <first 80 chars of INTENT>
Provider:      <PROVIDER> / region=<REGION> / plan=<PLAN> / arch=<ARCH>
VM:            <VM_NAME> @ <VM_IP>
Admin:         <ADMIN_USER> (ssh port <SSH_PORT>)
Ports:         <APP_PORTS joined>
TLS:           <TLS_HOST or "none">
Hardened:      <HARDENED>
Verification:  ssh-check=PASS  firewall-diff=PASS
Handoff:       <HANDOFF_TO>
Artefacts:
  - <run-dir>/plan.md
  - <run-dir>/cloud-init.yaml
  - <run-dir>/firewall-intent.yaml
  - <run-dir>/harden.log
  - <run-dir>/post-harden.txt
  - <run-dir>/sshd_config        (+ sshd_config.d/)
  - <run-dir>/ufw-status.txt
  - <run-dir>/ssh-check.json
  - <run-dir>/firewall-diff.json
AskUserQuestion count: <N, should be ≥ 6>
```

No prose after the ledger. The report is the contract.

---

## 6.c — Handoff (no AskUserQuestion; next-skill dispatch inferred)

If `TLS_HOST` was set AND the caller's intent mentions deploying an app
— dispatch to `/web-deploy` with the VM IP and admin credentials
(by env-var reference only, RULE 0.8). Surface:

> `Handoff → /web-deploy <VM_IP> --admin <ADMIN_USER> --tls <TLS_HOST>`

If the intent mentions auth / identity — surface:

> `Handoff → /auth-setup <VM_IP>`

Otherwise: `HANDOFF_TO=none`. User invokes the next skill manually when
ready.

**Never** run the next skill automatically — the user already clicked
their way through 6 phases; handing off to another multi-phase skill
without a pause is hostile UX.

---

## 6.d — Memory save (RULE memory-protocol)

Append to `memory/{project-or-infra}.md`:

```markdown
### VM provisioned: <VM_NAME> (YYYY-MM-DD) [E1]
- Provider: <PROVIDER> <PLAN> @ <REGION>
- IP: <VM_IP>
- Admin: <ADMIN_USER>
- Hardened: harden-base.sh rev <git-sha>
- Verify: ssh-check + firewall-diff both PASS
- Cost: <X>/month (cited @ <date>)
- Artefacts: <run-dir>/
```

Evidence grade E1 — facts are direct observations (we ran the commands,
we have the exit codes, we can re-verify on demand).

If the project file doesn't exist yet, create `memory/{slug}.md` and add
a single line to `MEMORY.md` under the right section.

---

## 6.e — Verify criterion

- [ ] Report emitted.
- [ ] All 9+ artefacts exist on disk at absolute paths.
- [ ] `memory/{project}.md` updated (or created) with the provision entry.
- [ ] `HANDOFF_TO` announced (or `none`).

---

## 6.f — Rollback instructions (always include in the report)

```
# destroy the VM + all its resources (idempotent)
_primitives/provision-<PROVIDER>.sh destroy <VM_NAME> --force

# purge local artefacts (plan, logs, captured configs)
rm -rf <run-dir>
```

Keep them visible — Future-Us will appreciate the 1-command path back.
