import { describe, it, expect, vi, beforeEach } from 'vitest';
import { transcribe } from '../src/lib/voice/stt-client';

describe('stt-client transcribe', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('POSTs multipart with audio field + Bearer header + returns transcript', async () => {
    const mock = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ transcript: 'hello world' }),
    });
    globalThis.fetch = mock as unknown as typeof fetch;

    const blob = new Blob([new Uint8Array([1, 2, 3, 4])], { type: 'audio/webm' });
    const text = await transcribe(
      { daemon_url: 'http://x', token: 't' },
      blob,
    );

    expect(text).toBe('hello world');
    expect(mock).toHaveBeenCalledTimes(1);
    const [url, init] = mock.mock.calls[0] as [string, RequestInit];
    expect(url).toBe('http://x/api/v1/cortex/stt');
    expect(init.method).toBe('POST');
    expect(init.body).toBeInstanceOf(FormData);
    const fd = init.body as FormData;
    const audio = fd.get('audio');
    expect(audio).toBeInstanceOf(Blob);
    expect((audio as Blob).type).toBe('audio/webm');
    // Explicitly DO NOT set Content-Type — browser must append boundary.
    const headers = (init.headers ?? {}) as Record<string, string>;
    expect(headers.Authorization).toBe('Bearer t');
    expect(headers['Content-Type']).toBeUndefined();
  });

  it('throws on non-2xx response', async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 500,
      statusText: 'Server Error',
      text: async () => 'boom',
    }) as unknown as typeof fetch;

    await expect(
      transcribe(
        { daemon_url: 'http://x', token: 't' },
        new Blob(['x']),
      ),
    ).rejects.toThrow('500');
  });

  it('throws when transcript field missing', async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({}),
    }) as unknown as typeof fetch;

    await expect(
      transcribe(
        { daemon_url: 'http://x', token: 't' },
        new Blob(['x']),
      ),
    ).rejects.toThrow(/transcript/);
  });
});
