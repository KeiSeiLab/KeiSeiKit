<script lang="ts">
  import PttButton from './PttButton.svelte';
  import type { CortexConfig } from '../lib/config';

  interface Props {
    disabled?: boolean;
    placeholder?: string;
    onsubmit: (text: string) => void;
    /** When provided, show a push-to-talk mic button that streams STT
     *  results back into the textarea. Hidden if omitted. */
    config?: CortexConfig;
    /** Optional status relay to parent (e.g. "recording…" / error). The
     *  parent owns the visible row — ChatInput no longer renders one to
     *  avoid duplicating ChatPanel's speaking / typing status. */
    onstatus?: (status: string | null) => void;
  }

  const { disabled = false, placeholder = 'Type a message…', onsubmit, config, onstatus }:
    Props = $props();

  let draft = $state('');

  function try_send(): void {
    const text = draft.trim();
    if (!text || disabled) return;
    onsubmit(text);
    draft = '';
  }

  function on_keydown(e: KeyboardEvent): void {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      try_send();
    }
  }

  /** Strip Unicode control chars (category C) — STT results occasionally
   *  contain stray U+FFFC / U+200B / U+0000 etc. that break the textarea
   *  caret or clipboard. */
  function sanitize_transcript(t: string): string {
    return t.replace(/[\p{C}]/gu, '');
  }

  function on_transcript(text: string): void {
    const clean = sanitize_transcript(text);
    if (!clean) return;
    draft = draft.length === 0 ? clean : `${draft} ${clean}`;
  }
</script>

<div class="chat-input">
  <label class="sr-only" for="chat-textarea">Chat message</label>
  <textarea
    id="chat-textarea"
    bind:value={draft}
    onkeydown={on_keydown}
    {disabled}
    {placeholder}
    rows="2"
    aria-label="Chat message"
  ></textarea>
  {#if config}
    <PttButton
      {config}
      disabled={disabled}
      ontranscript={on_transcript}
      onstatus={(s) => onstatus?.(s)}
    />
  {/if}
  <button
    type="button"
    onclick={try_send}
    disabled={disabled || draft.trim().length === 0}
    aria-label="Send message"
  >Send</button>
</div>

<style>
  .chat-input {
    display: flex;
    gap: 8px;
    align-items: stretch;
    padding: 8px;
    border-top: 1px solid var(--border);
    background: var(--bg);
  }
  textarea {
    flex: 1;
    min-height: 38px;
    max-height: 120px;
    resize: vertical;
    font-size: 14px;
  }
  button {
    padding: 0 16px;
    align-self: flex-end;
  }
  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
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
