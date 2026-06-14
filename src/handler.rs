use worker::{Context, Env, Fetch, Method, Request, Response, Result, Url};

const MAX_IMAGE_BYTES: u64 = 20 * 1024 * 1024; // 20 MiB
const HEX: &[u8; 16] = b"0123456789abcdef";

pub async fn handle(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if req.path() != "/" {
        return Response::error("Not Found", 404);
    }

    if !matches!(req.method(), Method::Post) {
        return json_error(405, "Method Not Allowed");
    }

    if !crate::auth::is_authorized(&req, &env)? {
        return json_error(401, "Unauthorized");
    }

    let hash_request = match parse_hash_request(&mut req).await {
        Ok(hash_request) => hash_request,
        Err(err) => return json_error(400, &format!("Bad Request: {err}")),
    };
    let url = hash_request.url.trim().to_string();

    if url.is_empty() {
        return json_error(400, "no URL or body provided");
    }

    let bytes = fetch_image_bytes(&url).await?;
    let (hex, signed) = compute_phash(&bytes)?;
    let response = crate::models::HashResponse { hex, i64: signed };

    Response::from_json(&response)
}

async fn parse_hash_request(req: &mut Request) -> Result<crate::models::HashRequest> {
    let content_type = req
        .headers()
        .get("content-type")?
        .unwrap_or_default()
        .to_ascii_lowercase();

    if content_type.starts_with("application/json") {
        return req.json::<crate::models::HashRequest>().await;
    }

    let body = req.text().await?;
    Ok(crate::models::HashRequest { url: body })
}

fn json_error(status: u16, message: &str) -> Result<Response> {
    let body = crate::models::ErrorResponse {
        error: message.to_string(),
        extra: None,
    };

    let resp = Response::from_json(&body)?;
    Ok(resp.with_status(status))
}

async fn fetch_image_bytes(url: &str) -> Result<Vec<u8>> {
    // media.discordapp.net blocks cloudflare worker IPs
    // cdn.discordapp.com doesn't for some reason.
    let rewritten = url.replace("media.discordapp.net", "cdn.discordapp.com");
    let target = rewritten.as_str();

    let parsed = target
        .parse::<Url>()
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

    let mut hex = String::with_capacity(hash.len() * 2);
    for &b in &hash {
        hex.push(char::from(HEX[usize::from(b >> 4)]));
        hex.push(char::from(HEX[usize::from(b & 0x0f)]));
    }
    let signed = i64::from_be_bytes(hash);

    Ok((hex, signed))
}
