<script lang="ts">
  /**
   * Custom pet appearance section of the Setup screen — extracted as its
   * own cube to keep `Setup.svelte` ≤200 LOC after the v0.40 provider /
   * cwd additions. Pure presentation: parent owns load/save side-effects.
   */
  import type { CortexConfig } from '../lib/config';
  import PortraitUploader from '../components/PortraitUploader.svelte';
  import type { StylizeResponse } from '../lib/portrait/upload';

  interface Props {
    /** Null until the user saves daemon URL + token. */
    current_config: CortexConfig | null;
    user_id: string;
    custom_result: StylizeResponse | null;
    onSuccess: (r: StylizeResponse) => void;
  }

  const { current_config, user_id, custom_result, onSuccess }: Props = $props();
</script>

<fieldset class="custom-pet-group">
  <legend>Custom pet appearance</legend>
  <p class="muted">
    Upload a portrait to generate a one-of-a-kind anime Live2D pet. The image
    is sent to fal.ai Flux 2 Pro for stylization, then mapped onto a Cubism
    rig of your choice.
  </p>
  {#if current_config}
    <PortraitUploader
      config={current_config}
      user_id={user_id}
      onSuccess={onSuccess}
    />
    {#if custom_result}
      <div class="custom-success">
        This pet now uses your custom rig.
        <a href="#/">Return to dashboard</a>
      </div>
    {/if}
  {:else}
    <div class="muted">Save your daemon URL and token first, then upload.</div>
  {/if}
</fieldset>

<style>
  .custom-pet-group {
    margin: 24px 0 8px;
    padding: 12px 14px;
    border: 1px solid var(--border, #e5e5e5);
    border-radius: 6px;
  }
  .custom-pet-group legend {
    padding: 0 6px;
    font-size: 13px;
    color: var(--muted, #666);
  }
  .custom-success {
    margin-top: 10px;
    padding: 8px 10px;
    background: rgba(79, 70, 229, 0.08);
    border-radius: 6px;
    font-size: 14px;
  }
</style>
