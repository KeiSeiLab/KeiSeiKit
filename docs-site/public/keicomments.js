// KeiComments — sovereign comment widget (vanilla JS, no React, no MDX).
// Mounts on <div id="keicomments-mount">, fetches kei-cortex
// /api/v1/cortex/comments/{page_id}, renders threaded list + post form.
// Page ID = window.location.pathname (one-to-one with the wiki page).

(function () {
  const mount = document.getElementById('keicomments-mount');
  if (!mount) return;

  const API_BASE =
    (window.KEISEI_CORTEX_URL || 'http://127.0.0.1:18080') +
    '/api/v1/cortex/comments';
  const TOKEN_KEY = 'keisei_cortex_token';
  const AUTHOR_KEY = 'keicomments_author';
  const pageId = encodeURIComponent(window.location.pathname);

  function getToken() {
    return (
      localStorage.getItem(TOKEN_KEY) ||
      window.KEISEI_CORTEX_TOKEN ||
      ''
    );
  }
  function getAuthor() {
    return localStorage.getItem(AUTHOR_KEY) || 'anonymous';
  }
  function setAuthor(v) {
    localStorage.setItem(AUTHOR_KEY, v);
  }

  function h(tag, attrs, ...children) {
    const el = document.createElement(tag);
    for (const k in attrs || {}) {
      if (k.startsWith('on')) {
        el.addEventListener(k.slice(2).toLowerCase(), attrs[k]);
      } else if (k === 'class') {
        el.className = attrs[k];
      } else {
        el.setAttribute(k, attrs[k]);
      }
    }
    for (const c of children) {
      if (c == null) continue;
      el.appendChild(typeof c === 'string' ? document.createTextNode(c) : c);
    }
    return el;
  }

  async function api(method, path, body) {
    const headers = { 'content-type': 'application/json' };
    const tok = getToken();
    if (tok) headers.authorization = `Bearer ${tok}`;
    const res = await fetch(`${API_BASE}${path}`, {
      method,
      headers,
      body: body ? JSON.stringify(body) : undefined,
    });
    if (!res.ok) throw new Error(`${method} ${path}: ${res.status}`);
    return res.json();
  }

  function renderTree(comments) {
    const byParent = new Map();
    for (const c of comments) {
      const k = c.parent_id || '';
      if (!byParent.has(k)) byParent.set(k, []);
      byParent.get(k).push(c);
    }
    function node(c, depth) {
      const kids = byParent.get(c.id) || [];
      return h(
        'div',
        { class: 'keic-comment', style: `margin-left:${depth * 20}px` },
        h('div', { class: 'keic-meta' },
          h('strong', null, c.author),
          ' · ',
          h('span', { class: 'keic-time' }, new Date(c.ts).toLocaleString())
        ),
        h('div', { class: 'keic-body' }, c.body),
        ...kids.map((kid) => node(kid, depth + 1))
      );
    }
    return (byParent.get('') || []).map((c) => node(c, 0));
  }

  async function refresh() {
    list.replaceChildren(h('div', { class: 'keic-loading' }, 'Loading…'));
    try {
      const data = await api('GET', `/by-page/${pageId}`);
      const tree = renderTree(data.comments || []);
      list.replaceChildren(
        ...(tree.length ? tree : [h('div', { class: 'keic-empty' }, 'No comments yet.')])
      );
    } catch (e) {
      list.replaceChildren(
        h('div', { class: 'keic-error' },
          'Comments unavailable (cortex offline or token missing). ',
          h('br'),
          h('small', null, String(e))
        )
      );
    }
  }

  async function submit() {
    const body = textarea.value.trim();
    const author = authorInput.value.trim() || 'anonymous';
    if (!body) return;
    setAuthor(author);
    btn.disabled = true;
    try {
      await api('POST', `/by-page/${pageId}`, { author, body });
      textarea.value = '';
      await refresh();
    } catch (e) {
      alert('Post failed: ' + e.message);
    } finally {
      btn.disabled = false;
    }
  }

  // Layout
  const list = h('div', { class: 'keic-list' });
  const textarea = h('textarea', {
    rows: '3',
    placeholder: 'Add a comment…',
    class: 'keic-textarea',
  });
  const authorInput = h('input', {
    type: 'text',
    placeholder: 'name',
    value: getAuthor(),
    class: 'keic-author',
  });
  const btn = h('button', { class: 'keic-submit', onclick: submit }, 'Post');

  mount.replaceChildren(
    list,
    h('div', { class: 'keic-form' },
      authorInput,
      textarea,
      btn
    )
  );

  // Inject minimal styles once
  if (!document.getElementById('keic-styles')) {
    const css = `
.keic-comment { padding: 8px 0; border-top: 1px solid var(--sl-color-hairline, #e5e7eb); }
.keic-comment:first-child { border-top: 0; }
.keic-meta { font-size: 0.85em; opacity: 0.7; margin-bottom: 4px; }
.keic-body { white-space: pre-wrap; }
.keic-form { margin-top: 12px; display: flex; gap: 8px; flex-wrap: wrap; align-items: flex-start; }
.keic-author { width: 140px; padding: 4px 6px; }
.keic-textarea { flex: 1 1 280px; min-width: 240px; padding: 6px; font-family: inherit; }
.keic-submit { padding: 6px 14px; cursor: pointer; }
.keic-submit:disabled { opacity: 0.5; cursor: wait; }
.keic-loading, .keic-empty, .keic-error { padding: 8px 0; opacity: 0.7; font-size: 0.9em; }
.keic-error { color: var(--sl-color-text-accent, #c1440e); }
`;
    const tag = h('style', { id: 'keic-styles' });
    tag.textContent = css;
    document.head.appendChild(tag);
  }

  refresh();
})();
