//! fal.ai Flux 2 Pro client — stylize a portrait into an anime frame.
//!
//! The public surface is a single async function `stylize(source_png, style)`.
//! Internally we: (1) upload the source image to fal storage via the
//! two-step `/storage/upload/initiate` handshake, (2) POST a generation
//! request to the queue endpoint, (3) poll status until `COMPLETED`, (4)
//! download the first image in the result. The whole pipeline has a 60-second
//! wall-clock budget, past which we return `Error::Timeout` so the handler
//! can surface an HTTP 504.
//!
//! `FAL_KEY` is read from the environment on every call; the daemon does not
//! cache it because the user may rotate it without restarting.

use std::time::Duration;

/// Style preset — drives the prompt and nothing else.
#[derive(Debug, Clone, Copy)]
pub enum Style {
    Cute,
    Cool,
    Studious,
}

impl Style {
    /// Parse the wire-level string. Unknown values fall back to `Cute`.
    pub fn from_wire(s: &str) -> Self {
        match s {
            "anime-cool" => Self::Cool,
            "anime-studious" => Self::Studious,
            _ => Self::Cute,
        }
    }

    fn prompt(self) -> &'static str {
        match self {
            Self::Cute => CUTE_PROMPT,
            Self::Cool => COOL_PROMPT,
            Self::Studious => STUDIOUS_PROMPT,
        }
    }
}

const CUTE_PROMPT: &str = "soft pastel anime character, round expressive eyes, gentle smile, kawaii aesthetic, warm pink and peach tones, clean line art, centered head-and-shoulders portrait, plain neutral background, facing forward";
const COOL_PROMPT: &str = "anime character with sharp angular features, cool blue and slate color palette, confident calm expression, stylish modern outfit, crisp line art, centered head-and-shoulders portrait, plain neutral background, facing forward";
const STUDIOUS_PROMPT: &str = "anime character wearing round glasses, neat hair, studious calm expression, academic sweater or blazer, soft even lighting, muted color palette, centered head-and-shoulders portrait, plain neutral background, facing forward";

/// Errors surfaced to the caller; handlers map them onto HTTP codes.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("FAL_KEY environment variable not set")]
    NoApiKey,
    #[error("fal http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("fal returned status {0}: {1}")]
    BadStatus(u16, String),
    #[error("fal response missing field {0}")]
    BadShape(&'static str),
    #[error("fal polling exceeded 60s budget")]
    Timeout,
}

const UPLOAD_INITIATE_URL: &str = "https://rest.alpha.fal.ai/storage/upload/initiate";
const GENERATION_URL: &str = "https://queue.fal.run/fal-ai/flux-pro/v1.1-ultra";
const BUDGET: Duration = Duration::from_secs(60);
const POLL_INTERVAL: Duration = Duration::from_millis(800);
const BODY_PREVIEW_CAP: usize = 512;

/// Stylize `source_png` into an anime portrait. Returns the raw PNG bytes
/// of the generated image (caller writes them to disk).
pub async fn stylize(source_png: &[u8], style: Style) -> Result<Vec<u8>, Error> {
    let key = std::env::var("FAL_KEY").map_err(|_| Error::NoApiKey)?;
    let client = reqwest::Client::new();
    let deadline = tokio::time::Instant::now() + BUDGET;
    let uploaded_url = upload_image(&client, &key, source_png).await?;
    let status_url = enqueue(&client, &key, &uploaded_url, style).await?;
    let result_url = poll_until_done(&client, &key, &status_url, deadline).await?;
    download_image(&client, &key, &result_url).await
}

/// Step 1 — ask fal storage for a signed PUT URL, then PUT the image to it.
async fn upload_image(client: &reqwest::Client, key: &str, bytes: &[u8]) -> Result<String, Error> {
    let body = serde_json::json!({ "file_name": "portrait.png", "content_type": "image/png" });
    let resp = client
        .post(UPLOAD_INITIATE_URL)
        .header("Authorization", format!("Key {key}"))
        .json(&body)
        .send()
        .await?;
    let json = decode_json(resp).await?;
    let upload_url = json.get("upload_url").and_then(|v| v.as_str()).ok_or(Error::BadShape("upload_url"))?;
    let file_url = json.get("file_url").and_then(|v| v.as_str()).ok_or(Error::BadShape("file_url"))?;
    let put = client.put(upload_url).header("Content-Type", "image/png").body(bytes.to_vec()).send().await?;
    if !put.status().is_success() {
        return Err(Error::BadStatus(put.status().as_u16(), "PUT upload failed".into()));
    }
    Ok(file_url.to_string())
}

/// Step 2 — POST the generation request, return the status poll URL.
async fn enqueue(client: &reqwest::Client, key: &str, image_url: &str, style: Style) -> Result<String, Error> {
    let body = serde_json::json!({
        "image_url": image_url,
        "prompt": style.prompt(),
        "strength": 0.65,
        "enable_safety_checker": true,
    });
    let resp = client
        .post(GENERATION_URL)
        .header("Authorization", format!("Key {key}"))
        .json(&body)
        .send()
        .await?;
    let json = decode_json(resp).await?;
    json.get("status_url")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or(Error::BadShape("status_url"))
}

/// Step 3 — poll the status URL until `COMPLETED`, or give up at deadline.
async fn poll_until_done(client: &reqwest::Client, key: &str, status_url: &str, deadline: tokio::time::Instant) -> Result<String, Error> {
    loop {
        if tokio::time::Instant::now() >= deadline {
            return Err(Error::Timeout);
        }
        let resp = client.get(status_url).header("Authorization", format!("Key {key}")).send().await?;
        let json = decode_json(resp).await?;
        let status = json.get("status").and_then(|v| v.as_str()).unwrap_or("");
        if status == "COMPLETED" {
            return extract_first_image_url(&json);
        }
        if status == "FAILED" || status == "ERROR" {
            return Err(Error::BadStatus(502, format!("fal reported {status}")));
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

/// Step 4 — download the PNG bytes from the fal CDN URL.
async fn download_image(client: &reqwest::Client, key: &str, url: &str) -> Result<Vec<u8>, Error> {
    let resp = client.get(url).header("Authorization", format!("Key {key}")).send().await?;
    if !resp.status().is_success() {
        return Err(Error::BadStatus(resp.status().as_u16(), "download failed".into()));
    }
    Ok(resp.bytes().await?.to_vec())
}

/// Dig out `images[0].url` from the completed status payload.
fn extract_first_image_url(json: &serde_json::Value) -> Result<String, Error> {
    json.get("images")
        .and_then(|a| a.as_array())
        .and_then(|a| a.first())
        .and_then(|o| o.get("url"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or(Error::BadShape("images[0].url"))
}

/// Decode a fal JSON response, turning non-2xx into `BadStatus` with body.
/// Body capped at `BODY_PREVIEW_CAP` so a large upstream error page cannot
/// propagate through our logs or error channel.
async fn decode_json(resp: reqwest::Response) -> Result<serde_json::Value, Error> {
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(Error::BadStatus(
            status.as_u16(),
            truncate(&body, BODY_PREVIEW_CAP),
        ));
    }
    Ok(resp.json::<serde_json::Value>().await?)
}

/// Cap a string at `max` bytes on a char boundary. Keeps fal error previews
/// bounded regardless of what upstream sent back.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_caps_long_strings() {
        let long = "a".repeat(10_000);
        assert_eq!(truncate(&long, 256).len(), 256);
    }

    #[test]
    fn truncate_leaves_short_strings() {
        assert_eq!(truncate("hi", 256), "hi");
    }

    #[test]
    fn style_from_wire_defaults_to_cute() {
        assert!(matches!(Style::from_wire("wat"), Style::Cute));
    }
}
