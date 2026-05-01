# Phase 2 — Plan Mode Doc

> Goal: produce a written, user-approved plan (RULE 0.5) that enumerates
> every apt change to the VM before any packet leaves the workstation.
> **Verify criterion:** user clicked "approve" on the plan; plan artefact
> exists at `<run-dir>/plan.md`.

---

## 2.a — Synthesise the plan

Write `<run-dir>/plan.md` (where `<run-dir>` is `./.keisei/vm-provision/<timestamp>/`) with
EXACTLY these sections — no more, no less:

```markdown
# VM-Provision Plan — <timestamp>

## Intent
<INTENT one-line>

## Target
- Provider: <PROVIDER>
- Region:   <REGION>
- Plan:     <PLAN> (<arch>)
- VM name:  kei-<env>-<role>      # derived, ASK if ambiguous

## Access
- Admin user:  <ADMIN_USER>       # default keiadmin
- SSH port:    <SSH_PORT>         # default 22
- SSH pubkey:  <path>             # read from ~/.ssh/id_*.pub

## Ports to allow (ufw + provider cloud firewall)
<APP_PORTS — list>

## TLS
- Host:   <TLS_HOST or none>
- Method: <HTTP-01 | DNS-01 | none>

## Hardening steps (harden-base.sh)
- apt update + upgrade
- install: ufw fail2ban unattended-upgrades needrestart auditd audispd-plugins
- write /etc/ssh/sshd_config.d/99-kei.conf
- ufw default-deny-in + rate-limit ssh + allow APP_PORTS
- fail2ban sshd jail
- auditd baseline ruleset (/etc/audit/rules.d/99-kei.rules)
- unattended-upgrades (AUTO reboot = FALSE)

## Verification (hard gate before handoff)
- ssh-check  → exit 0
- firewall-diff (intent YAML vs live ufw) → exit 0

## Rollback
- `_primitives/provision-<provider>.sh destroy <VM_NAME>` — 1-command destroy.
- TF state: <path or "none — CLI-driven">

## Cost estimate
<Plan price per month from PROVIDER pricing page; CITE>
```

Cite the source for every price/region/plan detail. Numbers NOT cited =
NO-GO per RULE 0.4.

---

## 2.b — Build the `firewall-intent.yaml`

Write `<run-dir>/firewall-intent.yaml`:

```yaml
default:
  incoming: deny
  outgoing: allow
  routed: deny
rules:
  - port: <SSH_PORT>
    proto: tcp
    action: limit
    from: any
    comment: "ssh (rate-limited)"
  # one entry per APP_PORTS:
  - port: 443
    proto: tcp
    action: allow
    from: any
```

This file is the **source of truth** the Phase 5 `firewall-diff` will
compare against live `ufw status numbered` output. Drift = Phase 5 fail.

---

## 2.c — AskUserQuestion (customise ports, TLS, admin name)

One `AskUserQuestion` call with up to 4 questions:

1. **Admin user?** (stored as `ADMIN_USER`)
   - `keiadmin` (default)
   - Custom (user types — only free-text in Phase 2)

2. **SSH port?** (stored as `SSH_PORT`)
   - `22` (default; simpler)
   - `2222` (obscurity; not security, but reduces log noise)
   - Custom

3. **Application ports to open?** (multi-select, stored as `APP_PORTS`)
   - `443/tcp` — HTTPS (most apps)
   - `80/tcp`  — HTTP (only if ACME HTTP-01 or redirect)
   - `none`    — tunneled via Tailscale / private net only

4. **TLS?** (stored as `TLS_HOST` + method)
   - Caddy HTTP-01 (need 80/tcp + 443/tcp + DNS pointing to VM)
   - Caddy DNS-01 (no port 80 needed; need DNS provider API token)
   - None (app provides its own TLS or is behind a proxy)

---

## 2.d — Present the plan for approval

Render `plan.md` in chat. Ask ONE final AskUserQuestion:

**Proceed with this plan?**
- Approve → Phase 3.
- Iterate → loop back to 2.c with the user's change request.
- Abort → emit plan-only artefact and exit (`HANDOFF_TO=none`).

---

## 2.e — Verify criterion

- [ ] `plan.md` written to `<run-dir>/plan.md`.
- [ ] `firewall-intent.yaml` written to `<run-dir>/firewall-intent.yaml`.
- [ ] User clicked "Approve".

Emit:
`Phase 2 done: plan @ <run-dir>/plan.md. <len(APP_PORTS)> ports, TLS=<method>.`

Proceed to Phase 3.
