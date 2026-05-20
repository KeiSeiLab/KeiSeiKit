# USB Exobrain — macOS Walkthrough

> Platform-specific companion to `USB-BRAIN-GUIDE.md`. Read the top-level guide first for prerequisites, warnings, and invariants.

## 1. Create the brain directory

```bash
BRAIN=/Volumes/EXOBRAIN/my-brain
mkdir -p "$BRAIN"/{bin,memory,artifacts,manifests}
```

## 2. Download MCP server binaries

```bash
BASE=https://github.com/KeiSeiLab/KeiSeiKit-1.0/releases/download/v0.21.0
cd "$BRAIN/bin"
for n in darwin-arm64 darwin-x64 linux-x64 windows-x64.exe; do
  curl -fL -O "$BASE/kei-mcp-server-$n"
  curl -fL -O "$BASE/kei-mcp-server-$n.sha256"
done
curl -fL -O "$BASE/kei-mcp-server-linux-arm64" 2>/dev/null || echo "skipped linux-arm64"

for f in kei-mcp-server-*.sha256; do shasum -a 256 -c "$f"; done
chmod +x kei-mcp-server-darwin-* kei-mcp-server-linux-* 2>/dev/null || true
xattr -d com.apple.quarantine kei-mcp-server-darwin-* 2>/dev/null || true
```

macOS Gatekeeper quarantines every downloaded binary; `xattr -d` clears the attribute. Re-run that line later if Claude Code refuses to spawn the mcp server.

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

Marker lands at `~/.keisei/attached.toml`; client settings at `~/.claude/settings.json`.

## 5. Verify in Claude Code

Close + reopen Claude Code, or run `/help` → MCP servers. Confirm from the shell:

```bash
cat ~/.claude/settings.json | jq '.mcpServers["my-brain"]'
```

## 6. Multi-client mount

```bash
keisei mount "$BRAIN"
```

Writes to `~/.claude/settings.json`, `~/.cursor/mcp.json`, `~/.continue/config.json`, `~/Library/Application Support/Zed/settings.json`.

## 7. Project-scope

```bash
cd ~/path/to/your-repo
keisei attach "$BRAIN" --scope=project    # claude-code + cursor only
```

## 8. Detach + eject

```bash
keisei detach
diskutil eject /Volumes/EXOBRAIN
```

## macOS-specific troubleshooting

- **Claude Code can't spawn the MCP server** — check `chmod +x`, re-run `xattr -d com.apple.quarantine <binary>`, and confirm direct-run: `.../kei-mcp-server-darwin-arm64 --help`.
- **"BrainIsSymlink" on attach** — `$BRAIN` is a symlink. Pass the resolved path; `keisei` refuses symlink roots.
- **USB not at `/Volumes/EXOBRAIN`** — macOS auto-renames on name collision (`EXOBRAIN 1`). Confirm with `diskutil list`.
