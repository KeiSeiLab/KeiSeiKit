// Tiny typed client for `GET /api/v1/cortex/usage`. Mirrors the shape
// of `lib/api.ts::api<T>` but intentionally treats 404 as a soft state
// (ledger has no `cost_cents` column yet) — the BudgetStrip renders a
// muted "ledger unavailable" row in that case.

import type { CortexConfig } from '../config';

export interface UsageReport {
  today_cents: number;
  week_cents: number;
  month_cents: number;
  top_provider: string;
  top_model: string;
}

const PATH = '/api/v1/cortex/usage';

/**
 * Fetch the usage rollup. Returns `null` when the daemon responds 404
 * (ledger schema not migrated yet) so the caller can render an
 * "unavailable" state without try/catch wrapping.
 *
 * Throws on any other non-2xx response — callers handle real errors
 * (401, 503, network) via the standard error path.
 */
export async function getUsage(c: CortexConfig): Promise<UsageReport | null> {
  const res = await fetch(`${c.daemon_url}${PATH}`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${c.token}`,
      'Content-Type': 'application/json',
    },
    credentials: 'omit',
  });
  if (res.status === 404) return null;
  if (!res.ok) {
    let body = '';
    try {
      body = (await res.text()).slice(0, 200);
    } catch {
      // ignore — error message is sufficient without body
    }
    throw new Error(
      `${res.status} ${res.statusText}${body ? ` — ${body}` : ''}`,
    );
  }
  return (await res.json()) as UsageReport;
}
