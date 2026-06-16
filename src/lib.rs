#![warn(clippy::pedantic)]

use worker::{Context, Env, Request, Response, Result, event};

mod auth;
mod handler;
mod phash;

#[event(fetch)]
/// Worker request entrypoint.
///
/// # Errors
/// Returns an error if request handling fails at any stage.
pub async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    handler::handle(req, env, ctx)
        .await
        .or_else(|err| Response::error(format!("{err:?}"), 500))
}
