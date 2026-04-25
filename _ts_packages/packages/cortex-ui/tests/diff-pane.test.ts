import { describe, it, expect, vi, afterEach } from 'vitest';
import { render, cleanup, fireEvent } from '@testing-library/svelte';

/**
 * `diff` is not yet installed in this workspace (orchestrator owns
 * `pnpm add` per RULE 0.13). Mock it with a tiny line-by-line differ that
 * mimics the chunk shape DiffPane consumes: `{value, added?, removed?}`.
 */
vi.mock('diff', () => ({
  diffLines: (oldStr: string, newStr: string) => {
    const o = oldStr.split('\n');
    const n = newStr.split('\n');
    const out: Array<{ value: string; added?: boolean; removed?: boolean }> = [];
    const max = Math.max(o.length, n.length);
    for (let i = 0; i < max; i++) {
      const ol = o[i];
      const nl = n[i];
      if (ol === nl && ol !== undefined) out.push({ value: `${ol}\n` });
      else {
        if (ol !== undefined) out.push({ value: `${ol}\n`, removed: true });
        if (nl !== undefined) out.push({ value: `${nl}\n`, added: true });
      }
    }
    return out;
  },
}));

import DiffPane from '../src/components/DiffPane.svelte';

afterEach(() => cleanup());

describe('DiffPane', () => {
  it('shows empty-state copy when both sides are empty', () => {
    const { container } = render(DiffPane, {
      props: { oldText: '', newText: '' },
    });
    expect(container.textContent ?? '').toContain('No diff to preview');
  });

  it('renders + / - markers for added and removed lines', () => {
    const { container } = render(DiffPane, {
      props: { oldText: 'one\ntwo\n', newText: 'one\ntwo-edited\n', filename: 'x.ts' },
    });
    const adds = container.querySelectorAll('.diff-add');
    const dels = container.querySelectorAll('.diff-del');
    expect(adds.length).toBeGreaterThanOrEqual(1);
    expect(dels.length).toBeGreaterThanOrEqual(1);
    const added_text = Array.from(adds).map((e) => e.textContent ?? '').join('');
    const removed_text = Array.from(dels).map((e) => e.textContent ?? '').join('');
    expect(added_text).toContain('two-edited');
    expect(removed_text).toContain('two');
  });

  it('fires onApply when Apply clicked', async () => {
    const apply = vi.fn();
    const { container } = render(DiffPane, {
      props: { oldText: 'a\n', newText: 'b\n', onApply: apply },
    });
    const btn = container.querySelector('.diff-apply') as HTMLButtonElement;
    expect(btn).toBeTruthy();
    expect(btn.disabled).toBe(false);
    await fireEvent.click(btn);
    expect(apply).toHaveBeenCalledTimes(1);
  });

  it('truncates past 1000 rows and shows "(N more)"', () => {
    // Build a 1500-line newText vs empty oldText → 1500 added lines.
    const lines: string[] = [];
    for (let i = 0; i < 1500; i++) lines.push(`line-${i}`);
    const big = lines.join('\n') + '\n';
    const { container } = render(DiffPane, {
      props: { oldText: '', newText: big },
    });
    const rows = container.querySelectorAll('.diff-row');
    expect(rows.length).toBe(1000);
    const trunc = container.querySelector('.diff-truncated');
    expect(trunc?.textContent ?? '').toContain('500 more');
  });
});
