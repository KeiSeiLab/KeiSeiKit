---
name: vm-provision
description: End-to-end VPS provisioning — select provider → plan → provision → harden → verify (ssh-check + firewall-diff hard-gate) → handoff. 6 phases, ≥6 AskUserQuestion calls, defensive-only. Stops if either verification primitive fails.
argument-hint: <optional one-line intent, e.g. "staging api hetzner eu">
---

# /vm-provision — 6-Phase VPS Pipeline (index)

## When to use

- Provisioning a new VPS from scratch: select provider → plan → provision → harden → verify → handoff.
- Setting up a hardened production or staging server with firewall verification as a hard gate.
- Any new VM where the security posture (fail2ban, UFW, non-root SSH) must be verified before handoff.

You turn a short intent ("staging API in EU") into a **hardened, verified
VPS** ready to host an app. Six phases. Every provider choice, plan detail,
and fix is surfaced as an `AskUserQuestion` click — no silent defaults.

This `SKILL.md` is the INDEX. Each phase lives in its own file, executed in
order. Never skip a phase. Never re-order phases.

---

## Pipeline overview

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 1 | [phase-1-select-provider.md](phase-1-select-provider.md) | Provider + region + plan + ARM/x86 | 2× |
| 2 | [phase-2-plan.md](phase-2-plan.md) | Plan Mode doc: ports, TLS, admin user | 1× |
| 3 | [phase-3-provision.md](phase-3-provision.md) | Provision + SSH first contact | 1× |
| 4 | [phase-4-harden.md](phase-4-harden.md) | Run `harden-base.sh` over SSH | 1× |
| 5 | [phase-5-verify.md](phase-5-verify.md) | `ssh-check` + `firewall-diff` **HARD GATE** | 1× |
| 6 | [phase-6-handoff.md](phase-6-handoff.md) | Artifact list + optional `/web-deploy` | — (final report) |

**Minimum AskUserQuestion count across a complete pipeline: 6+** — pure-
click contract. Only the intent argument and per-port customisations are
typed.

---

## Hard-Gate Invariant (LOAD-BEARING)

> **No application is deployed onto a VM that has not passed BOTH
> `ssh-check` (exit 0) and `firewall-diff` (exit 0) in Phase 5.**

Enforced by Phase 5:

- `ssh-check --config /etc/ssh/sshd_config --drop-in /etc/ssh/sshd_config.d` → exit 0.
- `ufw status numbered | firewall-diff --intent firewall-intent.yaml --stdin` → exit 0.
- Any non-zero exit → STOP the pipeline; loop back to Phase 4 after the user
  approves a remediation path.

The verify step is DEFENSIVE ONLY (read + parse). It never scans the host
for open CVEs or probes third-party endpoints.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `INTENT` | arg | 1-line user description of the target VM |
| `PROVIDER` | Phase 1 | hetzner / vultr / digitalocean / upcloud / linode |
| `REGION`   | Phase 1 | provider-specific region code |
| `PLAN`     | Phase 1 | cx22 / cax11 / vc2-1c-1gb / …  |
| `ARCH`     | Phase 1 | x86_64 / arm64 |
| `ADMIN_USER` | Phase 2 | default `keiadmin` |
| `SSH_PORT`   | Phase 2 | default 22; custom permitted |
| `APP_PORTS`  | Phase 2 | e.g. `[443/tcp, 80/tcp]` |
| `TLS_HOST`   | Phase 2 | optional FQDN for Caddy |
| `VM_IP`    | Phase 3 | IPv4 of the created VM |
| `VM_NAME`  | Phase 3 | provider resource label |
| `HARDENED` | Phase 4 | true when harden-base.sh exited 0 |
| `SSH_CHECK_OK` | Phase 5 | exit 0 of `ssh-check` |
| `FW_DIFF_OK`   | Phase 5 | exit 0 of `firewall-diff` |
| `HANDOFF_TO`   | Phase 6 | next skill (e.g. `/web-deploy`) or `none` |

---

## Final report (emit after Phase 6)

```
=== /VM-PROVISION REPORT ===
Intent:        <first 80 chars of INTENT>
Provider:      <PROVIDER> / region=<REGION> / plan=<PLAN> / arch=<ARCH>
VM:            <VM_NAME> @ <VM_IP>
Admin:         <ADMIN_USER> (ssh port <SSH_PORT>)
Ports:         <APP_PORTS>
TLS:           <TLS_HOST or "none">
Hardened:      <HARDENED>
Verification:  ssh-check=<PASS/FAIL> firewall-diff=<PASS/FAIL>
Handoff:       <HANDOFF_TO>
Artifacts:     <terraform state path | cloud-init.yaml path>
```

---

## Rules (enforced at every phase)

- **Pure-click contract.** Only `INTENT` (argument) and custom port values
  (Phase 2.c) are typed. Every other decision is an `AskUserQuestion`.
- **Hard gate (Phase 5).** `ssh-check` AND `firewall-diff` must exit 0
  before Phase 6. Neither can be skipped.
- **RULE -1 NO DOWNGRADE.** Any phase that fails returns 2-3 constructive
  paths, never "can't be done".
- **RULE 0.8 Secrets Single Source.** All provider tokens come from
  `~/.claude/secrets/.env` (or per-project `secrets/*.env`). NEVER read
  a token from the conversation, NEVER write one to a file.
- **RULE 0.4 NO HALLUCINATION.** Provider specifics (prices, region codes,
  plan IDs) must be fetched at time of use, not recalled. Cite source.
- **RULE 0.5 Plan Mode First.** Phase 2 writes the plan; no provisioning
  happens before the user clicks "approve".
- **Defensive-only.** No scanning tools, no CVE probes, no third-party
  attack surface analysis. Pure config linting.
- **Surgical changes.** Harden only the VM being provisioned. Never touch
  the caller's workstation config.
- **Constructor Pattern (RULE ZERO).** Each phase file ≤ 200 LOC;
  generated cloud-init / Caddyfile artefacts never exceed 200 LOC — split
  into role-specific files if they would.

---

## References

- [phase-1-select-provider.md](phase-1-select-provider.md) · [phase-2-plan.md](phase-2-plan.md) · [phase-3-provision.md](phase-3-provision.md) · [phase-4-harden.md](phase-4-harden.md) · [phase-5-verify.md](phase-5-verify.md) · [phase-6-handoff.md](phase-6-handoff.md)
- `_blocks/deploy-hetzner-cloud.md` — Hetzner Cloud specifics (Phase 1)
- `_blocks/deploy-vps-generic.md` — provider-agnostic cloud-init + TF skeleton (Phase 1/3)
- `_blocks/security-ssh-hardening.md` — sshd drop-in baseline (Phase 4/5)
- `_blocks/security-firewall-ufw.md` — ufw intent schema (Phase 2/5)
- `_blocks/security-tls-caddy.md` — TLS (Phase 6 handoff)
- `_blocks/security-audit-logging.md` — auditd baseline (Phase 4)
- `_blocks/security-patching.md` — unattended-upgrades (Phase 4)
- `_primitives/provision-hetzner.sh` · `_primitives/provision-vultr.sh` — provisioners (Phase 3)
- `_primitives/harden-base.sh` — hardening script (Phase 4)
- `_primitives/_rust/ssh-check/` · `_primitives/_rust/firewall-diff/` — verify gate (Phase 5)
- `skills/web-deploy/SKILL.md` — optional Phase 6 handoff
