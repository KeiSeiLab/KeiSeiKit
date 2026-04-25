# Phase 3 — Provision + SSH First Contact

> Goal: create the VM via the right `_primitives/provision-<provider>.sh`,
> wait for `cloud-init` to finish, establish SSH as `ADMIN_USER`.
> **Verify criterion:** `VM_IP` resolves to a live sshd that accepts the
> admin key; `cloud-init status --wait` = `done`.

---

## 3.a — Render cloud-init user-data

Copy `_blocks/deploy-vps-generic.md`'s `cloud-init.yaml` template to
`<run-dir>/cloud-init.yaml`, substituting:

- `${env}`, `${role}` from Phase 2's derived VM name.
- `${ADMIN_PUBKEY}` — read `~/.ssh/id_ed25519.pub` (or ask Phase 2.c which
  pubkey). **NEVER** read private keys; pubkeys only.

Render once; do not parameterise further — surgical changes only.

---

## 3.b — Choose provisioner + run

Dispatch by `PROVIDER`:

- `hetzner` → `_primitives/provision-hetzner.sh create <VM_NAME> --type <PLAN> --location <REGION> --user-data <run-dir>/cloud-init.yaml`
- `vultr`   → `_primitives/provision-vultr.sh   create <VM_NAME> --plan <PLAN> --region <REGION> --user-data <run-dir>/cloud-init.yaml`
- `digitalocean` / `upcloud` — use each provider's official CLI directly
  (no wrapper primitive yet); CITE the command in the plan before running.

Both primitives are idempotent — a second invocation with the same name
prints the existing IP and exits 0. Re-runs after a network blip do NOT
create duplicates.

Capture stdout (just the IPv4) into `VM_IP`.

---

## 3.c — SSH first contact (TOFU)

```bash
for i in $(seq 1 60); do
  ssh -o ConnectTimeout=3 \
      -o StrictHostKeyChecking=accept-new \
      -o UserKnownHostsFile=~/.ssh/known_hosts \
      "${ADMIN_USER}@${VM_IP}" "cloud-init status --wait" && break
  sleep 5
done
```

- `StrictHostKeyChecking=accept-new` is TOFU for the FIRST connect only.
  After this, subsequent connects use strict mode (default).
- 60 × 5 s = 5 min timeout; long enough for cloud-init on any of the
  supported providers.
- `cloud-init status --wait` blocks until cloud-init finishes — no
  time-based sleep.

If the loop exhausts without a successful SSH: STOP. Pull provider
console logs (`hcloud server ssh-log <name>` / vultr console screenshot)
and surface the failure mode:

- DNS/IP issue → wait + retry (1 constructive path).
- Wrong pubkey → revoke the VM (`provision-<p>.sh destroy`), fix Phase 2,
  retry.
- Cloud-init crashed on first boot → enable rescue mode via provider
  console, read `/var/log/cloud-init-output.log`, fix template, retry.

---

## 3.d — AskUserQuestion (confirm IP + ready to harden)

One `AskUserQuestion`:

**VM is up at `<VM_IP>`. Cloud-init finished, admin SSH works.**
- Proceed to hardening (Phase 4).
- Pause (inspect the VM first; re-invoke skill when ready).
- Abort + destroy (calls `destroy` on the provisioner, returns to Phase 2).

---

## 3.e — Verify criterion

- [ ] `VM_IP` set.
- [ ] `cloud-init status` returns `done` (not `error`, not `disabled`).
- [ ] `ssh ${ADMIN_USER}@${VM_IP} 'true'` exits 0.
- [ ] `known_hosts` contains the VM's host key (pinned for future connects).

Emit:
`Phase 3 done: <VM_NAME> up @ <VM_IP>, admin=<ADMIN_USER>, cloud-init=done.`

Proceed to Phase 4.

---

## 3.f — Constructive-fail paths

- **Create returned no IP (provisioner exit 2).** Root cause likely API
  outage or quota. Paths: (A) retry after 2 min; (B) try sibling region;
  (C) fall through to an alternate provider (loops back to Phase 1).
- **cloud-init errored.** Pull logs via rescue; typical causes: bad yaml
  indentation, unreachable apt mirror. Fix template; re-provision fresh
  (destroy the broken VM first — partial state = harder to reason about).
- **SSH never responded.** Check provider firewall / cloud-init user
  creation — some provider images rename `root` → `debian` and our
  `keiadmin` sudoers file didn't take. Remediation: add the provider's
  default user to the admin whitelist for 1 run, then switch.
