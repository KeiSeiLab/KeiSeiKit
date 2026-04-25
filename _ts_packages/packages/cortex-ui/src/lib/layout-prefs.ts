/**
 * Layout preferences — localStorage helpers for App.svelte chrome state.
 *
 * Single-source-of-truth for the keys used across App / Setup / FileTree /
 * TerminalPane. Each helper is wrapped in a try/catch so a private-mode or
 * disabled-storage browser still renders with safe defaults instead of
 * throwing on first paint.
 *
 * Keys:
 *   kei-cortex-sidebar   → 'open' | 'closed'
 *   kei-cortex-terminal  → 'open' | 'closed'
 *   kei-cortex-cwd       → string (project root path)
 *   kei-cortex-provider  → 'anthropic' | 'openai' | 'kimi'
 */

export const KEY_SIDEBAR = 'kei-cortex-sidebar';
export const KEY_TERMINAL = 'kei-cortex-terminal';
export const KEY_CWD = 'kei-cortex-cwd';
export const KEY_PROVIDER = 'kei-cortex-provider';

export type ToggleState = 'open' | 'closed';
export type Provider = 'anthropic' | 'openai' | 'kimi';

export const PROVIDERS: readonly Provider[] = ['anthropic', 'openai', 'kimi'] as const;

function safe_get(key: string): string | null {
  try {
    if (typeof localStorage === 'undefined') return null;
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

function safe_set(key: string, value: string): void {
  try {
    if (typeof localStorage === 'undefined') return;
    localStorage.setItem(key, value);
  } catch {
    /* private mode / quota — silent */
  }
}

/** Sidebar default: closed on narrow viewports (<900px), else open. */
export function load_sidebar(): ToggleState {
  const stored = safe_get(KEY_SIDEBAR);
  if (stored === 'open' || stored === 'closed') return stored;
  if (typeof window === 'undefined') return 'open';
  return window.innerWidth < 900 ? 'closed' : 'open';
}

export function save_sidebar(s: ToggleState): void {
  safe_set(KEY_SIDEBAR, s);
}

/** Terminal default: always closed unless the user explicitly opened it. */
export function load_terminal(): ToggleState {
  const stored = safe_get(KEY_TERMINAL);
  return stored === 'open' ? 'open' : 'closed';
}

export function save_terminal(s: ToggleState): void {
  safe_set(KEY_TERMINAL, s);
}

/**
 * Project root cwd. Resolution order: ?cwd= URL param → localStorage → '/'.
 * The URL form lets a daemon launch URL pin the working dir; subsequent
 * visits remember via localStorage.
 */
export function load_cwd(): string {
  if (typeof window !== 'undefined') {
    try {
      const params = new URLSearchParams(window.location.search);
      const url_cwd = params.get('cwd')?.trim();
      if (url_cwd) {
        safe_set(KEY_CWD, url_cwd);
        return url_cwd;
      }
    } catch {
      /* no window.location.search → server-side render fallback */
    }
  }
  const stored = safe_get(KEY_CWD);
  if (stored && stored.trim()) return stored;
  return '/';
}

export function save_cwd(cwd: string): void {
  safe_set(KEY_CWD, cwd.trim() || '/');
}

/** Provider default: anthropic — matches existing daemon behaviour. */
export function load_provider(): Provider {
  const stored = safe_get(KEY_PROVIDER);
  if (stored === 'anthropic' || stored === 'openai' || stored === 'kimi') return stored;
  return 'anthropic';
}

export function save_provider(p: Provider): void {
  safe_set(KEY_PROVIDER, p);
}
