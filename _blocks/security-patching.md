# SECURITY — Patching (unattended-upgrades + needrestart + reboot window)

**Goal:** security patches applied within 24 h of release, service restarts + kernel reboots happen within a declared maintenance window (NOT ad-hoc at 3 AM UTC on a random Tuesday).

**Install:**
```
sudo apt install -y unattended-upgrades needrestart
```

**`/etc/apt/apt.conf.d/50unattended-upgrades` (essential lines, Debian 12 / Ubuntu 22.04+):**
```
Unattended-Upgrade::Origins-Pattern {
    "origin=Debian,codename=${distro_codename}-security";
    "origin=Debian,codename=${distro_codename}-updates";
};
Unattended-Upgrade::Automatic-Reboot "false";
Unattended-Upgrade::Automatic-Reboot-Time "04:00";
Unattended-Upgrade::Mail "admin@example.com";
Unattended-Upgrade::MailReport "on-change";
```
`Automatic-Reboot "false"` is the SAFE default — an automatic reboot without coordination kills in-flight requests. Pair with `needrestart` to SURFACE reboot requirement, then schedule the window explicitly (below).

**`/etc/apt/apt.conf.d/20auto-upgrades`:**
```
APT::Periodic::Update-Package-Lists "1";
APT::Periodic::Unattended-Upgrade  "1";
APT::Periodic::AutocleanInterval   "7";
```
Triggers daily via `/lib/systemd/system/apt-daily.timer` + `apt-daily-upgrade.timer`.

**needrestart:** after each upgrade, prints services that loaded old library versions and need restart. `/etc/needrestart/needrestart.conf`:
```
$nrconf{restart} = 'l';    # list only; do NOT auto-restart services
$nrconf{kernelhints} = -1; # suppress "reboot hint" interactive prompt (non-TTY cron)
```
`nrconf{restart} = 'a'` (auto) is tempting but dangerous — restarting `postgresql` or a stateful app during a migration = corruption.

**Reboot window pattern (declared, env-var-driven):**
```bash
# /etc/systemd/system/kei-reboot-window.service + .timer
# Only reboots if /var/run/reboot-required exists AND the current time
# falls inside the declared window.
[Service]
Type=oneshot
EnvironmentFile=/etc/default/kei-reboot-window
ExecStart=/usr/local/bin/kei-reboot-window

# /etc/default/kei-reboot-window
KEI_REBOOT_DOW="Sun"          # day-of-week
KEI_REBOOT_HOUR="04"          # 24h, UTC
KEI_REBOOT_MIN="15"
KEI_DRAIN_CMD=""              # optional pre-reboot drain (e.g. drain a load-balancer slot)
```
`kei-reboot-window` script checks `[ -f /var/run/reboot-required ]`, verifies it is the declared DOW/hour, runs `$KEI_DRAIN_CMD`, then `systemctl reboot`. Commit the script once; reuse the env file per-host.

**Provider-specific:**
- **Hetzner Cloud / Vultr / UpCloud / DigitalOcean / Linode** — nothing extra; cloud-init already installs the packages per `deploy-vps-generic.md`.
- **AWS EC2** — `ec2-instance-connect` may briefly reject SSH during a reboot — tolerate in orchestration retries.

**Auditability:** `unattended-upgrades` logs to `/var/log/unattended-upgrades/unattended-upgrades.log`. Forward via journald (see `security-audit-logging.md`). Package a short summary in the skill Phase 5 report.

**Forbidden:** `Unattended-Upgrade::Automatic-Reboot "true"` on stateful services; `$nrconf{restart} = 'a'` on a database host; silently skipping the reboot window to "avoid downtime" (real fix: HA, not skipped patches); installing `.deb` packages from third-party repos without pinning + signature verification; disabling the `apt-daily.timer` — disables ALL security updates.
