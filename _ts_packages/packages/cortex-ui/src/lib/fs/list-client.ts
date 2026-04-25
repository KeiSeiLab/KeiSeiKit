/**
 * Thin client for `GET /api/v1/cortex/fs/list?path=<path>`.
 *
 * NOTE: The daemon endpoint does NOT exist yet (see `components/INTEGRATION.md`
 * "Missing endpoints"). This client is shipped now so the FileTree component
 * can be wired and unit-tested without blocking on the daemon work. Until the
 * route lands the client returns `[]` on 404 — the tree degrades to an empty
 * lazy-load instead of throwing.
 */

import type { CortexConfig } from '../config';
import type { FileEntry, ListDirResponse } from './types';

/**
 * List directory contents on the daemon host. Returns `[]` when the endpoint
 * is missing (404), so the calling UI can render an empty tree rather than
 * surface a hard error during the rollout window.
 */
export async function listDir(
  c: CortexConfig,
  path: string,
): Promise<FileEntry[]> {
  const url = `${c.daemon_url}/api/v1/cortex/fs/list?path=${encodeURIComponent(path)}`;
  const res = await fetch(url, {
    headers: {
      Authorization: `Bearer ${c.token}`,
      'Content-Type': 'application/json',
    },
    credentials: 'omit',
  });
  if (res.status === 404) return [];
  if (!res.ok) {
    let body = '';
    try {
      body = (await res.text()).slice(0, 200);
    } catch {
      /* ignore body decode errors */
    }
    throw new Error(`${res.status} ${res.statusText}${body ? ` — ${body}` : ''}`);
  }
  const data = (await res.json()) as ListDirResponse;
  return data.entries ?? [];
}
