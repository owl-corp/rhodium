use worker::{event, Context, Env, Request, Response, Result};

mod auth;
mod handler;
mod phash;

#[event(fetch)]
pub async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let func_resp = handler::handle(req, env, ctx).await;

    if let Err(err) = &func_resp {
        return Response::error(err.to_string(), 500);
    }

    func_resp
}
