# SECURITY — SSH Hardening (sshd_config.d/99-kei.conf)

**Rule:** hardening goes into a drop-in under `/etc/ssh/sshd_config.d/`, NEVER by editing `/etc/ssh/sshd_config` directly. The main file ships with distro-owned defaults; drop-ins win on later-read order and survive package upgrades cleanly.

**Reference file `/etc/ssh/sshd_config.d/99-kei.conf`:**
```
# KeiSeiKit hardened SSH — pinned 2026-04-21, auditable via ssh-check.
Protocol 2
PasswordAuthentication no
ChallengeResponseAuthentication no
KbdInteractiveAuthentication no
PermitRootLogin prohibit-password
PermitEmptyPasswords no
UsePAM yes
MaxAuthTries 3
MaxSessions 4
LoginGraceTime 20
AllowUsers keiadmin
AllowTcpForwarding no
X11Forwarding no
PermitTunnel no
ClientAliveInterval 120
ClientAliveCountMax 2
LogLevel VERBOSE
# Modern crypto only (OpenSSH ≥ 8.9, default Debian 12 / Ubuntu 22.04+):
KexAlgorithms curve25519-sha256,curve25519-sha256@libssh.org,sntrup761x25519-sha512@openssh.com
Ciphers chacha20-poly1305@openssh.com,aes256-gcm@openssh.com,aes128-gcm@openssh.com
MACs hmac-sha2-512-etm@openssh.com,hmac-sha2-256-etm@openssh.com
HostKeyAlgorithms ssh-ed25519,rsa-sha2-512,rsa-sha2-256
```
Apply with `sshd -t` (config test) before `systemctl reload ssh`. `reload` NOT `restart` — restart kills existing sessions; reload re-reads config while keeping them.

**Field-by-field rationale:**
- `PasswordAuthentication no` — passwords are the #1 SSH brute-force vector. Keys only.
- `PermitRootLogin prohibit-password` — root only via key, never password. `no` blocks even emergency cloud-console rescue paths on some providers; `prohibit-password` is the pragmatic middle.
- `MaxAuthTries 3` — reduces per-connection key/password attempts; combine with fail2ban for per-IP bans (separate concern).
- `AllowUsers keiadmin` — whitelist is simpler than group-based DENY and audits trivially. Adding users = explicit edit.
- `LogLevel VERBOSE` — logs the key fingerprint used; without it you can't tell which admin logged in after compromise.
- `ClientAliveInterval 120` + `ClientAliveCountMax 2` — idle sessions die in 4 minutes. Lost laptops don't leave open shells.
- `AllowTcpForwarding no` / `PermitTunnel no` — disables SSH-as-VPN. Enable per-use-case via `Match User tunneluser` only.

**Modern KEX/Cipher/MAC lists (2026-04-21):**
- KEX: `sntrup761x25519-sha512@openssh.com` is post-quantum hybrid (default since OpenSSH 9.9) [VERIFIED https://www.openssh.com/releasenotes.html]; `curve25519-sha256` is the classic ECDH.
- Ciphers: AEAD only (`chacha20-poly1305`, `aes*-gcm`). Dropped CBC-mode — vulnerable to Terrapin CVE-2023-48795 without strict-KEX.
- MACs: ETM (Encrypt-Then-MAC) only. Legacy MAC-Then-Encrypt is dropped.
- HostKey: prefer `ssh-ed25519`; keep `rsa-sha2-*` for older client compatibility. Drop `ssh-rsa` (SHA-1, broken).

**Verification (KeiSeiKit primitive):**
`_primitives/_rust/ssh-check/` parses BOTH `sshd_config` AND every `sshd_config.d/*.conf` (in filename sort order, last wins per directive), reports violations of the matrix above with `file:line` precision. Run BEFORE every `systemctl reload ssh` and BEFORE the skill phase-5 verify gate.

**Forbidden:** editing `/etc/ssh/sshd_config` in-place when a drop-in directory exists; `PermitRootLogin yes`; `PasswordAuthentication yes`; accepting any `diffie-hellman-group1-*` / `ssh-rsa` / CBC ciphers; restarting sshd before `sshd -t` passes; relying on fail2ban alone without key-only auth.
