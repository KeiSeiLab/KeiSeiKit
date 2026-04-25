import { describe, it, expect, vi, afterEach, beforeEach } from 'vitest';
import { render, cleanup, fireEvent, waitFor } from '@testing-library/svelte';

/**
 * App.svelte mounts BudgetStrip, FileTree, and (toggleable) TerminalPane.
 * Mock the API + WebSocket boundary so the layout test stays focused on
 * markup, toggle persistence, and the data-testid contract — not on
 * upstream state (BudgetStrip/usage-client and FileTree/list-client are
 * unit-tested separately).
 */
vi.mock('../src/lib/usage/usage-client', () => ({
  getUsage: vi.fn().mockResolvedValue(null), // → "ledger unavailable"
}));

vi.mock('../src/lib/fs/list-client', () => ({
  listDir: vi.fn().mockResolvedValue([]),
}));

// Dashboard / LedgerStream / MemorySearch / PetEditor pull these in. Stub
// the whole module so layout tests don't talk to real fetch.
vi.mock('../src/lib/api', () => ({
  summary: vi.fn().mockResolvedValue({
    total_dnas: 0,
    active_pets: [],
    ledger_last_ts: null,
    recent_sessions: 0,
  }),
  pet: vi.fn().mockResolvedValue({ pet: null }),
  ledger: vi.fn().mockResolvedValue({ rows: [] }),
  memory_search: vi.fn().mockResolvedValue({ hits: [] }),
  chat_stream: vi.fn(),
  chatStreamWithProvider: vi.fn(),
  synthesize: vi.fn(),
  transcribe: vi.fn(),
  applyToolEdit: vi.fn(),
}));

// xterm.js touches the DOM at construct time; stub it for jsdom.
vi.mock('xterm', () => ({
  Terminal: class {
    loadAddon(): void { /* noop */ }
    open(): void { /* noop */ }
    write(): void { /* noop */ }
    onData(): void { /* noop */ }
    dispose(): void { /* noop */ }
  },
}));
vi.mock('xterm-addon-fit', () => ({
  FitAddon: class { fit(): void { /* noop */ } },
}));

import App from '../src/App.svelte';

beforeEach(() => {
  localStorage.clear();
  // pre-seed credentials so App skips the Setup splash
  localStorage.setItem('kei-cortex-daemon', 'http://localhost:9797');
  localStorage.setItem('kei-cortex-token', 't');
  // route to dashboard for a known render path
  window.location.hash = '#/';
});

afterEach(() => cleanup());

describe('App layout', () => {
  it('mounts the BudgetStrip in the header when configured', async () => {
    const { getByTestId } = render(App);
    await waitFor(() => {
      expect(getByTestId('budget-strip')).toBeTruthy();
    });
  });

  it('renders the sidebar by default and FileTree inside it', async () => {
    localStorage.setItem('kei-cortex-sidebar', 'open');
    const { getByTestId } = render(App);
    await waitFor(() => {
      expect(getByTestId('sidebar')).toBeTruthy();
    });
    const sidebar = getByTestId('sidebar');
    // FileTree mounts a tree role inside the sidebar
    expect(sidebar.querySelector('[role="tree"]')).toBeTruthy();
  });

  it('persists sidebar toggle state to localStorage', async () => {
    localStorage.setItem('kei-cortex-sidebar', 'open');
    const { getByTestId, queryByTestId } = render(App);
    const btn = await waitFor(() => getByTestId('sidebar-toggle'));
    expect(btn.getAttribute('aria-pressed')).toBe('true');
    await fireEvent.click(btn);
    await waitFor(() => {
      expect(localStorage.getItem('kei-cortex-sidebar')).toBe('closed');
    });
    await waitFor(() => {
      expect(queryByTestId('sidebar')).toBeNull();
    });
  });

  it('terminal dock is hidden by default and toggles on', async () => {
    const { getByTestId, queryByTestId } = render(App);
    const btn = await waitFor(() => getByTestId('terminal-toggle'));
    expect(btn.getAttribute('aria-pressed')).toBe('false');
    expect(queryByTestId('bottom-dock')).toBeNull();
    await fireEvent.click(btn);
    await waitFor(() => {
      expect(getByTestId('bottom-dock')).toBeTruthy();
    });
    expect(localStorage.getItem('kei-cortex-terminal')).toBe('open');
  });
});
