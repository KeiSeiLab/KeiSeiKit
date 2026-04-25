<script lang="ts">
  import { save_config, load_config, type CortexConfig } from '../lib/config';
  import {
    load_provider, save_provider,
    load_cwd, save_cwd,
    PROVIDERS, type Provider,
  } from '../lib/layout-prefs';
  import SetupCustomPet from './SetupCustomPet.svelte';
  import type { StylizeResponse } from '../lib/portrait/upload';

  interface Props {
    on_saved: () => void;
    /**
     * Optional user_id for the portrait uploader. Defaults to the string
     * `"default"` — a single-pet install works out of the box. Multi-pet
     * callers can override.
     */
    user_id?: string;
  }

  const { on_saved, user_id = 'default' }: Props = $props();

  const KEY_RENDERER = 'kei-cortex-renderer';

  const existing = load_config();
  let daemon_url = $state(existing?.daemon_url ?? 'http://localhost:9797');
  let token = $state(existing?.token ?? '');
  let renderer = $state<'sprite32' | 'live2d'>(load_renderer());
  let provider = $state<Provider>(load_provider());
  let cwd = $state<string>(load_cwd());
  let error = $state<string | null>(null);

  // Config is only non-null once the user has saved a daemon URL + token.
  // PortraitUploader needs both to talk to the daemon.
  let current_config = $state<CortexConfig | null>(existing);
  let custom_result = $state<StylizeResponse | null>(null);

  function load_renderer(): 'sprite32' | 'live2d' {
    if (typeof localStorage === 'undefined') return 'sprite32';
    const stored = localStorage.getItem(KEY_RENDERER);
    if (stored === 'live2d' || stored === 'sprite32') return stored;
    return 'sprite32';
  }

  function submit(event: Event): void {
    event.preventDefault();
    if (!token.trim()) {
      error = 'Token required';
      return;
    }
    const cfg = { daemon_url: daemon_url.trim(), token: token.trim() };
    save_config(cfg);
    save_provider(provider);
    save_cwd(cwd);
    current_config = cfg;
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem(KEY_RENDERER, renderer);
    }
    error = null;
    on_saved();
  }

  function on_portrait_success(r: StylizeResponse): void {
    custom_result = r;
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('kei-cortex-custom-model', r.model_json);
      localStorage.setItem(KEY_RENDERER, 'live2d');
    }
  }
</script>

<h2>Setup</h2>
<p class="muted">
  Connect to a local <code>kei-cortex</code> daemon. Credentials are stored in
  localStorage only — never transmitted except to the daemon you specify.
</p>

<form onsubmit={submit}>
  <label for="daemon">Daemon URL</label>
  <input
    id="daemon"
    type="url"
    bind:value={daemon_url}
    placeholder="http://localhost:9797"
    required
  />

  <label for="token">Bearer token</label>
  <input id="token" type="password" bind:value={token} required />

  <fieldset class="renderer-group">
    <legend>Pet renderer</legend>
    <label class="radio-row">
      <input
        type="radio"
        name="renderer"
        value="sprite32"
        checked={renderer === 'sprite32'}
        onchange={() => (renderer = 'sprite32')}
      />
      <span>32px pixel sprite (default, fast)</span>
    </label>
    <label class="radio-row">
      <input
        type="radio"
        name="renderer"
        value="live2d"
        checked={renderer === 'live2d'}
        onchange={() => (renderer = 'live2d')}
      />
      <span>Live2D (animated, breathing + blinking)</span>
    </label>
  </fieldset>

  <fieldset class="provider-group" data-testid="provider-group">
    <legend>AI Provider</legend>
    {#each PROVIDERS as p (p)}
      <label class="radio-row">
        <input
          type="radio"
          name="provider"
          value={p}
          checked={provider === p}
          onchange={() => (provider = p)}
          data-testid="provider-{p}"
        />
        <span>{p}</span>
      </label>
    {/each}
  </fieldset>

  <label for="cwd">Project root</label>
  <input
    id="cwd"
    type="text"
    bind:value={cwd}
    placeholder="/path/to/project"
    data-testid="cwd-input"
  />

  {#if error}
    <div class="error">{error}</div>
  {/if}

  <div style="margin-top: 16px;">
    <button type="submit">Save</button>
  </div>
</form>

<SetupCustomPet
  {current_config}
  {user_id}
  {custom_result}
  onSuccess={on_portrait_success}
/>

<style>
  .renderer-group,
  .provider-group {
    margin: 14px 0 8px;
    padding: 10px 12px;
    border: 1px solid var(--border, #e5e5e5);
    border-radius: 6px;
  }
  .renderer-group legend,
  .provider-group legend {
    padding: 0 6px;
    font-size: 13px;
    color: var(--muted, #666);
  }
  .radio-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin: 4px 0;
    font-size: 14px;
    cursor: pointer;
  }
  .radio-row input[type='radio'] {
    width: auto;
    margin: 0;
  }
</style>
