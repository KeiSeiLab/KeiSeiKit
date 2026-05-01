# DEPLOY — Generic VPS (provider-agnostic cloud-init + ssh-first-contact)

**Target providers:** DigitalOcean Droplets, Vultr, UpCloud, Linode/Akamai. Each has slightly different Terraform providers + CLIs, but the Day-0 contract is identical: **boot a Debian/Ubuntu image with a cloud-init user-data blob; add one admin SSH key; nothing else.**

**Day-0 cloud-init blob (`cloud-init.yaml`) — universal:**
```yaml
#cloud-config
hostname: kei-${env}-${role}
timezone: UTC
package_update: true
package_upgrade: true
packages:
  - ufw
  - fail2ban
  - unattended-upgrades
  - auditd
  - needrestart
  - curl
  - jq
users:
  - name: keiadmin
    groups: sudo
    shell: /bin/bash
    sudo: ALL=(ALL) NOPASSWD:ALL
    ssh_authorized_keys:
      - ${ADMIN_PUBKEY}
ssh_pwauth: false
disable_root: true
write_files:
  - path: /etc/ssh/sshd_config.d/99-kei.conf
    permissions: '0644'
    content: |
      PasswordAuthentication no
      PermitRootLogin no
      MaxAuthTries 3
      AllowUsers keiadmin
      ClientAliveInterval 120
      ClientAliveCountMax 2
runcmd:
  - [ systemctl, restart, ssh ]
  - [ ufw, default, deny,  incoming ]
  - [ ufw, default, allow, outgoing ]
  - [ ufw, allow,   22/tcp ]
  - [ ufw, --force, enable ]
```
The blob is intentionally provider-neutral. Provider-specific bits (private-network bring-up, metadata service quirks) go in a short appendix the provisioner appends. See `_primitives/harden-base.sh` for post-boot hardening re-runs.

**SSH-first-contact (`ssh-first-contact.sh` pattern):**
```bash
# Wait for cloud-init to finish AND sshd to be ready on the new IP.
for i in $(seq 1 60); do
  ssh -o ConnectTimeout=3 -o StrictHostKeyChecking=accept-new \
      "keiadmin@$IP" "cloud-init status --wait" && break
  sleep 5
done
ssh "keiadmin@$IP" "sudo test -f /var/lib/cloud/instance/boot-finished"
```
`StrictHostKeyChecking=accept-new` is OK only for the FIRST contact (TOFU). Store the fingerprint to `~/.ssh/known_hosts`; subsequent connects use default strict mode. Never use `StrictHostKeyChecking=no` — accepts MitM silently.

**Terraform skeleton (provider-agnostic via vars):**
```hcl
variable "provider_kind" {}                   # "digitalocean" | "vultr" | "upcloud" | "linode"
variable "region"       {}
variable "size_slug"    {}                    # provider-specific size id
variable "admin_pubkey" {}                    # raw ssh-ed25519 …
locals {
  user_data = templatefile("${path.module}/cloud-init.yaml", { ADMIN_PUBKEY = var.admin_pubkey })
}
# ... then a module-per-provider resource that all read `local.user_data`
```
Keep TF state **local per-env-per-dev by default**; upgrade to remote backend (R2, S3, Terraform Cloud) only when ≥ 2 humans share state.

**Per-provider gotchas (verified 2026-04-21):**
- **DigitalOcean:** Marketplace "Docker" images skip unattended-upgrades — start from plain Debian 12 instead. IPv6 requires `ipv6 = true` on the droplet.
- **Vultr:** `vultr-cli` needs `VULTR_API_KEY`; default firewall is OPEN — attach a firewall group or rely solely on ufw.
- **UpCloud:** IPs rotate on full stop+start unless you request `floating_ip`. Consider Finnish ASN if Hetzner is blocked or rate-limited for your geo.
- **Linode:** cloud-init runs before disk resize on some plans → `growpart` may need a rerun on first `ssh`.

**Forbidden:** baking the admin private key into an AMI/snapshot; reusing one SSH keypair across envs; letting cloud-init pull scripts from a mutable URL (`curl … | bash` in `runcmd:` — pin to a hash); running `apt-get dist-upgrade -y` in `runcmd` without `needrestart` to surface pending reboots.
