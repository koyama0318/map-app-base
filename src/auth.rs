use axum::body::Body;
use axum::{http::Request, middleware::Next, response::Response};

pub async fn jwt_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    Ok(next.run(req).await)
}
