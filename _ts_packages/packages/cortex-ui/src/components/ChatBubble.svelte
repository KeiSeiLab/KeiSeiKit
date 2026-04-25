<script lang="ts">
  import type { ChatMsg } from '../lib/chat/types';

  interface Props {
    msg: ChatMsg;
    show_sentiment?: boolean;
  }

  const { msg, show_sentiment = false }: Props = $props();
</script>

<!-- text-only render for safety: never use {@html msg.text} — LLM output
     is untrusted and must NOT be parsed as HTML (prompt-injection XSS). -->
<div class="bubble bubble-{msg.role}" class:pending={msg.pending}>
  <div class="bubble-text">{msg.text}{#if msg.pending}<span class="caret" aria-hidden="true">▍</span>{/if}</div>
  {#if show_sentiment && msg.sentiment && msg.role === 'pet'}
    <span class="sentiment-chip" aria-label="sentiment">{msg.sentiment}</span>
  {/if}
</div>

<style>
  .bubble {
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-width: 80%;
    padding: 8px 12px;
    border-radius: 10px;
    border: 1px solid var(--border);
    background: var(--card);
    word-wrap: break-word;
    overflow-wrap: anywhere;
  }
  .bubble-user {
    align-self: flex-end;
    background: var(--accent);
    color: #fff;
    border-color: var(--accent);
  }
  .bubble-pet {
    align-self: flex-start;
  }
  .bubble-text {
    white-space: pre-wrap;
    font-size: 14px;
    line-height: 1.45;
  }
  .caret {
    display: inline-block;
    margin-left: 2px;
    animation: blink 1s step-end infinite;
    color: var(--muted);
  }
  @keyframes blink {
    50% { opacity: 0; }
  }
  .sentiment-chip {
    align-self: flex-start;
    font-size: 11px;
    padding: 1px 8px;
    border-radius: 10px;
    background: var(--bg);
    border: 1px solid var(--border);
    color: var(--muted);
    text-transform: lowercase;
    letter-spacing: 0.02em;
  }
  .pending {
    opacity: 0.95;
  }
</style>
