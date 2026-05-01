# Daytona REST API Verification (2026-04-30)

Source: https://raw.githubusercontent.com/daytonaio/daytona/main/libs/api-client-go/api/openapi.yaml
Spec format: OpenAPI 3.0.0
Total endpoints in spec: 201 paths (grep `^  /` count)
SHA-256: 81a1e3f6af322fc03975edcfe3dfc36eb050cf5870598ba968667fbb3fb4f07d

## client.rs path coverage

| client.rs call | spec path | match | notes |
|---|---|---|---|
| GET `/sandboxes/{name}` | GET `/sandbox/{sandboxIdOrName}` | ❌ plural vs singular; param name differs | spec uses `/sandbox` (singular), param is `sandboxIdOrName` |
| GET `/sandboxes` | GET `/sandbox` | ❌ plural vs singular | spec uses `/sandbox` (singular) |
| POST `/sandboxes` | POST `/sandbox` | ❌ plural vs singular | spec uses `/sandbox` (singular) |
| POST `/sandboxes/{name}/start` | POST `/sandbox/{sandboxIdOrName}/start` | ❌ plural vs singular; param name differs | spec uses `/sandbox` (singular) |
| POST `/sandboxes/{name}/stop` | POST `/sandbox/{sandboxIdOrName}/stop` | ❌ plural vs singular; param name differs | spec uses `/sandbox` (singular) |
| DELETE `/sandboxes/{name}` | DELETE `/sandbox/{sandboxIdOrName}` | ❌ plural vs singular; param name differs | spec uses `/sandbox` (singular) |
| POST `/sandboxes/{name}/exec` | POST `/toolbox/{sandboxId}/toolbox/process/execute` | ❌ completely wrong path | no `/sandbox/*/exec` endpoint exists; exec is under `/toolbox/{sandboxId}/toolbox/process/execute` (deprecated) and `/toolbox/{sandboxId}/toolbox/process/session/{sessionId}/exec` |
| PUT `/sandboxes/{name}/files/{path}` | POST `/toolbox/{sandboxId}/toolbox/files/upload` | ❌ wrong prefix, method, structure | upload is POST not PUT; path is query param not path segment; endpoint is under `/toolbox/` prefix; no path-segment file addressing |
| GET `/sandboxes/{name}/files/{path}` | GET `/toolbox/{sandboxId}/toolbox/files/download` | ❌ wrong prefix and structure | download uses `?path=<remote_path>` query param, not path segment; endpoint is under `/toolbox/` prefix |

## Mismatches summary

1. **Global prefix wrong**: all `client.rs` CRUD calls use `/sandboxes` (plural) but the spec uses `/sandbox` (singular) throughout.
2. **Path parameter name wrong**: client uses `{name}` but spec uses `{sandboxIdOrName}` — functionally the same since Daytona accepts both ID and name, but the URL template is wrong.
3. **exec endpoint completely wrong**: `POST /sandboxes/{name}/exec` does not exist in the spec. The actual exec endpoint is `POST /toolbox/{sandboxId}/toolbox/process/execute` (deprecated) which requires a different base URL (toolbox proxy URL, not the management API base). This is an architectural difference — exec goes via the toolbox API, not the management API.
4. **File upload/download wrong**: `PUT /sandboxes/{name}/files/{path}` and `GET /sandboxes/{name}/files/{path}` do not exist in the spec. The spec has `POST /toolbox/{sandboxId}/toolbox/files/upload` and `GET /toolbox/{sandboxId}/toolbox/files/download` under the toolbox API (different base URL), with `path` as a **query parameter** not a path segment.

## Conclusion

7 of 9 client.rs calls have wrong paths (5 require only the `sandboxes` → `sandbox` rename; 2 require full architectural rework because exec and file operations use a separate toolbox API endpoint, not the management API).

## Suggested patches

### Patch A — Global rename: `/sandboxes` → `/sandbox` (safe, mechanical)

Applies to: `get_sandbox`, `list_sandboxes`, `create_sandbox`, `start_sandbox`, `stop_sandbox`, `delete_sandbox`.

```diff
-        let url = format!("{}/sandboxes/{}", self.base_url, name);
+        let url = format!("{}/sandbox/{}", self.base_url, name);

-        let url = format!("{}/sandboxes", self.base_url);
+        let url = format!("{}/sandbox", self.base_url);

-        let url = format!("{}/sandboxes/{}/start", self.base_url, name);
+        let url = format!("{}/sandbox/{}/start", self.base_url, name);

-        let url = format!("{}/sandboxes/{}/stop", self.base_url, name);
+        let url = format!("{}/sandbox/{}/stop", self.base_url, name);
```

`delete_sandbox` uses the same `format!("{}/sandboxes/{}", ...)` pattern — same rename applies.

### Patch B — exec and file operations (architectural, needs orchestrator decision)

`exec`, `upload_file`, `download_file` use the management API base URL but the spec puts these operations under the **toolbox API** (a different proxy URL obtained from `GET /sandbox/{sandboxId}/toolbox-proxy-url`). The client needs a second base URL or a factory method to build toolbox URLs. This cannot be a 1-line patch.

Orchestrator should decide: either (a) add a `toolbox_base_url` field to `DaytonaClient`, (b) have the caller pre-resolve the toolbox URL and pass it, or (c) implement `get_toolbox_proxy_url` first and have exec/files call it lazily.

**Do NOT auto-apply Patch B** without architecture review.

### Patch C — exec body field (if Patch B is applied)

The spec's `ExecuteRequest` schema should be verified; the current client sends `{ "command": cmd }`. The actual field name in the spec's `ExecuteRequest` schema should be confirmed before wiring.
