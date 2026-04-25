<script lang="ts">
  /**
   * DiffPane — unified-style diff preview, GitHub-flavoured.
   *
   * Deps: `diff` (npm) `^7` — current verified line at install time. Pin the
   * exact resolved version in `package.json` after `pnpm add diff@latest`.
   *
   * Render: `Diff.diffLines(oldText, newText)` chunks split into lines, emitted
   * as added (+), removed (-), or context. Truncated past TRUNCATE_AFTER lines
   * with a "(N more)" footer to keep the DOM bounded on huge diffs.
   *
   * A11y: `role="region"` with `aria-label`. Apply / Reject are real buttons.
   */
  import { diffLines } from 'diff';

  interface Props {
    oldText: string;
    newText: string;
    filename?: string;
    onApply?: () => void;
    onReject?: () => void;
  }

  const { oldText, newText, filename, onApply, onReject }: Props = $props();

  const TRUNCATE_AFTER = 1000;
  type RowKind = 'add' | 'del' | 'ctx';
  type Row = { kind: RowKind; text: string };

  function chunk_to_rows(value: string, added?: boolean, removed?: boolean): Row[] {
    const kind: RowKind = added ? 'add' : removed ? 'del' : 'ctx';
    const lines = value.split('\n');
    if (lines.length > 0 && lines[lines.length - 1] === '') lines.pop();
    return lines.map((text) => ({ kind, text }));
  }

  function build_rows(o: string, n: string): { rows: Row[]; hidden: number } {
    if (o === '' && n === '') return { rows: [], hidden: 0 };
    const all: Row[] = [];
    for (const c of diffLines(o, n)) all.push(...chunk_to_rows(c.value, c.added, c.removed));
    if (all.length <= TRUNCATE_AFTER) return { rows: all, hidden: 0 };
    return { rows: all.slice(0, TRUNCATE_AFTER), hidden: all.length - TRUNCATE_AFTER };
  }

  const built = $derived(build_rows(oldText, newText));
  const rows = $derived(built.rows);
  const hidden = $derived(built.hidden);
  const is_empty = $derived(oldText === '' && newText === '');

  function marker(k: RowKind): string {
    return k === 'add' ? '+' : k === 'del' ? '-' : ' ';
  }
</script>

<section class="diff-pane" role="region" aria-label="Diff preview">
  <header class="diff-header">
    <span class="diff-filename" aria-label="filename">{filename ?? '(unnamed)'}</span>
    <span class="diff-actions">
      <button type="button" class="diff-btn diff-apply"
        aria-label="Apply diff" disabled={is_empty} onclick={() => onApply?.()}>Apply</button>
      <button type="button" class="diff-btn secondary diff-reject"
        aria-label="Reject diff" disabled={is_empty} onclick={() => onReject?.()}>Reject</button>
    </span>
  </header>
  <div class="diff-body" aria-live="polite">
    {#if is_empty}
      <p class="muted diff-empty">No diff to preview</p>
    {:else}
      <pre class="diff-table" aria-label="diff lines">{#each rows as r, i (i)}<span class="diff-row diff-{r.kind}" data-kind={r.kind} role="row"><span class="diff-marker" aria-hidden="true">{marker(r.kind)}</span><span class="diff-text">{r.text}</span>
</span>{/each}{#if hidden > 0}<span class="diff-truncated muted">... ({hidden} more)</span>{/if}</pre>
    {/if}
  </div>
</section>

<style>
  .diff-pane {
    display: flex; flex-direction: column; width: 100%;
    border: 1px solid var(--border); border-radius: 10px;
    background: var(--card); overflow: hidden;
    min-height: 160px; max-height: 520px;
  }
  .diff-header {
    display: flex; align-items: center; justify-content: space-between; gap: 8px;
    padding: 8px 12px; border-bottom: 1px solid var(--border); background: var(--bg);
  }
  .diff-filename {
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
    font-size: 13px; color: var(--fg);
    overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
  }
  .diff-actions { display: flex; gap: 6px; flex-shrink: 0; }
  .diff-btn {
    padding: 4px 10px; font-size: 12px; border-radius: 4px;
    border: 1px solid var(--accent); background: var(--accent); color: #fff; cursor: pointer;
  }
  .diff-btn.secondary {
    background: var(--card); color: var(--fg); border-color: var(--border);
  }
  .diff-btn:disabled { opacity: 0.5; cursor: not-allowed; }
  .diff-body { flex: 1; overflow: auto; padding: 0; background: var(--bg); }
  .diff-empty { text-align: center; margin: 24px auto; }
  .diff-table {
    margin: 0; padding: 8px 0; background: transparent; border: none; border-radius: 0;
    font-size: 12.5px; line-height: 1.45;
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  }
  .diff-row { display: block; padding: 0 12px; }
  .diff-add { background: rgba(46, 160, 67, 0.18); color: var(--fg); }
  .diff-del { background: rgba(248, 81, 73, 0.18); color: var(--fg); }
  .diff-ctx { color: var(--muted); }
  .diff-marker { display: inline-block; width: 1.2em; color: var(--muted); }
  .diff-add .diff-marker { color: #2ea043; }
  .diff-del .diff-marker { color: var(--danger); }
  .diff-text { white-space: pre; }
  .diff-truncated {
    display: block; padding: 6px 12px; font-size: 12px;
    border-top: 1px dashed var(--border);
  }
</style>
