use worker::{event, Context, Env, Request, Response, Result};

mod auth;
mod handler;
mod phash;

#[event(fetch)]
pub async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    handler::handle(req, env, ctx).await
}
