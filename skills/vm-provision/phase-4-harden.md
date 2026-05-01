# Phase 4 — Harden via `harden-base.sh`

> Goal: run `_primitives/harden-base.sh` on the VM, over SSH, idempotently.
> **Verify criterion:** script exited 0; `systemctl is-active` returns
> `active` for `ssh`, `ufw`, `fail2ban`, `auditd`.

---

## 4.a — Ship the script

The script lives on the workstation; copy to the VM and run with `sudo`:

```bash
scp _primitives/harden-base.sh "${ADMIN_USER}@${VM_IP}:/tmp/harden-base.sh"
ssh "${ADMIN_USER}@${VM_IP}" "sudo bash /tmp/harden-base.sh \
  --admin-user ${ADMIN_USER} \
  --ssh-port  ${SSH_PORT} \
  $(for p in ${APP_PORTS[@]}; do echo --allow-port $p; done)"
```

Why not `curl … | bash`? Because that depends on a hosted URL AND a
trusted TLS cert. `scp` the file you already audited locally. Lower
surface area, reproducible.

The script is **idempotent** — safe to re-run. Re-runs converge the VM to
the declared state; missing directives get rewritten, extra ones are left
alone.

---

## 4.b — Stream logs

`harden-base.sh` logs to stderr with timestamps. Capture to
`<run-dir>/harden.log`:

```bash
ssh "${ADMIN_USER}@${VM_IP}" "sudo bash /tmp/harden-base.sh …" 2> >(tee <run-dir>/harden.log >&2)
```

If the script exits non-zero: STOP. Do NOT proceed to Phase 5. Surface
the last 30 lines of `<run-dir>/harden.log` + ask the user to choose:

- (A) **Fix locally + re-ship** — edit the primitive (if bug is there) or
  adjust flags. Commit the fix under `checkpoint:` before retry.
- (B) **Patch the VM manually** — user logs in, fixes, we re-run the
  script to ensure idempotency.
- (C) **Destroy + reprovision** — when remediation risk > cost of a
  fresh VM (2 min on Hetzner).

---

## 4.c — Post-hardening live-check

After exit 0, SSH back in and confirm:

```bash
ssh "${ADMIN_USER}@${VM_IP}" "
  set -e
  systemctl is-active ssh ufw fail2ban auditd unattended-upgrades.service 2>/dev/null || true
  ufw status | head -20
  sudo auditctl -l | head -10
"
```

All four services must be `active`. `auditctl -l` must show the baseline
rules (sshd_config, sudoers, identity, module, time). Record the output
in `<run-dir>/post-harden.txt`.

---

## 4.d — AskUserQuestion (ready to verify?)

One `AskUserQuestion`:

**Hardening applied. Four services active; auditd rules loaded.**
- Run verification gate (Phase 5).
- Apply one more pass (typo in `APP_PORTS`, extra user, etc. — loops 4.a
  with a delta).
- Pause (leave the VM in current state).

---

## 4.e — Verify criterion

- [ ] `harden-base.sh` exited 0.
- [ ] `ssh / ufw / fail2ban / auditd` all `active`.
- [ ] `<run-dir>/harden.log` + `<run-dir>/post-harden.txt` captured.

Emit:
`Phase 4 done: 4/4 services active. Log: <run-dir>/harden.log.`

Proceed to Phase 5 (hard gate).

---

## 4.f — Non-obvious failure modes

- **`systemctl reload ssh` fails because `sshd -t` rejects the drop-in.**
  Usually a custom `SSH_PORT` collides with ufw still configured for 22.
  Fix: ensure ufw rule + sshd Port match BEFORE reload. `harden-base.sh`
  writes both in one pass, but if an out-of-band edit happened between
  runs, you get this.
- **fail2ban service flaps.** Usually a systemd-journal backend mismatch
  on very old Debian. Verify `backend = systemd` in
  `/etc/fail2ban/jail.local` (script sets this).
- **auditd refuses `-e 2`.** Means an earlier rules load is still
  mastered; `augenrules --load` forces reload. Already in the script.

None of these require a Level-2 escalation — all three have known fixes.
