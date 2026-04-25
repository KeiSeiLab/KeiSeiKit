<script lang="ts">
  import { onMount } from 'svelte';
  import type { CortexConfig } from '../lib/config';
  import type { Summary } from '../lib/types';
  import { summary as fetch_summary } from '../lib/api';

  interface Props {
    config: CortexConfig;
  }

  const { config }: Props = $props();

  let data = $state<Summary | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(true);

  onMount(async () => {
    try {
      data = await fetch_summary(config);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  });
</script>

<h2>Dashboard</h2>

{#if loading}
  <p class="muted">Loading summary…</p>
{:else if error}
  <div class="error">Failed to load: {error}</div>
{:else if data}
  <div class="cards">
    <div class="card">
      <h3>Total DNAs</h3>
      <div class="value">{data.total_dnas}</div>
    </div>
    <div class="card">
      <h3>Active pets</h3>
      <div class="value">{data.active_pets.length}</div>
    </div>
    <div class="card">
      <h3>Recent sessions</h3>
      <div class="value">{data.recent_sessions}</div>
    </div>
  </div>

  <h3>Pets</h3>
  <ul>
    {#each data.active_pets as uid}
      <li><a href="#/pet/{uid}">{uid}</a></li>
    {/each}
  </ul>
{/if}
