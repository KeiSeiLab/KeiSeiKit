# SECURITY — Audit Logging (auditd + journald forwarding)

**Goal:** every privileged action (sudo, ssh login, sensitive file edit) leaves a tamper-evident trail that survives the VM being reimaged.

**Stack:**
- `auditd` — Linux kernel audit framework, writes to `/var/log/audit/audit.log` in human-unfriendly but machine-parseable K/V format.
- `journald` — systemd's binary journal (`/var/log/journal/`), captures stdout/stderr of every service plus syslog stream.
- **Off-box shipping** (optional but recommended) — forward journald to a remote log collector (Loki, Vector, rsyslog+TLS). Local logs are destroyed on reimage.

**Install + enable:**
```
sudo apt install -y auditd audispd-plugins
sudo systemctl enable --now auditd
```

**Reference `/etc/audit/rules.d/99-kei.rules`:**
```
# KeiSeiKit audit baseline — pinned 2026-04-21. Loaded by augenrules on boot.
## 1. SSH events
-w /etc/ssh/sshd_config        -p wa -k sshd_config
-w /etc/ssh/sshd_config.d/     -p wa -k sshd_config
-w /root/.ssh/                 -p wa -k ssh_keys_root
-w /home/keiadmin/.ssh/        -p wa -k ssh_keys_admin

## 2. Sudo events
-w /etc/sudoers                -p wa -k sudoers
-w /etc/sudoers.d/             -p wa -k sudoers
-a always,exit -F arch=b64 -S execve -F euid=0 -F auid>=1000 -F auid!=unset -k sudo_root

## 3. Privilege / identity changes
-w /etc/passwd                 -p wa -k identity
-w /etc/group                  -p wa -k identity
-w /etc/shadow                 -p wa -k identity
-w /etc/gshadow                -p wa -k identity

## 4. Loading / unloading kernel modules
-a always,exit -F arch=b64 -S init_module   -S finit_module -S delete_module -k module

## 5. Time changes (detect attempts to skew audit timestamps)
-a always,exit -F arch=b64 -S adjtimex      -S settimeofday -S clock_settime -k time
-w /etc/localtime              -p wa -k time

## 6. Make the config itself immutable (place LAST)
-e 2
```
`-e 2` locks the ruleset until reboot (tamper-resistant). Load with `sudo augenrules --load && sudo systemctl restart auditd`. Test with `sudo ausearch -k sshd_config | tail`.

**Human-readable summaries:** `sudo aureport -au` (auth events), `aureport -m` (module loads), `aureport -k` (keyed rule hits). Use these in incident response; raw `audit.log` is only for ingest pipelines.

**journald tuning — `/etc/systemd/journald.conf.d/99-kei.conf`:**
```
[Journal]
Storage=persistent
Compress=yes
SystemMaxUse=500M
SystemKeepFree=1G
MaxFileSec=1week
ForwardToSyslog=no
```
`Storage=persistent` creates `/var/log/journal/` — without it, `journalctl` history disappears on reboot. `MaxFileSec=1week` rotates weekly; combine with off-box shipping so you don't lose events.

**Off-box shipping patterns:**
- **systemd-journal-upload** — built-in, ships via HTTPS to a `systemd-journal-remote` receiver. Mutual-TLS recommended.
- **Vector** (<https://vector.dev>) — pull from `journald` source, push to Loki/S3/syslog-TLS. Modern, Rust-native. Uses `/run/log/journal/` + unix socket.
- **rsyslog → remote** — legacy path; useful if you already operate a syslog collector.

Any choice: use TLS, authenticate the receiver, do NOT push cleartext logs across the internet. Logs often contain secrets even when the app tries not to log them.

**Failure-mode handling:** `auditd` can be configured to panic the kernel when the audit queue fills — reasonable for high-compliance, DANGEROUS for general VMs. Default `/etc/audit/auditd.conf` has `disk_full_action = SUSPEND` and `disk_error_action = SUSPEND` — keep these; tune to `HALT` only if regulatory driver requires it.

**Verification (skill Phase 5):**
- `sudo auditctl -l` returns the non-empty rule list.
- `systemctl is-active auditd` = `active`.
- `journalctl --disk-usage` shows a non-zero persistent journal.
- (Optional) an off-box log-receiver shows entries within the last N minutes.

**Forbidden:** deleting `/var/log/audit/audit.log` or `/var/log/journal/*` on a live host (breaks chain-of-custody); running auditd with `-e 0` (unlocked, attacker can disable the kernel audit); shipping logs in cleartext; logging secrets (app-level concern — redact before `logger()`); disabling persistent journald.
