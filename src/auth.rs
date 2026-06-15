use worker::{Env, Request, Result};

const SECRET_BINDING: &str = "RHODIUM_API_KEY";

pub fn is_authorized(req: &Request, env: &Env) -> Result<bool> {
    let Some(header) = req.headers().get("Authorization")? else {
        return Ok(false);
    };

    let Some(token) = header.strip_prefix("Bearer ") else {
        return Ok(false);
    };
    let token = token.trim();

    if token.is_empty() {
        return Ok(false);
    }

    let stored = env.secret(SECRET_BINDING)?.to_string();

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
