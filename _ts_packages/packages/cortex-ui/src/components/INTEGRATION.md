# cortex-ui — DiffPane / FileTree / TerminalPane integration

Wave 34 ships three new components decoupled from the existing layout.
The orchestrator wires them in a follow-up. This file is the handoff.

## Wiring (orchestrator owns)

1. **DiffPane → PetEditor** — render `<DiffPane>` below `<ChatPanel>` whenever
   the editor's `diffPending` state is truthy. Pass `oldText`, `newText`,
   `filename`. Wire `onApply` / `onReject` to the editor's persistence
   actions. Clear `diffPending` after either callback fires.

2. **FileTree → App.svelte** — add as a left sidebar with a header toggle
   button. Pass `config={config}`, `root={'/'}` (or workspace root),
   `currentPath={editorPath}`, and `onSelect={(p) => loadFileIntoEditor(p)}`.

3. **TerminalPane → App.svelte** — bottom dock, collapsed by default with a
   toggle button. Pass `cwd={'/'}`, `daemon_url`, `token`. Bind WS lifecycle
   to dock visibility (mount only when expanded — `onDestroy` closes WS).

## Missing daemon endpoints (separate wave)

These routes do NOT exist on the daemon yet. The components degrade
gracefully (FileTree shows empty list on 404, TerminalPane shows
"read-only — no daemon endpoint" footer), but real interactivity needs:

- **`GET /api/v1/cortex/fs/list?path=<path>`** — returns
  `{ entries: [{ name, kind: "file" | "dir", size?, mtime? }] }`. Sort dirs
  first then files, both alpha. Bearer-auth identical to the existing
  cortex routes.

- **`WS /api/v1/cortex/term?cwd=<cwd>`** — bidirectional byte stream
  bridged to a PTY in the requested cwd. Bearer token sent as the
  `bearer` WS subprotocol per existing convention. Server echoes shell
  output as text frames; client sends raw input characters.

File a TODO with the daemon team to land these.

## Cleanup contract (already implemented in components)

- DiffPane: pure `$derived`, no listeners — no cleanup needed.
- FileTree: no global listeners or sockets — no cleanup needed.
- TerminalPane: `onDestroy` disposes the xterm instance, disconnects the
  ResizeObserver, and closes the WebSocket.
