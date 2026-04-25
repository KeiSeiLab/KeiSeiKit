<script lang="ts">
  import { onMount } from 'svelte';
  import type { CortexConfig } from '../lib/config';
  import type { PetManifest } from '../lib/types';
  import { pet as fetch_pet, applyToolEdit } from '../lib/api';
  import Live2DPet from '../components/Live2DPet.svelte';
  import ChatPanel from '../components/ChatPanel.svelte';
  import DiffPane from '../components/DiffPane.svelte';
  import PetManifestPanels from './PetManifestPanels.svelte';
  import { Emotion } from '../lib/live2d/emotions';
  import { sentimentToEmotion } from '../lib/chat/sentiment-to-emotion';
  import type { SentimentTag } from '../lib/chat/types';
  import {
    reduce_tool_event, is_tool_event,
    type ToolEvent, type PendingDiff,
  } from '../lib/chat/diff-sentry';

  interface Props {
    config: CortexConfig;
    user_id: string;
    /**
     * Optional bindable pending-diff cell — tests inject sample diffs by
     * setting this before mount; production wires it via the chat-stream
     * sentry below. Null means no diff is currently pending.
     */
    pendingDiff?: PendingDiff | null;
  }

  let {
    config,
    user_id,
    pendingDiff = $bindable(null),
  }: Props = $props();

  const KEY_RENDERER = 'kei-cortex-renderer';
  const DEFAULT_LIVE2D_MODEL =
    './live2d-models/haru/haru_greeter_t03.model3.json';

  let manifest = $state<PetManifest | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(true);
  // Pet mood is now driven EXCLUSIVELY by server sentiment events from
  // ChatPanel → on_sentiment; no in-page mood buttons. Default Neutral.
  let emotion = $state<Emotion>(Emotion.Neutral);
  let renderer = $state<'sprite32' | 'live2d'>('sprite32');
  // Driven by ChatPanel's TTS playback loop; 0 == mouth closed.
  let mouth_open_y = $state(0);

  function species_for(pet_name: string): 'cat' | 'dog' | 'owl' | 'blob' {
    const first = pet_name.trim().toLowerCase().charAt(0);
    if (first === 'd') return 'dog';
    if (first === 'o') return 'owl';
    if (first === 'b') return 'blob';
    return 'cat';
  }

  /** Sprite path: always use the idle sprite; sentiment animation only
   *  applies to the Live2D renderer. Sprite-mode users still get an avatar,
   *  just without the per-sentiment pose. */
  function sprite_src(pet_name: string): string {
    return `./sprites/32px/${species_for(pet_name)}-idle.png`;
  }

  function load_renderer_pref(m: PetManifest | null): 'sprite32' | 'live2d' {
    // Preferred source of truth (future): `pet.toml` `renderer` field.
    const tomlField = (m?.meta as { renderer?: unknown } | undefined)?.renderer;
    if (tomlField === 'live2d' || tomlField === 'sprite32') return tomlField;
    if (typeof localStorage !== 'undefined') {
      const stored = localStorage.getItem(KEY_RENDERER);
      if (stored === 'live2d' || stored === 'sprite32') return stored;
    }
    return 'sprite32';
  }

  function on_sentiment(tag: SentimentTag): void {
    emotion = sentimentToEmotion(tag);
  }

  function on_rms(v: number): void {
    mouth_open_y = v;
  }

  /**
   * Sentry that ChatPanel will call once the Wave 30 tool_use events land
   * on the chat stream (Wave 39 todo). Currently kept as a referenced
   * helper so the diff-sentry imports stay live; tests inject diffs via
   * the bindable `pendingDiff` prop directly.
   */
  function on_tool_event(raw: unknown): void {
    if (!is_tool_event(raw)) return;
    pendingDiff = reduce_tool_event(pendingDiff, raw as ToolEvent);
  }
  // Keep `on_tool_event` referenced under `noUnusedLocals` if it ever ships.
  void on_tool_event;

  async function on_apply_diff(): Promise<void> {
    if (!pendingDiff) return;
    try {
      await applyToolEdit(config, pendingDiff);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      pendingDiff = null;
    }
  }

  function on_reject_diff(): void {
    pendingDiff = null;
  }

  onMount(async () => {
    if (!user_id) {
      error = 'missing user_id in route';
      loading = false;
      return;
    }
    try {
      const res = await fetch_pet(config, user_id);
      manifest = res.pet;
      renderer = load_renderer_pref(manifest);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  });
</script>

<h2>Pet: {user_id}</h2>

{#if manifest}
  <div class="pet-stage">
    <div class="pet-sprite-box">
      {#if renderer === 'live2d'}
        <Live2DPet
          modelPath={DEFAULT_LIVE2D_MODEL}
          mood={emotion}
          mouthOpenY={mouth_open_y}
          width={256}
          height={256}
        />
      {:else}
        <img
          class="pet-sprite"
          src={sprite_src(manifest.identity.pet_name)}
          alt="{manifest.identity.pet_name}"
          width="128"
          height="128"
        />
      {/if}
      <div class="pet-sprite-name">{manifest.identity.pet_name}</div>
      <div class="muted pet-sprite-emotion">mood: {emotion}</div>
    </div>
    <ChatPanel {config} {user_id} onsentiment={on_sentiment} onrms={on_rms} />
    {#if pendingDiff}
      <DiffPane
        oldText={pendingDiff.oldText}
        newText={pendingDiff.newText}
        filename={pendingDiff.filename}
        onApply={on_apply_diff}
        onReject={on_reject_diff}
      />
    {/if}
  </div>
{/if}

{#if loading}
  <p class="muted">Loading manifest…</p>
{:else if error}
  <div class="error">{error}</div>
{:else if manifest}
  <PetManifestPanels {manifest} />
{/if}

<style>
  .pet-stage {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    margin: 16px 0 24px;
  }
  .pet-sprite-emotion {
    font-size: 12px;
    text-transform: lowercase;
    letter-spacing: 0.02em;
  }
</style>
