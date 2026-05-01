# DEPLOY — AWS EC2 (Instance Connect + Elastic IP)

**SSH pattern — EC2 Instance Connect (60 s key window, no permanent authorized_keys):**
```
aws ec2-instance-connect send-ssh-public-key \
  --instance-id i-XXXXXXXXXXXXXXXXX \
  --instance-os-user ec2-user \
  --ssh-public-key file://~/.ssh/id_ed25519.pub
ssh ec2-user@<elastic-ip>     # within 60 s
```
Typical pattern: dedicated instance per project with an Elastic IP in a chosen region. Multi-project shared hosts are fine, but track co-tenancy (below).

**Network posture:**
- **Elastic IP** for any node that needs stable identity (client configs, DNS, firewall rules).
- **Security Group**: allow SSH (port 22) ONLY from Tailscale CGNAT (`100.64.0.0/10`) or a specific admin IP. NEVER `0.0.0.0/0:22` in prod.
- Application ports exposed through an ALB or nginx reverse proxy — not directly on the instance.
- IMDSv2 REQUIRED (`HttpTokens=required`). v1 is SSRF-exploitable.

**IAM:**
- Use IAM roles attached to the instance (`aws configure` on-instance hits the metadata endpoint).
- NEVER bake static AWS keys into AMI / env / user-data.
- Use a preconfigured named AWS profile (`--profile <name>`), not interactive console for read ops.

**Shared-host coordination:** if one instance runs multiple apps (e.g. API + marketing dashboards + internal tools), host-level change (apt / systemd / nginx) → cross-project impact check BEFORE reboot.

**Forbidden:** open port 22 to `0.0.0.0/0`, static AWS keys in repo / `.env` committed to git, IMDSv1, rebooting shared hosts without cross-project sanity check, asking user to log into console for read ops (profile is set up — use it).
