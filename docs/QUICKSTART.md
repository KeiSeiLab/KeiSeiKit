# Quickstart — KeiSeiKit Cortex Stack in 5 Minutes

Local AI assistant with a browser UI, terminal client, and VSCode panel.
Backed by a Rust daemon on `127.0.0.1:9797`. No cloud. Bring your own
API keys.

This guide walks you from clean clone to a chat window in your browser.
For the full install matrix (MCP-only, USB brain, Docker, Nix, etc.) see
[INSTALL.md](./INSTALL.md).

---

## Prerequisites

- macOS 13+ or Linux (Ubuntu 22.04+, Debian 12+, Alpine 3.19+).
- Rust toolchain (`rustup`, current stable). The installer builds 53
  crates from source — first build is ~3-5 min on Apple Silicon, ~6-8
  min on a modern x86 laptop.
- Python 3.9+ and `pip` (used as a subprocess for `faster-whisper`
  STT — RULE 0.2 exception #6, no Python in the daemon core).
- `node >= 18` and `pnpm` (used to build the Svelte UI bundle).
- `ffmpeg` on `PATH` (whisper audio demux).
- `git` and `sqlite` (sqlite ships bundled in the Rust crates;
  the system binary is only needed for `kei-brain-view summary`).

The installer soft-checks every prereq when it picks the `cortex`
profile and prints an exact apt/brew/pacman line for anything missing.

---

## 1. Clone and install

```bash
git clone <your-private-remote-url>/KeiSeiKit && cd KeiSeiKit
./install.sh --profile=cortex
```

The `cortex` profile provisions exactly the eight primitives listed in
`_primitives/MANIFEST.toml` — `kei-cortex` (HTTP daemon), `cortex-ui`
(Svelte 5 + Vite 5 web app), `kei-pet` (persona manifest schema),
`kei-shared` (DNA + substrate types), `kei-ledger` (SQLite work-unit
ledger), `kei-memory` (offline session retrospective),
`frustration-matrix` (longitudinal user-frustration scanner), and
`kei-skill-importer` (external-format skill importer).

Re-run with `--add=kei-tty` if you want the terminal client too.

The installer is idempotent — safe to re-run. It never overwrites
`~/.claude/settings.json` or any user manifest.

---

## 2. Run the setup wizard

In Claude Code (or any AGENTS.md-aware tool that loaded the kit):

```
/cortex-setup
```

Seven phases, mostly clicks. The wizard:

- **Phase 0** — detects OS, finds the `kei-cortex` binary path, surfaces
  any missing primitive (hard-fails fast if something didn't install).
- **Phase 1** — verifies `~/.claude/secrets/.env` has
  `ANTHROPIC_API_KEY`, `ELEVENLABS_API_KEY`, and `FAL_KEY`. For each
  missing key the wizard offers a free-text paste field — never logs
  the value to chat output (RULE 0.8).
- **Phase 2** — picks port (default `9797`), UI host (default
  `127.0.0.1:18080`), whisper model (`base.en` / `small.en` /
  `medium.en`).
- **Phase 3** — generates `~/.keisei/cortex.token` (32 hex bytes,
  `chmod 600`).
- **Phase 4** — `pip install -r scripts/requirements.txt`,
  pre-downloads the chosen whisper model with SHA-256 verification,
  builds the cortex-ui Svelte bundle (`pnpm install && pnpm build`).
- **Phase 5** — registers a launchd plist on macOS or a systemd user
  unit on Linux. Falls back to `manual` mode if neither supervisor is
  available.
- **Phase 6** — composes a `setup URL`
  (`http://127.0.0.1:18080/?daemon=…&token=…`), copies it to your
  clipboard, and offers a single click to open it in your default
  browser.

Pure-click except for the optional API-key paste fields and the
optional custom port / UI host. Re-running the wizard is idempotent —
the token, UI dist, and supervisor unit are preserved.

---

## 3. Verify the daemon is up

```bash
curl -sS -H "Authorization: Bearer $(cat ~/.keisei/cortex.token)" \
  http://127.0.0.1:9797/healthz
# Expect: {"ok":true,"version":"0.x.x"}
```

If the daemon is down, `launchctl list | grep keisei` (macOS) or
`systemctl --user status kei-cortex` (Linux) will tell you why.

---

## 4. Open the chat

The setup URL the wizard copied to your clipboard takes you to the
Svelte browser app. You'll see:

- **Sidebar** — Live2D pet renderer (idle animation, lip-sync to the
  TTS stream), `Setup` / `Dashboard` / `PetEditor` / `LedgerStream` /
  `MemorySearch` routes.
- **Main pane** — chat panel with PTT (push-to-talk) voice input,
  auto-TTS toggle, optional file-tree + terminal pane.
- **Top strip** — `BudgetStrip` showing today's Anthropic / ElevenLabs
  / fal.ai spend pulled from `/usage`.

Type a message. The daemon proxies it to Anthropic via
`/chat` (SSE-streamed tool use), returns the assistant reply, and —
if auto-TTS is on — pipes the text through `/tts` to ElevenLabs and
back into the Live2D mouth shape.

---

## 5. Add the terminal client (optional)

```bash
./install.sh --add=kei-tty
kei-tty chat
```

Ratatui-based TUI (`_primitives/_rust/kei-tty/src/ui.rs`). Same daemon,
same token. Use `kei-tty send --message "…"` for a one-shot pipe-friendly
mode.

---

## 6. Add the VSCode extension (optional)

```bash
cd _ts_packages/packages/vscode-cortex
pnpm install && pnpm package
code --install-extension keisei-cortex-0.1.0.vsix
```

Then `Cmd+Shift+K` (or `Ctrl+Shift+K` on Linux) opens the cortex chat
view. Right-click any selection to **Ask About Selection** —
`keisei-cortex.chatAboutSelection` sends the highlighted text into the
chat panel as context.

---

## 7. Wire the MCP server (optional)

The `kei-mcp` Rust binary (`_primitives/_rust/kei-mcp/`) exposes the
KeiSeiKit atom registry over the [Model Context
Protocol](https://modelcontextprotocol.io/). Build it once:

```bash
cd _primitives/_rust && cargo build -p kei-mcp --release
```

Then add to `~/.claude/mcp_servers.json` (or Cline / OpenClaw equivalent
— see [`_primitives/_rust/kei-mcp/README.md`](../_primitives/_rust/kei-mcp/README.md)
for all three configs):

```json
{
  "kei": {
    "command": "/abs/path/to/_primitives/_rust/target/release/kei-mcp",
    "args": [],
    "env": {
      "KEI_MCP_ATOMS_ROOT": "/abs/path/to/_primitives/_rust",
      "KEI_MCP_SKILLS_ROOT": "/abs/path/to/skills"
    }
  }
}
```

After a Claude Code reload, every atom under `_primitives/_rust/*/atoms/*.md`
appears as a callable MCP tool, and every skill under `skills/*/SKILL.md`
appears as a readable MCP resource.

---

## What's next

- [ARCHITECTURE.md](./ARCHITECTURE.md) — the layer diagram, atoms,
  recipes, frontends, sleep cycle.
- [SLEEP-LAYER.md](./SLEEP-LAYER.md) — how the nightly Phase A / B / C
  consolidation works.
- [REFERENCE.md](./REFERENCE.md) — every primitive flag, every hook
  exit code, every skill description.
- [SECURITY.md](./SECURITY.md) — threat model + mitigations.

If anything in this guide doesn't match the repo you're reading,
that's a bug — please open an issue.
