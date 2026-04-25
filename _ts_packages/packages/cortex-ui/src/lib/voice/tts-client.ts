/**
 * Text-to-speech client. POSTs a pet response to the daemon and returns
 * the raw mp3 blob (audio/mpeg). Playback + lip-sync live in lip-sync.ts.
 *
 * Server contract (LOCKED):
 *   POST <daemon>/api/v1/cortex/pet/:user_id/tts
 *   JSON body {"text": "..."}, max 3000 chars
 *   Authorization: Bearer <token>
 *   → 200 audio/mpeg binary (mp3)
 */

import type { CortexConfig } from '../config';

const MAX_CHARS = 3000;

function clamp_text(text: string): string {
  if (text.length <= MAX_CHARS) return text;
  return text.slice(0, MAX_CHARS);
}

async function read_error_body(res: Response): Promise<string> {
  try {
    return (await res.text()).slice(0, 200);
  } catch {
    return '';
  }
}

/** Synthesize `text` for `user_id`. Resolves to a Blob typed `audio/mpeg`
 *  (or whatever the server labels it). Throws on non-2xx. */
export async function synthesize(
  c: CortexConfig,
  user_id: string,
  text: string,
): Promise<Blob> {
  const url = `${c.daemon_url}/api/v1/cortex/pet/${encodeURIComponent(
    user_id,
  )}/tts`;
  const res = await fetch(url, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${c.token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ text: clamp_text(text) }),
    credentials: 'omit',
  });
  if (!res.ok) {
    const body = await read_error_body(res);
    throw new Error(
      `${res.status} ${res.statusText}${body ? ` — ${body}` : ''}`,
    );
  }
  return res.blob();
}
