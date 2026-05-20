# USB Exobrain — Windows Walkthrough

> Platform-specific companion to `USB-BRAIN-GUIDE.md`. Read the top-level guide first for prerequisites, warnings, and invariants.

Windows support is best-effort in v0.22 — `keisei` itself builds cleanly on Windows, but the filesystem-type advisory (`fs_type.rs`) returns `Unknown` pending a `GetVolumeInformationW` implementation. Meaning: the exFAT/FAT32 warning does NOT fire on Windows yet. Format your USB as NTFS manually for multi-client safety.

Shell snippets use PowerShell 7+.

## 1. Create the brain directory

Plug in the USB; Explorer will show a drive letter (e.g. `E:`).

```powershell
$BRAIN = "E:\my-brain"
New-Item -ItemType Directory -Path $BRAIN,"$BRAIN\bin","$BRAIN\memory","$BRAIN\artifacts","$BRAIN\manifests" -Force
```

## 2. Download MCP server binaries

```powershell
$BASE = "https://github.com/KeiSeiLab/KeiSeiKit-1.0/releases/download/v0.21.0"
Push-Location "$BRAIN\bin"

$names = @(
    "darwin-arm64", "darwin-x64",
    "linux-x64", "linux-arm64",
    "windows-x64.exe"
)
foreach ($n in $names) {
    Invoke-WebRequest -Uri "$BASE/kei-mcp-server-$n"        -OutFile "kei-mcp-server-$n"        -ErrorAction SilentlyContinue
    Invoke-WebRequest -Uri "$BASE/kei-mcp-server-$n.sha256" -OutFile "kei-mcp-server-$n.sha256" -ErrorAction SilentlyContinue
}

Get-ChildItem kei-mcp-server-*.sha256 | ForEach-Object {
    $expected = (Get-Content $_).Split(' ')[0]
    $target   = $_.Name -replace '\.sha256$',''
    $actual   = (Get-FileHash $target -Algorithm SHA256).Hash.ToLower()
    if ($actual -ne $expected) { Write-Error "FAIL: $target" }
}
Pop-Location
```

No `chmod +x` on Windows — `.exe` is executable by extension. No `xattr` concept (Windows does not use HFS-style quarantine).

## 3. Write `manifest.toml` (schema v2)

```powershell
@"
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
"@ | Set-Content -Path "$BRAIN\manifest.toml" -Encoding utf8NoBOM
```

Note the `utf8NoBOM` encoding — the toml parser does not handle a UTF-8 BOM gracefully.

## 4. Verify + attach

```powershell
keisei list-adapters
keisei status                         # "no brain attached"
keisei attach "$BRAIN" --scope=user
```

Marker lands at `%USERPROFILE%\.keisei\attached.toml`. Claude Code settings at `%USERPROFILE%\.claude\settings.json`.

## 5. Verify in Claude Code

```powershell
Get-Content "$HOME\.claude\settings.json" | ConvertFrom-Json | Select-Object -ExpandProperty mcpServers
```

## 6. Multi-client mount

```powershell
keisei mount $BRAIN
```

## 7. Project-scope

```powershell
cd C:\path\to\your-repo
keisei attach $BRAIN --scope=project      # claude-code + cursor only
```

## 8. Detach + eject

```powershell
keisei detach
# Eject via PowerShell:
$vol = Get-Volume -DriveLetter E
$vol | Dismount-Volume -Force
```

Or use the system tray "Safely Remove Hardware" icon — either path flushes pending writes before the device is physically removed.

## Windows-specific troubleshooting

- **FS advisory not firing** — v0.22 Windows build returns `Unknown` from `detect_fs_warning`. Format the stick as NTFS manually; exFAT is unsafe for `keisei mount`. A future release will wire `GetVolumeInformationW`.
- **Long-path failures** — the brain root plus any nested manifest path must fit inside Windows' MAX_PATH (260 chars) unless you've opted into long paths via `HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem\LongPathsEnabled = 1`. Keep `$BRAIN` short.
- **Drive letter reassignment** — Windows may hand out a different letter next plug-in. The marker stores an absolute path, so a letter change breaks resolution. Re-attach after the new letter appears.
- **Execution policy** — PowerShell will refuse to run helper scripts under default `Restricted` policy. `Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned` as a one-time fix.
