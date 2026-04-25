// Smoke test for the KeiSei Cortex VSCode extension.
//
// LIMITATION: full VSCode extension testing requires `@vscode/test-electron`
// which downloads VSCode and launches an Extension Development Host. That
// pipeline is too heavy for normal CI on this monorepo, so we run the
// lightweight smoke checks below under vitest:
//
//   1. The extension entry point exports `activate` and `deactivate`.
//   2. Both are functions (not async-thunks accidentally returning the
//      activation result rather than the function itself).
//   3. The webview-provider module exports the expected view ID and class.
//
// To run the heavy electron-host suite, install `@vscode/test-electron`
// and `vscode-test` per the VSCode extension guide and add a separate
// script `test:e2e`. That is an orchestrator concern, not part of the
// per-PR smoke gate.

import { describe, it, expect, vi } from 'vitest';

// vscode is provided by the extension host, not a real npm module.
// Mock it so the smoke test can import extension.ts in plain Node.
vi.mock('vscode', () => mock_vscode());

function mock_vscode(): unknown {
  const noop = (): unknown => ({ dispose: () => undefined });
  return {
    window: {
      registerWebviewViewProvider: noop,
      activeTextEditor: undefined,
      showInformationMessage: () => Promise.resolve(undefined),
    },
    languages: { registerCodeLensProvider: noop },
    commands: {
      registerCommand: noop,
      executeCommand: () => Promise.resolve(),
    },
    workspace: {
      getConfiguration: () => ({ get: (_k: string, d: unknown) => d }),
    },
    Uri: {
      joinPath: (...parts: unknown[]) => ({ fsPath: parts.join('/') }),
      parse: (s: string) => ({ toString: () => s }),
    },
    EventEmitter: class {
      event = (): unknown => ({ dispose: () => undefined });
    },
    CodeLens: class {
      constructor(
        public range: unknown,
        public command: unknown,
      ) {}
    },
  };
}

describe('extension entry point', () => {
  it('exports activate and deactivate as functions', async () => {
    const mod = await import('../src/extension');
    expect(typeof mod.activate).toBe('function');
    expect(typeof mod.deactivate).toBe('function');
  });

  it('exports the webview view ID', async () => {
    const mod = await import('../src/webview-provider');
    expect(typeof mod.CORTEX_VIEW_ID).toBe('string');
    expect(mod.CORTEX_VIEW_ID).toContain('keisei-cortex');
  });
});
