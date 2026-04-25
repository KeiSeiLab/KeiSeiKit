<script lang="ts">
  // Dropzone + style/rig pickers + submit button + progress + preview.
  // Talks to the Rust daemon's `/portrait/stylize` endpoint via the
  // thin wrapper in `lib/portrait/upload.ts`. Svelte 5 runes.

  import type { CortexConfig } from '../lib/config';
  import {
    uploadPortrait,
    type BaseModel,
    type PortraitStyle,
    type StylizeResponse,
  } from '../lib/portrait/upload';

  interface Props {
    config: CortexConfig;
    user_id: string;
    onSuccess?: (r: StylizeResponse) => void;
  }

  const { config, user_id, onSuccess }: Props = $props();

  let file = $state<File | null>(null);
  let drag_over = $state(false);
  let style = $state<PortraitStyle>('anime-cute');
  let base_model = $state<BaseModel>('haru');
  let busy = $state(false);
  let error = $state<string | null>(null);
  let result = $state<StylizeResponse | null>(null);

  const file_label = $derived(
    file ? `${file.name} (${(file.size / 1024).toFixed(0)} KB)` : 'No file',
  );

  const styles: { v: PortraitStyle; label: string }[] = [
    { v: 'anime-cute', label: 'Cute' }, { v: 'anime-cool', label: 'Cool' },
    { v: 'anime-studious', label: 'Studious' },
  ];
  const base_models: { v: BaseModel; label: string }[] = [
    { v: 'haru', label: 'Haru' }, { v: 'mao', label: 'Mao' },
    { v: 'hiyori', label: 'Hiyori' }, { v: 'mark', label: 'Mark' },
  ];

  function set_file(f: File | null): void {
    file = f;
    result = null;
    error = null;
  }

  function on_file_input(e: Event): void {
    const t = e.target as HTMLInputElement;
    set_file(t.files && t.files.length > 0 ? t.files[0] : null);
  }

  function on_drop(e: DragEvent): void {
    e.preventDefault();
    drag_over = false;
    const cand = e.dataTransfer?.files?.[0];
    if (!cand) return;
    if (!cand.type.startsWith('image/')) {
      error = 'Drop an image file (PNG or JPEG).';
      return;
    }
    set_file(cand);
  }

  function on_drag_over(e: DragEvent): void {
    e.preventDefault();
    drag_over = true;
  }

  async function submit(e: Event): Promise<void> {
    e.preventDefault();
    if (!file) { error = 'Pick or drop a portrait image first.'; return; }
    busy = true;
    error = null;
    try {
      const r = await uploadPortrait(config, user_id, file, style, base_model);
      result = r;
      onSuccess?.(r);
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    } finally {
      busy = false;
    }
  }
</script>

<form class="portrait-uploader" onsubmit={submit}>
  <div
    class="dropzone"
    class:drag-over={drag_over}
    class:has-file={!!file}
    ondrop={on_drop}
    ondragover={on_drag_over}
    ondragleave={() => (drag_over = false)}
    role="button"
    tabindex="0"
  >
    <input type="file" accept="image/png,image/jpeg"
      onchange={on_file_input} aria-label="Choose portrait file" />
    <div class="dropzone-label">Drop a portrait here, or click to browse</div>
    <div class="dropzone-filename muted">{file_label}</div>
  </div>

  <fieldset class="radio-group">
    <legend>Style</legend>
    {#each styles as s}
      <label class="radio-row">
        <input type="radio" name="style" value={s.v}
          checked={style === s.v} onchange={() => (style = s.v)} />
        <span>{s.label}</span>
      </label>
    {/each}
  </fieldset>

  <fieldset class="radio-group">
    <legend>Base rig</legend>
    {#each base_models as m}
      <label class="radio-row">
        <input type="radio" name="base_model" value={m.v}
          checked={base_model === m.v} onchange={() => (base_model = m.v)} />
        <span>{m.label}</span>
      </label>
    {/each}
  </fieldset>

  {#if error}<div class="error">{error}</div>{/if}

  <div class="actions">
    <button type="submit" disabled={busy || !file}>
      {busy ? 'Stylizing…' : 'Generate anime pet'}
    </button>
    {#if busy}<span class="muted">Flux 2 Pro (~10–30s)…</span>{/if}
  </div>

  {#if result}
    <div class="result">
      <img class="preview" src={result.preview_url} alt="stylized portrait" />
      <div class="result-meta muted">
        Custom rig: <code>{result.rig_dir}</code><br />
        Took {result.took_ms} ms
      </div>
    </div>
  {/if}
</form>

<style>
  .portrait-uploader { margin: 12px 0; }
  .dropzone {
    position: relative; border: 2px dashed var(--border, #e5e5e5);
    border-radius: 8px; padding: 24px 16px; text-align: center;
    background: var(--card, #f7f7f8); cursor: pointer;
    transition: border-color 0.1s;
  }
  .dropzone.drag-over { border-color: var(--accent, #4f46e5); }
  .dropzone.has-file { border-style: solid; }
  .dropzone input[type='file'] {
    position: absolute; inset: 0; opacity: 0; cursor: pointer;
    width: 100%; height: 100%;
  }
  .dropzone-label { font-weight: 500; }
  .dropzone-filename { margin-top: 4px; }
  .radio-group {
    margin: 14px 0 8px; padding: 10px 12px;
    border: 1px solid var(--border, #e5e5e5); border-radius: 6px;
  }
  .radio-group legend {
    padding: 0 6px; font-size: 13px; color: var(--muted, #666);
  }
  .radio-row {
    display: flex; align-items: center; gap: 8px; margin: 4px 0;
    font-size: 14px; cursor: pointer;
  }
  .radio-row input[type='radio'] { width: auto; margin: 0; }
  .actions {
    display: flex; gap: 12px; align-items: center; margin-top: 14px;
  }
  .result {
    margin-top: 16px; padding: 12px;
    border: 1px solid var(--border, #e5e5e5); border-radius: 8px;
    background: var(--card, #f7f7f8);
  }
  .preview {
    display: block; max-width: 256px; max-height: 256px;
    border-radius: 6px; margin-bottom: 8px;
  }
  .result-meta code { font-size: 12px; }
</style>
