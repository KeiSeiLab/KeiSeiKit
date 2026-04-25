import { describe, it, expect, vi, beforeEach } from 'vitest';
import { summary, ledger, memory_search, pet } from '../src/lib/api';

describe('api wrapper', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('adds Authorization header', async () => {
    const mock = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({
        total_dnas: 0,
        active_pets: [],
        ledger_last_ts: null,
        recent_sessions: 0,
      }),
    });
    globalThis.fetch = mock as unknown as typeof fetch;
    await summary({ daemon_url: 'http://x', token: 't' });
    expect(mock).toHaveBeenCalledWith(
      'http://x/api/v1/cortex/summary',
      expect.objectContaining({
        headers: expect.objectContaining({ Authorization: 'Bearer t' }),
      }),
    );
  });

  it('throws on non-2xx', async () => {
    globalThis.fetch = vi
      .fn()
      .mockResolvedValue({
        ok: false,
        status: 401,
        statusText: 'Unauthorized',
      }) as unknown as typeof fetch;
    await expect(
      summary({ daemon_url: 'http://x', token: 't' }),
    ).rejects.toThrow('401');
  });

  it('encodes query params for memory_search', async () => {
    const mock = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ hits: [] }),
    });
    globalThis.fetch = mock as unknown as typeof fetch;
    await memory_search(
      { daemon_url: 'http://x', token: 't' },
      'user one',
      'kei/pet',
      'hello world',
    );
    const [url] = mock.mock.calls[0] as [string, RequestInit];
    expect(url).toContain('user_id=user%20one');
    expect(url).toContain('pet_name=kei%2Fpet');
    expect(url).toContain('q=hello%20world');
  });

  it('passes limit to ledger', async () => {
    const mock = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ rows: [] }),
    });
    globalThis.fetch = mock as unknown as typeof fetch;
    await ledger({ daemon_url: 'http://x', token: 't' }, 50);
    const [url] = mock.mock.calls[0] as [string, RequestInit];
    expect(url).toBe('http://x/api/v1/cortex/ledger/recent?limit=50');
  });

  it('encodes user_id in pet path', async () => {
    const mock = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ pet: {} }),
    });
    globalThis.fetch = mock as unknown as typeof fetch;
    await pet({ daemon_url: 'http://x', token: 't' }, 'alice@example.com');
    const [url] = mock.mock.calls[0] as [string, RequestInit];
    expect(url).toBe(
      'http://x/api/v1/cortex/pet/alice%40example.com',
    );
  });
});
