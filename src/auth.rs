use worker::{Env, Request, Result};

const KV_BINDING: &str = "rhodium_auth_kv";
const KV_KEY: &str = "api_key";

pub async fn is_authorized(req: &Request, env: &Env) -> Result<bool> {
    let header = match req.headers().get("Authorization")? {
        Some(h) => h,
        None => return Ok(false),
    };

    let token = match header.strip_prefix("Bearer ") {
        Some(t) => t.trim(),
        None => return Ok(false),
    };

    if token.is_empty() {
        return Ok(false);
    }

    let stored = env.kv(KV_BINDING)?.get(KV_KEY).text().await?;
    let stored = match stored {
        Some(s) => s,
        None => return Ok(false),
    };

    Ok(constant_time_eq(token.as_bytes(), stored.as_bytes()))
}

/// Length-aware constant-time byte comparison to avoid leaking the secret via timing.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
