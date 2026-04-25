/**
 * FS-list types — minimal contract for the FileTree component.
 *
 * Mirrors what the (yet-to-exist) `GET /api/v1/cortex/fs/list` endpoint is
 * expected to return. Keep in sync with the daemon side once that route is
 * implemented (see `INTEGRATION.md`).
 */

export type FileKind = 'file' | 'dir';

export interface FileEntry {
  /** Basename only (e.g. "src" or "main.ts"), never a full path. */
  name: string;
  kind: FileKind;
  /** Bytes, present for files only. Optional to keep dir entries small. */
  size?: number;
  /** UNIX seconds, daemon-side mtime if available. */
  mtime?: number;
}

export interface ListDirResponse {
  /** Always sorted: dirs first (alpha), then files (alpha). */
  entries: FileEntry[];
}
