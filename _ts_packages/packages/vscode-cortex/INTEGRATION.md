# Integration handoff — `@keisei/vscode-cortex`

## Build pipeline

```
# 1. Build the cortex-ui bundle (must complete first — produces ../cortex-ui/dist/).
cd _ts_packages/packages/cortex-ui
npm install
npm run build

# 2. Build this extension.
cd ../vscode-cortex
npm install
npm run compile

# 3. Package as a .vsix (assumes vsce is available either globally or via devDependency).
npm run package          # → keisei-vscode-cortex-0.1.0.vsix
```

## cortex-ui dist/ dependency

The webview provider resolves the cortex-ui bundle at runtime via:

```
vscode.Uri.joinPath(extension_uri, '..', 'cortex-ui', 'dist')
```

For this to work in a packaged `.vsix`, the cortex-ui dist must be copied
alongside the extension. Two options:

1. **Symlink (dev)**: leave `_ts_packages/packages/cortex-ui/` and
   `_ts_packages/packages/vscode-cortex/` side by side. VSCode resolves
   `..` from the extension install path; the F5 dev host reads them
   directly.

2. **Copy (prod)**: pre-publish step copies `cortex-ui/dist/` into
   `vscode-cortex/cortex-ui-dist/` and the provider's path resolution
   logic is updated to look there. (Not implemented in v0.1.0; tracked
   for v0.2.0.)

The Wave 21+ pipeline guarantees `cortex-ui/dist/` exists before this
step, so option 1 is sufficient for current release-engineering needs.

## Webview asset path: `localResourceRoots` + `asWebviewUri`

`localResourceRoots: [cortex_ui_root]` whitelists `../cortex-ui/dist/` as
the only filesystem location the webview is allowed to load files from.
The `rewrite_assets` helper rewrites every relative `href`/`src` in
`index.html` through `webview.asWebviewUri`, which converts an `fsPath`
into a `vscode-webview-resource://...` URL the webview origin trusts.
Live2D `.moc3`, `.json`, and `.wasm` assets pass through the same
rewriter.

## Publish

Orchestrator (not the agent) runs:

```
cd _ts_packages/packages/vscode-cortex
npm run package           # local .vsix for testing
vsce publish              # to VSCode Marketplace (requires PAT)
```

`vsce publish` requires a Marketplace publisher (`keisei`) and a Personal
Access Token via `vsce login keisei`. Both are out of scope for the
in-repo build.

## Per-RULE 0.13 boundary

Files written by the agent in this directory only. The agent did NOT
modify `cortex-ui/`, `daemon/`, or any sibling package. Branching,
committing, and pushing is performed by the orchestrator from the parent
session.
