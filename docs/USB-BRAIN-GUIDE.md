# USB Exobrain — Step-by-Step Test Recipe

> Put a KeiSeiKit "brain" on a portable USB drive, mount it into Claude Code (and Cursor / Continue / Zed if installed), verify MCP tools, unplug — verify clean detach.
>
> This top-level document holds prerequisites, the shared filesystem warning, the TOC, and platform-agnostic troubleshooting. Step-by-step walkthroughs live in the three platform sub-documents.

## Platform walkthroughs

- [macOS walkthrough](./USB-BRAIN-GUIDE-macos.md) — `/Volumes/<LABEL>`, APFS recommended, Gatekeeper / `xattr` handling.
- [Linux walkthrough](./USB-BRAIN-GUIDE-linux.md) — `/media/$USER/<LABEL>`, ext4 recommended, `umount` / optional systemd-udev auto-attach.
- [Windows walkthrough](./USB-BRAIN-GUIDE-windows.md) — drive letter, NTFS recommended, `Dismount-Volume` / PowerShell.

---

## 0. Prerequisites (all platforms)

On the host:

- KeiSeiKit installed: `./install.sh --profile=full` ran successfully, `cargo test -p keisei` shows 32/32 pass (v0.22).
- `keisei` CLI on PATH: `ls ~/.claude/agents/_primitives/_rust/target/release/keisei` returns the binary. Symlink into `~/.local/bin/` or `%USERPROFILE%\bin` as convenient.
- At least one supported client installed (Claude Code / Cursor / Continue / Zed) — `keisei` auto-detects all four.
- A USB stick or flash card physically attached and mounted by the OS.

On the USB drive:

- Free space: ~500 MB recommended (5 mcp-server binaries × ~90 MB each + room for memory/artifacts SQLite).
- Filesystem — see the warning below.

---

## ⚠️ Filesystem warning — exFAT / FAT32 are NOT safe for multi-client attach

SQLite WAL mode (used by `kei-memory`, `kei-artifact`, `kei-social-store` inside a brain) requires a filesystem with reliable shared-memory mmap. exFAT and FAT32 do NOT provide this, and `keisei mount` (multi-client fan-out) WILL corrupt the memory DBs if the brain lives on one of those filesystems.

Since v0.22 `keisei` itself prints an advisory on stderr when it detects an exFAT / FAT32 root at load time (via `statfs(2)` on macOS + Linux; Windows returns `Unknown` pending `GetVolumeInformationW`). The warning is non-blocking — single-client `keisei attach` on exFAT is still a supported path.

- On exFAT / FAT32: use the brain with **ONE client at a time** (`keisei attach --scope=user`). Do NOT run `keisei mount`.
- For reliable multi-client use, format the stick as **APFS (macOS)**, **ext4 (Linux)**, or **NTFS (Windows)**. exFAT is fine for single-client or read-only cross-platform transport.
- If you already ran `keisei mount` on an exFAT brain and see SQLite errors, see "SQLite corruption on mount-attach" in Troubleshooting below.

---

## Brain safety invariants (all platforms)

The following load-time checks fire regardless of OS. They prevent the most common misuse and attack patterns:

- **Name regex** — `brain.name` must match `^[a-z][a-z0-9_-]{0,63}$` (lowercase letter start, 1-64 chars, word chars + hyphen).
- **Relative paths** — every `[paths.*]` entry must be relative; no absolute paths, no `..` components. Canonical form must live under the brain root.
- **Symlink reject** — the brain root itself cannot be a symlink. Pass the resolved path; this closes a USB→host pivot surface.
- **Manifest size cap** — `manifest.toml` is bounded at 64 KiB.
- **Schema** — `schema_version ∈ {1, 2}`. v1 uses single-string `mcp_server`; v2 uses per-platform map.

---

## Platform-agnostic troubleshooting

### "BrainNotFound" on attach
- Check `<BRAIN>/manifest.toml` exists.
- Pass an absolute path to `keisei attach`, not a relative one.

### "PathEscape" on attach
- Every `[paths.*]` entry must be relative and resolve inside the brain root. No `../`, no absolute paths.

### "BrainIsSymlink" on attach
- The brain root is a symlink. Pass the resolved path instead — `keisei` refuses symlink roots to prevent accidental host-dir pivot via a crafted USB.

### "NoPlatformBinary" on first use
- Your platform's binary isn't in `bin/`. Check `std::env::consts::{OS, ARCH}` on your host — the expected filename is `kei-mcp-server-<os-renamed>-<arch-renamed>` where `macos→darwin`, `x86_64→x64`, `aarch64→arm64`.

### "NameConflict" on attach
- An MCP server with the same name already exists in the client's config. Either rename your brain (`name` in `manifest.toml`) or remove the existing entry manually.

### SQLite corruption on mount-attach
- `kei-memory` / `kei-artifact` / `kei-social-store` databases show "disk image is malformed" or "database is locked" after `keisei mount` on a USB drive.
- Root cause: the brain lives on **exFAT or FAT32**, which don't reliably support SQLite WAL shared-memory mmap. Multi-client attach corrupts the DB.
- Fix:
  1. Copy the brain dir to an APFS (macOS) / ext4 (Linux) / NTFS (Windows) volume.
  2. Re-attach via `keisei attach <new-path>` or `keisei mount <new-path>`.
  3. For single-client use on the same exFAT drive, restrict to `keisei attach --scope=user`. Do NOT use `keisei mount`.

### Filesystem-warning advisory not appearing
- `keisei` prints the exFAT/FAT32 warning to **stderr** (not stdout). Shells that redirect stderr away will hide it.
- Windows `detect_fs_warning` returns `Unknown` in v0.22 — format as NTFS manually.

---

## What this tests end-to-end

1. **Portable brain** — works from a USB drive, no installer needed on the brain itself.
2. **Per-platform dispatch** — schema v2 picks the right binary for the host OS+arch.
3. **Multi-client fan-out** — `keisei mount` attaches to every detected client in one call.
4. **Clean detach** — zero residue in host configs, preserves unrelated MCP servers.
5. **Safety gates** — path confinement, name regex, symlink rejection, manifest size bound, filesystem-type advisory (v0.22).
6. **Schema v1 compat** — an older v1 brain with a single-string `mcp_server` still works.

If the walkthrough for your platform passes end-to-end, the v0.22 exobrain is production-ready for single-user workflows on that platform.

For shared-brain scenarios (team members all mounting the same brain over git / S3) see the `kei-store` backend docs — `keisei attach s3://my-bucket/brain/` requires `--features s3` at install.
