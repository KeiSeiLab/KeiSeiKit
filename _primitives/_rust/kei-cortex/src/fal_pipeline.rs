//! fal.ai HTTP pipeline steps — upload, enqueue, poll, download.
//!
//! Each step is a single HTTPS round-trip.  All use the shared `HTTP_CLIENT`
//! from `http_helpers` (no per-request `reqwest::Client::new()`).
//! Response bodies are read through `read_capped` so a runaway upstream
//! cannot allocate unbounded memory.

use crate::fal::{Error, Style};
use crate::fal::{BODY_PREVIEW_CAP, ERROR_BODY_CAP, IMAGE_BODY_CAP};
use crate::fal_ssrf::validate_fal_url;
use crate::http_helpers::{read_capped, CappedReadError, HTTP_CLIENT};

pub(crate) const UPLOAD_INITIATE_URL: &str =
    "https://rest.alpha.fal.ai/storage/upload/initiate";
pub(crate) const GENERATION_URL: &str =
    "https://queue.fal.run/fal-ai/flux-pro/v1.1-ultra";

/// Step 1 — ask fal storage for a signed PUT URL, then PUT the image to it.
pub(crate) async fn upload_image(key: &str, bytes: &[u8]) -> Result<String, Error> {
    let body =
        serde_json::json!({ "file_name": "portrait.png", "content_type": "image/png" });
    let resp = HTTP_CLIENT
        .post(UPLOAD_INITIATE_URL)
        .header("Authorization", format!("Key {key}"))
        .json(&body)
        .send()
        .await?;
    let json = decode_json(resp).await?;
    let upload_url = json
        .get("upload_url")
        .and_then(|v| v.as_str())
        .ok_or(Error::BadShape("upload_url"))?;
    let file_url = json
        .get("file_url")
        .and_then(|v| v.as_str())
        .ok_or(Error::BadShape("file_url"))?;
    validate_fal_url(upload_url)?;
    validate_fal_url(file_url)?;
    HTTP_CLIENT
        .put(upload_url)
        .header("Content-Type", "image/png")
        .body(bytes.to_vec())
        .send()
        .await?
        .error_for_status()
        .map_err(Error::Http)?;
    Ok(file_url.to_string())
}

/// Step 2 — POST the generation request, return the validated status URL.
pub(crate) async fn enqueue(
    key: &str,
    image_url: &str,
    style: Style,
) -> Result<String, Error> {
    let body = serde_json::json!({
        "image_url": image_url,
        "prompt": style.prompt(),
        "strength": 0.65,
        "enable_safety_checker": true,
    });
    let resp = HTTP_CLIENT
        .post(GENERATION_URL)
        .header("Authorization", format!("Key {key}"))
        .json(&body)
        .send()
        .await?;
    let json = decode_json(resp).await?;
    let status_url = json
        .get("status_url")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or(Error::BadShape("status_url"))?;
    validate_fal_url(&status_url)?;
    Ok(status_url)
}

/// Step 3 — poll `status_url` until `COMPLETED`, or give up at `deadline`.
pub(crate) async fn poll_until_done(
    key: &str,
    status_url: &str,
    deadline: tokio::time::Instant,
    poll_interval: std::time::Duration,
) -> Result<String, Error> {
    loop {
        if tokio::time::Instant::now() >= deadline {
            return Err(Error::Timeout);
        }
        let resp = HTTP_CLIENT
            .get(status_url)
            .header("Authorization", format!("Key {key}"))
            .send()
            .await?;
        let json = decode_json(resp).await?;
        match json.get("status").and_then(|v| v.as_str()).unwrap_or("") {
            "COMPLETED" => return extract_first_image_url(&json),
            s @ ("FAILED" | "ERROR") => {
                return Err(Error::BadStatus(502, format!("fal reported {s}")));
            }
            _ => {}
        }
        tokio::time::sleep(poll_interval).await;
    }
}

/// Step 4 — download the PNG bytes from a validated fal CDN URL.
pub(crate) async fn download_image(key: &str, url: &str) -> Result<Vec<u8>, Error> {
    validate_fal_url(url)?;
    let resp = HTTP_CLIENT
        .get(url)
        .header("Authorization", format!("Key {key}"))
        .send()
        .await?;
    if !resp.status().is_success() {
        return Err(Error::BadStatus(resp.status().as_u16(), "download failed".into()));
    }
    match read_capped(resp, IMAGE_BODY_CAP).await {
        Ok(b) => Ok(b),
        Err(CappedReadError::TooLarge) => {
            Err(Error::BadStatus(502, "image body exceeded 64 MiB cap".into()))
        }
        Err(CappedReadError::Http(e)) => Err(Error::Http(e)),
    }
}

/// Dig out `images[0].url` from the completed status payload, then validate.
pub(crate) fn extract_first_image_url(json: &serde_json::Value) -> Result<String, Error> {
    let url = json
        .get("images")
        .and_then(|a| a.as_array())
        .and_then(|a| a.first())
        .and_then(|o| o.get("url"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or(Error::BadShape("images[0].url"))?;
    validate_fal_url(&url)?;
    Ok(url)
}

/// Decode a fal response into JSON, capping error/success bodies.
pub(crate) async fn decode_json(
    resp: reqwest::Response,
) -> Result<serde_json::Value, Error> {
    let status = resp.status();
    if !status.is_success() {
        let raw = read_capped(resp, ERROR_BODY_CAP).await.unwrap_or_default();
        let body = String::from_utf8_lossy(&raw).into_owned();
        return Err(Error::BadStatus(
            status.as_u16(),
            truncate(&body, BODY_PREVIEW_CAP),
        ));
    }
    let raw = match read_capped(resp, IMAGE_BODY_CAP).await {
        Ok(b) => b,
        Err(CappedReadError::TooLarge) => {
            return Err(Error::BadStatus(502, "response body exceeded 64 MiB cap".into()));
        }
        Err(CappedReadError::Http(e)) => return Err(Error::Http(e)),
    };
    serde_json::from_slice(&raw).map_err(|e| Error::BadStatus(502, e.to_string()))
}

/// Cap a string at `max` bytes on a char boundary.
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}
