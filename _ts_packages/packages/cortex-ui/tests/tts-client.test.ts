import { describe, it, expect, vi, beforeEach } from 'vitest';
import { synthesize } from '../src/lib/voice/tts-client';

describe('tts-client synthesize', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('POSTs JSON body + Bearer header + returns audio blob', async () => {
    const mp3 = new Blob([new Uint8Array([0xff, 0xfb, 0x90])], {
      type: 'audio/mpeg',
    });
    const mock = vi.fn().mockResolvedValue({
      ok: true,
      blob: async () => mp3,
    });
    globalThis.fetch = mock as unknown as typeof fetch;

    const out = await synthesize(
      { daemon_url: 'http://x', token: 't' },
      'alice',
      'hello there',
    );

    expect(out).toBeInstanceOf(Blob);
    expect(out.type).toBe('audio/mpeg');
    const [url, init] = mock.mock.calls[0] as [string, RequestInit];
    expect(url).toBe('http://x/api/v1/cortex/pet/alice/tts');
    expect(init.method).toBe('POST');
    const headers = (init.headers ?? {}) as Record<string, string>;
    expect(headers.Authorization).toBe('Bearer t');
    expect(headers['Content-Type']).toBe('application/json');
    expect(JSON.parse(init.body as string)).toEqual({ text: 'hello there' });
  });

  it('encodes user_id in path', async () => {
    const mock = vi.fn().mockResolvedValue({
      ok: true,
      blob: async () => new Blob(['x'], { type: 'audio/mpeg' }),
    });
    globalThis.fetch = mock as unknown as typeof fetch;

    await synthesize(
      { daemon_url: 'http://x', token: 't' },
      'alice@example.com',
      'hi',
    );
    const [url] = mock.mock.calls[0] as [string];
    expect(url).toBe('http://x/api/v1/cortex/pet/alice%40example.com/tts');
  });

  it('clamps text beyond 3000 chars', async () => {
    const mock = vi.fn().mockResolvedValue({
      ok: true,
      blob: async () => new Blob(['x'], { type: 'audio/mpeg' }),
    });
    globalThis.fetch = mock as unknown as typeof fetch;

    const long = 'a'.repeat(3500);
    await synthesize(
      { daemon_url: 'http://x', token: 't' },
      'u',
      long,
    );
    const [, init] = mock.mock.calls[0] as [string, RequestInit];
    const body = JSON.parse(init.body as string) as { text: string };
    expect(body.text.length).toBe(3000);
  });

  it('throws on non-2xx response', async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 413,
      statusText: 'Payload Too Large',
      text: async () => 'too long',
    }) as unknown as typeof fetch;

    await expect(
      synthesize(
        { daemon_url: 'http://x', token: 't' },
        'u',
        'hi',
      ),
    ).rejects.toThrow('413');
  });
});
