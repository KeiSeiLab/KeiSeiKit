# Phase 6 — Production Deploy

> Goal: deploy the approved site to production and verify it is live.
> **Verify criterion:** production URL responds 200; deploy commit recorded;
> user notified with live URL and summary report.

---

## 6.a — Run production deploy

Invoke the production deploy target:

```
/web-deploy --env production
```

The deploy skill handles:
- Final build (`npm run build`)
- Upload / push to the configured hosting target (Vercel, Netlify, CF Pages,
  S3+CDN, VPS via rsync, etc.)
- DNS propagation check (waits up to 60 s for the apex domain to resolve)

---

## 6.b — Post-deploy smoke test

After the deploy reports success, verify the live URL:

1. HTTP HEAD on the production URL → expect `200 OK`.
2. Check `<title>` tag matches the project name from Phase 0.
3. If a sitemap was generated in Phase 4, verify `<domain>/sitemap.xml` returns 200.

If any check fails: report to user; do NOT mark phase complete.

---

## 6.c — One AskUserQuestion: final confirmation

Present the live URL and smoke-test results. Ask:

- **Done — mark pipeline complete** — proceed to exit report.
- **Issue found — rollback** — invoke `/web-deploy --rollback` to the
  previous production snapshot.

---

## 6.d — Exit report

Emit the pipeline completion summary:

```
=== SITE-CREATE PIPELINE COMPLETE ===
Project:   <project-name>
Live URL:  <production-url>
Phases:    0-6 all complete
Sections:  <N> locked (mock-render)
Audits:    <a11y / seo / responsive / perf findings applied>
Deploy:    <hosting target> — <deploy ID or commit SHA>
```

---

## 6.e — Verify criterion

- Production URL live and smoke-tested (200 OK, correct `<title>`).
- Deploy commit or deploy ID recorded in exit report.
- User confirmed pipeline complete.

Pipeline ends here.
