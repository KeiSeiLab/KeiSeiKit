/**
 * Speech-to-text client. POSTs a recorded audio blob to the daemon and
 * returns the plain transcript string.
 *
 * Server contract (LOCKED):
 *   POST <daemon>/api/v1/cortex/stt
 *   multipart/form-data, field "audio" = Blob
 *   Authorization: Bearer <token>
 *   → 200 {"transcript": "..."}
 */

import type { CortexConfig } from '../config';

interface SttResponse {
  transcript: string;
}

function build_form(audio: Blob): FormData {
  const fd = new FormData();
  // Explicit MIME prefix: some MediaRecorder configurations (notably Safari
  // early audio-only chunks) yield blobs with an empty `type`, which the
  // server then treats as `application/octet-stream` and rejects. Wrap to
  // guarantee the multipart part carries a correct `Content-Type`.
  const mime = audio.type || 'audio/webm';
  const wrapped = new Blob([audio], { type: mime });
  const filename = mime.includes('webm') ? 'audio.webm' : 'audio.bin';
  fd.append('audio', wrapped, filename);
  return fd;
}

async function read_error_body(res: Response): Promise<string> {
  try {
    return (await res.text()).slice(0, 200);
  } catch {
    return '';
  }
}

/** Transcribe `audio` via the cortex daemon. Throws on non-2xx or malformed
 *  JSON. Do NOT set Content-Type manually — the browser must append the
 *  multipart boundary, which it only does when Content-Type is absent. */
export async function transcribe(
  c: CortexConfig,
  audio: Blob,
): Promise<string> {
  const url = `${c.daemon_url}/api/v1/cortex/stt`;
  const res = await fetch(url, {
    method: 'POST',
    headers: { Authorization: `Bearer ${c.token}` },
    body: build_form(audio),
    credentials: 'omit',
  });
  if (!res.ok) {
    const body = await read_error_body(res);
    throw new Error(
      `${res.status} ${res.statusText}${body ? ` — ${body}` : ''}`,
    );
  }
  const parsed = (await res.json()) as SttResponse;
  if (typeof parsed?.transcript !== 'string') {
    throw new Error('stt response missing transcript');
  }
  return parsed.transcript;
}
