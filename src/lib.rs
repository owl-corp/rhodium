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
    let func_resp = handler::handle(req, env, ctx).await;

    if let Err(err) = &func_resp {
        return Response::error(err.to_string(), 500);
    }

    func_resp
}
