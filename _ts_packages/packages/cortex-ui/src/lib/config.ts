export interface CortexConfig {
  daemon_url: string;
  token: string;
}

const DEFAULT_DAEMON = 'http://localhost:9797';
const KEY_DAEMON = 'kei-cortex-daemon';
const KEY_TOKEN = 'kei-cortex-token';

/** Strip ALL whitespace (spaces, tabs, newlines, zero-width) from a token. */
function sanitize_token(raw: string): string {
  return raw.replace(/\s+/g, '');
}

export function load_config(): CortexConfig | null {
  const params = new URLSearchParams(window.location.search);
  const url_daemon = params.get('daemon')?.trim();
  const url_token_raw = params.get('token');
  const url_token = url_token_raw ? sanitize_token(url_token_raw) : null;
  if (url_daemon) localStorage.setItem(KEY_DAEMON, url_daemon);
  if (url_token) localStorage.setItem(KEY_TOKEN, url_token);
  const daemon_url =
    url_daemon ?? localStorage.getItem(KEY_DAEMON) ?? DEFAULT_DAEMON;
  const stored_token = localStorage.getItem(KEY_TOKEN);
  const token = url_token ?? (stored_token ? sanitize_token(stored_token) : '');
  if (!token) return null;
  return { daemon_url, token };
}

export function save_config(c: CortexConfig): void {
  localStorage.setItem(KEY_DAEMON, c.daemon_url);
  localStorage.setItem(KEY_TOKEN, c.token);
}

export function clear_config(): void {
  localStorage.removeItem(KEY_DAEMON);
  localStorage.removeItem(KEY_TOKEN);
}
