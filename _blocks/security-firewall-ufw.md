# SECURITY — Firewall (ufw default-deny + rate limiting + nftables alt)

**Posture — default-deny-in / allow-out:**
```
ufw default deny incoming
ufw default allow outgoing
ufw default deny routed     # do NOT forward unless explicitly routing
ufw limit 22/tcp comment 'ssh (rate-limited: 6 conn / 30s)'
ufw logging medium
ufw --force enable
```
`ufw limit` = per-source-IP brute-force mitigation at the kernel level (iptables `recent` module). Use for SSH — *never* use it for app traffic (false positives on shared-NAT clients).

**Layer ordering (read top-down):**
1. **Cloud Firewall** (Hetzner Cloud Firewall / AWS Security Group / DO Firewall) — drops at the provider edge, BEFORE packets hit the VM. Cheapest layer.
2. **ufw** on the VM — defence in depth; also covers provider-firewall misconfigs and private-network paths.
3. **App-level auth** — sshd keys, TLS client certs, app tokens.

Both the Cloud Firewall AND ufw must agree on the port allow-list. A mismatch means "it works from provider console but not from Tailscale" or vice-versa. Use `_primitives/_rust/firewall-diff/` to compare intended rules (YAML) against running `ufw status`.

**Intended-rules YAML schema (`firewall-intent.yaml`):**
```yaml
default:
  incoming: deny
  outgoing: allow
  routed: deny
rules:
  - port: 22
    proto: tcp
    action: limit
    from: any
    comment: "ssh (rate-limited)"
  - port: 443
    proto: tcp
    action: allow
    from: any
    comment: "https / caddy"
  - port: 80
    proto: tcp
    action: allow
    from: any
    comment: "http / acme-http-01"
```
`firewall-diff` round-trips this against live `ufw status numbered` JSON-parse and prints additions/deletions. Exit 0 iff live ≡ intent.

**Rate limiting patterns:**
- `limit` — built-in; 6 connections / 30 s per IP. Good for SSH.
- Per-app — do it inside the app or a reverse proxy (nginx `limit_req`, Caddy `rate_limit`), not in ufw. Kernel rate-limit doesn't understand HTTP methods.
- ICMP — `ufw default allow outgoing` covers outbound; inbound ICMP should be `allow` (echo) for monitoring, NOT blanket-blocked (blocks path-MTU discovery).

**IPv6:** `/etc/default/ufw` → `IPV6=yes` (default Debian 12). Verify via `ufw status verbose` shows the (v6) rules. Missing IPv6 rules = a trivial bypass on dual-stack VMs.

**Logging:** `ufw logging medium` writes to `/var/log/ufw.log`. Forward to journald (default on systemd) or an off-box log collector. Logging `high` is too chatty for steady state; use it only during incident response.

**nftables alternative (for hosts that have Docker-installed iptables-nft):**
ufw is a thin wrapper over iptables/nftables; on Docker-heavy hosts, Docker's daemon aggressively rewrites iptables and can bypass ufw. Two options:
1. **DOCKER_OPTS=`--iptables=false`** (and do NAT yourself — advanced).
2. **`ufw-docker`** companion (<https://github.com/chaifeng/ufw-docker>, not bundled in Debian — pin a tagged release, review the script BEFORE install).

On non-Docker hosts, ufw is sufficient. On Docker hosts, EITHER isolate (dedicated host + Cloud Firewall only) OR use `ufw-docker` — don't half-configure.

**Forbidden:** `ufw default allow incoming` "temporarily"; `allow from any to any port 22` without `limit`; skipping the IPv6 rule set; letting Docker silently override ufw without disabling its iptables chain; relying on `ufw` as the ONLY layer when a Cloud Firewall is available.
