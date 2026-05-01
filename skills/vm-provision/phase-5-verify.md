# Phase 5 — Verification Hard Gate (`ssh-check` + `firewall-diff`)

> Goal: fail-closed verification. Phase 6 refuses to run unless BOTH
> `ssh-check` AND `firewall-diff` exit 0.
> **Verify criterion:** `SSH_CHECK_OK = true` AND `FW_DIFF_OK = true`.

---

## 5.a — Pull config artefacts from the VM

```bash
scp "${ADMIN_USER}@${VM_IP}:/etc/ssh/sshd_config"            <run-dir>/sshd_config
ssh "${ADMIN_USER}@${VM_IP}" "sudo tar -C /etc/ssh -cf - sshd_config.d" \
  | tar -C <run-dir>/ -xf -
ssh "${ADMIN_USER}@${VM_IP}" "sudo ufw status numbered"      > <run-dir>/ufw-status.txt
```

The ufw status requires `sudo` on most distros — the admin user has it
via `NOPASSWD:ALL` from `harden-base.sh`. If `sudo` requires TTY, prefix
`sudo -n` and surface the failure.

All captured files are READ ONLY, for `ssh-check` / `firewall-diff` to
parse. We NEVER push config back from the workstation.

---

## 5.b — Run `ssh-check`

```bash
_primitives/_rust/ssh-check/target/release/ssh-check \
  --config  <run-dir>/sshd_config \
  --drop-in <run-dir>/sshd_config.d \
  --allow-user "${ADMIN_USER}" \
  --json > <run-dir>/ssh-check.json
SSH_EXIT=$?
```

Exit 0 → `SSH_CHECK_OK=true`. Exit 2 → `SSH_CHECK_OK=false` and
`<run-dir>/ssh-check.json` lists the violating directives with
`file:line` precision. Exit 1 → usage/parse error; surface the stderr and
loop back to Phase 4.

---

## 5.c — Run `firewall-diff`

```bash
_primitives/_rust/firewall-diff/target/release/firewall-diff \
  --intent <run-dir>/firewall-intent.yaml \
  --status-file <run-dir>/ufw-status.txt \
  --json > <run-dir>/firewall-diff.json
FW_EXIT=$?
```

Exit 0 → `FW_DIFF_OK=true`. Exit 2 → the JSON lists `missing` (in intent,
not live) and `extra` (in live, not intent) rules; `default_mismatches`
flags a non-deny inbound policy.

---

## 5.d — Decision tree

| `ssh-check` | `firewall-diff` | Action |
|---|---|---|
| 0 | 0 | Proceed to Phase 6. |
| 2 | 0 | Loop to 4.a with the sshd_config.d fix + re-ship `harden-base.sh`. |
| 0 | 2 | Ask user: apply the `missing`/`extra` deltas via `ufw` commands, or update `firewall-intent.yaml` (the intent was wrong). ONE AskUserQuestion. |
| 2 | 2 | Both failed — show both JSON reports; recommend a single fresh `harden-base.sh` re-run first (common-mode fix), then re-verify. |
| 1 | 1 | Workstation issue (missing binary, bad path) — NOT a VM problem. Rebuild the Rust primitives (`cargo build --release` in `_primitives/_rust/`). |

---

## 5.e — The AskUserQuestion

Exactly ONE AskUserQuestion, gated on the decision tree above:

**Verification results:** `ssh-check=<PASS|FAIL>`,
`firewall-diff=<PASS|FAIL>`. Pick one:

- **Proceed** (only shown when both PASS) → Phase 6.
- **Fix and retry** → loop to Phase 4 (or to 5.c if intent YAML is wrong).
- **Ignore and proceed** — **BLOCKED.** The hard-gate invariant refuses
  this path per `SKILL.md`. You can abort, but you cannot bypass.

---

## 5.f — Verify criterion

- [ ] `ssh-check` exit 0.
- [ ] `firewall-diff` exit 0.
- [ ] `<run-dir>/ssh-check.json` and `<run-dir>/firewall-diff.json` saved.

Emit:
`Phase 5 done: hard-gate PASSED. Artefacts in <run-dir>/.`

Proceed to Phase 6.

---

## 5.g — Non-obvious pitfalls

- **sshd_config.d drop-in not loaded.** Debian 12's default
  `/etc/ssh/sshd_config` includes the `.d` directory via an `Include`
  directive. We don't follow `Include` on purpose (security — includes
  can escape the intended tree). Pass `--drop-in` explicitly.
- **ufw status shows IPv6 rules as duplicates.** Intent is IPv4-only by
  default; `firewall-diff`'s normalisation treats `(v6)` rules with same
  port/proto as "expected" and does not flag them. If you need strict
  v6-only rules, open a separate intent file.
- **`MaxAuthTries` at 6 or 10** (Debian default). `harden-base.sh` sets
  3. If a previous manual edit raised it and we re-ran without rewriting,
  ssh-check will FAIL `maxauthtries`. Fix: re-run `harden-base.sh`.
