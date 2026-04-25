//! Static HTML for the wizard form.
//!
//! The form no longer POSTs `application/x-www-form-urlencoded` directly
//! — that body-type is SOP-safe (no CORS preflight) which allowed any
//! web page to CSRF POST the handler. Instead, a small inline `<script>`
//! serialises form values to JSON and POSTs via `fetch()` with
//! `Content-Type: application/json`. JSON bodies trigger CORS preflight
//! so the Same-Origin-Policy now engages.

pub const FORM_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>kei-forge</title>
</head>
<body>
<h1>kei-forge — scaffold an atom</h1>
<p>Per <a href="/static/SCHEMA-LOCKED.md">locked substrate schema</a>.</p>
<form id="forge-form">
  <p>
    <label>crate:
      <input name="crate" required pattern="[a-z][a-z0-9-]*" placeholder="kei-task">
    </label>
  </p>
  <p>
    <label>verb:
      <input name="verb" required pattern="[a-z][a-z0-9-]*" placeholder="add-dependency">
    </label>
  </p>
  <p>
    <label>kind:
      <select name="kind" required>
        <option value="command">command</option>
        <option value="query">query</option>
        <option value="stream">stream</option>
        <option value="transform">transform</option>
      </select>
    </label>
  </p>
  <p>
    <label>description:<br>
      <textarea name="description" rows="3" cols="60" maxlength="200"
                placeholder="One-line purpose. Used in atoms/&lt;verb&gt;.md"></textarea>
    </label>
  </p>
  <p><button type="submit">forge atom</button></p>
</form>
<pre id="result"></pre>
<script>
document.getElementById('forge-form').addEventListener('submit', async (e) => {
  e.preventDefault();
  const fd = new FormData(e.target);
  const payload = {
    crate: fd.get('crate'),
    verb: fd.get('verb'),
    kind: fd.get('kind'),
    description: fd.get('description')
  };
  const resp = await fetch('/forge', {
    method: 'POST',
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify(payload)
  });
  const text = await resp.text();
  document.getElementById('result').textContent =
    'HTTP ' + resp.status + '\n' + text;
});
</script>
</body>
</html>
"#;
