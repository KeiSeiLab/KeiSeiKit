// CortexWebviewProvider — hosts the cortex-ui SvelteKit bundle inside a
// VSCode webview pane. The cortex-ui source lives in `../cortex-ui` and is
// expected to have been built into `../cortex-ui/dist/` before packaging
// (see INTEGRATION.md).
//
// Constructor Pattern: this module owns webview lifecycle + HTML rewriting.
// It does NOT know about commands, editor selection, or code lenses — those
// modules call us via a postMessage façade.

import * as vscode from 'vscode';
import * as fs from 'node:fs';
import * as path from 'node:path';
import * as os from 'node:os';

export const CORTEX_VIEW_ID = 'keisei-cortex.chatView';
const CONFIG_KEY_DAEMON = 'keisei-cortex.daemonUrl';
const CONFIG_KEY_TOKEN_PATH = 'keisei-cortex.tokenPath';

interface InboundMessage {
  type: 'status' | 'nav' | 'log';
  payload?: unknown;
}

export class CortexWebviewProvider implements vscode.WebviewViewProvider {
  private view: vscode.WebviewView | undefined;
  private readonly cortex_ui_root: vscode.Uri;

  constructor(private readonly extension_uri: vscode.Uri) {
    // Sibling package: ../cortex-ui (resolved relative to this extension's
    // installed location). At dev time this points into the workspace; at
    // package time the build copies dist/ alongside.
    this.cortex_ui_root = vscode.Uri.joinPath(
      extension_uri,
      '..',
      'cortex-ui',
      'dist',
    );
  }

  public resolveWebviewView(
    view: vscode.WebviewView,
    _ctx: vscode.WebviewViewResolveContext,
    _token: vscode.CancellationToken,
  ): void {
    this.view = view;
    view.webview.options = {
      enableScripts: true,
      localResourceRoots: [this.cortex_ui_root],
    };
    view.webview.html = this.render_html(view.webview);
    view.webview.onDidReceiveMessage((m) => this.handle_inbound(m));
    view.onDidDispose(() => {
      this.view = undefined;
    });
    // Push initial config once the webview script has had a chance to wire
    // its message listener. The cortex-ui Setup route reads `daemon` and
    // `token` from URL params first, but we also push them dynamically so
    // the embed works even after the page has been served from cache.
    queueMicrotask(() => this.post_init());
  }

  /**
   * Reveal the webview pane (no-op if disposed). Called by the openChat
   * command and the chatAboutSelection command before postMessage.
   */
  public reveal(): void {
    this.view?.show?.(true);
  }

  /**
   * Send the current editor selection to the cortex-ui chat composer.
   * The cortex-ui side listens for `compose` messages on `window` and
   * pre-fills its textarea (see cortex-ui docs).
   */
  public post_compose(text: string, source_uri: string): void {
    this.view?.webview.postMessage({
      type: 'compose',
      payload: { text, source: source_uri },
    });
  }

  private post_init(): void {
    const cfg = vscode.workspace.getConfiguration();
    const daemon = cfg.get<string>(CONFIG_KEY_DAEMON, 'http://127.0.0.1:9797');
    const token_path = cfg.get<string>(
      CONFIG_KEY_TOKEN_PATH,
      '~/.keisei/cortex.token',
    );
    const token = read_token(token_path);
    this.view?.webview.postMessage({
      type: 'init',
      payload: { daemon_url: daemon, token },
    });
  }

  private handle_inbound(m: InboundMessage): void {
    if (!m || typeof m.type !== 'string') return;
    if (m.type === 'log') {
      // eslint-disable-next-line no-console
      console.log('[keisei-cortex/webview]', m.payload);
      return;
    }
    if (m.type === 'status' || m.type === 'nav') {
      // Future: surface status in VSCode status bar. For now log only.
      // eslint-disable-next-line no-console
      console.log(`[keisei-cortex/${m.type}]`, m.payload);
    }
  }

  private render_html(webview: vscode.Webview): string {
    const index_path = path.join(this.cortex_ui_root.fsPath, 'index.html');
    const raw = safe_read(index_path);
    if (!raw) return fallback_html('Cortex UI bundle not found at ' + index_path);
    const rewritten = rewrite_assets(raw, this.cortex_ui_root, webview);
    return inject_csp_and_nonce(rewritten, webview.cspSource);
  }
}

function safe_read(p: string): string | null {
  try {
    return fs.readFileSync(p, 'utf-8');
  } catch {
    return null;
  }
}

function read_token(token_path: string): string {
  const expanded = token_path.startsWith('~')
    ? path.join(os.homedir(), token_path.slice(1))
    : token_path;
  try {
    return fs.readFileSync(expanded, 'utf-8').trim();
  } catch {
    return '';
  }
}

/** Replace relative asset paths (./assets/*, /assets/*, ./live2d/*) with
 *  webview-safe URIs. cortex-ui is built with `base: './'` so all asset
 *  hrefs/srcs are relative — we only need to resolve them to fsPath under
 *  cortex-ui/dist and convert each one through asWebviewUri. */
function rewrite_assets(
  html: string,
  root: vscode.Uri,
  webview: vscode.Webview,
): string {
  return html.replace(
    /(href|src)="(\.?\/[^"#?]+)"/g,
    (_match, attr: string, rel: string) => {
      const clean = rel.replace(/^\.?\//, '');
      const uri = vscode.Uri.joinPath(root, clean);
      return `${attr}="${webview.asWebviewUri(uri)}"`;
    },
  );
}

/** Inject a Content-Security-Policy meta tag if not present. cortex-ui may
 *  not include one (it's served by Vite dev/preview which has its own CSP),
 *  so we add a webview-friendly policy that allows the daemon URL via
 *  connect-src and live2d wasm via script-src. */
function inject_csp_and_nonce(html: string, csp_source: string): string {
  if (html.includes('http-equiv="Content-Security-Policy"')) return html;
  const csp =
    `default-src 'none'; ` +
    `style-src ${csp_source} 'unsafe-inline'; ` +
    `script-src ${csp_source} 'unsafe-eval' 'unsafe-inline'; ` +
    `img-src ${csp_source} data: blob: https:; ` +
    `font-src ${csp_source} data:; ` +
    `connect-src ${csp_source} http://127.0.0.1:9797 http://localhost:9797 ws://127.0.0.1:9797 ws://localhost:9797;`;
  const tag = `<meta http-equiv="Content-Security-Policy" content="${csp}">`;
  return html.replace(/<head>/i, `<head>${tag}`);
}

function fallback_html(reason: string): string {
  const msg = reason.replace(/[<>&]/g, (c) =>
    c === '<' ? '&lt;' : c === '>' ? '&gt;' : '&amp;',
  );
  return `<!doctype html><html><body><h2>KeiSei Cortex</h2><p>${msg}</p>` +
    `<p>Run <code>npm run build</code> in <code>../cortex-ui</code>, ` +
    `then re-package this extension.</p></body></html>`;
}
