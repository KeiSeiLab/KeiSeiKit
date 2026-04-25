<script lang="ts">
  /**
   * FileTree — single-level lazy expand directory browser.
   *
   * Backend: `GET /api/v1/cortex/fs/list?path=<path>` — NOT YET IMPLEMENTED on
   * the daemon (see `INTEGRATION.md`). Until the route lands `listDir` returns
   * an empty array on 404 so the tree degrades to "expanded but empty".
   *
   * Noise filter: never list `node_modules`, `.git`, `target`, `dist`,
   * `_archive`, `.svelte-kit`, `.cache` regardless of what the daemon returns.
   *
   * A11y: `role="tree"` root + `role="treeitem"` rows + `aria-expanded` on
   * each dir. Keyboard: Enter / Space toggles a dir, Enter selects a file.
   */
  import { SvelteMap } from 'svelte/reactivity';
  import { listDir } from '../lib/fs/list-client';
  import type { CortexConfig } from '../lib/config';
  import type { FileEntry } from '../lib/fs/types';

  interface Props {
    config: CortexConfig;
    root: string;
    onSelect: (path: string) => void;
    currentPath?: string;
  }

  const { config, root, onSelect, currentPath }: Props = $props();

  const NOISE = new Set([
    'node_modules', '.git', 'target', 'dist', '_archive', '.svelte-kit', '.cache',
  ]);

  type NodeState = {
    entries: FileEntry[] | null; // null = not yet loaded
    expanded: boolean;
    loading: boolean;
    error: string | null;
  };

  // SvelteMap is reactive on .set/.delete; plain Map inside $state is NOT.
  const nodes = new SvelteMap<string, NodeState>();

  function path_join(parent: string, child: string): string {
    if (parent === '' || parent === '/') return `/${child}`;
    return `${parent.replace(/\/+$/, '')}/${child}`;
  }

  function visible(es: FileEntry[]): FileEntry[] {
    return es.filter((e) => !NOISE.has(e.name));
  }

  async function load_path(path: string): Promise<void> {
    const cur = nodes.get(path) ?? blank();
    nodes.set(path, { ...cur, loading: true, error: null });
    try {
      const entries = await listDir(config, path);
      nodes.set(path, { entries: visible(entries), expanded: true, loading: false, error: null });
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      nodes.set(path, { ...blank(), error: msg });
    }
  }

  function blank(): NodeState {
    return { entries: null, expanded: false, loading: false, error: null };
  }

  async function toggle_dir(path: string): Promise<void> {
    const cur = nodes.get(path);
    if (cur?.expanded) {
      nodes.set(path, { ...cur, expanded: false });
      return;
    }
    if (cur?.entries) {
      nodes.set(path, { ...cur, expanded: true });
      return;
    }
    await load_path(path);
  }

  function on_file_click(path: string): void {
    onSelect(path);
  }

  function on_key(ev: KeyboardEvent, action: () => void): void {
    if (ev.key === 'Enter' || ev.key === ' ') {
      ev.preventDefault();
      action();
    }
  }

  // Auto-expand the root on mount.
  $effect(() => {
    if (!nodes.has(root)) void load_path(root);
  });

  const root_state = $derived(nodes.get(root) ?? blank());
</script>

<nav class="file-tree" role="tree" aria-label="Project file tree">
  <header class="ft-header">
    <span class="ft-root" title={root}>{root}</span>
  </header>
  {#if root_state.loading && !root_state.entries}
    <p class="muted ft-loading">Loading…</p>
  {:else if root_state.error}
    <p class="error ft-error" role="alert">{root_state.error}</p>
  {:else if root_state.entries && root_state.entries.length === 0}
    <p class="muted ft-empty">(empty)</p>
  {:else if root_state.entries}
    <ul class="ft-list">
      {#each root_state.entries as e (e.name)}
        {@const child = path_join(root, e.name)}
        {@const c_state = nodes.get(child)}
        <li>
          {#if e.kind === 'dir'}
            <span
              role="treeitem"
              tabindex="0"
              class="ft-row ft-dir"
              aria-expanded={c_state?.expanded ?? false}
              onclick={() => toggle_dir(child)}
              onkeydown={(ev) => on_key(ev, () => toggle_dir(child))}
            >
              <span class="ft-icon" aria-hidden="true">{c_state?.expanded ? '▾' : '▸'}</span>
              <span class="ft-name">{e.name}/</span>
            </span>
            {#if c_state?.expanded && c_state.entries}
              <ul class="ft-list ft-nested">
                {#each c_state.entries as ce (ce.name)}
                  {@const cpath = path_join(child, ce.name)}
                  <li>
                    <span
                      role="treeitem"
                      tabindex="0"
                      class="ft-row ft-{ce.kind}"
                      class:ft-current={cpath === currentPath}
                      onclick={() =>
                        ce.kind === 'dir' ? toggle_dir(cpath) : on_file_click(cpath)}
                      onkeydown={(ev) =>
                        on_key(ev, () =>
                          ce.kind === 'dir' ? toggle_dir(cpath) : on_file_click(cpath))}
                    >
                      <span class="ft-icon" aria-hidden="true">{ce.kind === 'dir' ? '▸' : '·'}</span>
                      <span class="ft-name">{ce.name}{ce.kind === 'dir' ? '/' : ''}</span>
                    </span>
                  </li>
                {/each}
              </ul>
            {/if}
          {:else}
            <span
              role="treeitem"
              tabindex="0"
              class="ft-row ft-file"
              class:ft-current={child === currentPath}
              onclick={() => on_file_click(child)}
              onkeydown={(ev) => on_key(ev, () => on_file_click(child))}
            >
              <span class="ft-icon" aria-hidden="true">·</span>
              <span class="ft-name">{e.name}</span>
            </span>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
</nav>

<style>
  .file-tree {
    border: 1px solid var(--border); border-radius: 10px;
    background: var(--card); padding: 8px 4px;
    min-width: 200px; max-width: 360px;
    max-height: 520px; overflow-y: auto;
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
    font-size: 12.5px;
  }
  .ft-header {
    padding: 0 8px 6px; border-bottom: 1px solid var(--border);
    margin-bottom: 6px; color: var(--muted);
  }
  .ft-root { font-weight: 600; color: var(--fg); }
  .ft-list { list-style: none; margin: 0; padding: 0; }
  .ft-nested { padding-left: 16px; }
  .ft-row {
    display: flex; align-items: center; gap: 6px;
    padding: 2px 8px; cursor: pointer; border-radius: 4px;
    color: var(--fg);
  }
  .ft-row:hover { background: var(--bg); }
  .ft-row:focus { outline: 2px solid var(--accent); outline-offset: -2px; }
  .ft-icon { width: 1em; color: var(--muted); }
  .ft-name { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .ft-current { background: var(--accent); color: #fff; }
  .ft-current .ft-icon { color: #fff; }
  .ft-loading, .ft-empty { padding: 6px 8px; }
  .ft-error { margin: 6px 8px; }
</style>
