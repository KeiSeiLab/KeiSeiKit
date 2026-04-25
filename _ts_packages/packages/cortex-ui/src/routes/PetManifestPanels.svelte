<script lang="ts">
  /**
   * Read-only manifest detail panels — extracted from PetEditor to keep
   * the parent under the 200-LOC Constructor Pattern budget after the
   * v0.40 DiffPane integration. Pure presentation; no side effects.
   */
  import type { PetManifest } from '../lib/types';

  interface Props {
    manifest: PetManifest;
  }

  const { manifest }: Props = $props();
</script>

<details>
  <summary>Identity</summary>
  <pre>{JSON.stringify(manifest.identity, null, 2)}</pre>
</details>

<details>
  <summary>Voice</summary>
  <pre>{JSON.stringify(manifest.voice, null, 2)}</pre>
</details>

<details>
  <summary>Edge</summary>
  <pre>{JSON.stringify(manifest.edge, null, 2)}</pre>
</details>

<details>
  <summary>Forbidden</summary>
  <pre>{JSON.stringify(manifest.forbidden, null, 2)}</pre>
</details>

<div style="margin-top: 16px;">
  <button class="secondary" disabled>Tune (coming soon)</button>
</div>
