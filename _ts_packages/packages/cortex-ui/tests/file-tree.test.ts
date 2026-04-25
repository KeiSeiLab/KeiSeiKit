import { describe, it, expect, vi, afterEach, beforeEach } from 'vitest';
import { render, cleanup, fireEvent, waitFor } from '@testing-library/svelte';
import type { FileEntry } from '../src/lib/fs/types';

const list_calls: string[] = [];
const stub: Record<string, FileEntry[]> = {};

vi.mock('../src/lib/fs/list-client', () => ({
  listDir: async (_c: unknown, path: string) => {
    list_calls.push(path);
    return stub[path] ?? [];
  },
}));

import FileTree from '../src/components/FileTree.svelte';

afterEach(() => cleanup());

beforeEach(() => {
  list_calls.length = 0;
  for (const k of Object.keys(stub)) delete stub[k];
});

describe('FileTree', () => {
  it('hides noise dirs (node_modules, .git, target, …)', async () => {
    stub['/'] = [
      { name: 'src', kind: 'dir' },
      { name: 'node_modules', kind: 'dir' },
      { name: '.git', kind: 'dir' },
      { name: 'target', kind: 'dir' },
      { name: 'README.md', kind: 'file' },
    ];
    const { container } = render(FileTree, {
      props: {
        config: { daemon_url: 'http://x', token: 't' },
        root: '/',
        onSelect: () => {},
      },
    });
    await waitFor(() => {
      expect(container.querySelectorAll('.ft-dir, .ft-file').length).toBeGreaterThan(0);
    });
    const text = container.textContent ?? '';
    expect(text).toContain('src/');
    expect(text).toContain('README.md');
    expect(text).not.toContain('node_modules');
    expect(text).not.toContain('.git');
    expect(text).not.toContain('target/');
  });

  it('expands a directory on click and lists its children', async () => {
    stub['/'] = [{ name: 'src', kind: 'dir' }];
    stub['/src'] = [
      { name: 'main.ts', kind: 'file', size: 10 },
      { name: 'lib', kind: 'dir' },
    ];
    const { container } = render(FileTree, {
      props: {
        config: { daemon_url: 'http://x', token: 't' },
        root: '/',
        onSelect: () => {},
      },
    });
    await waitFor(() => {
      expect(container.querySelector('.ft-dir')).toBeTruthy();
    });
    const dir_row = container.querySelector('.ft-dir') as HTMLElement;
    await fireEvent.click(dir_row);
    await waitFor(() => {
      expect(list_calls).toContain('/src');
    });
    await waitFor(() => {
      expect(container.textContent ?? '').toContain('main.ts');
    });
    expect(container.textContent ?? '').toContain('lib/');
  });

  it('marks the currentPath row with the ft-current class', async () => {
    stub['/'] = [
      { name: 'a.txt', kind: 'file' },
      { name: 'b.txt', kind: 'file' },
    ];
    const { container } = render(FileTree, {
      props: {
        config: { daemon_url: 'http://x', token: 't' },
        root: '/',
        onSelect: () => {},
        currentPath: '/a.txt',
      },
    });
    await waitFor(() => {
      expect(container.querySelectorAll('.ft-file').length).toBe(2);
    });
    const current = container.querySelectorAll('.ft-current');
    expect(current.length).toBe(1);
    expect((current[0].textContent ?? '').includes('a.txt')).toBe(true);
  });
});
