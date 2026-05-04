// utils.ts — pure helpers for KeiComments. No React, no DOM side-effects.
// Constructor Pattern: one file, one responsibility (data shaping + storage).

export interface Comment {
  id: string;
  page_id: string;
  author: string;
  body: string;
  parent_id?: string;
  created_at: string;
  updated_at: string;
  deleted: boolean;
}

export interface KeiCommentsProps {
  pageId: string;
  apiBase?: string;
}

export interface GroupedComments {
  roots: Comment[];
  childrenByParent: Map<string, Comment[]>;
}

const STORAGE_KEY = "keiwiki.author";

export function timeAgo(iso: string): string {
  const then = new Date(iso).getTime();
  if (Number.isNaN(then)) return "";
  const diffSec = Math.max(0, Math.floor((Date.now() - then) / 1000));
  if (diffSec < 30) return "just now";
  if (diffSec < 60) return `${diffSec}s ago`;
  const diffMin = Math.floor(diffSec / 60);
  if (diffMin < 60) return `${diffMin}m ago`;
  const diffHr = Math.floor(diffMin / 60);
  if (diffHr < 24) return `${diffHr}h ago`;
  const diffDay = Math.floor(diffHr / 24);
  if (diffDay < 30) return `${diffDay}d ago`;
  const diffMon = Math.floor(diffDay / 30);
  if (diffMon < 12) return `${diffMon}mo ago`;
  const diffYr = Math.floor(diffDay / 365);
  return `${diffYr}y ago`;
}

export function escapeHtml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

export function groupReplies(comments: Comment[]): GroupedComments {
  const sorted = [...comments].sort(
    (a, b) =>
      new Date(a.created_at).getTime() - new Date(b.created_at).getTime(),
  );
  const roots: Comment[] = [];
  const childrenByParent = new Map<string, Comment[]>();
  for (const c of sorted) {
    if (c.parent_id) {
      const list = childrenByParent.get(c.parent_id) ?? [];
      list.push(c);
      childrenByParent.set(c.parent_id, list);
    } else {
      roots.push(c);
    }
  }
  return { roots, childrenByParent };
}

export function getStoredAuthor(): string | null {
  if (typeof window === "undefined") return null;
  try {
    const v = window.localStorage.getItem(STORAGE_KEY);
    return v && v.trim().length > 0 ? v : null;
  } catch {
    return null;
  }
}

export function setStoredAuthor(name: string): void {
  if (typeof window === "undefined") return;
  try {
    window.localStorage.setItem(STORAGE_KEY, name.trim());
  } catch {
    // localStorage unavailable — silent no-op
  }
}

export function authorInitial(name: string): string {
  const trimmed = name.trim();
  if (trimmed.length === 0) return "?";
  return trimmed.charAt(0).toUpperCase();
}
