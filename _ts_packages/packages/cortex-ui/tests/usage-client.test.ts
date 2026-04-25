import { describe, it, expect, vi, beforeEach } from 'vitest';
import { getUsage } from '../src/lib/usage/usage-client';

describe('usage-client', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('sends the bearer header to /api/v1/cortex/usage', async () => {
    const mock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({
        today_cents: 12,
        week_cents: 34,
        month_cents: 56,
        top_provider: 'anthropic',
        top_model: 'claude-3-5-sonnet',
      }),
    });
    globalThis.fetch = mock as unknown as typeof fetch;
    const r = await getUsage({ daemon_url: 'http://x', token: 't' });
    expect(mock).toHaveBeenCalledWith(
      'http://x/api/v1/cortex/usage',
      expect.objectContaining({
        method: 'GET',
        headers: expect.objectContaining({ Authorization: 'Bearer t' }),
      }),
    );
    expect(r).not.toBeNull();
    expect(r!.today_cents).toBe(12);
  });

  it('returns null on 404 (ledger unavailable)', async () => {
    globalThis.fetch = vi
      .fn()
      .mockResolvedValue({
        ok: false,
        status: 404,
        statusText: 'Not Found',
      }) as unknown as typeof fetch;
    const r = await getUsage({ daemon_url: 'http://x', token: 't' });
    expect(r).toBeNull();
  });
});
