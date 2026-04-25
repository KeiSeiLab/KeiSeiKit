// Entry point for the KeiSei Cortex VSCode extension.
//
// Constructor Pattern:
//   - This file owns the activation lifecycle ONLY.
//   - View provider lives in webview-provider.ts.
//   - Commands live in commands.ts.
//   - Code lens lives in code-lens.ts.

import * as vscode from 'vscode';
import { CortexWebviewProvider, CORTEX_VIEW_ID } from './webview-provider';
import { register_commands } from './commands';
import { CortexCodeLensProvider } from './code-lens';

const EXT_ID = 'keisei-cortex';

/**
 * Build the disposables needed for activation. Registered as a single
 * subscription bundle so deactivation is "delete one array" — no manual
 * tracking. Each helper module returns its own Disposable[].
 */
function build_subscriptions(
  provider: CortexWebviewProvider,
): vscode.Disposable[] {
  const view = vscode.window.registerWebviewViewProvider(
    CORTEX_VIEW_ID,
    provider,
    { webviewOptions: { retainContextWhenHidden: true } },
  );
  const commands = register_commands(provider);
  const lens = vscode.languages.registerCodeLensProvider(
    [
      { language: 'typescript' },
      { language: 'javascript' },
      { language: 'typescriptreact' },
      { language: 'javascriptreact' },
      { language: 'python' },
      { language: 'rust' },
      { language: 'go' },
    ],
    new CortexCodeLensProvider(),
  );
  return [view, lens, ...commands];
}

/**
 * VSCode extension activation hook. Called once per session, after the
 * `onStartupFinished` event fires. Registers the webview provider, commands,
 * and code lens. All registrations go into `context.subscriptions` so they
 * are auto-disposed on extension unload.
 */
export function activate(context: vscode.ExtensionContext): void {
  log_info('activating KeiSei Cortex');
  const provider = new CortexWebviewProvider(context.extensionUri);
  const subs = build_subscriptions(provider);
  context.subscriptions.push(...subs);
  log_info(`activated, ${subs.length} disposables registered`);
}

/**
 * VSCode extension deactivation hook. Disposables registered against
 * `context.subscriptions` in `activate` are released by the host before
 * this function runs, so we only handle ad-hoc cleanup here.
 */
export function deactivate(): void {
  log_info('deactivating KeiSei Cortex');
}

function log_info(msg: string): void {
  // Use console.log for now; an OutputChannel can be added later if needed.
  // The VSCode log output is captured automatically when the extension host
  // is launched with the dev flag.
  // eslint-disable-next-line no-console
  console.log(`[${EXT_ID}] ${msg}`);
}
