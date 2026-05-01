# Forgejo Actions — self-hosted CI

Parallel CI on the private Forgejo (Tailscale `<private-forgejo>:3000`)
that doesn't depend on github.com — keeps private code on
self-hosted infrastructure while still getting per-commit
verification.

## Why a separate `.forgejo/workflows/` and not just `.github/workflows/`?

Forgejo Actions reads BOTH directories by default. We split them
because the GHA workflow has 2 quirks irrelevant on self-hosted:

1. **GHA Linux runner has 7 GB RAM + 14 GB on `/mnt`** — workspace OOMs
   during link. Self-hosted runner can have whatever RAM the host has.
2. **Per-category matrix** is faster on self-hosted (parallel jobs)
   but *slower* on GHA (each matrix job = full container + cache pull).
   So we keep GHA monolithic, split self-hosted into 8 logical groups.

## One-time runner setup

Pick a host (the same VPS that runs Forgejo, or a separate beefier
box). Tailscale is fine — runner only needs to reach Forgejo.

```bash
# 1. Get the binary
wget -O /usr/local/bin/forgejo-runner \
    https://code.forgejo.org/forgejo/runner/releases/download/v6.5.0/forgejo-runner-amd64
chmod +x /usr/local/bin/forgejo-runner

# 2. Get a registration token from Forgejo:
#    Forgejo web UI → Site Administration → Actions → Runners → Create new
#    (OR per-org: Org settings → Actions → Runners)
#    (OR per-repo: Repo settings → Actions → Runners — narrowest scope)

# 3. Register
sudo useradd -r -s /usr/sbin/nologin -d /var/lib/forgejo-runner forgejo-runner
sudo mkdir -p /var/lib/forgejo-runner
sudo chown forgejo-runner: /var/lib/forgejo-runner
cd /var/lib/forgejo-runner
sudo -u forgejo-runner forgejo-runner register --no-interactive \
    --instance http://<private-forgejo>:3000 \
    --token <REGISTRATION_TOKEN_FROM_WEB_UI> \
    --name "$(hostname)-runner" \
    --labels self-hosted,docker,linux,amd64

# 4. systemd unit
sudo tee /etc/systemd/system/forgejo-runner.service <<'UNIT'
[Unit]
Description=Forgejo Actions Runner
After=network.target

[Service]
Type=simple
User=forgejo-runner
WorkingDirectory=/var/lib/forgejo-runner
ExecStart=/usr/local/bin/forgejo-runner daemon
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
UNIT

sudo systemctl daemon-reload
sudo systemctl enable --now forgejo-runner

# 5. Verify in Forgejo web UI:
#    Site Admin → Actions → Runners → status: Idle (green dot)
```

## Repo-level enable

```bash
# Via API
curl -X PATCH http://<private-forgejo>:3000/api/v1/repos/denis/KeiSeiKit \
    -u "denis:$FORGEJO_TOKEN" \
    -H 'Content-Type: application/json' \
    -d '{"has_actions": true}'

# OR via web UI:
#   Repo → Settings → Repository → enable "Actions"
```

## Trigger

Push to `main` triggers the workflow automatically. Watch progress:
http://<private-forgejo>:3000/denis/KeiSeiKit/actions

## Differences from GHA workflow

| Job | GHA | Forgejo |
|---|---|---|
| `rust-assembler` | ubuntu+macOS matrix | docker (Linux only) |
| `rust-primitives` | monolithic (OOM-prone) | **8-group matrix** (parallel, fast) |
| `ts-packages` | node 20+22 matrix | node 22 only |
| `install-dry-run` | 3 profiles | (skip — runs locally on dev machines) |
| `shell-lint` | ubuntu+shellcheck apt | shellcheck-alpine container |
| `workflow-lint` | actionlint | (skip — handled by GHA mirror) |

## Cost

Free. No GitHub Actions minutes. No GitHub LFS bandwidth.
Sensitive-IP never leaves Tailscale.

## Maintenance

The runner pulls images on first run for each container reference; subsequent
runs are cached. Periodic `docker system prune -af` recommended (cron job
on the runner host).
