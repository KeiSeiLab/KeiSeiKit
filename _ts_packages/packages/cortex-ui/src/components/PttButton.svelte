<script lang="ts">
  import { createRecorder } from '../lib/voice/recorder';
  import { transcribe } from '../lib/api';
  import type { CortexConfig } from '../lib/config';

  interface Props {
    config: CortexConfig;
    disabled?: boolean;
    /** Fired with the transcribed text once STT completes. */
    ontranscript: (text: string) => void;
    /** Fired with a human-readable status change (recording / uploading /
     *  permission-denied / error). Parent renders it in the status row. */
    onstatus?: (status: string | null) => void;
  }

  const { config, disabled = false, ontranscript, onstatus }: Props =
    $props();

  let recording = $state(false);
  let busy = $state(false);
  let handle: ReturnType<typeof createRecorder> | null = null;

  function set_status(s: string | null): void {
    onstatus?.(s);
  }

  async function begin(): Promise<void> {
    if (recording || busy || disabled) return;
    handle = createRecorder();
    try {
      await handle.start();
      recording = true;
      set_status('recording…');
    } catch (err) {
      handle = null;
      const msg = err instanceof Error ? err.message : String(err);
      const denied = /denied|NotAllowed/i.test(msg);
      set_status(denied ? 'microphone permission denied' : `mic error: ${msg}`);
    }
  }

  async function finish(): Promise<void> {
    if (!recording || !handle) return;
    recording = false;
    busy = true;
    set_status('transcribing…');
    const h = handle;
    handle = null;
    try {
      const blob = await h.stop();
      const text = await transcribe(config, blob);
      if (text.trim()) ontranscript(text);
      set_status(null);
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      set_status(`stt failed: ${msg}`);
    } finally {
      busy = false;
    }
  }

  function on_pointerdown(e: PointerEvent): void {
    e.preventDefault();
    void begin();
  }

  function on_release(): void {
    void finish();
  }

  function on_keydown(e: KeyboardEvent): void {
    if (e.key === ' ' || e.key === 'Enter') {
      e.preventDefault();
      if (!recording) void begin();
    }
  }

  function on_keyup(e: KeyboardEvent): void {
    if (e.key === ' ' || e.key === 'Enter') {
      e.preventDefault();
      void finish();
    }
  }
</script>

<button
  type="button"
  class="ptt"
  class:recording
  disabled={disabled || busy}
  aria-label="Hold to record"
  aria-pressed={recording}
  onpointerdown={on_pointerdown}
  onpointerup={on_release}
  onpointerleave={on_release}
  onpointercancel={on_release}
  onkeydown={on_keydown}
  onkeyup={on_keyup}
>
  <span class="dot" aria-hidden="true"></span>
  <span class="sr-only">{recording ? 'Recording' : 'Hold to record'}</span>
</button>

<style>
  .ptt {
    width: 38px;
    height: 38px;
    border-radius: 50%;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: var(--card);
    border: 1px solid var(--border);
    color: var(--fg);
    align-self: flex-end;
    cursor: pointer;
  }
  .ptt:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .ptt .dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--muted);
    display: block;
  }
  .ptt.recording {
    border-color: var(--danger);
    background: rgba(220, 38, 38, 0.12);
  }
  .ptt.recording .dot {
    background: var(--danger);
    animation: pulse 1s ease-in-out infinite;
  }
  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }
</style>
