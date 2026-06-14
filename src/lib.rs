#![warn(clippy::pedantic)]

use worker::{Context, Env, Request, Response, Result, event};

mod auth;
mod handler;
mod models;
mod phash;

#[event(fetch)]
/// Worker request entrypoint.
///
/// # Errors
/// Returns an error if request handling fails at any stage.
///
/// # Panics
/// Panics if the error response cannot be created, which should never happen.
pub async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    match handler::handle(req, env, ctx).await {
        Ok(response) => Ok(response),
        Err(err) => {
            let error_response = models::ErrorResponse {
                error: err.to_string(),
                extra: None,
            };

            let response = Response::from_json(&error_response).unwrap_or_else(|_| {
                Response::error("Internal Server Error", 500)
                    .expect("Failed to create error response")
            });
            Ok(response.with_status(500))
        }
    }
}
