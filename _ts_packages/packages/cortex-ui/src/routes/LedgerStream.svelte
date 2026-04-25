<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import type { CortexConfig } from '../lib/config';
  import type { LedgerRow } from '../lib/types';
  import { ledger as fetch_ledger } from '../lib/api';

  interface Props {
    config: CortexConfig;
  }

  const { config }: Props = $props();

  let rows = $state<LedgerRow[]>([]);
  let error = $state<string | null>(null);
  let timer: ReturnType<typeof setInterval> | null = null;

  async function refresh(): Promise<void> {
    try {
      const res = await fetch_ledger(config, 20);
      rows = res.rows;
      error = null;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }

  function format_ts(ts: number): string {
    return new Date(ts * 1000).toLocaleString();
  }

  onMount(() => {
    refresh();
    timer = setInterval(refresh, 5000);
  });

  onDestroy(() => {
    if (timer !== null) clearInterval(timer);
  });
</script>

<h2>Ledger</h2>
<p class="muted">Most recent 20 rows; refreshes every 5 s.</p>

{#if error}
  <div class="error">{error}</div>
{/if}

{#if rows.length === 0 && !error}
  <p class="muted">No entries yet.</p>
{:else}
  {#each rows as row (row.id)}
    <div class="row">
      <span><strong>{row.status}</strong></span>
      <span class="muted">{row.id.slice(0, 8)}…</span>
      <span class="muted">{row.dna ?? '—'}</span>
      <span class="muted" style="margin-left: auto;">{format_ts(row.started_ts)}</span>
    </div>
  {/each}
{/if}
