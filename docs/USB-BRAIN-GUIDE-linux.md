# USB Exobrain — Linux Walkthrough

> Platform-specific companion to `USB-BRAIN-GUIDE.md`. Read the top-level guide first for prerequisites, warnings, and invariants.

On Linux, auto-mounted removable media typically lands at `/media/$USER/<LABEL>` (GNOME, KDE, auto-mounters) or `/run/media/$USER/<LABEL>` (systemd-udisks2). Substitute your actual mount point below.

## 1. Create the brain directory

```bash
BRAIN=/media/$USER/EXOBRAIN/my-brain
mkdir -p "$BRAIN"/{bin,memory,artifacts,manifests}
```

## 2. Download MCP server binaries

```bash
BASE=https://github.com/KeiSei84/KeiSeiKit/releases/download/v0.21.0
cd "$BRAIN/bin"
for n in darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64.exe; do
  curl -fL -O "$BASE/kei-mcp-server-$n" 2>/dev/null || echo "skipped $n"
  curl -fL -O "$BASE/kei-mcp-server-$n.sha256" 2>/dev/null || true
done

for f in kei-mcp-server-*.sha256; do sha256sum -c "$f"; done
chmod +x kei-mcp-server-linux-* kei-mcp-server-darwin-* 2>/dev/null || true
```

No Gatekeeper / `xattr` step on Linux. The `chmod +x` is still required — the executable bit is not restored by `curl`.

## 3. Write `manifest.toml` (schema v2)

```bash
cat > "$BRAIN/manifest.toml" <<'EOF'
[brain]
schema_version = 2
name = "my-brain"
created = "2026-04-22T00:00:00Z"

[paths]
memory = "memory/"
artifacts = "artifacts/"
manifests = "manifests/"

[paths.mcp_server]
darwin-arm64 = "bin/kei-mcp-server-darwin-arm64"
darwin-x64   = "bin/kei-mcp-server-darwin-x64"
linux-x64    = "bin/kei-mcp-server-linux-x64"
linux-arm64  = "bin/kei-mcp-server-linux-arm64"
windows-x64  = "bin/kei-mcp-server-windows-x64.exe"
EOF
```

## 4. Verify + attach

```bash
keisei list-adapters
keisei status                         # "no brain attached"
keisei attach "$BRAIN" --scope=user
```

Marker lands at `~/.keisei/attached.toml`; Claude Code settings at `~/.claude/settings.json`.

## 5. Verify in Claude Code

```bash
jq '.mcpServers["my-brain"]' ~/.claude/settings.json
```

## 6. Multi-client mount

```bash
keisei mount "$BRAIN"
```

Linux adapter paths: `~/.claude/settings.json`, `~/.cursor/mcp.json`, `~/.continue/config.json`, `~/.config/zed/settings.json`.

## 7. Project-scope

```bash
cd ~/path/to/your-repo
keisei attach "$BRAIN" --scope=project    # claude-code + cursor only
```

## 8. Detach + unmount

```bash
keisei detach
umount /media/$USER/EXOBRAIN             # or: sync && udisksctl unmount -b /dev/sdX1
```

If `umount` returns "target is busy", close any shell with its CWD under the mount, then retry. `lsof +f -- /media/$USER/EXOBRAIN` lists open handles.

## Linux-specific troubleshooting

- **Filesystem detection** — `keisei` calls `statfs(2)` at load time. exFAT (`EXFAT_SUPER_MAGIC = 0x2011bab0`) and FAT32 (`MSDOS_SUPER_MAGIC = 0x4d44`) trigger the SQLite-WAL-unsafe advisory. Format the USB as ext4 for reliable multi-client use.
- **Auto-mounted `noexec` partitions** — some distros mount removable media `noexec` by default. If the mcp server refuses to run, remount read-write-executable: `sudo mount -o remount,exec /media/$USER/EXOBRAIN`. Alternatively add a line to `/etc/fstab` keyed by UUID (`blkid /dev/sdX1`).
- **Permissions drift** — if you copy a brain from macOS via tar / rsync, the executable bit may not survive. Re-apply `chmod +x bin/kei-mcp-server-*`.
- **Optional — systemd auto-attach** — a `systemd-udev` rule can run `keisei attach <mount>/my-brain --scope=user` whenever a labelled stick shows up, and a matching `udev` remove rule can call `keisei detach`. Out of scope for this guide; see `ArchWiki: Udisks#Auto-mount`.
