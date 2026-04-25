# KeiSei Cortex — VSCode Extension

Embeds the KeiSei Cortex UI (local AI pet companion + chat) inside VSCode as
a webview pane and adds editor integration: a "Ask KeiSei" code lens above
function and class declarations, plus a Cmd/Ctrl+Shift+K keybinding to open
the chat from anywhere in the editor.

## What it does

- Sidebar webview hosts the existing `@keisei/cortex-ui` Svelte bundle.
  Live2D pet, chat, ledger stream, memory search — all the routes the
  standalone web app exposes.
- "Ask KeiSei: <selection>" command posts the current editor selection
  into the chat composer.
- "Ask KeiSei" code lens appears above function / class declarations in
  TS, JS, Python, Rust, Go.

## Prerequisites

The extension talks to a local `kei-cortex` daemon. You must have the
daemon running on the URL configured in settings (defaults to
`http://127.0.0.1:9797`). To set up the daemon, run the
`/cortex-setup` skill in Claude Code, or follow the kei-cortex install
instructions.

The bearer token is read from a file (default `~/.keisei/cortex.token`).
Generate or rotate it through the cortex daemon CLI.

## Install

For end users (once published):

```
code --install-extension keisei.vscode-cortex
```

For local development:

1. Build the cortex-ui bundle: `cd ../cortex-ui && npm install && npm run build`.
2. Build this extension: `cd ../vscode-cortex && npm install && npm run compile`.
3. Press `F5` in VSCode with this folder open to launch an Extension
   Development Host with the extension loaded.

## Settings

| Key                          | Default                       | Description                                |
|------------------------------|-------------------------------|--------------------------------------------|
| `keisei-cortex.daemonUrl`    | `http://127.0.0.1:9797`       | URL of the kei-cortex daemon.              |
| `keisei-cortex.tokenPath`    | `~/.keisei/cortex.token`      | File containing the bearer token.          |

## Known limitations

- The webview requires the daemon listening on the configured URL —
  there is no offline mode.
- The code lens uses regex detection (no LSP); false positives are
  possible on minified or unusual files.
- The webview's CSP allows the daemon URL via `connect-src`. If you run
  the daemon on a non-default port or host, set `keisei-cortex.daemonUrl`
  before opening the chat.
- `pixi-live2d-display` requires `unsafe-eval` in the script CSP. This is
  acceptable for a local-only webview (no remote scripts loaded) but the
  reader should be aware.

## License

MIT.
