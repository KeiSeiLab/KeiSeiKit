<script lang="ts">
  /**
   * TerminalPane — xterm.js wrapper, WebSocket-bridged to the daemon.
   *
   * Deps: `xterm` (npm) `^5`, `xterm-addon-fit` (npm) `^0.10` — current
   * verified lines at install time. Pin exact versions via
   * `pnpm add xterm@latest xterm-addon-fit@latest`.
   *
   * Backend: `WS /api/v1/cortex/term?cwd=<cwd>` — NOT YET IMPLEMENTED on the
   * daemon (see `INTEGRATION.md`). Until the route lands the pane runs in
   * read-only mode: terminal renders, fit addon resizes, but no input is sent
   * upstream and a one-line "(read-only — no daemon endpoint)" footer shows.
   *
   * A11y: outer wrapper is `role="region"` with `aria-label="Terminal"`. The
   * xterm canvas itself manages focus internally.
   */
  import { onMount, onDestroy } from 'svelte';
  import { Terminal } from 'xterm';
  import { FitAddon } from 'xterm-addon-fit';

  interface Props {
    cwd: string;
    /** Daemon base URL — used to derive the WS endpoint. */
    daemon_url: string;
    /** Bearer token sent as the WS subprotocol. */
    token: string;
    onCommand?: (cmd: string) => void;
  }

  const { cwd, daemon_url, token, onCommand }: Props = $props();

  let host_el = $state<HTMLDivElement | null>(null);
  let term: Terminal | null = null;
  let fit: FitAddon | null = null;
  let ws: WebSocket | null = null;
  let ws_open = $state(false);
  let ws_unavailable = $state(false);
  let line_buf = '';
  let ro_observer: ResizeObserver | null = null;

  function ws_url(): string {
    const base = daemon_url.replace(/^http/, 'ws');
    return `${base}/api/v1/cortex/term?cwd=${encodeURIComponent(cwd)}`;
  }

  function on_local_data(data: string): void {
    if (ws_open && ws) {
      ws.send(data);
      return;
    }
    // Fallback: line-buffer and emit completed lines via onCommand.
    for (const ch of data) {
      if (ch === '\r' || ch === '\n') {
        if (line_buf) onCommand?.(line_buf);
        line_buf = '';
        term?.write('\r\n');
      } else if (ch === '\x7f' || ch === '\b') {
        if (line_buf) {
          line_buf = line_buf.slice(0, -1);
          term?.write('\b \b');
        }
      } else {
        line_buf += ch;
        term?.write(ch);
      }
    }
  }

  function open_socket(): void {
    let sock: WebSocket;
    try {
      sock = new WebSocket(ws_url(), ['bearer', token]);
    } catch {
      ws_unavailable = true;
      return;
    }
    ws = sock;
    sock.onopen = () => { ws_open = true; };
    sock.onmessage = (ev) => {
      const d = ev.data;
      if (typeof d === 'string') term?.write(d);
    };
    sock.onerror = () => { ws_unavailable = true; };
    sock.onclose = () => { ws_open = false; };
  }

  function fit_safe(): void {
    try { fit?.fit(); } catch { /* host not yet sized */ }
  }

  onMount(() => {
    if (!host_el) return;
    term = new Terminal({
      convertEol: true,
      fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Consolas, monospace',
      fontSize: 12.5,
      theme: { background: '#0e0e11', foreground: '#e8e8ea' },
    });
    fit = new FitAddon();
    term.loadAddon(fit);
    term.open(host_el);
    fit_safe();
    term.onData(on_local_data);
    if (typeof ResizeObserver !== 'undefined') {
      ro_observer = new ResizeObserver(() => fit_safe());
      ro_observer.observe(host_el);
    }
    open_socket();
  });

  onDestroy(() => {
    ro_observer?.disconnect();
    ro_observer = null;
    try { ws?.close(); } catch { /* already closing */ }
    ws = null;
    term?.dispose();
    term = null;
    fit = null;
  });
</script>

<section class="term-pane" role="region" aria-label="Terminal">
  <header class="term-header">
    <span class="term-cwd" title={cwd}>{cwd}</span>
    <span class="term-status muted">
      {#if ws_open}connected
      {:else if ws_unavailable}read-only — no daemon endpoint
      {:else}connecting…
      {/if}
    </span>
  </header>
  <div bind:this={host_el} class="term-host" aria-label="Terminal output"></div>
</section>

<style>
  .term-pane {
    display: flex; flex-direction: column;
    width: 100%;
    border: 1px solid var(--border); border-radius: 10px;
    background: var(--card); overflow: hidden;
    min-height: 220px; max-height: 520px;
  }
  .term-header {
    display: flex; align-items: center; justify-content: space-between;
    gap: 8px; padding: 6px 10px;
    background: var(--bg); border-bottom: 1px solid var(--border);
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
    font-size: 12px;
  }
  .term-cwd {
    color: var(--fg); font-weight: 500;
    overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
  }
  .term-status { font-size: 11px; }
  .term-host {
    flex: 1; min-height: 180px;
    padding: 6px;
    background: #0e0e11;
  }
</style>
