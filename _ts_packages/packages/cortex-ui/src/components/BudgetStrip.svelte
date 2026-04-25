<script lang="ts">
  // Compact horizontal cost strip — polls /api/v1/cortex/usage every
  // 60s by default. Color thresholds (green <$1, yellow <$5, red ≥$5)
  // apply to the today_cents bucket. Svelte 5 runes; matches the
  // PortraitUploader / ChatPanel style in this package.

  import { onMount, onDestroy } from 'svelte';
  import type { CortexConfig } from '../lib/config';
  import { getUsage, type UsageReport } from '../lib/usage/usage-client';

  interface Props {
    config: CortexConfig;
    refreshIntervalMs?: number;
  }

  const { config, refreshIntervalMs = 60_000 }: Props = $props();

  let report = $state<UsageReport | null>(null);
  let loading = $state(true);
  let unavailable = $state(false);
  let error = $state<string | null>(null);
  let timer: ReturnType<typeof setInterval> | null = null;

  // Color buckets for `today_cents`. Values match the spec: green<$1,
  // yellow<$5, red≥$5. Cents → dollars done at render time.
  const GREEN_MAX_CENTS = 100;
  const YELLOW_MAX_CENTS = 500;

  const today_color = $derived(color_for(report?.today_cents));
  const today_dollars = $derived(fmt(report?.today_cents));
  const week_dollars = $derived(fmt(report?.week_cents));
  const month_dollars = $derived(fmt(report?.month_cents));
  const top_label = $derived(top_for(report));

  function color_for(cents: number | undefined): string {
    if (cents === undefined) return 'muted';
    if (cents < GREEN_MAX_CENTS) return 'green';
    if (cents < YELLOW_MAX_CENTS) return 'yellow';
    return 'red';
  }

  function fmt(cents: number | undefined): string {
    if (cents === undefined) return '—';
    return `$${(cents / 100).toFixed(2)}`;
  }

  function top_for(r: UsageReport | null): string {
    if (!r) return '—';
    if (!r.top_provider && !r.top_model) return 'no usage yet';
    return `${r.top_provider} · ${r.top_model}`;
  }

  async function refresh(): Promise<void> {
    try {
      const r = await getUsage(config);
      if (r === null) {
        unavailable = true;
        report = null;
      } else {
        unavailable = false;
        report = r;
      }
      error = null;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    void refresh();
    timer = setInterval(() => void refresh(), refreshIntervalMs);
  });

  onDestroy(() => {
    if (timer) clearInterval(timer);
    timer = null;
  });
</script>

<div class="budget-strip" role="status" aria-live="polite" data-testid="budget-strip">
  {#if loading}
    <span class="skeleton" data-testid="budget-loading">Loading usage…</span>
  {:else if unavailable}
    <span class="muted" data-testid="budget-unavailable">ledger unavailable</span>
  {:else if error}
    <span class="error" data-testid="budget-error">usage error: {error}</span>
  {:else if report}
    <span class="cell">Today: <span class="amt {today_color}" data-testid="budget-today">{today_dollars}</span></span>
    <span class="sep" aria-hidden="true">·</span>
    <span class="cell">Week: <span class="amt" data-testid="budget-week">{week_dollars}</span></span>
    <span class="sep" aria-hidden="true">·</span>
    <span class="cell">Month: <span class="amt" data-testid="budget-month">{month_dollars}</span></span>
    <span class="sep" aria-hidden="true">·</span>
    <span class="cell">Top: <span class="top" data-testid="budget-top">{top_label}</span></span>
  {/if}
</div>

<style>
  .budget-strip {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    height: 60px;
    padding: 0 1rem;
    font-size: 0.85rem;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    background: rgba(255, 255, 255, 0.03);
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    white-space: nowrap;
    overflow-x: auto;
  }
  .cell { color: var(--text-dim, #cccccc); }
  .amt { font-weight: 600; }
  .amt.green  { color: #5fc173; }
  .amt.yellow { color: #e7c14f; }
  .amt.red    { color: #e8615b; }
  .top { color: var(--text-bright, #ffffff); }
  .sep { color: var(--text-faint, #555555); }
  .skeleton { color: var(--text-faint, #777777); animation: pulse 1.4s ease-in-out infinite; }
  .muted { color: var(--text-faint, #777777); font-style: italic; }
  .error { color: #e8615b; }
  @keyframes pulse {
    0%, 100% { opacity: 0.5; }
    50%      { opacity: 1.0; }
  }
</style>
