# SECURITY — TLS via Caddy (automatic ACME, HTTP-01 / DNS-01)

**Why Caddy:** zero-config TLS. Caddy 2 auto-provisions certificates via Let's Encrypt / ZeroSSL on first request for a domain that resolves to it, auto-renews, and stores state under `/var/lib/caddy/`. Official docs: <https://caddyserver.com/docs/automatic-https> [VERIFIED 2026-04-21].

**One-liner install (Debian/Ubuntu, official repo):**
```
# Pinned to official Cloudsmith repo — NEVER `curl … | bash` a random domain.
sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https curl
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' \
  | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' \
  | sudo tee /etc/apt/sources.list.d/caddy-stable.list
sudo apt update && sudo apt install -y caddy
```
This installs the `caddy` systemd service owned by `caddy:caddy`. **Never run Caddy as root** — it uses `CAP_NET_BIND_SERVICE` ambient capability to bind low ports.

**Minimal `/etc/caddy/Caddyfile`:**
```
{
    # Global options
    email admin@example.com                # ACME account contact (change!)
    # auto_https disable_redirects          # uncomment only if fronted by another TLS-terminating proxy
}

api.example.com {
    encode zstd gzip
    log {
        output file /var/log/caddy/api.log {
            roll_size 10mb
            roll_keep 10
        }
    }
    reverse_proxy 127.0.0.1:8080
    header {
        Strict-Transport-Security "max-age=31536000; includeSubDomains"
        X-Content-Type-Options "nosniff"
        Referrer-Policy "strict-origin-when-cross-origin"
        -Server
    }
}
```
`caddy validate --config /etc/caddy/Caddyfile` BEFORE `systemctl reload caddy`. Reload ≠ restart; reload is zero-downtime.

**ACME challenge choice:**
- **HTTP-01** (default) — Caddy binds port 80, LE connects back, serves challenge. Requires: port 80 open to the internet, DNS pointing to the VM. Works for single-host public services.
- **DNS-01** — Caddy writes a TXT record via DNS provider API, doesn't need port 80 open. **Required for wildcard certs** (`*.example.com`) and for LAN-only hosts. Needs a DNS-provider plugin (e.g. `caddy-dns/cloudflare`) compiled into the binary — use `xcaddy build` or the Cloudsmith `caddy-dns-*` packages.

**DNS-01 with Cloudflare (`caddy-dns/cloudflare`):**
```
*.internal.example.com, internal.example.com {
    tls {
        dns cloudflare {env.CF_API_TOKEN}
    }
    reverse_proxy 127.0.0.1:8080
}
```
`CF_API_TOKEN` — store in `/etc/caddy/caddy.env` (chmod 0640, `caddy:caddy`), load via systemd drop-in `EnvironmentFile=`. Never bake the token into the Caddyfile (RULE 0.8 — see `domain-has-secrets.md`).

**CT log awareness:** every LE cert is published to Certificate Transparency logs. **Any subdomain you cert is publicly searchable** via crt.sh. Use DNS-01 + wildcard for internal services whose names should not leak.

**Firewall interop (see `security-firewall-ufw.md`):** `ufw allow 80,443/tcp` is required for HTTP-01 and for public HTTPS. Do NOT open 80 if using DNS-01 exclusively and not redirecting HTTP→HTTPS publicly; skip the redirect with `auto_https disable_redirects`.

**Hardening:**
- `HSTS` as shown above — 1 year, include subdomains. Add `preload` only after submitting to the HSTS preload list.
- `-Server` header strip — removes Caddy version disclosure.
- Rate limit via `caddy-ratelimit` module (needs `xcaddy build` with the plugin) for per-IP throttling; otherwise rely on cloud/ufw layer.

**Forbidden:** running Caddy as root; embedding DNS/ACME API tokens in the Caddyfile; using `tls internal` (self-signed, ephemeral CA) for anything reachable from outside localhost; skipping `caddy validate` before reload; self-hosting ACME (step-ca is great, but needs its own runbook — out of scope here).
