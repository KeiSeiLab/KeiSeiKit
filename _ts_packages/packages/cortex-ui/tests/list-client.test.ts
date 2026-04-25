import { describe, it, expect, vi, beforeEach } from 'vitest';
import { listDir } from '../src/lib/fs/list-client';

describe('listDir — fs/list client', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('passes Bearer header and encoded path', async () => {
    const mock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      statusText: 'OK',
      json: async () => ({
        entries: [
          { name: 'src', kind: 'dir' },
          { name: 'README.md', kind: 'file', size: 12 },
        ],
      }),
    });
    globalThis.fetch = mock as unknown as typeof fetch;
    const out = await listDir(
      { daemon_url: 'http://x', token: 'abc' },
      '/tmp/space dir',
    );
    const [url, init] = mock.mock.calls[0] as [string, RequestInit];
    expect(url).toBe('http://x/api/v1/cortex/fs/list?path=%2Ftmp%2Fspace%20dir');
    expect((init.headers as Record<string, string>).Authorization).toBe('Bearer abc');
    expect(out).toHaveLength(2);
    expect(out[0]).toEqual({ name: 'src', kind: 'dir' });
  });

  it('returns empty array on 404 (endpoint not yet implemented)', async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
      statusText: 'Not Found',
      text: async () => 'no such route',
      json: async () => {
        throw new Error('should not be called on 404');
      },
    }) as unknown as typeof fetch;
    const out = await listDir(
      { daemon_url: 'http://x', token: 't' },
      '/',
    );
    expect(out).toEqual([]);
  });
});
