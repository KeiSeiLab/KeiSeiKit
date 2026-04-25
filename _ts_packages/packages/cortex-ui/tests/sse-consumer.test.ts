import { describe, it, expect, vi, beforeEach } from 'vitest';
import { streamChat } from '../src/lib/chat/sse-consumer';
import type { ChatEvent } from '../src/lib/chat/types';

/** Build a Response-like object whose .body is a ReadableStream serving
 *  the given byte chunks sequentially, mimicking a real SSE frame flow. */
function mock_sse_response(chunks: string[]): Response {
  const encoder = new TextEncoder();
  const stream = new ReadableStream<Uint8Array>({
    start(controller) {
      for (const c of chunks) controller.enqueue(encoder.encode(c));
      controller.close();
    },
  });
  return new Response(stream, {
    status: 200,
    headers: { 'Content-Type': 'text/event-stream' },
  });
}

describe('streamChat SSE consumer', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('parses token/sentiment/done events in order from one stream', async () => {
    const events: ChatEvent[] = [];
    const body = [
      'data: {"type":"token","text":"Hi"}\n\n',
      'data: {"type":"token","text":" there"}\n\n',
      'data: {"type":"sentiment","tag":"happy","confidence":0.82}\n\n',
      'data: {"type":"done","conversation_id":"c-1"}\n\n',
    ];
    globalThis.fetch = vi
      .fn()
      .mockResolvedValue(mock_sse_response(body)) as unknown as typeof fetch;

    await streamChat({
      url: 'http://x',
      token: 't',
      user_id: 'u',
      message: 'hi',
      onEvent: (e) => events.push(e),
    });

    expect(events).toHaveLength(4);
    expect(events[0]).toEqual({ type: 'token', text: 'Hi' });
    expect(events[1]).toEqual({ type: 'token', text: ' there' });
    expect(events[2]).toMatchObject({
      type: 'sentiment',
      tag: 'happy',
      confidence: 0.82,
    });
    expect(events[3]).toEqual({ type: 'done', conversation_id: 'c-1' });
  });

  it('handles events split across chunk boundaries', async () => {
    const events: ChatEvent[] = [];
    // Split in the middle of a JSON payload to exercise the buffer.
    const body = [
      'data: {"type":"tok',
      'en","text":"A"}\n\ndata: {"type":"done","conversation_id":"c-2"}\n\n',
    ];
    globalThis.fetch = vi
      .fn()
      .mockResolvedValue(mock_sse_response(body)) as unknown as typeof fetch;

    await streamChat({
      url: 'http://x',
      token: 't',
      user_id: 'u',
      message: 'hi',
      onEvent: (e) => events.push(e),
    });

    expect(events).toEqual([
      { type: 'token', text: 'A' },
      { type: 'done', conversation_id: 'c-2' },
    ]);
  });

  it('posts to the correct URL with Bearer token + JSON body', async () => {
    const mock = vi.fn().mockResolvedValue(mock_sse_response([]));
    globalThis.fetch = mock as unknown as typeof fetch;

    await streamChat({
      url: 'http://x',
      token: 'secret',
      user_id: 'alice',
      message: 'hello',
      conversation_id: 'conv-99',
      onEvent: () => {},
    });

    const [url, init] = mock.mock.calls[0] as [string, RequestInit];
    expect(url).toBe('http://x/api/v1/cortex/pet/alice/chat');
    expect(init.method).toBe('POST');
    const headers = init.headers as Record<string, string>;
    expect(headers.Authorization).toBe('Bearer secret');
    expect(headers['Content-Type']).toBe('application/json');
    const body = JSON.parse(init.body as string) as {
      message: string;
      conversation_id: string;
    };
    expect(body.message).toBe('hello');
    expect(body.conversation_id).toBe('conv-99');
  });

  it('emits error event on non-ok response', async () => {
    const events: ChatEvent[] = [];
    globalThis.fetch = vi
      .fn()
      .mockResolvedValue(
        new Response('nope', { status: 500, statusText: 'Server Error' }),
      ) as unknown as typeof fetch;

    await streamChat({
      url: 'http://x',
      token: 't',
      user_id: 'u',
      message: 'hi',
      onEvent: (e) => events.push(e),
    });

    expect(events).toHaveLength(1);
    expect(events[0].type).toBe('error');
  });

  it('emits error event on fetch rejection', async () => {
    const events: ChatEvent[] = [];
    globalThis.fetch = vi
      .fn()
      .mockRejectedValue(new Error('network down')) as unknown as typeof fetch;

    await streamChat({
      url: 'http://x',
      token: 't',
      user_id: 'u',
      message: 'hi',
      onEvent: (e) => events.push(e),
    });

    expect(events).toEqual([{ type: 'error', message: 'network down' }]);
  });

  it('drops malformed JSON lines silently without throwing', async () => {
    const events: ChatEvent[] = [];
    const body = [
      'data: not-json\n\n',
      'data: {"type":"token","text":"ok"}\n\n',
    ];
    globalThis.fetch = vi
      .fn()
      .mockResolvedValue(mock_sse_response(body)) as unknown as typeof fetch;

    await streamChat({
      url: 'http://x',
      token: 't',
      user_id: 'u',
      message: 'hi',
      onEvent: (e) => events.push(e),
    });

    expect(events).toEqual([{ type: 'token', text: 'ok' }]);
  });

  // Wave 23 regressions — SSE framing strictness.

  it('splits on \\n\\n not \\n (single-newline inside payload is ignored)', async () => {
    const events: ChatEvent[] = [];
    // A single \n between `data:` lines is treated as a CONTINUATION inside
    // one frame, not a frame boundary. The server sends only one event.
    const body = [
      'data: {"type":"token",\ndata: "text":"joined"}\n\n',
    ];
    globalThis.fetch = vi
      .fn()
      .mockResolvedValue(mock_sse_response(body)) as unknown as typeof fetch;

    await streamChat({
      url: 'http://x',
      token: 't',
      user_id: 'u',
      message: 'hi',
      onEvent: (e) => events.push(e),
    });

    expect(events).toEqual([{ type: 'token', text: 'joined' }]);
  });

  it('skips :keepalive comment frames', async () => {
    const events: ChatEvent[] = [];
    const body = [
      ': keepalive\n\n',
      'data: {"type":"token","text":"ok"}\n\n',
      ':\n\n',
    ];
    globalThis.fetch = vi
      .fn()
      .mockResolvedValue(mock_sse_response(body)) as unknown as typeof fetch;

    await streamChat({
      url: 'http://x',
      token: 't',
      user_id: 'u',
      message: 'hi',
      onEvent: (e) => events.push(e),
    });

    expect(events).toEqual([{ type: 'token', text: 'ok' }]);
  });

  it('ignores retry: and event: fields', async () => {
    const events: ChatEvent[] = [];
    const body = [
      'retry: 5000\n\n',
      'event: custom\ndata: {"type":"token","text":"ok"}\n\n',
    ];
    globalThis.fetch = vi
      .fn()
      .mockResolvedValue(mock_sse_response(body)) as unknown as typeof fetch;

    await streamChat({
      url: 'http://x',
      token: 't',
      user_id: 'u',
      message: 'hi',
      onEvent: (e) => events.push(e),
    });

    expect(events).toEqual([{ type: 'token', text: 'ok' }]);
  });

  it('concatenates multiple data: lines with \\n inside one frame', async () => {
    const events: ChatEvent[] = [];
    // A two-line JSON payload must be joined by \n before parsing.
    const body = [
      'data: {"type":"token",\ndata: "text":"line1\\nline2"}\n\n',
    ];
    globalThis.fetch = vi
      .fn()
      .mockResolvedValue(mock_sse_response(body)) as unknown as typeof fetch;

    await streamChat({
      url: 'http://x',
      token: 't',
      user_id: 'u',
      message: 'hi',
      onEvent: (e) => events.push(e),
    });

    expect(events).toEqual([{ type: 'token', text: 'line1\nline2' }]);
  });

  it('emits frame-too-large error + stops on >256 KiB buffer', async () => {
    const events: ChatEvent[] = [];
    // A single never-terminated frame exceeding 256 KiB. No \n\n in sight.
    const huge = 'data: ' + 'A'.repeat(300_000);
    globalThis.fetch = vi
      .fn()
      .mockResolvedValue(mock_sse_response([huge])) as unknown as typeof fetch;

    await streamChat({
      url: 'http://x',
      token: 't',
      user_id: 'u',
      message: 'hi',
      onEvent: (e) => events.push(e),
    });

    expect(events).toHaveLength(1);
    expect(events[0]).toEqual({ type: 'error', message: 'frame too large' });
  });
});
