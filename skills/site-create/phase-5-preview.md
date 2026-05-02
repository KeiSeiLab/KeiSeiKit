# Phase 5 — Preview Deploy

> Goal: deploy the site to a temporary preview URL so the user can inspect
> it in a real browser before committing to production.
> **Verify criterion:** preview URL is live and accessible; user has
> confirmed the site looks correct at the URL.

---

## 5.a — Run preview deploy

Invoke the preview deploy target:

```
/web-deploy --env preview
```

The deploy skill returns a preview URL (e.g. `https://preview-<hash>.vercel.app` or
a Netlify/CF Pages preview link depending on the project's deployment target).

If the project has no preview environment configured, build locally and start
a local server:

```bash
npm run build && npm run preview
# or: npx astro preview
```

In that case the "preview URL" is `http://localhost:4321` (or the port reported).

---

## 5.b — One AskUserQuestion: proceed to production?

Present the preview URL and ask:

- **Deploy to production** — site looks correct, proceed to Phase 6.
- **Return to audit** — found issues; loop back to Phase 4.
- **Return to sections** — a section needs rework; loop back to Phase 3.
- **Abort** — cancel the pipeline; site stays at preview only.

---

## 5.c — Verify criterion

- Preview deploy completed without error.
- User has visited the URL (or local server) and approved.
- No outstanding `mock-render status` drift rows.

Emit:
`Phase 5 done: preview live at <url>. Proceeding to production deploy.`

Proceed to Phase 6.
