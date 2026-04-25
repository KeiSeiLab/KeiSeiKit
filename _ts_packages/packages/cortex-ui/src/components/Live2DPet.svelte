<script lang="ts">
  // Ported from AIRI (https://github.com/moeru-ai/airi), MIT License.
  // Original: packages/stage-ui-live2d/src/components/scenes/Live2D.vue
  // Adapted from Vue 3 <script setup> + Pinia stores to Svelte 5 runes.
  // Only the model-loading + emotion-switching + breathing/blink loop is
  // reproduced; Canvas.vue + Model.vue's richer feature-set (hot reload,
  // theme tint, focus tracking, render-scale probes) is deferred.

  import { onMount, onDestroy } from 'svelte';
  import { Emotion } from '../lib/live2d/emotions';
  import {
    createEmotionMotionManager,
    type EmotionMotionManager,
  } from '../lib/live2d/motion-manager';
  import type { Live2DPetProps } from '../lib/live2d/types';

  const {
    modelPath,
    mood = Emotion.Neutral,
    width = 256,
    height = 256,
    mouthOpenY = 0,
  }: Live2DPetProps = $props();

  let canvas = $state<HTMLCanvasElement | null>(null);
  let load_error = $state<string | null>(null);
  let ready = $state(false);

  // Non-reactive runtime refs (PIXI objects cannot be wrapped in $state).
  let app: { destroy?: (r?: boolean, opts?: unknown) => void } | null = null;
  let live2d_model: { destroy?: (opts?: unknown) => void } | null = null;
  let motion_mgr: EmotionMotionManager | null = null;

  onMount(() => {
    // Guard: tests run under jsdom which has no WebGL. Skip boot and leave
    // the canvas in place so `@testing-library` can assert a clean mount.
    if (!has_webgl()) {
      load_error = 'WebGL not available; renderer disabled (tests/jsdom).';
      return;
    }
    void boot();
  });

  onDestroy(() => {
    try {
      if (live2d_model?.destroy) {
        live2d_model.destroy({ children: true, texture: true, baseTexture: true });
      }
    } catch { /* noop */ }
    try {
      app?.destroy?.(true, { children: true, texture: true, baseTexture: true });
    } catch { /* noop */ }
    live2d_model = null;
    app = null;
    motion_mgr = null;
  });

  // Reactive bridge: mood prop → motion manager.
  $effect(() => {
    if (!ready || !motion_mgr) return;
    void motion_mgr.setEmotion(mood);
  });

  // Reactive bridge: mouthOpenY prop → Cubism `ParamMouthOpenY`. Each link
  // in the chain is optional so a stub / jsdom / partially-loaded model
  // never throws into user space.
  $effect(() => {
    if (!ready) return;
    const core = (live2d_model as unknown as {
      internalModel?: { coreModel?: { setParameterValueById?: (id: string, v: number) => void } };
    } | null)?.internalModel?.coreModel;
    const setter = core?.setParameterValueById;
    if (typeof setter !== 'function') return;
    const v = Number.isFinite(mouthOpenY) ? Math.max(0, Math.min(1, mouthOpenY)) : 0;
    try { setter.call(core, 'ParamMouthOpenY', v); } catch { /* noop */ }
  });

  async function boot(): Promise<void> {
    try {
      const { Application, Ticker } = await import('pixi.js');
      // pixi-live2d-display imports the Cubism 4 runtime + registers the
      // motion-manager plugin onto PIXI when loaded.
      const plugin = await import('pixi-live2d-display/cubism4');
      const Live2DModel = plugin.Live2DModel;
      // Without a ticker, the model paints once and stays frozen: no
      // breathing, no blinking, no emotion-switch motion. Must be called
      // before `Live2DModel.from`.
      Live2DModel.registerTicker(Ticker);
      if (!canvas) throw new Error('canvas element missing');

      app = new Application({
        view: canvas,
        width,
        height,
        backgroundAlpha: 0,
        antialias: true,
        autoStart: true,
      });

      const model = await Live2DModel.from(modelPath, { autoInteract: false });
      fit_model(model, width, height);
      // @ts-expect-error — PIXI container.addChild signature.
      app!.stage.addChild(model);

      live2d_model = model as unknown as typeof live2d_model;
      motion_mgr = createEmotionMotionManager(
        (model as { internalModel: { motionManager: unknown } }).internalModel
          .motionManager as Parameters<typeof createEmotionMotionManager>[0],
      );
      ready = true;
      // Initial emotion.
      await motion_mgr.setEmotion(mood);
    } catch (err) {
      load_error =
        err instanceof Error
          ? err.message
          : `failed to load Live2D model: ${String(err)}`;
      // Keep ready=false so the $effect above doesn't fire on a null mgr.
    }
  }

  function has_webgl(): boolean {
    if (typeof document === 'undefined') return false;
    // jsdom ships `HTMLCanvasElement.getContext` as a "not implemented"
    // stub that logs to stderr. Short-circuit when we can detect jsdom.
    const nav = typeof navigator !== 'undefined' ? navigator.userAgent : '';
    if (nav.includes('jsdom')) return false;
    try {
      const c = document.createElement('canvas');
      return !!(
        c.getContext('webgl2') ||
        c.getContext('webgl') ||
        c.getContext('experimental-webgl')
      );
    } catch {
      return false;
    }
  }

  function fit_model(
    m: { scale: { set: (v: number) => void }; x: number; y: number; width: number; height: number },
    cw: number,
    ch: number,
  ): void {
    const scale = Math.min(cw / m.width, ch / m.height) * 0.95;
    m.scale.set(scale);
    m.x = (cw - m.width) / 2;
    m.y = (ch - m.height) / 2;
  }
</script>

<div class="live2d-pet" style:width={`${width}px`} style:height={`${height}px`}>
  <canvas
    bind:this={canvas}
    class="live2d-canvas"
    width={width}
    height={height}
    aria-label="Live2D pet ({mood})"
  ></canvas>
  {#if load_error}
    <div class="live2d-error muted" role="status">{load_error}</div>
  {/if}
</div>

<style>
  .live2d-pet {
    position: relative;
    display: inline-block;
    border: 1px solid var(--border, #e5e5e5);
    border-radius: 8px;
    background: var(--card, #f7f7f8);
    overflow: hidden;
  }
  .live2d-canvas {
    display: block;
    width: 100%;
    height: 100%;
  }
  .live2d-error {
    position: absolute;
    inset: auto 0 8px 0;
    text-align: center;
    font-size: 11px;
    color: var(--muted, #666);
    padding: 2px 6px;
    background: rgba(255, 255, 255, 0.7);
  }
</style>
