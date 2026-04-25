// Portrait-upload API client. One function, no state, no imports from
// `api.ts` (that helper is JSON-only — we need multipart + no
// Content-Type header here because the browser must set the multipart
// boundary itself).

import type { CortexConfig } from '../config';

/** Mirror of the Rust `StylizeResponse` struct. */
export interface StylizeResponse {
  /** Relative directory e.g. `"live2d-models/custom-abc123/"`. */
  rig_dir: string;
  /** Absolute-path URL to the new rig's `.model3.json`. */
  model_json: string;
  /** Absolute-path URL to the preview PNG. */
  preview_url: string;
  /** Server-measured duration of the pipeline. */
  took_ms: number;
}

/** Style preset sent to the daemon. Mirrors the Rust `Style` enum wire form. */
export type PortraitStyle =
  | 'anime-cute'
  | 'anime-cool'
  | 'anime-studious';

/** Cubism base-rig choice. Mirrors the server's allow-list. */
export type BaseModel = 'haru' | 'mao' | 'hiyori' | 'mark';

/**
 * Upload a portrait to the cortex daemon and receive the URL of the
 * newly-minted custom Live2D rig.
 *
 * Throws `Error` with the server's error message (truncated to 200 chars)
 * on any non-2xx response, so UI code can display it directly.
 */
export async function uploadPortrait(
  c: CortexConfig,
  user_id: string,
  file: File,
  style: PortraitStyle,
  base_model: BaseModel,
): Promise<StylizeResponse> {
  if (file.size > 10 * 1024 * 1024) {
    throw new Error('file must be 10 MB or smaller');
  }
  const form = new FormData();
  form.append('file', file);
  form.append('style', style);
  form.append('base_model', base_model);

  const path = `/api/v1/cortex/pet/${encodeURIComponent(user_id)}/portrait/stylize`;
  const res = await fetch(`${c.daemon_url}${path}`, {
    method: 'POST',
    headers: {
      // DO NOT set Content-Type here — the browser appends the correct
      // `multipart/form-data; boundary=...` when it sees a FormData body.
      Authorization: `Bearer ${c.token}`,
    },
    body: form,
    credentials: 'omit',
  });

  if (!res.ok) {
    let body = '';
    try {
      body = (await res.text()).slice(0, 200);
    } catch {
      /* ignore */
    }
    throw new Error(
      `${res.status} ${res.statusText}${body ? ` — ${body}` : ''}`,
    );
  }
  return (await res.json()) as StylizeResponse;
}
