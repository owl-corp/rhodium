use worker::{Context, Env, Fetch, Headers, Method, Request, Response, Result, Url};

const MAX_IMAGE_BYTES: u64 = 20 * 1024 * 1024; // 20 MiB

pub async fn handle(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if !matches!(req.method(), Method::Post) {
        return Response::error("Method Not Allowed", 405);
    }

    if !crate::auth::is_authorized(&req, &env).await? {
        return Response::error("Unauthorized", 401);
    }

    let url = req.text().await?;
    let url = url.trim().to_string();

    if url.is_empty() {
        return Response::error("Bad Request: empty URL", 400);
    }

    let bytes = fetch_image_bytes(&url).await?;
    let (hex, signed) = compute_phash(&bytes)?;

    let body = format!(r#"{{"hex":"{}","i64":{}}}"#, hex, signed);

    let headers = Headers::new();
    headers.set("Content-Type", "application/json")?;

    Ok(Response::ok(body)?.with_headers(headers))
}

async fn fetch_image_bytes(url: &str) -> Result<Vec<u8>> {
    // media.discordapp.net blocks cloudflare worker IPs
    // cdn.discordapp.com doesn't for some reason.
    let rewritten = url.replace("media.discordapp.net", "cdn.discordapp.com");
    let target = rewritten.as_str();

    let parsed: Url = target
        .parse()
        .map_err(|_| worker::Error::RustError("Invalid URL".into()))?;

    let mut image_response = Fetch::Url(parsed).send().await?;

    if image_response.status_code() < 200 || image_response.status_code() >= 300 {
        return Err(worker::Error::RustError(format!(
            "Failed to fetch image: HTTP {}",
            image_response.status_code()
        )));
    }

    if let Some(len) = image_response.headers().get("content-length")?
        && let Ok(n) = len.parse::<u64>()
        && n > MAX_IMAGE_BYTES
    {
        return Err(worker::Error::RustError("Image too large".into()));
    }

    image_response.bytes().await
}

fn compute_phash(bytes: &[u8]) -> Result<(String, i64)> {
    let img = crate::phash::decode_image(bytes)
        .ok_or_else(|| worker::Error::RustError("Failed to decode image".into()))?;

    let hash = crate::phash::phash(&img);

    let hex = hash.iter().map(|b| format!("{:02x}", b)).collect();
    let signed = i64::from_be_bytes(hash);

    Ok((hex, signed))
}
