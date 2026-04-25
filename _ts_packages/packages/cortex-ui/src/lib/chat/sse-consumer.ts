// Single-shot SSE consumer for the /api/v1/cortex/pet/:user_id/chat endpoint.
// We use fetch + ReadableStream rather than EventSource so we can attach a
// Bearer header (EventSource cannot). No reconnection; one POST per message.
//
// Event framing follows the SSE spec (W3C EventSource §9.2): events are
// delimited by `\n\n` (double newline). Each event consists of one or more
// lines; lines beginning with `:` are comments (keep-alives) and ignored.
// Inside an event, multiple `data:` lines are joined by `\n`; `retry:` and
// `event:` fields are currently ignored (we only consume the default message
// stream). A 256 KiB buffer cap guards against a pathological server frame.

import type { ChatEvent, SentimentTag } from './types';
import { SENTIMENT_TAGS } from './types';

/** Hard cap on the accumulator buffer. 256 KiB covers any realistic SSE
 *  frame while bounding memory if the server misbehaves. */
const MAX_BUFFER_BYTES = 256 * 1024;

export interface StreamChatOpts {
  url: string;
  token: string;
  user_id: string;
  message: string;
  conversation_id?: string;
  /**
   * Optional provider override; appended as `?provider=<p>` to the chat
   * endpoint URL. Daemon picks default when omitted.
   */
  provider?: string;
  onEvent: (e: ChatEvent) => void;
  signal?: AbortSignal;
}

export async function streamChat(opts: StreamChatOpts): Promise<void> {
  try {
    const res = await fetch(build_url(opts.url, opts.user_id, opts.provider), {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${opts.token}`,
        'Content-Type': 'application/json',
        Accept: 'text/event-stream',
      },
      body: JSON.stringify({
        message: opts.message,
        conversation_id: opts.conversation_id,
      }),
      credentials: 'omit',
      signal: opts.signal,
    });
    if (!res.ok || !res.body) {
      opts.onEvent({
        type: 'error',
        message: `chat stream ${res.status} ${res.statusText}`,
      });
      return;
    }
    await consume_stream(res.body, opts.onEvent);
  } catch (err) {
    opts.onEvent({
      type: 'error',
      message: err instanceof Error ? err.message : String(err),
    });
  }
}

function build_url(daemon: string, user_id: string, provider?: string): string {
  const base = `${daemon}/api/v1/cortex/pet/${encodeURIComponent(user_id)}/chat`;
  if (!provider) return base;
  return `${base}?provider=${encodeURIComponent(provider)}`;
}

/** Drain the stream, decode UTF-8, split on `\n\n` (SSE frame boundary),
 *  dispatch each complete frame. Enforces `MAX_BUFFER_BYTES` so a never-
 *  terminated frame cannot exhaust memory. */
async function consume_stream(
  body: ReadableStream<Uint8Array>,
  onEvent: (e: ChatEvent) => void,
): Promise<void> {
  const reader = body.pipeThrough(new TextDecoderStream()).getReader();
  let buffer = '';
  try {
    while (true) {
      const { value, done } = await reader.read();
      if (done) break;
      buffer += value;
      buffer = drain_frames(buffer, onEvent);
      // Apply the cap to the TAIL after draining. A frame-terminated input
      // with huge content is fine — only a never-terminated frame grows
      // unbounded.
      if (buffer.length > MAX_BUFFER_BYTES) {
        onEvent({ type: 'error', message: 'frame too large' });
        try { await reader.cancel(); } catch { /* noop */ }
        return;
      }
    }
    // Flush a final trailing frame that the server forgot to terminate.
    if (buffer.length > 0) dispatch_frame(buffer, onEvent);
  } finally {
    reader.releaseLock();
  }
}

/** Pull every complete `\n\n`-delimited frame off the buffer; return the
 *  tail (partial frame) to accumulate more bytes into. */
function drain_frames(
  buffer: string,
  onEvent: (e: ChatEvent) => void,
): string {
  let start = 0;
  let idx = buffer.indexOf('\n\n', start);
  while (idx !== -1) {
    const frame = buffer.slice(start, idx);
    dispatch_frame(frame, onEvent);
    start = idx + 2;
    idx = buffer.indexOf('\n\n', start);
  }
  return buffer.slice(start);
}

/** Parse a single SSE frame: skip comments/retry/event, concatenate `data:`
 *  lines with `\n`, then JSON.parse the merged payload. */
function dispatch_frame(frame: string, onEvent: (e: ChatEvent) => void): void {
  const lines = frame.split('\n');
  const data_parts: string[] = [];
  for (const raw of lines) {
    const line = raw.endsWith('\r') ? raw.slice(0, -1) : raw;
    if (line.length === 0) continue;
    if (line.startsWith(':')) continue; // SSE comment / keep-alive
    if (line.startsWith('retry:') || line.startsWith('event:')) continue;
    if (line.startsWith('data:')) {
      data_parts.push(line.slice(5).replace(/^ /, ''));
    }
  }
  if (data_parts.length === 0) return;
  const payload = data_parts.join('\n');
  const parsed = parse_event(payload);
  if (parsed) onEvent(parsed);
}

/** Parse a JSON payload into a ChatEvent. Returns null on malformed input. */
function parse_event(payload: string): ChatEvent | null {
  try {
    const obj = JSON.parse(payload) as Record<string, unknown>;
    const t = obj.type;
    if (t === 'token' && typeof obj.text === 'string') {
      return { type: 'token', text: obj.text };
    }
    if (t === 'sentiment' && is_tag(obj.tag)) {
      const conf = typeof obj.confidence === 'number' ? obj.confidence : 0;
      return { type: 'sentiment', tag: obj.tag, confidence: conf };
    }
    if (t === 'done' && typeof obj.conversation_id === 'string') {
      return { type: 'done', conversation_id: obj.conversation_id };
    }
    if (t === 'error' && typeof obj.message === 'string') {
      return { type: 'error', message: obj.message };
    }
  } catch {
    /* malformed json; drop silently */
  }
  return null;
}

function is_tag(x: unknown): x is SentimentTag {
  return typeof x === 'string' && (SENTIMENT_TAGS as readonly string[]).includes(x);
}
