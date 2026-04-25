import { describe, it, expect, beforeEach } from 'vitest';
import { load_config, save_config, clear_config } from '../src/lib/config';

function set_location(search: string): void {
  // jsdom lets us mutate window.location via history; pass as relative URL
  // since the jsdom default origin is http://localhost/ and replaceState()
  // refuses cross-origin-ish rewrites from the default about:blank.
  const url = search || '/';
  window.history.replaceState({}, '', url);
}

describe('config', () => {
  beforeEach(() => {
    localStorage.clear();
    set_location('');
  });

  it('returns null when no token present', () => {
    set_location('');
    expect(load_config()).toBeNull();
  });

  it('URL param overrides localStorage for daemon', () => {
    save_config({ daemon_url: 'http://stored:9797', token: 'tkn' });
    set_location('?daemon=http://override:8080');
    const cfg = load_config();
    expect(cfg).not.toBeNull();
    expect(cfg!.daemon_url).toBe('http://override:8080');
    expect(cfg!.token).toBe('tkn');
  });

  it('URL param token bootstraps config even when localStorage empty', () => {
    set_location('?token=fromurl');
    const cfg = load_config();
    expect(cfg).not.toBeNull();
    expect(cfg!.token).toBe('fromurl');
    expect(cfg!.daemon_url).toBe('http://localhost:9797');
  });

  it('falls back to default daemon when nothing stored', () => {
    save_config({ daemon_url: 'http://localhost:9797', token: 't' });
    const cfg = load_config();
    expect(cfg!.daemon_url).toBe('http://localhost:9797');
  });

  it('clear_config removes both keys', () => {
    save_config({ daemon_url: 'http://x', token: 't' });
    clear_config();
    expect(load_config()).toBeNull();
  });
});
