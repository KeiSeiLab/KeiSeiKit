import type { CortexConfig } from './config';
import type { Summary, PetManifest, LedgerRow, MemoryHit } from './types';
import { streamChat } from './chat/sse-consumer';
import type { ChatEvent } from './chat/types';
import type { PendingDiff } from './chat/diff-sentry';
import type { Provider } from './layout-prefs';
import { transcribe as voice_transcribe } from './voice/stt-client';
import { synthesize as voice_synthesize } from './voice/tts-client';

async function api<T>(
  c: CortexConfig,
  path: string,
  init?: RequestInit,
): Promise<T> {
  const res = await fetch(`${c.daemon_url}${path}`, {
    ...init,
    headers: {
      ...(init?.headers ?? {}),
      Authorization: `Bearer ${c.token}`,
      'Content-Type': 'application/json',
    },
    // Bearer auth via explicit header; no cookies needed. credentials:'omit'
    // is the safe cross-origin default and avoids Safari quirks around
    // credentials:'include' stripping Authorization on some redirects.
    credentials: 'omit',
  });
  if (!res.ok) {
    let body = '';
    try { body = (await res.text()).slice(0, 200); } catch { /* ignore */ }
    throw new Error(`${res.status} ${res.statusText}${body ? ` — ${body}` : ''}`);
  }
  return res.json() as Promise<T>;
}

export const summary = (c: CortexConfig) =>
  api<Summary>(c, '/api/v1/cortex/summary');

export const pet = (c: CortexConfig, user_id: string) =>
  api<{ pet: PetManifest }>(
    c,
    `/api/v1/cortex/pet/${encodeURIComponent(user_id)}`,
  );

export const ledger = (c: CortexConfig, limit = 20) =>
  api<{ rows: LedgerRow[] }>(c, `/api/v1/cortex/ledger/recent?limit=${limit}`);

export const memory_search = (
  c: CortexConfig,
  user_id: string,
  pet_name: string,
  q: string,
) =>
  api<{ hits: MemoryHit[] }>(
    c,
    `/api/v1/cortex/memory/search?user_id=${encodeURIComponent(user_id)}&pet_name=${encodeURIComponent(pet_name)}&q=${encodeURIComponent(q)}`,
  );

/**
 * Thin wrapper around `streamChat` that plugs the existing daemon URL +
 * bearer token through the same style as the other helpers in this file.
 * The caller owns the onEvent handler and is responsible for throttling UI
 * updates. Returns when the server closes the stream (or on first error).
 * Pass `signal` to allow the caller to cancel a slow stream mid-flight.
 */
export const chat_stream = (
  c: CortexConfig,
  user_id: string,
  message: string,
  conversation_id: string | undefined,
  onEvent: (e: ChatEvent) => void,
  signal?: AbortSignal,
) =>
  streamChat({
    url: c.daemon_url,
    token: c.token,
    user_id,
    message,
    conversation_id,
    onEvent,
    signal,
  });

/** STT: upload a recorded audio blob, return the transcript string. */
export const transcribe = (c: CortexConfig, audio: Blob) =>
  voice_transcribe(c, audio);

/** TTS: synthesize pet response text, return an `audio/mpeg` blob. */
export const synthesize = (c: CortexConfig, user_id: string, text: string) =>
  voice_synthesize(c, user_id, text);

export interface ChatStreamOpts {
  /** AI provider override; defaults to whichever the daemon picks. */
  provider?: Provider;
  /** Continuation token for multi-turn dialog. */
  conversation_id?: string;
  /** Caller-owned abort signal for in-flight cancellation. */
  signal?: AbortSignal;
}

/**
 * Provider-aware chat stream — extends `chat_stream` with a `provider`
 * query param so the Setup screen's selection is honoured. The original
 * positional signature stays untouched (back-compat); new callers should
 * prefer this options-bag form.
 */
export const chatStreamWithProvider = (
  c: CortexConfig,
  user_id: string,
  message: string,
  onEvent: (e: ChatEvent) => void,
  opts: ChatStreamOpts = {},
) =>
  streamChat({
    url: c.daemon_url,
    token: c.token,
    user_id,
    message,
    conversation_id: opts.conversation_id,
    provider: opts.provider,
    onEvent,
    signal: opts.signal,
  });

/**
 * Persist a pending tool edit. Daemon endpoint may not yet exist; treat
 * 404 as "feature not wired" and resolve quietly so the UI clears the
 * pending diff regardless. All other non-2xx surface as errors.
 */
export async function applyToolEdit(
  c: CortexConfig,
  diff: PendingDiff,
): Promise<void> {
  const res = await fetch(`${c.daemon_url}/api/v1/cortex/tool/apply`, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${c.token}`,
      'Content-Type': 'application/json',
    },
    credentials: 'omit',
    body: JSON.stringify({
      path: diff.filename,
      old_text: diff.oldText,
      new_text: diff.newText,
    }),
  });
  if (res.status === 404) return; // endpoint not wired yet (Wave 39 todo)
  if (!res.ok) {
    let body = '';
    try { body = (await res.text()).slice(0, 200); } catch { /* ignore */ }
    throw new Error(`${res.status} ${res.statusText}${body ? ` — ${body}` : ''}`);
  }
}
