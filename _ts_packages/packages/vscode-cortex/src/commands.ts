// Commands — registered with VSCode in extension.ts and routed through
// the CortexWebviewProvider. Each command is one function; this module
// owns nothing else.
//
// Constructor Pattern: command bodies stay tiny. All webview state /
// HTML rewriting / messaging is delegated to CortexWebviewProvider.

import * as vscode from 'vscode';
import { CortexWebviewProvider, CORTEX_VIEW_ID } from './webview-provider';

const CMD_OPEN_CHAT = 'keisei-cortex.openChat';
const CMD_CHAT_ABOUT_SELECTION = 'keisei-cortex.chatAboutSelection';

/**
 * Register all KeiSei Cortex commands. Returns the disposables so the
 * caller (extension.ts::activate) can push them onto context.subscriptions
 * in one shot.
 */
export function register_commands(
  provider: CortexWebviewProvider,
): vscode.Disposable[] {
  return [
    vscode.commands.registerCommand(CMD_OPEN_CHAT, () => open_chat(provider)),
    vscode.commands.registerCommand(CMD_CHAT_ABOUT_SELECTION, () =>
      chat_about_selection(provider),
    ),
  ];
}

/**
 * Reveal the Cortex webview pane. Called by the keybinding (Cmd/Ctrl+
 * Shift+K) and the explicit "Open Chat" command.
 */
async function open_chat(provider: CortexWebviewProvider): Promise<void> {
  // Focus the activity bar view first; this tells VSCode to instantiate
  // the webview if it has not been resolved yet.
  await vscode.commands.executeCommand(`${CORTEX_VIEW_ID}.focus`);
  provider.reveal();
}

/**
 * Read the current editor selection, focus the webview, and push the
 * selected text into the cortex-ui chat composer. If there is no
 * selection, fall back to the line under the cursor; if there is no
 * editor at all, surface a status message and return.
 */
async function chat_about_selection(
  provider: CortexWebviewProvider,
): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  if (!editor) {
    void vscode.window.showInformationMessage(
      'KeiSei: open a file and select text first.',
    );
    return;
  }
  const text = read_selection_or_line(editor);
  if (!text) {
    void vscode.window.showInformationMessage(
      'KeiSei: empty selection — nothing to ask about.',
    );
    return;
  }
  await open_chat(provider);
  const source = editor.document.uri.toString();
  provider.post_compose(text, source);
}

function read_selection_or_line(editor: vscode.TextEditor): string {
  const sel = editor.selection;
  if (!sel.isEmpty) return editor.document.getText(sel);
  const line = editor.document.lineAt(sel.active.line).text.trim();
  return line;
}
