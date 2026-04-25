<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { load_config, type CortexConfig } from './lib/config';
  import {
    load_sidebar, save_sidebar,
    load_terminal, save_terminal,
    load_cwd,
    type ToggleState,
  } from './lib/layout-prefs';
  import Setup from './routes/Setup.svelte';
  import Dashboard from './routes/Dashboard.svelte';
  import PetEditor from './routes/PetEditor.svelte';
  import LedgerStream from './routes/LedgerStream.svelte';
  import MemorySearch from './routes/MemorySearch.svelte';
  import BudgetStrip from './components/BudgetStrip.svelte';
  import FileTree from './components/FileTree.svelte';
  import TerminalPane from './components/TerminalPane.svelte';

  let hash = $state(window.location.hash || '#/');
  let cfg = $state<CortexConfig | null>(null);
  let sidebar = $state<ToggleState>(load_sidebar());
  let terminal = $state<ToggleState>(load_terminal());
  let cwd = $state<string>(load_cwd());
  let current_path = $state<string | undefined>(undefined);

  function on_hash(): void {
    hash = window.location.hash || '#/';
  }

  onMount(() => {
    cfg = load_config();
    window.addEventListener('hashchange', on_hash);
  });

  onDestroy(() => {
    window.removeEventListener('hashchange', on_hash);
  });

  const route = $derived.by(() => {
    const h = hash.replace(/^#\/?/, '');
    const [path, ...rest] = h.split('/');
    return { path: path ?? '', arg: rest.join('/') };
  });

  function toggle_sidebar(): void {
    sidebar = sidebar === 'open' ? 'closed' : 'open';
    save_sidebar(sidebar);
  }

  function toggle_terminal(): void {
    terminal = terminal === 'open' ? 'closed' : 'open';
    save_terminal(terminal);
  }

  function on_select_file(p: string): void {
    current_path = p;
  }
</script>

<header class="app-header">
  <div class="header-left">
    <button
      type="button"
      class="toggle-button"
      aria-pressed={sidebar === 'open'}
      aria-label="Toggle sidebar"
      onclick={toggle_sidebar}
      data-testid="sidebar-toggle"
    >☰</button>
    <h1>Cortex UI</h1>
    {#if cfg}
      <nav class="nav" role="navigation" aria-label="Main">
        <a href="#/">Dashboard</a>
        <a href="#/ledger">Ledger</a>
        <a href="#/memory">Memory</a>
        <a href="#/setup">Setup</a>
      </nav>
    {/if}
  </div>
  <div class="header-right">
    {#if cfg}
      <BudgetStrip config={cfg} />
    {/if}
    <button
      type="button"
      class="toggle-button"
      aria-pressed={terminal === 'open'}
      aria-label="Toggle terminal"
      onclick={toggle_terminal}
      data-testid="terminal-toggle"
    >▣</button>
  </div>
</header>

<div class="app-body">
  {#if cfg && sidebar === 'open'}
    <aside class="sidebar" aria-label="File browser" data-testid="sidebar">
      <FileTree
        config={cfg}
        root={cwd}
        currentPath={current_path ?? ''}
        onSelect={on_select_file}
      />
    </aside>
  {/if}

  <main class="main-pane" data-testid="main-pane">
    {#if !cfg}
      <Setup on_saved={() => (cfg = load_config())} />
    {:else if route.path === 'setup'}
      <Setup on_saved={() => (cfg = load_config())} />
    {:else if route.path === 'pet'}
      <PetEditor config={cfg} user_id={route.arg} />
    {:else if route.path === 'ledger'}
      <LedgerStream config={cfg} />
    {:else if route.path === 'memory'}
      <MemorySearch config={cfg} />
    {:else}
      <Dashboard config={cfg} />
    {/if}
  </main>
</div>

{#if cfg && terminal === 'open'}
  <section class="bottom-dock" aria-label="Terminal dock" data-testid="bottom-dock">
    <TerminalPane
      cwd={cwd}
      daemon_url={cfg.daemon_url}
      token={cfg.token}
    />
  </section>
{/if}
