# DEPLOY — Hetzner Cloud (CX22 / CAX11 + TF + Cloud Firewall)

**Why Hetzner:** cheapest EU VPS with reputable network. CX22 (x86, 2 vCPU / 4 GB / 40 GB) = **€3.79/mo + VAT**; CAX11 (Ampere ARM64, 2 vCPU / 4 GB / 40 GB) = **€3.79/mo + VAT**. Prices verified on <https://www.hetzner.com/cloud/> [VERIFIED 2026-04-21]. Hourly billing caps at the monthly rate — safe to spin down for tests.

**Terraform provider:** `hetznercloud/hcloud` (official). Pin version:
```hcl
terraform {
  required_providers {
    hcloud = { source = "hetznercloud/hcloud", version = "~> 1.49" }
  }
}
provider "hcloud" { token = var.hcloud_token }
```
Token via env: `export HCLOUD_TOKEN=$(grep ^HCLOUD_TOKEN ~/.claude/secrets/.env | cut -d= -f2)`. **NEVER commit the token** (RULE 0.8 — see `domain-has-secrets.md`).

**Minimal `hcloud_server` resource:**
```hcl
resource "hcloud_server" "node" {
  name        = "kei-${var.env}-${var.role}"
  image       = "debian-12"
  server_type = var.arch == "arm64" ? "cax11" : "cx22"
  location    = var.location                    # fsn1 / nbg1 / hel1 / ash / hil / sin
  ssh_keys    = [hcloud_ssh_key.admin.id]
  user_data   = file("${path.module}/cloud-init.yaml")
  firewalls   { firewall_id = hcloud_firewall.base.id }
  labels      = { project = "kei", env = var.env }
}
```
`ssh_keys` is **mandatory** — passing it disables the root password e-mail path.

**Cloud Firewall (stateful, IN by default DENY):**
```hcl
resource "hcloud_firewall" "base" {
  name = "kei-base"
  rule { direction = "in" protocol = "tcp" port = "22" source_ips = var.admin_cidrs }
  rule { direction = "in" protocol = "icmp"             source_ips = ["0.0.0.0/0", "::/0"] }
  # Add app ports (443, 80) only when an app is deployed behind the node.
}
```
Attach to the server via `firewalls { firewall_id = … }`. Cloud Firewall is the FIRST line of defense — it drops traffic before it hits the VM's ufw (see `security-firewall-ufw.md`). Both layers MUST agree.

**Locations:** `fsn1` (Falkenstein DE), `nbg1` (Nürnberg DE), `hel1` (Helsinki FI), `ash` (Ashburn US), `hil` (Hillsboro US), `sin` (Singapore). Pick region closest to users; ARM64 `cax*` available in EU only [VERIFIED 2026-04-21].

**Snapshots + rescue:** `hcloud_snapshot` for golden images; `hcloud server enable-rescue` before SSH lockout recovery. Back up `user_data` and TF state (remote backend: S3-compatible such as R2).

**Primitives provided by KeiSeiKit:**
- `_primitives/provision-hetzner.sh` — wrapper around `hcloud` CLI, idempotent create/destroy, checks existing server by name first.
- Complement with `_primitives/harden-base.sh` run over SSH after first boot.

**Forbidden:** hcloud token in `.tf` or `.tfvars` committed to git; Cloud Firewall with port 22 open to `0.0.0.0/0`; creating servers with `keep_disk = false` then snapshotting (destroys data); using Hetzner Storage Boxes for anything needing low latency (they're SFTP-over-WAN).
