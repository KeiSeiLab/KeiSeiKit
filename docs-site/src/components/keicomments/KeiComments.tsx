// KeiComments.tsx — sovereign React comment widget.
// Backend: kei-cortex daemon /api/v1/cortex/comments/{page_id}
// Constructor Pattern: 1 component = 1 widget. Helpers below stay local; sub-fns ≤30 LOC each.
import { useCallback, useEffect, useMemo, useState } from "react";
import type { CSSProperties } from "react";
import {
  authorInitial, escapeHtml, getStoredAuthor, groupReplies,
  setStoredAuthor, timeAgo,
  type Comment, type KeiCommentsProps,
} from "./utils";

const DEFAULT_API_BASE = "/api/v1/cortex/comments";
const REACTIONS = ["👍", "❤️", "🚀"] as const;

const S: Record<string, CSSProperties> = {
  section: { marginTop: "3rem", paddingTop: "1.5rem" },
  count: { color: "var(--sl-color-gray-3)", fontWeight: 400 },
  err: { background: "rgba(248,113,113,0.1)", color: "#f87171", padding: "0.5rem 0.75rem", borderRadius: 4, marginBottom: "1rem" },
  muted: { color: "var(--sl-color-gray-3)" },
  rowBase: { paddingTop: "0.75rem", borderTop: "1px solid var(--sl-color-gray-5)", display: "flex", gap: "0.75rem" },
  avatar: { flex: "0 0 32px", height: 32, width: 32, borderRadius: "50%", background: "var(--sl-color-accent-low)", color: "var(--sl-color-accent-high)", display: "grid", placeItems: "center", fontWeight: 600, fontSize: "0.875rem" },
  meta: { fontSize: "0.875rem", color: "var(--sl-color-gray-3)" },
  body: { marginTop: "0.25rem", lineHeight: 1.55 },
  actions: { display: "flex", gap: "0.5rem", marginTop: "0.5rem", fontSize: "0.875rem" },
  btn: { background: "none", border: "none", color: "var(--sl-color-accent)", cursor: "pointer", padding: "0.125rem 0.25rem", font: "inherit" },
  btnDanger: { background: "none", border: "none", color: "var(--sl-color-red, #f87171)", cursor: "pointer", padding: "0.125rem 0.25rem", font: "inherit" },
  composer: { marginTop: "1.5rem" },
  nameRow: { display: "flex", gap: "0.5rem", alignItems: "center" },
  input: { flex: 1, padding: "0.5rem", borderRadius: 4, border: "1px solid var(--sl-color-gray-5)", background: "var(--sl-color-bg)", color: "var(--sl-color-white)" },
  primary: { padding: "0.5rem 1rem", borderRadius: 4, border: "1px solid var(--sl-color-accent)", background: "var(--sl-color-accent)", color: "var(--sl-color-white)", cursor: "pointer" },
  textarea: { width: "100%", padding: "0.5rem", borderRadius: 4, border: "1px solid var(--sl-color-gray-5)", background: "var(--sl-color-bg)", color: "var(--sl-color-white)", fontFamily: "inherit", resize: "vertical" },
  submitRow: { display: "flex", gap: "0.5rem", marginTop: "0.5rem", justifyContent: "flex-end" },
};

function renderBody(body: string): string {
  return escapeHtml(body).replace(/\n/g, "<br/>");
}

interface RowProps {
  comment: Comment;
  mine: boolean;
  depth: number;
  onReply: (id: string) => void;
  onDelete: (id: string) => void;
  onReact: (id: string, emoji: string) => void;
}

function CommentRow({ comment, mine, depth, onReply, onDelete, onReact }: RowProps) {
  const row = { ...S.rowBase, marginLeft: depth > 0 ? "2.25rem" : 0 };
  if (comment.deleted) {
    return (
      <div style={row}>
        <div style={S.avatar}>·</div>
        <div style={{ flex: 1, fontStyle: "italic", color: "var(--sl-color-gray-3)" }}>[deleted]</div>
      </div>
    );
  }
  return (
    <div style={row}>
      <div style={S.avatar}>{authorInitial(comment.author)}</div>
      <div style={{ flex: 1, minWidth: 0 }}>
        <div style={S.meta}>
          <strong style={{ color: "var(--sl-color-white)" }}>{comment.author}</strong>
          {" · "}{timeAgo(comment.created_at)}
        </div>
        <div style={S.body} dangerouslySetInnerHTML={{ __html: renderBody(comment.body) }} />
        <div style={S.actions}>
          <button type="button" onClick={() => onReply(comment.id)} style={S.btn}>🔁 Reply</button>
          {REACTIONS.map((e) => (
            <button key={e} type="button" onClick={() => onReact(comment.id, e)} style={S.btn} aria-label={`react ${e}`}>{e}</button>
          ))}
          {mine && <button type="button" onClick={() => onDelete(comment.id)} style={S.btnDanger}>🗑 Delete</button>}
        </div>
      </div>
    </div>
  );
}

export default function KeiComments({ pageId, apiBase = DEFAULT_API_BASE }: KeiCommentsProps) {
  const [comments, setComments] = useState<Comment[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [draft, setDraft] = useState<string>("");
  const [replyTo, setReplyTo] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [author, setAuthor] = useState<string | null>(() => getStoredAuthor());
  const [nameDraft, setNameDraft] = useState<string>("");
  const [submitting, setSubmitting] = useState<boolean>(false);

  const url = `${apiBase}/${encodeURIComponent(pageId)}`;

  const reload = useCallback(async () => {
    setLoading(true);
    try {
      const res = await fetch(url, { headers: { accept: "application/json" } });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = (await res.json()) as { comments: Comment[] };
      setComments(Array.isArray(data.comments) ? data.comments : []);
      setError(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : "failed to load comments");
    } finally {
      setLoading(false);
    }
  }, [url]);

  useEffect(() => { void reload(); }, [reload]);

  const grouped = useMemo(() => groupReplies(comments), [comments]);
  const visibleCount = comments.filter((c) => !c.deleted).length;

  const onSubmit = async () => {
    if (!author || draft.trim().length === 0) return;
    setSubmitting(true);
    try {
      const res = await fetch(url, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ author, body: draft, parent_id: replyTo ?? undefined }),
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      setDraft(""); setReplyTo(null); await reload();
    } catch (e) {
      setError(e instanceof Error ? e.message : "failed to post");
    } finally { setSubmitting(false); }
  };

  const onDelete = async (id: string) => {
    if (!author || !window.confirm("Delete this comment?")) return;
    try {
      const res = await fetch(`${apiBase}/${encodeURIComponent(id)}`, {
        method: "DELETE",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ author }),
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      await reload();
    } catch (e) {
      setError(e instanceof Error ? e.message : "failed to delete");
    }
  };

  const onReact = async (id: string, emoji: string) => {
    if (!author) return;
    try {
      await fetch(`${apiBase}/${encodeURIComponent(id)}/react`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ author, emoji }),
      });
    } catch { /* best-effort */ }
  };

  const saveName = () => {
    const v = nameDraft.trim();
    if (v.length === 0) return;
    setStoredAuthor(v); setAuthor(v);
  };

  const rows = grouped.roots.length === 0 ? null : (
    <div>
      {grouped.roots.map((root) => (
        <div key={root.id}>
          <CommentRow comment={root} mine={author === root.author} depth={0} onReply={setReplyTo} onDelete={onDelete} onReact={onReact} />
          {(grouped.childrenByParent.get(root.id) ?? []).map((child) => (
            <CommentRow key={child.id} comment={child} mine={author === child.author} depth={1} onReply={setReplyTo} onDelete={onDelete} onReact={onReact} />
          ))}
        </div>
      ))}
    </div>
  );

  const lineCount = Math.max(3, Math.min(12, draft.split("\n").length + 1));
  const submitDisabled = submitting || draft.trim().length === 0;

  return (
    <section className="not-content keicomments" style={S.section}>
      <hr />
      <h3 style={{ marginBottom: "0.5rem" }}>
        Discussion {visibleCount > 0 && <span style={S.count}>· {visibleCount}</span>}
      </h3>
      {error && <div style={S.err}>⚠ {error}</div>}
      {loading
        ? <p style={S.muted}>Loading…</p>
        : visibleCount === 0
          ? <p style={S.muted}>Be the first to comment.</p>
          : rows}
      <div style={S.composer}>
        {!author ? (
          <div style={S.nameRow}>
            <input type="text" placeholder="Pick a display name" value={nameDraft}
              onChange={(e) => setNameDraft(e.target.value)} style={S.input} />
            <button type="button" onClick={saveName} style={S.primary}>Save</button>
          </div>
        ) : (
          <div>
            {replyTo && (
              <div style={{ ...S.meta, marginBottom: "0.25rem" }}>
                Replying… <button type="button" onClick={() => setReplyTo(null)} style={S.btn}>cancel</button>
              </div>
            )}
            <textarea value={draft} onChange={(e) => setDraft(e.target.value)} rows={lineCount}
              placeholder={`Comment as ${author}…`} style={S.textarea} />
            <div style={S.submitRow}>
              <button type="button" onClick={onSubmit} disabled={submitDisabled}
                style={{ ...S.primary, cursor: submitting ? "wait" : "pointer", opacity: submitDisabled ? 0.6 : 1 }}>
                {submitting ? "Posting…" : "Post"}
              </button>
            </div>
          </div>
        )}
      </div>
    </section>
  );
}
