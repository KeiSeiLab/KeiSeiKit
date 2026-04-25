// CortexCodeLensProvider — adds a clickable "Ask KeiSei" lens above
// function / class declarations. Uses regex detection (NOT a real LSP)
// since we want to support many languages without per-language ASTs.
//
// Constructor Pattern: this module owns lens detection ONLY. The click
// handler delegates to the existing chatAboutSelection command, which
// re-uses the editor-selection path for free.

import * as vscode from 'vscode';

const CMD_CHAT_ABOUT_SELECTION = 'keisei-cortex.chatAboutSelection';

// Detect declarations across TS/JS/Python/Rust/Go.
//   - JS/TS: `function name`, `class Name`, `const name = (`, `name(...) {`
//   - Python: `def name`, `class Name`
//   - Rust:   `fn name`, `pub fn name`, `impl X for Y`, `struct`, `enum`
//   - Go:     `func name`, `func (r *X) name`, `type X struct`
const DECL_PATTERNS: RegExp[] = [
  /^\s*(export\s+)?(async\s+)?function\s+\w+/,
  /^\s*(export\s+)?(default\s+)?class\s+\w+/,
  /^\s*(pub\s+)?(async\s+)?fn\s+\w+/,
  /^\s*(pub\s+)?(struct|enum|trait)\s+\w+/,
  /^\s*def\s+\w+/,
  /^\s*class\s+\w+/,
  /^\s*func\s+(\(\s*\w+\s+[*\w]+\s*\)\s+)?\w+/,
  /^\s*type\s+\w+\s+(struct|interface)/,
];

export class CortexCodeLensProvider implements vscode.CodeLensProvider {
  private readonly emitter = new vscode.EventEmitter<void>();
  public readonly onDidChangeCodeLenses = this.emitter.event;

  public provideCodeLenses(
    document: vscode.TextDocument,
    token: vscode.CancellationToken,
  ): vscode.CodeLens[] {
    const lenses: vscode.CodeLens[] = [];
    const limit = Math.min(document.lineCount, 5000);
    for (let i = 0; i < limit; i++) {
      if (token.isCancellationRequested) break;
      const text = document.lineAt(i).text;
      if (!matches_decl(text)) continue;
      lenses.push(make_lens(document.lineAt(i).range));
    }
    return lenses;
  }
}

function matches_decl(line: string): boolean {
  if (line.length > 400) return false; // skip pathological minified lines
  for (const re of DECL_PATTERNS) {
    if (re.test(line)) return true;
  }
  return false;
}

function make_lens(range: vscode.Range): vscode.CodeLens {
  return new vscode.CodeLens(range, {
    title: '$(comment-discussion) Ask KeiSei',
    tooltip: 'Send this declaration to the KeiSei Cortex chat.',
    command: CMD_CHAT_ABOUT_SELECTION,
    arguments: [],
  });
}
