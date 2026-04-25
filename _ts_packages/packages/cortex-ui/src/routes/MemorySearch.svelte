<script lang="ts">
  import type { CortexConfig } from '../lib/config';
  import type { MemoryHit } from '../lib/types';
  import { memory_search } from '../lib/api';

  interface Props {
    config: CortexConfig;
  }

  const { config }: Props = $props();

  let user_id = $state('');
  let pet_name = $state('');
  let q = $state('');
  let hits = $state<MemoryHit[]>([]);
  let error = $state<string | null>(null);
  let loading = $state(false);

  async function submit(event: Event): Promise<void> {
    event.preventDefault();
    error = null;
    loading = true;
    try {
      const res = await memory_search(config, user_id, pet_name, q);
      hits = res.hits;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  }

  function format_ts(ts: number): string {
    return new Date(ts * 1000).toLocaleString();
  }
</script>

<h2>Memory search</h2>

<form onsubmit={submit}>
  <label for="user_id">User ID</label>
  <input id="user_id" bind:value={user_id} required />

  <label for="pet_name">Pet name</label>
  <input id="pet_name" bind:value={pet_name} required />

  <label for="q">Query</label>
  <input id="q" bind:value={q} required />

  <div style="margin-top: 12px;">
    <button type="submit" disabled={loading}>
      {loading ? 'Searching…' : 'Search'}
    </button>
  </div>
</form>

{#if error}
  <div class="error">{error}</div>
{/if}

{#if hits.length > 0}
  <h3 style="margin-top: 20px;">{hits.length} hits</h3>
  {#each hits as hit (hit.id)}
    <div class="card">
      <div class="muted">
        #{hit.id} · {hit.role} · {format_ts(hit.ts)}
      </div>
      <div style="margin-top: 6px;">{hit.text}</div>
    </div>
  {/each}
{/if}
